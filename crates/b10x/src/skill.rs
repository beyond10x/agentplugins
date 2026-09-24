//! `b10x skill`: print an installed skill or agent. A host loads a newly installed plugin only in a
//! new session, so the session that ran setup reads the skill it needs through this instead of
//! searching the host's plugin cache.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::version;

/// Where each installed `b10x` plugin lives: Claude Code's recorded install path first, then the
/// newest version in Codex's cache.
#[must_use]
pub fn root(home: &Path, marketplace: &str, plugin: &str) -> Option<PathBuf> {
    let id = format!("{plugin}@{marketplace}");
    let claude = std::fs::read_to_string(home.join(".claude/plugins/installed_plugins.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|value| {
            let installs = value.get("plugins")?.get(&id)?.as_array()?.clone();
            installs
                .iter()
                .find(|install| {
                    install.get("scope").and_then(serde_json::Value::as_str) == Some("user")
                })
                .or_else(|| installs.first())
                .and_then(|install| install.get("installPath"))
                .and_then(serde_json::Value::as_str)
                .map(PathBuf::from)
        })
        .filter(|path| path.is_dir());
    claude.or_else(|| {
        let cache = home
            .join(".codex/plugins/cache")
            .join(marketplace)
            .join(plugin);
        std::fs::read_dir(&cache)
            .ok()?
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .filter_map(|name| version::key(&name).map(|key| (key, name)))
            .max()
            .map(|(_, name)| cache.join(name))
    })
}

fn names(directory: &Path, file: Option<&str>) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let path = entry.path();
                    match file {
                        Some(file) => path.join(file).is_file().then(|| entry.file_name()),
                        None => (path.extension().and_then(|e| e.to_str()) == Some("md"))
                            .then(|| path.file_stem().map(std::ffi::OsStr::to_os_string))
                            .flatten(),
                    }
                })
                .filter_map(|name| name.into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The text `b10x skill <id>` prints: a plugin's skills and agents for `plugin`, one file's
/// contents for `plugin:name`.
pub fn text(home: &Path, marketplace: &str, id: &str) -> Result<String, String> {
    let (plugin, name) = match id.split_once(':') {
        Some((plugin, name)) => (plugin, Some(name)),
        None => (id, None),
    };
    let root = root(home, marketplace, plugin).ok_or_else(|| {
        format!("`{plugin}@{marketplace}` is not installed for Claude Code or Codex; run `b10x setup plan`")
    })?;
    let Some(name) = name else {
        let skills = names(&root.join("skills"), Some("SKILL.md"));
        let agents = names(&root.join("agents"), None);
        let mut out = format!("{plugin} ({})\n", root.display());
        for skill in skills {
            let _ = writeln!(out, "  skill  {plugin}:{skill}");
        }
        for agent in agents {
            let _ = writeln!(out, "  agent  {plugin}:{agent}");
        }
        return Ok(out);
    };
    for candidate in [
        root.join("skills").join(name).join("SKILL.md"),
        root.join("agents").join(format!("{name}.md")),
    ] {
        if let Ok(text) = std::fs::read_to_string(&candidate) {
            return Ok(text);
        }
    }
    Err(format!(
        "`{plugin}` has no skill or agent `{name}`; `b10x skill {plugin}` lists them"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch() -> PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "b10x-skill-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn claude_install_path_is_read_and_skills_listed() {
        let home = scratch();
        let install = home.join("cache/ess/0.30.0");
        std::fs::create_dir_all(install.join("skills/specify")).unwrap();
        std::fs::write(
            install.join("skills/specify/SKILL.md"),
            "---\nname: specify\n---\nbody",
        )
        .unwrap();
        std::fs::create_dir_all(install.join("agents")).unwrap();
        std::fs::write(install.join("agents/author.md"), "agent").unwrap();
        std::fs::create_dir_all(home.join(".claude/plugins")).unwrap();
        let recorded = serde_json::json!({"plugins": {"ess@b10x": [{"scope": "user", "installPath": install}]}});
        std::fs::write(
            home.join(".claude/plugins/installed_plugins.json"),
            recorded.to_string(),
        )
        .unwrap();

        let listing = text(&home, "b10x", "ess").unwrap();
        assert!(
            listing.contains("skill  ess:specify") && listing.contains("agent  ess:author"),
            "{listing}"
        );
        assert!(text(&home, "b10x", "ess:specify")
            .unwrap()
            .ends_with("body"));
        assert_eq!(text(&home, "b10x", "ess:author").unwrap(), "agent");
        assert!(text(&home, "b10x", "ess:nothing").is_err());
        std::fs::remove_dir_all(&home).unwrap();
    }

    #[test]
    fn codex_cache_takes_the_newest_version() {
        let home = scratch();
        for version in ["0.9.0", "0.30.0"] {
            let dir = home
                .join(".codex/plugins/cache/b10x/ess")
                .join(version)
                .join("skills/specify");
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("SKILL.md"), version).unwrap();
        }
        assert_eq!(text(&home, "b10x", "ess:specify").unwrap(), "0.30.0");
        assert!(text(&home, "b10x", "aep").is_err());
        std::fs::remove_dir_all(&home).unwrap();
    }
}
