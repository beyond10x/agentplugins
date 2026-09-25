//! The session-start check. Offline and quick: it reads what the hosts recorded on disk, runs each
//! installed product's `--version`, and compares with the newest releases the last plan saw. It
//! prints one line per problem, nothing when all is well, and never fails the session.

use std::path::Path;

use crate::catalog::Catalog;
use crate::inventory::{copies_on_path, home};
use crate::version;

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

/// The lines to print.
#[must_use]
pub fn lines(
    catalog: &Catalog,
    plugins: &[Recorded],
    binary_version: Probe<'_>,
    last: Option<(u64, &serde_json::Map<String, serde_json::Value>)>,
    now: u64,
) -> Vec<String> {
    let mut lines = Vec::new();
    let name = catalog.marketplace.name.as_str();
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
            let newest = last
                .and_then(|(_, map)| map.get(binary.install.repository()))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            match binary_version(&binary.name) {
                None => lines.push(format!(
                    "b10x: the `{}` plugin is installed but the `{}` CLI is not on PATH; /{}:init installs it.",
                    product.id, binary.name, product.id
                )),
                Some((path, found)) => {
                    let found = found.unwrap_or_else(|| "an unknown version".to_owned());
                    if let Some(newest) = newest {
                        let behind = matches!(
                            (version::key(&found), version::key(&newest)),
                            (Some(have), Some(want)) if have < want
                        );
                        if behind {
                            lines.push(format!(
                                "b10x: `{}` {found} ({path}) is older than the newest release {newest}; /{}:upgrade updates it.",
                                binary.name, product.id
                            ));
                        }
                    }
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
    let plugins = recorded(&home);
    let first = |name: &str| {
        copies_on_path(name)
            .into_iter()
            .next()
            .map(|copy| (copy.path, copy.version))
    };
    let cached = latest(&home);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let last = cached.as_ref().map(|(at, map)| (*at, map));
    for line in lines(catalog, &plugins, &first, last, now) {
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
            1_000 + 86_400
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
        let out = lines(&catalog, &plugins, &found, Some((0, &map)), 30 * 86_400);
        assert_eq!(out.len(), 3, "{out:#?}");
        assert!(out
            .iter()
            .any(|l| l.contains("older than the newest release 0.30.0")
                && l.contains("/ess:upgrade")));
        assert!(out
            .iter()
            .any(|l| l.contains("workspace-hygiene@beyond10x")));
        assert!(out.iter().any(|l| l.contains("30 days ago")));
    }

    #[test]
    fn points_at_init_when_nothing_is_set_up() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("b10x", "b10x", "0.14.0")];
        let out = lines(&catalog, &plugins, &|_: &str| None, None, 0);
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
        let out = lines(&catalog, &plugins, &|_: &str| None, None, 0);
        assert!(!out.iter().any(|l| l.contains("/b10x:init")), "{out:#?}");
    }
}
