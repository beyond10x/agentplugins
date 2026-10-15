//! The session-start check. Quick: it reads what the hosts recorded on disk, runs each installed
//! product's `--version`, and compares with the newest releases recorded in
//! `~/.local/state/b10x/latest.json`. That record is refreshed from GitHub at most once a day,
//! within a few seconds; offline, the old record stands. It prints one line per problem, nothing
//! when all is well, and never fails the session.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::Path;

use crate::catalog::Catalog;
use crate::inventory::{copies_on_path, home};
use crate::pins::{self, Pin, Pins, Spec};
use crate::{resolve, version};

/// The releases the skills were last verified against: the repository's `verified.json`.
const VERIFIED: &str = include_str!("../../../verified.json");

/// How often the newest-release record is refreshed from GitHub.
const REFRESH_EVERY: u64 = 86_400;

/// How long one refresh may take, in seconds; the repositories are read in parallel.
const REFRESH_SECONDS: u64 = 3;

/// CLI → the release its skills were last verified against.
#[must_use]
pub fn verified() -> BTreeMap<String, String> {
    serde_json::from_str(VERIFIED).unwrap_or_default()
}

/// Refresh the newest-release record when the last try is a day old: the marketplace repository
/// and every catalog binary's, each within [`REFRESH_SECONDS`]. Failures leave the record as it was.
pub fn refresh(home: &Path, catalog: &Catalog, now: u64) {
    let record = resolve::recorded(home);
    let last = ["checked_at", "attempted_at"]
        .iter()
        .filter_map(|key| record.get(*key).and_then(serde_json::Value::as_u64))
        .max();
    if last.is_some_and(|last| now.saturating_sub(last) < REFRESH_EVERY) {
        return;
    }
    let mut repositories = BTreeSet::from([catalog.marketplace.repository.clone()]);
    for product in &catalog.products {
        for binary in &product.binaries {
            repositories.insert(binary.install.repository().to_owned());
        }
    }
    let tags = resolve::latest_tags(&repositories, REFRESH_SECONDS);
    resolve::record(home, &tags, now);
}

/// A recorded install: `name`, `marketplace`, version.
pub type Recorded = (String, String, Option<String>);

/// Finds the first copy of a binary on `PATH`: its path and version.
pub type Probe<'a> = &'a dyn Fn(&str) -> Option<(String, Option<String>)>;

/// How old the last check of newest releases may be before the check says so.
const STALE_DAYS: u64 = 7;

/// Installed plugins as the hosts recorded them.
#[must_use]
pub fn recorded(home: &Path) -> Vec<Recorded> {
    let mut plugins = Vec::new();
    if let Ok(text) = std::fs::read_to_string(home.join(".claude/plugins/installed_plugins.json")) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(map) = value.get("plugins").and_then(serde_json::Value::as_object) {
                for (id, installs) in map {
                    let Some((name, marketplace)) = id.rsplit_once('@') else {
                        continue;
                    };
                    let version = installs
                        .as_array()
                        .and_then(|list| list.first())
                        .and_then(|install| install.get("version"))
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned);
                    plugins.push((name.to_owned(), marketplace.to_owned(), version));
                }
            }
        }
    }
    plugins
}

/// What the last plan saw: newest release per repository, and when (Unix seconds).
#[must_use]
pub fn latest(home: &Path) -> Option<(u64, serde_json::Map<String, serde_json::Value>)> {
    let text = std::fs::read_to_string(crate::resolve::cache_path(home)).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    let at = value.get("checked_at")?.as_u64()?;
    let map = value.get("latest")?.as_object()?.clone();
    Some((at, map))
}

/// What the check compares against besides the machine: the pins and the releases the skills were
/// verified against.
pub struct Expected<'a> {
    /// The pins that apply in the session's directory: the local pin file over the repository's.
    pub pins: &'a Pins,
    /// CLI → the release the skills were verified against.
    pub verified: &'a BTreeMap<String, String>,
}

/// The lines to print.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn lines(
    catalog: &Catalog,
    plugins: &[Recorded],
    binary_version: Probe<'_>,
    last: Option<(u64, &serde_json::Map<String, serde_json::Value>)>,
    now: u64,
    expected: &Expected<'_>,
) -> Vec<String> {
    let mut lines = Vec::new();
    let name = catalog.marketplace.name.as_str();
    let newest_of = |repository: &str| {
        last.and_then(|(_, map)| map.get(repository))
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
    };
    let oldest_plugin = plugins
        .iter()
        .filter(|(_, marketplace, _)| marketplace == name)
        .filter_map(|(_, _, found)| found.as_deref())
        .filter(|found| version::key(found).is_some())
        .min_by_key(|found| version::key(found));
    if let (Some(have), Some(newest)) = (oldest_plugin, newest_of(&catalog.marketplace.repository))
    {
        if version::key(have) < version::key(&newest) {
            lines.push(format!(
                "b10x: the Beyond10x plugins ({have}) are older than the newest release {newest}; /b10x:upgrade updates them."
            ));
        }
    }
    let pinned = |binary: &str| expected.pins.get(binary);
    for (plugin, marketplace, _) in plugins {
        if catalog.retired_marketplace(marketplace) || catalog.retired_plugins.contains_key(plugin)
        {
            lines.push(format!(
                "b10x: an earlier install, `{plugin}@{marketplace}`, is still here; /b10x:upgrade replaces it."
            ));
        }
    }
    let installed: Vec<&str> = plugins
        .iter()
        .filter(|(_, marketplace, _)| marketplace == name)
        .map(|(plugin, _, _)| plugin.as_str())
        .collect();
    let mut any_product = false;
    for product in &catalog.products {
        if !product
            .plugins
            .iter()
            .any(|plugin| installed.contains(&plugin.as_str()))
        {
            continue;
        }
        any_product = true;
        for binary in product.binaries.iter().filter(|binary| !binary.optional) {
            let newest = newest_of(binary.install.repository());
            match binary_version(&binary.name) {
                None => lines.push(format!(
                    "b10x: the `{}` plugin is installed but the `{}` CLI is not on PATH; /{}:init installs it.",
                    product.id, binary.name, product.id
                )),
                // A pinned CLI is held to its pin below, not to the newest release; a local pin it
                // matches says what it holds back, so the newer release is not read as a fault.
                Some((path, found)) if pinned(&binary.name).is_some() => {
                    if let (Some(pin), Some(found), Some(newest)) =
                        (pinned(&binary.name).filter(|pin| pin.local), found, newest)
                    {
                        if let Some(line) = held_back(&binary.name, &found, &path, pin, &newest) {
                            lines.push(line);
                        }
                    }
                }
                Some((path, found)) => {
                    let found = found.unwrap_or_else(|| "an unknown version".to_owned());
                    if let Some(newest) = newest {
                        let behind = matches!(
                            (version::key(&found), version::key(&newest)),
                            (Some(have), Some(want)) if have < want
                        );
                        if behind {
                            lines.push(format!(
                                "b10x: `{}` {found} ({path}) is older than the newest release {newest}; /b10x:upgrade updates it.",
                                binary.name
                            ));
                        }
                    }
                }
            }
        }
    }
    for product in &catalog.products {
        for binary in &product.binaries {
            let Some(pin) = pinned(&binary.name) else {
                continue;
            };
            let Ok(parsed) = Spec::parse(&pin.spec) else {
                continue;
            };
            let (spec, file) = (&pin.spec, pin.file.display());
            if let Some((path, found)) = binary_version(&binary.name) {
                let found = found.unwrap_or_else(|| "an unknown version".to_owned());
                if !parsed.matches(&found) {
                    lines.push(if pin.local {
                        let reason = pin
                            .reason
                            .as_ref()
                            .map(|reason| format!(" ({})", pins::one_line(reason)))
                            .unwrap_or_default();
                        format!(
                            "b10x: `{}` {found} ({path}) differs from the local pin {spec} in {file}{reason}; `b10x install {}` installs the pinned release.",
                            binary.name, binary.name
                        )
                    } else {
                        format!(
                            "b10x: `{}` {found} ({path}) does not match the pin {spec} in {file}; `b10x install {}` installs the pinned release.",
                            binary.name, binary.name
                        )
                    });
                }
            }
            if let Some(described) = expected.verified.get(&binary.name) {
                if parsed.older_than(described) {
                    let holder = if pin.local {
                        "a local pin holds"
                    } else {
                        "this repository pins"
                    };
                    lines.push(format!(
                        "b10x: skills describe {} {described}; {holder} {spec} ({file}).",
                        binary.name
                    ));
                }
            }
        }
    }
    // A pin of a name outside the catalog pins nothing here, but `b10x upgrade` refuses it: one
    // line per such name, so the session learns why before it upgrades.
    for (binary, pin) in expected.pins {
        if catalog.binary(binary).is_none() {
            lines.push(format!(
                "b10x: {} pins `{binary}`, which is not a catalog binary; `b10x upgrade` refuses until it is removed.",
                pin.file.display()
            ));
        }
    }
    if any_product {
        let days = last.map(|(at, _)| now.saturating_sub(at) / 86_400);
        match days {
            None => lines.push(
                "b10x: newest releases have not been checked on this machine; /b10x:upgrade checks them."
                    .to_owned(),
            ),
            Some(days) if days >= STALE_DAYS => lines.push(format!(
                "b10x: newest releases were last checked {days} days ago; /b10x:upgrade checks again."
            )),
            Some(_) => {}
        }
    } else if !installed
        .iter()
        .any(|plugin| catalog.product_of(catalog.current_name(plugin)).is_some())
    {
        lines.push(
            "b10x: no Beyond10x product is set up yet; /b10x:init asks what you want to do and installs it."
                .to_owned(),
        );
    }
    lines
}

/// The line for a CLI at its local pin while a newer release exists; `None` when it is off the pin
/// (said elsewhere) or nothing newer is out.
fn held_back(name: &str, found: &str, path: &str, pin: &Pin, newest: &str) -> Option<String> {
    let spec = Spec::parse(&pin.spec).ok()?;
    if !spec.matches(found) || !spec.older_than(newest) {
        return None;
    }
    let reason = pin
        .reason
        .as_ref()
        .map(|reason| format!(": {}", pins::one_line(reason)))
        .unwrap_or_default();
    Some(format!(
        "b10x: `{name}` {found} ({path}) is held back by a local pin ({} in {}){reason}; the newest release {newest} stays uninstalled until the pin is removed.",
        pin.spec,
        pin.file.display()
    ))
}

/// The pins that apply in `here` and the warnings to print. A pin file that cannot be read is one
/// warning and pins nothing, so the session still starts; the other file still applies.
fn session_pins(
    here: Option<&Path>,
    home: &Path,
    b10x_pins: Option<OsString>,
    xdg: Option<OsString>,
) -> (Pins, Vec<String>) {
    let mut warnings = Vec::new();
    let repository = match here.map(|here| pins::read(here, home)) {
        Some(Ok(found)) => found,
        Some(Err(error)) => {
            warnings.push(format!("b10x: {error}"));
            None
        }
        None => None,
    };
    let local = match pins::read_local_from(home, b10x_pins, xdg) {
        Ok(found) => found,
        Err(error) => {
            warnings.push(format!(
                "b10x: {error}; local pins are ignored until it is fixed."
            ));
            None
        }
    };
    (pins::merge(repository.as_ref(), local.as_ref()), warnings)
}

/// Run the check and print its lines.
pub fn run(catalog: &Catalog) {
    let home = home();
    let now = resolve::now();
    refresh(&home, catalog, now);
    let plugins = recorded(&home);
    let first = |name: &str| {
        copies_on_path(name)
            .into_iter()
            .next()
            .map(|copy| (copy.path, copy.version))
    };
    let cached = latest(&home);
    let last = cached.as_ref().map(|(at, map)| (*at, map));
    let here = std::env::current_dir().ok();
    let (pins, warnings) = session_pins(
        here.as_deref(),
        &home,
        std::env::var_os("B10X_PINS"),
        std::env::var_os("XDG_CONFIG_HOME"),
    );
    for warning in warnings {
        println!("{warning}");
    }
    let verified = verified();
    let expected = Expected {
        pins: &pins,
        verified: &verified,
    };
    for line in lines(catalog, &plugins, &first, last, now, &expected) {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pins::PinFile;

    fn plugin(name: &str, market: &str, version: &str) -> Recorded {
        (name.to_owned(), market.to_owned(), Some(version.to_owned()))
    }

    fn cache(ess: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        map.insert("beyond10x/ess".to_owned(), serde_json::json!(ess));
        map
    }

    static NOTHING_VERIFIED: BTreeMap<String, String> = BTreeMap::new();
    static NO_PINS: Pins = BTreeMap::new();

    fn unpinned() -> Expected<'static> {
        Expected {
            pins: &NO_PINS,
            verified: &NOTHING_VERIFIED,
        }
    }

    fn ess_at(version: &'static str) -> impl Fn(&str) -> Option<(String, Option<String>)> {
        move |_: &str| {
            Some((
                "/opt/b10x-home/.local/bin/ess".to_owned(),
                Some(version.to_owned()),
            ))
        }
    }

    #[test]
    fn the_embedded_verified_releases_parse() {
        let verified = verified();
        for cli in ["aep", "ess", "worktree"] {
            assert!(
                verified.get(cli).and_then(|v| version::key(v)).is_some(),
                "{cli}: {verified:?}"
            );
        }
    }

    #[test]
    fn plugins_older_than_the_newest_agentplugins_release_name_upgrade() {
        let catalog = Catalog::embedded();
        let plugins = [
            plugin("b10x", "b10x", "0.14.7"),
            plugin("ess", "b10x", "0.14.7"),
        ];
        let mut map = cache("0.30.0");
        map.insert(
            "beyond10x/agentplugins".to_owned(),
            serde_json::json!("0.14.10"),
        );
        let out = lines(
            &catalog,
            &plugins,
            &ess_at("0.30.0"),
            Some((1_000, &map)),
            1_000,
            &unpinned(),
        );
        assert_eq!(
            out,
            ["b10x: the Beyond10x plugins (0.14.7) are older than the newest release 0.14.10; /b10x:upgrade updates them."]
        );
    }

    #[test]
    fn a_pinned_cli_is_held_to_its_pin_and_newer_skills_are_noted() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("ess", "b10x", "0.14.0")];
        let map = cache("0.32.1");
        let file = PinFile {
            path: std::path::PathBuf::from("/work/repo/b10x.toml"),
            pins: BTreeMap::from([("ess".to_owned(), pins::Entry::new("0.32.0"))]),
            local: false,
        };
        let merged = pins::merge(Some(&file), None);
        let verified = BTreeMap::from([("ess".to_owned(), "0.32.1".to_owned())]);
        let expected = Expected {
            pins: &merged,
            verified: &verified,
        };
        // At the pin: no "older than the newest" nag, only the skills note.
        let out = lines(
            &catalog,
            &plugins,
            &ess_at("0.32.0"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(
            out,
            ["b10x: skills describe ess 0.32.1; this repository pins 0.32.0 (/work/repo/b10x.toml)."]
        );
        // Off the pin: the fixing command is named.
        let out = lines(
            &catalog,
            &plugins,
            &ess_at("0.32.1"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert!(
            out.iter()
                .any(|l| l.contains("does not match the pin 0.32.0")
                    && l.contains("`b10x install ess`")),
            "{out:#?}"
        );
    }

    #[test]
    fn silent_when_current_and_recently_checked() {
        let catalog = Catalog::embedded();
        let plugins = [
            plugin("b10x", "b10x", "0.14.0"),
            plugin("ess", "b10x", "0.14.0"),
        ];
        let found = |_: &str| {
            Some((
                "/opt/b10x-home/.local/bin/ess".to_owned(),
                Some("0.30.0".to_owned()),
            ))
        };
        let map = cache("0.30.0");
        assert_eq!(
            lines(
                &catalog,
                &plugins,
                &found,
                Some((1_000, &map)),
                1_000 + 86_400,
                &unpinned()
            ),
            Vec::<String>::new()
        );
    }

    #[test]
    fn names_an_old_cli_a_legacy_install_and_a_stale_check() {
        let catalog = Catalog::embedded();
        let plugins = [
            plugin("ess", "b10x", "0.14.0"),
            plugin("workspace-hygiene", "beyond10x", "0.10.0"),
        ];
        let found = |_: &str| {
            Some((
                "/opt/b10x-home/.cargo/bin/ess".to_owned(),
                Some("0.26.0".to_owned()),
            ))
        };
        let map = cache("0.30.0");
        let out = lines(
            &catalog,
            &plugins,
            &found,
            Some((0, &map)),
            30 * 86_400,
            &unpinned(),
        );
        assert_eq!(out.len(), 3, "{out:#?}");
        assert!(out
            .iter()
            .any(|l| l.contains("older than the newest release 0.30.0")
                && l.contains("/b10x:upgrade")));
        assert!(out
            .iter()
            .any(|l| l.contains("workspace-hygiene@beyond10x")));
        assert!(out.iter().any(|l| l.contains("30 days ago")));
    }

    #[test]
    fn points_at_init_when_nothing_is_set_up() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("b10x", "b10x", "0.14.0")];
        let out = lines(&catalog, &plugins, &|_: &str| None, None, 0, &unpinned());
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("/b10x:init"));
    }

    #[test]
    fn a_legacy_plugin_is_not_nothing_set_up() {
        let catalog = Catalog::embedded();
        let plugins = [
            plugin("b10x", "b10x", "0.12.0"),
            plugin("aep-plan", "b10x", "0.12.0"),
        ];
        let out = lines(&catalog, &plugins, &|_: &str| None, None, 0, &unpinned());
        assert!(!out.iter().any(|l| l.contains("/b10x:init")), "{out:#?}");
    }

    const LOCAL: &str = "/opt/b10x-home/.config/b10x/pins.toml";

    fn locally_pinned_aep(reason: Option<&str>) -> Pins {
        let file = PinFile {
            path: std::path::PathBuf::from(LOCAL),
            pins: BTreeMap::from([(
                "aep".to_owned(),
                pins::Entry {
                    spec: "0.71.3".to_owned(),
                    reason: reason.map(str::to_owned),
                },
            )]),
            local: true,
        };
        pins::merge(None, Some(&file))
    }

    fn aep_at(version: &'static str) -> impl Fn(&str) -> Option<(String, Option<String>)> {
        move |name: &str| {
            (name == "aep").then(|| {
                (
                    "/opt/b10x-home/.local/bin/aep".to_owned(),
                    Some(version.to_owned()),
                )
            })
        }
    }

    fn aep_newest(newest: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        map.insert("beyond10x/aep".to_owned(), serde_json::json!(newest));
        map
    }

    #[test]
    fn a_cli_at_its_local_pin_is_held_back_without_inviting_an_upgrade() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("aep", "b10x", "0.14.0")];
        let map = aep_newest("0.72.0");
        let pins = locally_pinned_aep(Some("0.72.0 refused existing stores"));
        let expected = Expected {
            pins: &pins,
            verified: &NOTHING_VERIFIED,
        };
        let out = lines(
            &catalog,
            &plugins,
            &aep_at("0.71.3"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(
            out,
            [format!("b10x: `aep` 0.71.3 (/opt/b10x-home/.local/bin/aep) is held back by a local pin (0.71.3 in {LOCAL}): 0.72.0 refused existing stores; the newest release 0.72.0 stays uninstalled until the pin is removed.")]
        );
        assert!(!out[0].contains("/b10x:upgrade"));
        // Without a reason the line has none; with nothing newer the check is silent.
        let pins = locally_pinned_aep(None);
        let expected = Expected {
            pins: &pins,
            verified: &NOTHING_VERIFIED,
        };
        let out = lines(
            &catalog,
            &plugins,
            &aep_at("0.71.3"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(
            out,
            [format!("b10x: `aep` 0.71.3 (/opt/b10x-home/.local/bin/aep) is held back by a local pin (0.71.3 in {LOCAL}); the newest release 0.72.0 stays uninstalled until the pin is removed.")]
        );
        let current = aep_newest("0.71.3");
        let out = lines(
            &catalog,
            &plugins,
            &aep_at("0.71.3"),
            Some((1_000, &current)),
            1_000,
            &expected,
        );
        assert_eq!(out, Vec::<String>::new());
    }

    #[test]
    fn a_cli_off_its_local_pin_is_named_with_the_install_that_matches_it() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("aep", "b10x", "0.14.0")];
        let map = aep_newest("0.72.0");
        let pins = locally_pinned_aep(Some("0.72.0 refused existing stores"));
        let verified = BTreeMap::from([("aep".to_owned(), "0.72.0".to_owned())]);
        let expected = Expected {
            pins: &pins,
            verified: &verified,
        };
        let out = lines(
            &catalog,
            &plugins,
            &aep_at("0.72.0"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(
            out,
            [
                format!("b10x: `aep` 0.72.0 (/opt/b10x-home/.local/bin/aep) differs from the local pin 0.71.3 in {LOCAL} (0.72.0 refused existing stores); `b10x install aep` installs the pinned release."),
                format!("b10x: skills describe aep 0.72.0; a local pin holds 0.71.3 ({LOCAL})."),
            ]
        );
        assert!(out.iter().all(|line| !line.contains("/b10x:upgrade")));
    }

    #[test]
    fn a_malformed_local_pin_file_is_one_warning_and_pins_nothing() {
        let root = std::env::temp_dir().join(format!("b10x-check-local-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let home = root.join("home");
        let repo = home.join("work/repo");
        std::fs::create_dir_all(&repo).unwrap();
        std::fs::write(repo.join(pins::FILE), "[pins]\ness = \"0.59.0\"\n").unwrap();
        let local = root.join("pins.toml");
        std::fs::write(
            &local,
            "[pins]\naep = { version = \"0.71.3\", why = \"x\" }\n",
        )
        .unwrap();
        let (found, warnings) = session_pins(Some(&repo), &home, Some(local.clone().into()), None);
        assert_eq!(warnings.len(), 1, "{warnings:#?}");
        assert!(
            warnings[0].starts_with(&format!("b10x: {}: ", local.display()))
                && warnings[0].ends_with("; local pins are ignored until it is fixed."),
            "{}",
            warnings[0]
        );
        assert_eq!(
            found.keys().collect::<Vec<_>>(),
            ["ess"],
            "the repository pin stands"
        );
        // Unparseable TOML is the same single line.
        std::fs::write(&local, "[pins\n").unwrap();
        let (found, warnings) = session_pins(None, &home, Some(local.clone().into()), None);
        assert_eq!(warnings.len(), 1, "{warnings:#?}");
        assert!(found.is_empty());
        // A good local file applies and wins.
        std::fs::write(&local, "[pins]\ness = \"0.58\"\n").unwrap();
        let (found, warnings) = session_pins(Some(&repo), &home, Some(local.clone().into()), None);
        assert!(warnings.is_empty(), "{warnings:#?}");
        assert!(found
            .get("ess")
            .is_some_and(|pin| pin.local && pin.spec == "0.58"));
        std::fs::remove_dir_all(&root).unwrap();
    }

    // Adversary: the session prints each warning with `println!`, so a warning holding a newline
    // is more than one line on the session's screen.
    #[test]
    fn adversary_an_unparseable_local_file_is_one_printed_line() {
        let root = std::env::temp_dir().join(format!("b10x-adv-toml-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let home = root.join("home");
        let local = root.join("pins.toml");
        for text in ["[pins\n", "aep = = 0.71.3\n", "\u{1}garbage\n"] {
            std::fs::write(&local, text).unwrap();
            let (found, warnings) = session_pins(None, &home, Some(local.clone().into()), None);
            assert!(found.is_empty());
            assert_eq!(warnings.len(), 1, "{text:?}: {warnings:#?}");
            assert!(
                !warnings[0].contains('\n'),
                "{text:?} prints {} lines:\n{}",
                warnings[0].lines().count(),
                warnings[0]
            );
        }
        std::fs::remove_dir_all(&root).unwrap();
    }

    // Adversary: the other malformed shapes; expected to hold.
    #[test]
    fn adversary_other_malformed_local_files_are_one_line_and_never_panic() {
        let root = std::env::temp_dir().join(format!("b10x-adv-shapes-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let home = root.join("home");
        std::fs::create_dir_all(&home).unwrap();
        let local = root.join("pins.toml");
        // Binary garbage, `pins` not a table, an array of tables.
        for bytes in [
            &b"\xff\xfe\x00\x01"[..],
            b"pins = 3\n",
            b"[[pins]]\naep = \"0.71.3\"\n",
        ] {
            std::fs::write(&local, bytes).unwrap();
            let (found, warnings) = session_pins(None, &home, Some(local.clone().into()), None);
            assert!(found.is_empty());
            assert_eq!(warnings.len(), 1, "{bytes:?}: {warnings:#?}");
            assert!(!warnings[0].contains('\n'), "{}", warnings[0]);
        }
        // An empty file pins nothing and says nothing.
        std::fs::write(&local, "").unwrap();
        let (found, warnings) = session_pins(None, &home, Some(local.clone().into()), None);
        assert!(found.is_empty() && warnings.is_empty(), "{warnings:#?}");
        // A directory at the named path, and at the default path.
        std::fs::remove_file(&local).unwrap();
        std::fs::create_dir_all(&local).unwrap();
        let (_, warnings) = session_pins(None, &home, Some(local.clone().into()), None);
        assert_eq!(warnings.len(), 1, "{warnings:#?}");
        std::fs::create_dir_all(home.join(".config/b10x/pins.toml")).unwrap();
        let (_, warnings) = session_pins(None, &home, None, None);
        assert_eq!(warnings.len(), 1, "{warnings:#?}");
        // `$B10X_PINS` empty falls through to the default; naming a missing file is one line.
        let (_, warnings) = session_pins(None, &home, Some(OsString::new()), None);
        assert_eq!(warnings.len(), 1, "{warnings:#?}");
        let (_, warnings) = session_pins(None, &home, Some(root.join("nope.toml").into()), None);
        assert_eq!(warnings.len(), 1, "{warnings:#?}");
        std::fs::remove_dir_all(&root).unwrap();
    }

    // Adversary: a TOML reason may hold a newline; the held-back line then prints as two lines.
    #[test]
    fn adversary_a_reason_with_a_newline_keeps_the_held_back_line_one_line() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("aep", "b10x", "0.14.0")];
        let map = aep_newest("0.72.0");
        let pins = locally_pinned_aep(Some("0.72.0 refused\nexisting stores"));
        let expected = Expected {
            pins: &pins,
            verified: &NOTHING_VERIFIED,
        };
        let out = lines(
            &catalog,
            &plugins,
            &aep_at("0.71.3"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(out.len(), 1, "{out:#?}");
        assert!(!out[0].contains('\n'), "{}", out[0]);
    }

    // Adversary: the held-back line for a line pin, a `v` spelling and a repository pin under it;
    // expected to hold.
    #[test]
    fn adversary_held_back_never_invites_an_upgrade_for_any_spelling() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("aep", "b10x", "0.14.0")];
        let repository = PinFile {
            path: std::path::PathBuf::from("/work/repo/b10x.toml"),
            pins: BTreeMap::from([("aep".to_owned(), pins::Entry::new("0.70"))]),
            local: false,
        };
        for spec in ["0.71", "v0.71.3", "v0.71"] {
            let local = PinFile {
                path: std::path::PathBuf::from(LOCAL),
                pins: BTreeMap::from([("aep".to_owned(), pins::Entry::new(spec))]),
                local: true,
            };
            let merged = pins::merge(Some(&repository), Some(&local));
            let verified = BTreeMap::from([("aep".to_owned(), "0.72.0".to_owned())]);
            let expected = Expected {
                pins: &merged,
                verified: &verified,
            };
            for newest in ["0.72.0", "v0.72.0"] {
                for found in ["0.71.3", "v0.71.3"] {
                    let map = aep_newest(newest);
                    let out = lines(
                        &catalog,
                        &plugins,
                        &aep_at(found),
                        Some((1_000, &map)),
                        1_000,
                        &expected,
                    );
                    assert!(
                        out.iter()
                            .any(|l| l.contains("is held back by a local pin")),
                        "{spec} {newest} {found}: {out:#?}"
                    );
                    assert!(
                        out.iter()
                            .all(|l| !l.contains("/b10x:upgrade") && !l.contains("0.70")),
                        "{spec} {newest} {found}: {out:#?}"
                    );
                }
            }
        }
        // An unknown installed version is off the pin, never an upgrade invitation.
        let local = PinFile {
            path: std::path::PathBuf::from(LOCAL),
            pins: BTreeMap::from([("aep".to_owned(), pins::Entry::new("0.71.3"))]),
            local: true,
        };
        let merged = pins::merge(None, Some(&local));
        let expected = Expected {
            pins: &merged,
            verified: &NOTHING_VERIFIED,
        };
        let map = aep_newest("0.72.0");
        let unknown = |name: &str| {
            (name == "aep").then(|| ("/opt/b10x-home/.local/bin/aep".to_owned(), None))
        };
        let out = lines(
            &catalog,
            &plugins,
            &unknown,
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(out.len(), 1, "{out:#?}");
        assert!(
            out[0].contains("an unknown version") && !out[0].contains("/b10x:upgrade"),
            "{out:#?}"
        );
    }

    #[test]
    fn a_local_pin_of_a_name_outside_the_catalog_is_named_once() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("aep", "b10x", "0.14.0")];
        let map = aep_newest("0.71.3");
        let local = PinFile {
            path: std::path::PathBuf::from(LOCAL),
            pins: BTreeMap::from([("nope".to_owned(), pins::Entry::new("0.1.0"))]),
            local: true,
        };
        let merged = pins::merge(None, Some(&local));
        let expected = Expected {
            pins: &merged,
            verified: &NOTHING_VERIFIED,
        };
        let out = lines(
            &catalog,
            &plugins,
            &aep_at("0.71.3"),
            Some((1_000, &map)),
            1_000,
            &expected,
        );
        assert_eq!(
            out,
            [format!("b10x: {LOCAL} pins `nope`, which is not a catalog binary; `b10x upgrade` refuses until it is removed.")]
        );
    }

    #[test]
    fn a_relative_b10x_pins_is_one_warning_in_the_check() {
        let (found, warnings) = session_pins(
            None,
            Path::new("/opt/b10x-home"),
            Some("pins.toml".into()),
            None,
        );
        assert!(found.is_empty());
        assert_eq!(
            warnings,
            ["b10x: $B10X_PINS must be an absolute path: pins.toml; local pins are ignored until it is fixed."]
        );
    }
}
