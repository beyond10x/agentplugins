//! The session-start check. Quick: it reads what the hosts recorded on disk, runs each installed
//! product's `--version`, and compares with the newest releases recorded in
//! `~/.local/state/b10x/latest.json`. That record is refreshed from GitHub at most once a day,
//! within a few seconds; offline, the old record stands. It prints one line per problem, nothing
//! when all is well, and never fails the session.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::catalog::Catalog;
use crate::inventory::{copies_on_path, home};
use crate::pins::{self, PinFile, Spec};
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

/// What the check compares against besides the machine: the repository's pins and the releases the
/// skills were verified against.
pub struct Expected<'a> {
    /// The pin file that applies in the session's directory.
    pub pins: Option<&'a PinFile>,
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
    let pinned = |binary: &str| {
        expected
            .pins
            .and_then(|file| Some((file.pins.get(binary)?, file.path.display().to_string())))
    };
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
                // A pinned CLI is held to its pin below, not to the newest release.
                Some(_) if pinned(&binary.name).is_some() => {}
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
            let Some((spec, file)) = pinned(&binary.name) else {
                continue;
            };
            let Ok(parsed) = Spec::parse(spec) else {
                continue;
            };
            if let Some((path, found)) = binary_version(&binary.name) {
                let found = found.unwrap_or_else(|| "an unknown version".to_owned());
                if !parsed.matches(&found) {
                    lines.push(format!(
                        "b10x: `{}` {found} ({path}) does not match the pin {spec} in {file}; `b10x install {}` installs the pinned release.",
                        binary.name, binary.name
                    ));
                }
            }
            if let Some(described) = expected.verified.get(&binary.name) {
                if parsed.older_than(described) {
                    lines.push(format!(
                        "b10x: skills describe {} {described}; this repository pins {spec} ({file}).",
                        binary.name
                    ));
                }
            }
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
    let pin_file = match std::env::current_dir().map(|here| pins::read(&here, &home)) {
        Ok(Ok(found)) => found,
        Ok(Err(error)) => {
            println!("b10x: {error}");
            None
        }
        Err(_) => None,
    };
    let verified = verified();
    let expected = Expected {
        pins: pin_file.as_ref(),
        verified: &verified,
    };
    for line in lines(catalog, &plugins, &first, last, now, &expected) {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugin(name: &str, market: &str, version: &str) -> Recorded {
        (name.to_owned(), market.to_owned(), Some(version.to_owned()))
    }

    fn cache(ess: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        map.insert("beyond10x/ess".to_owned(), serde_json::json!(ess));
        map
    }

    static NOTHING_VERIFIED: BTreeMap<String, String> = BTreeMap::new();

    fn unpinned() -> Expected<'static> {
        Expected {
            pins: None,
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
            pins: BTreeMap::from([("ess".to_owned(), "0.32.0".to_owned())]),
        };
        let verified = BTreeMap::from([("ess".to_owned(), "0.32.1".to_owned())]);
        let expected = Expected {
            pins: Some(&file),
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
        assert!(lines(
            &catalog,
            &plugins,
            &found,
            Some((1_000, &map)),
            1_000 + 86_400,
            &unpinned()
        )
        .is_empty());
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
}
