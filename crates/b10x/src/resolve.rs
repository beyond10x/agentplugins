//! Versions resolved at run time. The catalog names no version, so the version each plugin will
//! install and each binary must have is read here: a plugin the marketplace carries from its
//! `plugin.json`, a plugin it points at from that repository's newest release, a binary bound to
//! `latest` from the newest release.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::catalog::Catalog;
use crate::install::listed_targets;
use crate::inventory::Inventory;
use crate::pins::{PinFile, Pinned, Spec};
use crate::version;

/// Versions setup compares against.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resolved {
    /// Plugin name → the version the marketplace installs now.
    pub plugins: BTreeMap<String, String>,
    /// `owner/repo` → newest release tag.
    pub latest: BTreeMap<String, String>,
    /// Binary name → the targets the release the plan installs (the pinned one, else the newest)
    /// carries a prebuilt archive for, as its `SHA256SUMS` lists them; absent or empty when none.
    #[serde(default)]
    pub archive_targets: BTreeMap<String, BTreeSet<String>>,
    /// Binary name → its pin in the repository's `b10x.toml`, resolved to a release.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub pinned: BTreeMap<String, Pinned>,
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

/// Where the newest release tags seen by the last plan are kept, for the offline session check.
#[must_use]
pub fn cache_path(home: &Path) -> std::path::PathBuf {
    home.join(".local/state/b10x/latest.json")
}

/// Seconds since the Unix epoch.
#[must_use]
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// The newest-release record: `checked_at` (last successful read), `attempted_at` (last try) and
/// `latest` (`owner/repo` → tag).
#[must_use]
pub fn recorded(home: &Path) -> serde_json::Map<String, serde_json::Value> {
    std::fs::read_to_string(cache_path(home))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default()
}

/// Merge newly read tags into the record. `checked_at` moves only when something was read.
pub fn record(home: &Path, tags: &BTreeMap<String, String>, at: u64) {
    let mut value = recorded(home);
    let mut latest = value
        .get("latest")
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default();
    for (repository, tag) in tags {
        latest.insert(repository.clone(), serde_json::json!(tag));
    }
    if !tags.is_empty() {
        value.insert("checked_at".to_owned(), serde_json::json!(at));
    }
    value.insert("attempted_at".to_owned(), serde_json::json!(at));
    value.insert("latest".to_owned(), serde_json::Value::Object(latest));
    let path = cache_path(home);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, serde_json::Value::Object(value).to_string());
}

/// Record the newest release tags a plan read, and when.
pub fn remember(home: &Path, resolved: &Resolved) {
    record(home, &resolved.latest, now());
}

/// The newest release tag of `owner/repo`, from the `releases/latest` redirect (no API token).
#[must_use]
pub fn latest_tag(repository: &str) -> Option<String> {
    latest_tag_within(repository, 30)
}

/// [`latest_tag`], giving up after `seconds`.
#[must_use]
pub fn latest_tag_within(repository: &str, seconds: u64) -> Option<String> {
    let seconds = seconds.to_string();
    let output = Command::new("curl")
        .args([
            "-fsS",
            "-o",
            "/dev/null",
            "--connect-timeout",
            &seconds,
            "--max-time",
            &seconds,
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

/// The newest release tag of each repository, read in parallel, each within `seconds`.
#[must_use]
pub fn latest_tags(repositories: &BTreeSet<String>, seconds: u64) -> BTreeMap<String, String> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = repositories
            .iter()
            .map(|repository| {
                scope.spawn(move || {
                    latest_tag_within(repository, seconds).map(|tag| (repository.clone(), tag))
                })
            })
            .collect();
        handles
            .into_iter()
            .filter_map(|handle| handle.join().ok().flatten())
            .collect()
    })
}

/// Every tag of `owner/repo`, from `git ls-remote` (no API token).
#[must_use]
pub fn tags(repository: &str) -> Option<Vec<String>> {
    let output = Command::new("git")
        .args([
            "ls-remote",
            "--tags",
            "--refs",
            &format!("https://github.com/{repository}.git"),
        ])
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| line.split_once("refs/tags/"))
            .map(|(_, tag)| tag.trim().to_owned())
            .collect()
    })
}

/// Whether `owner/repo` has a published GitHub Release for `tag`.
#[must_use]
pub fn release_exists(repository: &str, tag: &str) -> bool {
    Command::new("curl")
        .args([
            "-fsS",
            "-o",
            "/dev/null",
            "--max-time",
            "30",
            "-H",
            "Accept: application/vnd.github+json",
            &format!("https://api.github.com/repos/{repository}/releases/tags/{tag}"),
        ])
        .output()
        .is_ok_and(|output| output.status.success())
}

/// The release a pin resolves to: the newest release when it satisfies the pin, else the newest
/// matching tag of the repository.
#[must_use]
pub fn pinned_tag(repository: &str, spec: Spec, newest: Option<&str>) -> Option<String> {
    if let Some(newest) = newest.filter(|newest| spec.matches(newest)) {
        return Some(newest.to_owned());
    }
    let tags = tags(repository)?;
    spec.select(tags.iter().map(String::as_str))
        .map(str::to_owned)
}

/// The version a marketplace checkout carries: the highest version among its carried plugins.
#[must_use]
pub fn carried_version(source: &Source<'_>) -> Option<String> {
    let marketplace = source
        .read(".claude-plugin/marketplace.json")
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())?;
    let carried: serde_json::Value = serde_json::json!({
        "plugins": marketplace
            .get("plugins")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter(|entry| entry.get("source").is_some_and(serde_json::Value::is_string))
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });
    plugin_versions(&carried, source, &mut BTreeMap::new())
        .into_values()
        .max_by_key(|found| version::key(found))
}

/// Whether a host's marketplace clone is older than the marketplace's newest release. The hosts
/// refresh their clone only when a plan is applied, so a plan read from a stale clone would call
/// old plugins current.
#[must_use]
pub fn stale(clone: &Path, newest: Option<&str>) -> bool {
    let have = carried_version(&Source::Clone(clone));
    matches!(
        (have.as_deref().and_then(version::key), newest.and_then(version::key)),
        (Some(have), Some(newest)) if have < newest
    )
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

/// Resolve every version the plan needs; a binary pinned in `pins` resolves to its pinned release.
#[must_use]
pub fn resolve(catalog: &Catalog, source: &Source<'_>, pins: Option<&PinFile>) -> Resolved {
    let mut latest = BTreeMap::new();
    let plugins = source
        .read(".claude-plugin/marketplace.json")
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .map(|marketplace| plugin_versions(&marketplace, source, &mut latest))
        .unwrap_or_default();
    let mut archive_targets = BTreeMap::new();
    let mut pinned = BTreeMap::new();
    let mut sums_of = BTreeMap::new();
    for product in &catalog.products {
        for binary in &product.binaries {
            let repository = binary.install.repository();
            let newest = tag(&mut latest, repository);
            let pin = pins.and_then(|file| {
                let spec = file.pins.get(&binary.name)?;
                let tag = Spec::parse(spec)
                    .ok()
                    .and_then(|parsed| pinned_tag(repository, parsed, newest.as_deref()));
                Some(Pinned {
                    spec: spec.clone(),
                    tag,
                    file: file.path.to_string_lossy().into_owned(),
                })
            });
            let installs = match &pin {
                Some(pin) => pin.tag.clone(),
                None => newest,
            };
            if let Some(pin) = pin {
                pinned.insert(binary.name.clone(), pin);
            }
            if let (Some(tag), Some(_)) = (installs, &binary.install.archive) {
                let sums: &Option<String> = sums_of
                    .entry((repository.to_owned(), tag.clone()))
                    .or_insert_with(|| {
                        fetch(&format!(
                            "https://github.com/{repository}/releases/download/{tag}/SHA256SUMS"
                        ))
                    });
                archive_targets.insert(
                    binary.name.clone(),
                    sums.as_deref()
                        .map(|sums| listed_targets(sums, &binary.name, &tag))
                        .unwrap_or_default(),
                );
            }
        }
    }
    Resolved {
        plugins,
        latest,
        archive_targets,
        pinned,
    }
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

    fn marketplace_at(name: &str, version: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("b10x-clone-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for plugin in ["b10x", "ess"] {
            let manifest = root.join(format!("plugins/{plugin}/.claude-plugin"));
            std::fs::create_dir_all(&manifest).unwrap();
            std::fs::write(
                manifest.join("plugin.json"),
                format!(r#"{{"name":"{plugin}","version":"{version}"}}"#),
            )
            .unwrap();
        }
        std::fs::create_dir_all(root.join(".claude-plugin")).unwrap();
        std::fs::write(
            root.join(".claude-plugin/marketplace.json"),
            r#"{"plugins":[{"name":"b10x","source":"./plugins/b10x"},{"name":"ess","source":"./plugins/ess"},{"name":"x","source":{"source":"url","url":"https://github.com/a/x.git"}}]}"#,
        )
        .unwrap();
        root
    }

    #[test]
    fn a_clone_older_than_the_newest_release_is_stale() {
        let clone = marketplace_at("stale", "0.14.7");
        assert_eq!(
            carried_version(&Source::Clone(&clone)).as_deref(),
            Some("0.14.7")
        );
        assert!(stale(&clone, Some("0.14.10")), "0.14.7 < 0.14.10");
        assert!(!stale(&clone, Some("0.14.7")));
        assert!(!stale(&clone, None), "offline: the clone is all there is");
        std::fs::remove_dir_all(&clone).unwrap();
    }

    #[test]
    fn pointed_plugins_take_the_known_newest_tag() {
        let marketplace = serde_json::json!({"plugins":[{"name":"ess","source":{"source":"git-subdir","url":"https://github.com/beyond10x/ess.git","path":"plugins/ess"}}]});
        let mut latest = BTreeMap::from([("beyond10x/ess".to_owned(), "0.30.0".to_owned())]);
        let versions = plugin_versions(&marketplace, &Source::Remote("x/y"), &mut latest);
        assert_eq!(versions.get("ess").map(String::as_str), Some("0.30.0"));
    }
}
