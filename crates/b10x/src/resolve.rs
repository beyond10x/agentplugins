//! Versions resolved at run time. The catalog names no version, so the version each plugin will
//! install and each binary must have is read here: a plugin the marketplace carries from its
//! `plugin.json`, a plugin it points at from that repository's newest release, a binary bound to
//! `latest` from the newest release.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::catalog::{Bind, Catalog};
use crate::inventory::Inventory;

/// Versions setup compares against.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resolved {
    /// Plugin name → the version the marketplace installs now.
    pub plugins: BTreeMap<String, String>,
    /// `owner/repo` → newest release tag.
    pub latest: BTreeMap<String, String>,
}

/// Where marketplace files are read from.
pub enum Source<'a> {
    /// A local clone of the marketplace repository.
    Clone(&'a Path),
    /// The repository's default branch on GitHub.
    Remote(&'a str),
}

impl Source<'_> {
    /// Read one repository file.
    #[must_use]
    pub fn read(&self, relative: &str) -> Option<String> {
        match self {
            Source::Clone(root) => std::fs::read_to_string(root.join(relative)).ok(),
            Source::Remote(repository) => fetch(&format!(
                "https://raw.githubusercontent.com/{repository}/HEAD/{relative}"
            )),
        }
    }
}

/// `curl` the body of a URL, or `None`.
#[must_use]
pub fn fetch(url: &str) -> Option<String> {
    let output = Command::new("curl")
        .args(["-fsSL", "--retry", "2", "--max-time", "30", url])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The newest release tag of `owner/repo`, from the `releases/latest` redirect (no API token).
#[must_use]
pub fn latest_tag(repository: &str) -> Option<String> {
    let output = Command::new("curl")
        .args([
            "-fsS",
            "-o",
            "/dev/null",
            "--max-time",
            "30",
            "-w",
            "%{redirect_url}",
            &format!("https://github.com/{repository}/releases/latest"),
        ])
        .output()
        .ok()?;
    let location = String::from_utf8_lossy(&output.stdout).into_owned();
    let tag = location.rsplit_once("/releases/tag/")?.1.trim();
    (!tag.is_empty()).then(|| tag.to_owned())
}

/// `owner/repo` from a GitHub URL.
#[must_use]
pub fn repository_of(url: &str) -> Option<String> {
    let rest = url
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split_once("github.com/")?
        .1;
    let mut parts = rest.split('/');
    let owner = parts.next()?;
    let name = parts.next()?;
    Some(format!("{owner}/{name}"))
}

/// The marketplace clone a host registered, if any.
#[must_use]
pub fn clone_of<'a>(inventory: &'a Inventory, catalog: &Catalog) -> Option<&'a Path> {
    [inventory.claude.as_ref(), inventory.codex.as_ref()]
        .into_iter()
        .flatten()
        .flat_map(|state| state.marketplaces.iter())
        .filter(|market| market.name == catalog.marketplace.name)
        .filter_map(|market| market.location.as_deref())
        .map(Path::new)
        .find(|path| path.join(".claude-plugin/marketplace.json").is_file())
}

/// The catalog to plan with: the marketplace's copy when it parses, else the embedded one.
#[must_use]
pub fn catalog(source: &Source<'_>) -> Catalog {
    source
        .read("catalog.json")
        .and_then(|text| Catalog::parse(&text).ok())
        .unwrap_or_else(Catalog::embedded)
}

/// Plugin versions from a marketplace document; remote entries take their repository's newest tag.
pub fn plugin_versions(
    marketplace: &serde_json::Value,
    source: &Source<'_>,
    latest: &mut BTreeMap<String, String>,
) -> BTreeMap<String, String> {
    let mut versions = BTreeMap::new();
    let entries = marketplace
        .get("plugins")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for entry in entries {
        let Some(name) = entry.get("name").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let version = match entry.get("source") {
            Some(serde_json::Value::String(path)) => {
                let relative = format!(
                    "{}/.claude-plugin/plugin.json",
                    path.trim_start_matches("./")
                );
                source
                    .read(&relative)
                    .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
                    .and_then(|manifest| {
                        manifest
                            .get("version")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_owned)
                    })
            }
            Some(object) => object
                .get("url")
                .and_then(serde_json::Value::as_str)
                .and_then(repository_of)
                .and_then(|repository| tag(latest, &repository)),
            None => None,
        };
        if let Some(version) = version {
            versions.insert(
                name.to_owned(),
                crate::version::parse(&version).unwrap_or(version),
            );
        }
    }
    versions
}

fn tag(latest: &mut BTreeMap<String, String>, repository: &str) -> Option<String> {
    if let Some(tag) = latest.get(repository) {
        return Some(tag.clone());
    }
    let tag = latest_tag(repository)?;
    latest.insert(repository.to_owned(), tag.clone());
    Some(tag)
}

/// Resolve every version the plan needs.
#[must_use]
pub fn resolve(catalog: &Catalog, source: &Source<'_>) -> Resolved {
    let mut latest = BTreeMap::new();
    let plugins = source
        .read(".claude-plugin/marketplace.json")
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .map(|marketplace| plugin_versions(&marketplace, source, &mut latest))
        .unwrap_or_default();
    for product in &catalog.products {
        for binary in &product.binaries {
            if binary.bind == Bind::Latest {
                tag(&mut latest, binary.install.repository());
            }
        }
    }
    Resolved { plugins, latest }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repositories_come_from_github_urls() {
        assert_eq!(
            repository_of("https://github.com/beyond10x/worktree.git").as_deref(),
            Some("beyond10x/worktree")
        );
        assert_eq!(repository_of("https://gitlab.com/a/b.git"), None);
    }

    #[test]
    fn carried_plugins_read_their_manifest() {
        let root = std::env::temp_dir().join(format!("b10x-resolve-{}", std::process::id()));
        let manifest = root.join("plugins/aep/.claude-plugin");
        std::fs::create_dir_all(&manifest).unwrap();
        std::fs::write(
            manifest.join("plugin.json"),
            r#"{"name":"aep","version":"0.12.0"}"#,
        )
        .unwrap();
        let marketplace = serde_json::json!({"plugins":[{"name":"aep","source":"./plugins/aep"}]});
        let versions = plugin_versions(&marketplace, &Source::Clone(&root), &mut BTreeMap::new());
        std::fs::remove_dir_all(&root).unwrap();
        assert_eq!(versions.get("aep").map(String::as_str), Some("0.12.0"));
    }

    #[test]
    fn pointed_plugins_take_the_known_newest_tag() {
        let marketplace = serde_json::json!({"plugins":[{"name":"ess","source":{"source":"git-subdir","url":"https://github.com/beyond10x/ess.git","path":"plugins/ess"}}]});
        let mut latest = BTreeMap::from([("beyond10x/ess".to_owned(), "0.30.0".to_owned())]);
        let versions = plugin_versions(&marketplace, &Source::Remote("x/y"), &mut latest);
        assert_eq!(versions.get("ess").map(String::as_str), Some("0.30.0"));
    }
}
