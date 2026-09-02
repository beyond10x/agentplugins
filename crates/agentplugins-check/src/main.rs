//! Validates the curated marketplace identity and focused plugin contents.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const PLUGINS: &[(&str, &[&str])] = &[
    (
        "beyond10x",
        &[
            "skills/beyond10x/SKILL.md",
            "skills/beyond10x/references/resources.md",
            "skills/plugin-creator/SKILL.md",
            "skills/plugin-creator/references/compatibility.md",
        ],
    ),
    (
        "aep-planning",
        &[
            "skills/planning/SKILL.md",
            "skills/planning/references/critic-rubric.md",
            "skills/story-migration/SKILL.md",
            "agents/decomposer.md",
            "agents/plan-reviewer.md",
            "agents/reverse-engineer.md",
            "agents/plan-critic-acceptance.md",
            "agents/plan-critic-design.md",
            "agents/plan-critic-scope.md",
            "agents/plan-critic-parallel-safety.md",
        ],
    ),
    (
        "adp",
        &[
            "skills/wave/SKILL.md",
            "agents/story-scoper.md",
            "agents/implementor.md",
            "agents/adversary.md",
        ],
    ),
    ("ess-schema", &["skills/schema-validation/SKILL.md"]),
    ("workspace-hygiene", &["skills/worktree/SKILL.md"]),
];

fn json(path: &Path) -> Result<serde_json::Value, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("parsing {}: {error}", path.display()))
}

fn marketplace(root: &Path, relative: &str) -> Result<(), String> {
    let document = json(&root.join(relative))?;
    if document.get("name").and_then(serde_json::Value::as_str) != Some("beyond10x") {
        return Err(format!(
            "{relative} does not declare marketplace `beyond10x`"
        ));
    }
    let entries = document
        .get("plugins")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{relative} has no plugins array"))?;
    if entries.len() != PLUGINS.len() {
        return Err(format!(
            "{relative} contains {} plugins; expected {} focused plugins",
            entries.len(),
            PLUGINS.len()
        ));
    }
    for (index, (plugin, _)) in PLUGINS.iter().enumerate() {
        let actual = entries[index]
            .get("name")
            .and_then(serde_json::Value::as_str);
        if actual != Some(plugin) {
            return Err(format!(
                "{relative} plugin {index} is {actual:?}; expected `{plugin}`"
            ));
        }
        let count = entries
            .iter()
            .filter(|entry| entry.get("name").and_then(serde_json::Value::as_str) == Some(plugin))
            .count();
        if count != 1 {
            return Err(format!(
                "{relative} contains {count} entries for `{plugin}`"
            ));
        }
    }
    Ok(())
}

fn plugin(root: &Path, name: &str, required: &[&str]) -> Result<(), String> {
    let directory = root.join("plugins").join(name);
    for relative in [".codex-plugin/plugin.json", ".claude-plugin/plugin.json"] {
        let document = json(&directory.join(relative))?;
        if document.get("name").and_then(serde_json::Value::as_str) != Some(name) {
            return Err(format!("{name}/{relative} carries another plugin name"));
        }
        if document.get("version").and_then(serde_json::Value::as_str)
            != Some(env!("CARGO_PKG_VERSION"))
        {
            return Err(format!(
                "{name}/{relative} does not carry workspace version {}",
                env!("CARGO_PKG_VERSION")
            ));
        }
    }
    for relative in required {
        if !directory.join(relative).is_file() {
            return Err(format!("plugin `{name}` is missing `{relative}`"));
        }
    }
    Ok(())
}

fn check(root: &Path) -> Result<(), String> {
    marketplace(root, ".agents/plugins/marketplace.json")?;
    marketplace(root, ".claude-plugin/marketplace.json")?;
    for (name, required) in PLUGINS {
        plugin(root, name, required)?;
    }
    Ok(())
}

fn verify_release(root: &Path, version: &str) -> Result<(), String> {
    if version != env!("CARGO_PKG_VERSION") {
        return Err(format!(
            "tag `{version}` does not match workspace version {}",
            env!("CARGO_PKG_VERSION")
        ));
    }

    let changelog = std::fs::read_to_string(root.join("CHANGELOG.md"))
        .map_err(|error| format!("reading CHANGELOG.md: {error}"))?;
    let heading = format!("## [{version}] — ");
    if !changelog.lines().any(|line| line.starts_with(&heading)) {
        return Err(format!("CHANGELOG.md has no dated `{heading}` heading"));
    }

    let object_type = Command::new("git")
        .args(["cat-file", "-t", &format!("refs/tags/{version}")])
        .current_dir(root)
        .output()
        .map_err(|error| format!("running git cat-file: {error}"))?;
    if !object_type.status.success() || object_type.stdout != b"tag\n" {
        return Err(format!("`{version}` is missing or is not an annotated tag"));
    }

    Ok(())
}

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("checker is under the repository root")
        .to_path_buf();
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let result = match arguments.as_slice() {
        [] => check(&root),
        [release, verify, version] if release == "release" && verify == "verify" => {
            check(&root).and_then(|()| verify_release(&root, version))
        }
        _ => Err("usage: agentplugins-check [release verify <version>]".to_owned()),
    };

    match result {
        Ok(()) => {
            println!(
                "valid: marketplace beyond10x, {} focused plugin(s)",
                PLUGINS.len()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_committed_marketplace_is_valid() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        check(root).expect("the committed marketplace validates");
    }

    /// The four critics and the rubric they share are required content, not optional extras: a
    /// panel missing one perspective still returns four confident verdicts, and nothing else in
    /// this repository would notice the file had gone.
    #[test]
    fn a_deleted_critic_fails_the_check() {
        let required: &[&str] = PLUGINS
            .iter()
            .find(|(name, _)| *name == "aep-planning")
            .map(|(_, required)| *required)
            .expect("aep-planning is one of the focused plugins");
        let critic = "agents/plan-critic-design.md";
        assert!(
            required.contains(&critic),
            "the critic panel must be required content"
        );

        let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        let sandbox =
            std::env::temp_dir().join(format!("agentplugins-check-critic-{}", std::process::id()));
        let plugin_root = sandbox.join("plugins").join("aep-planning");
        let write = |target: &Path, bytes: &str| {
            std::fs::create_dir_all(target.parent().expect("every entry has a directory"))
                .expect("the sandbox is writable");
            std::fs::write(target, bytes).expect("the sandbox is writable");
        };
        for manifest in [".codex-plugin/plugin.json", ".claude-plugin/plugin.json"] {
            let committed =
                std::fs::read_to_string(repository.join("plugins/aep-planning").join(manifest))
                    .expect("the committed manifest is readable");
            write(&plugin_root.join(manifest), &committed);
        }
        for relative in required.iter().filter(|relative| **relative != critic) {
            write(&plugin_root.join(relative), "");
        }

        let error = plugin(&sandbox, "aep-planning", required)
            .expect_err("a plugin missing one of its critics must fail the check");
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        assert_eq!(
            error,
            format!("plugin `aep-planning` is missing `{critic}`")
        );
    }

    #[test]
    fn a_release_version_must_match_the_workspace() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        let error = verify_release(root, "9.9.9").expect_err("a mismatched version must fail");
        assert!(error.contains("does not match workspace version"));
    }
}
