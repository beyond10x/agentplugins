//! Plugins this marketplace lists but does not carry.
//!
//! A product that ships its own plugin keeps the plugin beside the binary it describes, at the
//! binary's version, so the two cannot drift. This marketplace only points at it: a `git-subdir`
//! entry naming the product repository, the plugin directory, a release tag and that tag's full
//! commit. Pointing at a tag is what lets the catalog go stale instead, so the two halves below
//! check it from both sides:
//!
//! - [`shape`] runs in the offline gate and refuses an entry that is not a pinned `git-subdir` at
//!   the declared repository and path.
//! - [`verify`] runs with network access (`agentplugins-check remote`, on every pull request, `main`
//!   push and a daily schedule) and refuses a `sha` that is not the tag's commit, a manifest at that
//!   commit whose version is not the tag, and a tag that is not the repository's newest release.

use std::path::Path;
use std::process::Command;

/// One plugin that lives in its product's own repository.
pub struct Remote {
    /// Plugin and manifest name.
    pub name: &'static str,
    /// The only repository the entry may point at.
    pub url: &'static str,
    /// The plugin directory inside that repository.
    pub path: &'static str,
}

/// Every plugin this marketplace lists from another repository, in marketplace order after the
/// plugins it carries.
pub const REMOTE: &[Remote] = &[Remote {
    name: "worktree",
    url: "https://github.com/beyond10x/worktree.git",
    path: "plugins/worktree",
}];

/// The Claude Code marketplace file; Codex reads no remote entries.
const CLAUDE_MARKETPLACE: &str = ".claude-plugin/marketplace.json";

/// A pinned entry as the marketplace declares it.
pub struct Pin {
    /// Which remote plugin.
    pub remote: &'static Remote,
    /// Release tag.
    pub tag: String,
    /// The tag's full commit.
    pub sha: String,
}

fn bare_version(tag: &str) -> Option<(u64, u64, u64)> {
    let mut parts = tag.split('.');
    let version = (
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    );
    parts.next().is_none().then_some(version)
}

fn full_sha(sha: &str) -> bool {
    sha.len() == 40
        && sha
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

/// Read and check every remote entry's shape, without the network.
pub fn shape(root: &Path) -> Result<Vec<Pin>, String> {
    let document = super::json(&root.join(CLAUDE_MARKETPLACE))?;
    let entries = document
        .get("plugins")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{CLAUDE_MARKETPLACE} has no plugins array"))?;
    let mut pins = Vec::new();
    for remote in REMOTE {
        let entry = entries
            .iter()
            .find(|entry| {
                entry.get("name").and_then(serde_json::Value::as_str) == Some(remote.name)
            })
            .ok_or_else(|| format!("{CLAUDE_MARKETPLACE} does not list `{}`", remote.name))?;
        let source = entry
            .get("source")
            .ok_or_else(|| format!("`{}` has no source", remote.name))?;
        let field = |key: &str| source.get(key).and_then(serde_json::Value::as_str);
        if field("source") != Some("git-subdir") {
            return Err(format!("`{}` source is not `git-subdir`", remote.name));
        }
        if field("url") != Some(remote.url) || field("path") != Some(remote.path) {
            return Err(format!(
                "`{}` must point at {} path {}",
                remote.name, remote.url, remote.path
            ));
        }
        let tag = field("ref").unwrap_or_default();
        if bare_version(tag).is_none() {
            return Err(format!(
                "`{}` ref `{tag}` is not a bare release tag",
                remote.name
            ));
        }
        let sha = field("sha").unwrap_or_default();
        if !full_sha(sha) {
            return Err(format!(
                "`{}` sha `{sha}` is not a full commit",
                remote.name
            ));
        }
        if entry.get("version").is_some() {
            return Err(format!(
                "`{}` declares a version; the manifest at the pinned commit owns it",
                remote.name
            ));
        }
        pins.push(Pin {
            remote,
            tag: tag.to_owned(),
            sha: sha.to_owned(),
        });
    }
    Ok(pins)
}

fn git(arguments: &[&str], directory: Option<&Path>) -> Result<String, String> {
    let mut command = Command::new("git");
    command.args(arguments);
    if let Some(directory) = directory {
        command.current_dir(directory);
    }
    let output = command
        .output()
        .map_err(|error| format!("running git {}: {error}", arguments.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout).map_err(|error| format!("git output: {error}"))
}

/// The commit each release tag of `url` points at, peeled through annotated tags.
fn tags(url: &str) -> Result<Vec<(String, String)>, String> {
    let listing = git(&["ls-remote", "--tags", url], None)?;
    let mut direct = std::collections::BTreeMap::new();
    let mut peeled = std::collections::BTreeMap::new();
    for line in listing.lines() {
        let Some((sha, reference)) = line.split_once('\t') else {
            continue;
        };
        let Some(name) = reference.strip_prefix("refs/tags/") else {
            continue;
        };
        match name.strip_suffix("^{}") {
            Some(name) => peeled.insert(name.to_owned(), sha.to_owned()),
            None => direct.insert(name.to_owned(), sha.to_owned()),
        };
    }
    Ok(direct
        .into_iter()
        .filter(|(name, _)| bare_version(name).is_some())
        .map(|(name, sha)| {
            let commit = peeled.remove(&name).unwrap_or(sha);
            (name, commit)
        })
        .collect())
}

/// Check every pin against its repository. Needs the network and `git`.
pub fn verify(root: &Path) -> Result<(), String> {
    for pin in shape(root)? {
        let name = pin.remote.name;
        let releases = tags(pin.remote.url)?;
        let commit = releases
            .iter()
            .find(|(tag, _)| *tag == pin.tag)
            .map(|(_, commit)| commit)
            .ok_or_else(|| format!("`{name}`: {} has no tag `{}`", pin.remote.url, pin.tag))?;
        if *commit != pin.sha {
            return Err(format!(
                "`{name}`: sha {} is not tag `{}` ({commit})",
                pin.sha, pin.tag
            ));
        }
        let newest = releases
            .iter()
            .filter_map(|(tag, _)| bare_version(tag).map(|version| (version, tag)))
            .max()
            .map(|(_, tag)| tag.clone())
            .unwrap_or_default();
        if newest != pin.tag {
            return Err(format!(
                "`{name}` pins `{}`; {} has released `{newest}`",
                pin.tag, pin.remote.url
            ));
        }

        let scratch = std::env::temp_dir().join(format!(
            "agentplugins-check-{name}-{}-{}",
            std::process::id(),
            &pin.sha[..12]
        ));
        let _ = std::fs::remove_dir_all(&scratch);
        std::fs::create_dir_all(&scratch)
            .map_err(|error| format!("creating {}: {error}", scratch.display()))?;
        let result = (|| {
            git(&["init", "--quiet", "--bare"], Some(&scratch))?;
            git(
                &["fetch", "--quiet", "--depth", "1", pin.remote.url, &pin.sha],
                Some(&scratch),
            )?;
            for manifest in [".claude-plugin/plugin.json", ".codex-plugin/plugin.json"] {
                let text = git(
                    &[
                        "show",
                        &format!("{}:{}/{manifest}", pin.sha, pin.remote.path),
                    ],
                    Some(&scratch),
                )?;
                let document: serde_json::Value = serde_json::from_str(&text)
                    .map_err(|error| format!("`{name}` {manifest}: {error}"))?;
                if document.get("name").and_then(serde_json::Value::as_str) != Some(name) {
                    return Err(format!("`{name}` {manifest} carries another plugin name"));
                }
                let version = document.get("version").and_then(serde_json::Value::as_str);
                if version != Some(pin.tag.as_str()) {
                    return Err(format!(
                        "`{name}` {manifest} at {} declares version {version:?}, not `{}`",
                        pin.sha, pin.tag
                    ));
                }
            }
            Ok(())
        })();
        let _ = std::fs::remove_dir_all(&scratch);
        result?;
        println!(
            "remote `{name}`: {} at {} is the newest release",
            pin.tag, pin.sha
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_tags_are_bare_semver() {
        assert_eq!(bare_version("0.6.0"), Some((0, 6, 0)));
        assert_eq!(bare_version("v0.6.0"), None);
        assert_eq!(bare_version("0.6"), None);
        assert_eq!(bare_version("0.6.0.1"), None);
    }

    #[test]
    fn a_sha_is_forty_lowercase_hex() {
        assert!(full_sha(&"a".repeat(40)));
        assert!(!full_sha(&"A".repeat(40)));
        assert!(!full_sha(&"a".repeat(39)));
    }

    fn marketplace_with(source: &serde_json::Value) -> tempdir::Scratch {
        let scratch = tempdir::Scratch::new();
        let path = scratch.0.join(".claude-plugin");
        std::fs::create_dir_all(&path).unwrap();
        let document = serde_json::json!({
            "name": "b10x",
            "plugins": [{"name": "worktree", "source": source}],
        });
        std::fs::write(path.join("marketplace.json"), document.to_string()).unwrap();
        scratch
    }

    mod tempdir {
        pub struct Scratch(pub std::path::PathBuf);
        impl Scratch {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicUsize, Ordering};
                static NEXT: AtomicUsize = AtomicUsize::new(0);
                let path = std::env::temp_dir().join(format!(
                    "agentplugins-check-remote-test-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::SeqCst)
                ));
                std::fs::create_dir_all(&path).unwrap();
                Self(path)
            }
        }
        impl Drop for Scratch {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }

    fn pinned() -> serde_json::Value {
        serde_json::json!({
            "source": "git-subdir",
            "url": "https://github.com/beyond10x/worktree.git",
            "path": "plugins/worktree",
            "ref": "0.6.0",
            "sha": "a".repeat(40),
        })
    }

    #[test]
    fn a_pinned_git_subdir_entry_passes() {
        let scratch = marketplace_with(&pinned());
        let pins = shape(&scratch.0).expect("pinned entry");
        assert_eq!(pins[0].tag, "0.6.0");
    }

    #[test]
    fn an_unpinned_or_redirected_entry_fails() {
        for (key, value) in [
            ("sha", serde_json::json!("abc")),
            ("ref", serde_json::json!("main")),
            (
                "url",
                serde_json::json!("https://github.com/someone/worktree.git"),
            ),
            ("path", serde_json::json!("plugins/other")),
            ("source", serde_json::json!("github")),
        ] {
            let mut source = pinned();
            source[key] = value;
            let scratch = marketplace_with(&source);
            assert!(shape(&scratch.0).is_err(), "{key} change must be refused");
        }
    }
}
