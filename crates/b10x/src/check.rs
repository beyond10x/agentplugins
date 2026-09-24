//! The session-start check. Offline and quick: it reads what the hosts recorded on disk and runs
//! each bound binary's `--version`, then prints one line per problem and nothing when all is well.
//! It never fails the session.

use std::path::Path;

use crate::catalog::{Bind, Catalog};
use crate::inventory::{copies_on_path, home};
use crate::version;

/// A recorded install: `name`, `marketplace`, version.
pub type Recorded = (String, String, Option<String>);

/// Finds the first copy of a binary on `PATH`: its path and version.
pub type Probe<'a> = &'a dyn Fn(&str) -> Option<(String, Option<String>)>;

/// Installed plugins as the hosts recorded them: (`name`, `marketplace`, version).
#[must_use]
pub fn recorded(home: &Path) -> Vec<(String, String, Option<String>)> {
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
                        .and_then(|list| {
                            list.iter()
                                .find(|install| {
                                    install.get("scope").and_then(serde_json::Value::as_str)
                                        == Some("user")
                                })
                                .or_else(|| list.first())
                        })
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

/// The lines to print.
#[must_use]
pub fn lines(catalog: &Catalog, plugins: &[Recorded], binary_version: Probe<'_>) -> Vec<String> {
    let mut lines = Vec::new();
    let name = catalog.marketplace.name.as_str();
    for (plugin, marketplace, _) in plugins {
        if catalog.retired_marketplace(marketplace) || catalog.retired_plugins.contains_key(plugin)
        {
            lines.push(format!(
                "b10x: legacy plugin `{plugin}@{marketplace}` is installed; run the b10x setup skill to migrate it."
            ));
        }
    }
    for product in &catalog.products {
        for binary in &product.binaries {
            let Bind::Plugin { plugin } = &binary.bind else {
                continue;
            };
            let Some(expected) = plugins
                .iter()
                .find(|(installed, marketplace, _)| installed == plugin && marketplace == name)
                .and_then(|(_, _, version)| version.clone())
            else {
                continue;
            };
            match binary_version(&binary.name) {
                None => lines.push(format!(
                    "b10x: the `{plugin}` plugin {expected} drives `{}`, which is not on PATH; run the b10x setup skill.",
                    binary.name
                )),
                Some((path, found)) => {
                    let found = found.unwrap_or_else(|| "an unknown version".to_owned());
                    if !version::same(&found, &expected) {
                        lines.push(format!(
                            "b10x: the `{plugin}` plugin {expected} describes {} {expected}, but PATH runs {found} ({path}); run the b10x setup skill.",
                            binary.name
                        ));
                    }
                }
            }
        }
    }
    lines
}

/// Run the check and print its lines.
pub fn run(catalog: &Catalog) {
    let plugins = recorded(&home());
    let first = |name: &str| {
        copies_on_path(name)
            .into_iter()
            .next()
            .map(|copy| (copy.path, copy.version))
    };
    for line in lines(catalog, &plugins, &first) {
        println!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugin(name: &str, market: &str, version: &str) -> (String, String, Option<String>) {
        (name.to_owned(), market.to_owned(), Some(version.to_owned()))
    }

    #[test]
    fn silent_when_everything_matches() {
        let catalog = Catalog::embedded();
        let plugins = [plugin("ess", "b10x", "0.30.0")];
        let found = |_: &str| {
            Some((
                "/opt/b10x-home/.local/bin/ess".to_owned(),
                Some("0.30.0".to_owned()),
            ))
        };
        assert!(lines(&catalog, &plugins, &found).is_empty());
    }

    #[test]
    fn names_the_binary_behind_its_plugin_and_legacy_ids() {
        let catalog = Catalog::embedded();
        let plugins = [
            plugin("ess", "b10x", "0.30.0"),
            plugin("workspace-hygiene", "beyond10x", "0.10.0"),
        ];
        let found = |_: &str| {
            Some((
                "/opt/b10x-home/.cargo/bin/ess".to_owned(),
                Some("0.26.0".to_owned()),
            ))
        };
        let out = lines(&catalog, &plugins, &found);
        assert_eq!(out.len(), 2, "{out:#?}");
        assert!(out.iter().any(|l| l.contains("PATH runs 0.26.0")));
        assert!(out
            .iter()
            .any(|l| l.contains("workspace-hygiene@beyond10x")));
    }
}
