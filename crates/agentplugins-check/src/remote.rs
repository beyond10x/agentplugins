//! Plugins this marketplace lists but does not carry.
//!
//! A product that ships its own plugin keeps it beside the binary it describes, at the binary's
//! version. This marketplace points at it with a `git-subdir` entry naming the product repository
//! and the plugin directory — and nothing else. It names no ref, commit or version, so nothing here
//! has to change when the product releases: the host installs whatever the product's default branch
//! serves, and `b10x setup` matches the binary to that plugin's version.
//!
//! - [`shape`] runs in the offline gate and refuses an entry that is not an unpinned `git-subdir`
//!   at the declared repository and path, in either marketplace file.
//! - [`verify`] runs with network access (`agentplugins-check remote`, on every pull request, `main`
//!   push and a daily schedule) and refuses a default branch whose plugin manifests name another
//!   plugin, disagree on the version, or declare a version the repository never released.

use std::path::Path;
use std::process::Command;

/// One plugin that lives in its product's own repository.
pub struct Remote {
    /// Plugin and manifest name.
    pub name: &'static str,
    /// The only repository the entry may point at.
    pub url: &'static str,
    /// `owner/repo`.
    pub repository: &'static str,
    /// The plugin directory inside that repository.
    pub path: &'static str,
}

/// Every plugin this marketplace lists from another repository, in marketplace order after the
/// plugins it carries.
pub const REMOTE: &[Remote] = &[
    Remote {
        name: "worktree",
        url: "https://github.com/beyond10x/worktree.git",
        repository: "beyond10x/worktree",
        path: "plugins/worktree",
    },
    Remote {
        name: "ess",
        url: "https://github.com/beyond10x/ess.git",
        repository: "beyond10x/ess",
        path: "plugins/ess",
    },
];

/// Both marketplace files; each lists every remote plugin.
const MARKETPLACES: &[&str] = &[
    ".claude-plugin/marketplace.json",
    ".agents/plugins/marketplace.json",
];

/// Keys that would pin an entry to one release and make this repository track the product's.
const PINS: &[&str] = &["ref", "sha", "version"];

fn bare_version(tag: &str) -> Option<(u64, u64, u64)> {
    let mut parts = tag.trim_start_matches('v').split('.');
    let version = (
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    );
    parts.next().is_none().then_some(version)
}

/// Check every remote entry's shape in both marketplace files, without the network.
pub fn shape(root: &Path) -> Result<(), String> {
    for relative in MARKETPLACES {
        let document = super::json(&root.join(relative))?;
        let entries = document
            .get("plugins")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("{relative} has no plugins array"))?;
        for remote in REMOTE {
            let entry = entries
                .iter()
                .find(|entry| {
                    entry.get("name").and_then(serde_json::Value::as_str) == Some(remote.name)
                })
                .ok_or_else(|| format!("{relative} does not list `{}`", remote.name))?;
            let source = entry
                .get("source")
                .ok_or_else(|| format!("{relative}: `{}` has no source", remote.name))?;
            let field = |key: &str| source.get(key).and_then(serde_json::Value::as_str);
            if field("source") != Some("git-subdir") {
                return Err(format!(
                    "{relative}: `{}` source is not `git-subdir`",
                    remote.name
                ));
            }
            let path = field("path").map(|path| path.trim_start_matches("./"));
            if field("url") != Some(remote.url) || path != Some(remote.path) {
                return Err(format!(
                    "{relative}: `{}` must point at {} path {}",
                    remote.name, remote.url, remote.path
                ));
            }
            for pin in PINS {
                if source.get(pin).is_some() || entry.get(pin).is_some() {
                    return Err(format!(
                        "{relative}: `{}` declares `{pin}`; remote entries follow the product's default branch and name no version",
                        remote.name
                    ));
                }
            }
        }
    }
    Ok(())
}

fn curl(url: &str) -> Result<String, String> {
    let output = Command::new("curl")
        .args(["-fsSL", "--retry", "2", "--max-time", "60", url])
        .output()
        .map_err(|error| format!("running curl: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "curl {url} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn release_tags(url: &str) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-remote", "--tags", "--refs", url])
        .output()
        .map_err(|error| format!("running git ls-remote: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git ls-remote {url} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_once("refs/tags/").map(|(_, tag)| tag.to_owned()))
        .filter(|tag| bare_version(tag).is_some())
        .collect())
}

/// Check every remote plugin on its repository's default branch. Needs the network, `curl` and `git`.
pub fn verify(root: &Path) -> Result<(), String> {
    shape(root)?;
    for remote in REMOTE {
        let name = remote.name;
        let mut versions = Vec::new();
        for manifest in [".claude-plugin/plugin.json", ".codex-plugin/plugin.json"] {
            let url = format!(
                "https://raw.githubusercontent.com/{}/HEAD/{}/{manifest}",
                remote.repository, remote.path
            );
            let document: serde_json::Value = serde_json::from_str(&curl(&url)?)
                .map_err(|error| format!("`{name}` {manifest}: {error}"))?;
            if document.get("name").and_then(serde_json::Value::as_str) != Some(name) {
                return Err(format!("`{name}` {manifest} carries another plugin name"));
            }
            let version = document
                .get("version")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("`{name}` {manifest} declares no version"))?
                .to_owned();
            versions.push(version);
        }
        if versions[0] != versions[1] {
            return Err(format!(
                "`{name}`: the Claude manifest says {} and the Codex manifest {}",
                versions[0], versions[1]
            ));
        }
        let version = &versions[0];
        let released = release_tags(remote.url)?
            .iter()
            .any(|tag| bare_version(tag) == bare_version(version));
        if !released {
            return Err(format!(
                "`{name}`: {}'s default branch serves plugin version {version}, which it never released",
                remote.repository
            ));
        }
        println!("remote `{name}`: default branch serves released version {version}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch(std::path::PathBuf);

    impl Scratch {
        fn with(claude: &serde_json::Value, codex: &serde_json::Value) -> Self {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "agentplugins-check-remote-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::SeqCst)
            ));
            for (relative, document) in [(MARKETPLACES[0], claude), (MARKETPLACES[1], codex)] {
                let file = path.join(relative);
                std::fs::create_dir_all(file.parent().unwrap()).unwrap();
                std::fs::write(file, document.to_string()).unwrap();
            }
            Self(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn entries(path_prefix: &str) -> serde_json::Value {
        serde_json::json!({"plugins": REMOTE.iter().map(|remote| serde_json::json!({
            "name": remote.name,
            "source": {"source": "git-subdir", "url": remote.url, "path": format!("{path_prefix}{}", remote.path)},
        })).collect::<Vec<_>>()})
    }

    #[test]
    fn unpinned_entries_in_both_files_pass() {
        let scratch = Scratch::with(&entries(""), &entries("./"));
        shape(&scratch.0).expect("unpinned entries");
    }

    #[test]
    fn a_pin_or_a_redirect_is_refused() {
        for (key, value) in [
            ("ref", serde_json::json!("0.6.0")),
            ("sha", serde_json::json!("a".repeat(40))),
            (
                "url",
                serde_json::json!("https://github.com/someone/worktree.git"),
            ),
            ("path", serde_json::json!("plugins/other")),
            ("source", serde_json::json!("github")),
        ] {
            let mut claude = entries("");
            claude["plugins"][0]["source"][key] = value;
            let scratch = Scratch::with(&claude, &entries("./"));
            assert!(shape(&scratch.0).is_err(), "{key} must be refused");
        }
    }

    #[test]
    fn a_missing_codex_entry_is_refused() {
        let scratch = Scratch::with(&entries(""), &serde_json::json!({"plugins": []}));
        assert!(shape(&scratch.0).is_err());
    }
}
