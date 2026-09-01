//! Validates curated marketplace identity, focused plugin contents, and retired names.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PLUGINS: &[(&str, &[&str])] = &[
    (
        "aep-planning",
        &[
            "skills/planning/SKILL.md",
            "agents/decomposer.md",
            "agents/plan-reviewer.md",
            "agents/reverse-engineer.md",
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
    for (plugin, _) in PLUGINS {
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
    }
    for relative in required {
        if !directory.join(relative).is_file() {
            return Err(format!("plugin `{name}` is missing `{relative}`"));
        }
    }
    Ok(())
}

fn scan(root: &Path) -> Result<(), String> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("reading {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| error.to_string())?;
            if entry
                .file_type()
                .map_err(|error| error.to_string())?
                .is_dir()
            {
                if !matches!(entry.file_name().to_str(), Some(".git" | "target")) {
                    pending.push(entry.path());
                }
                continue;
            }
            let path = entry.path();
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let lower = text.to_ascii_lowercase();
            let historical = ["engineering", "protocols"].join("-");
            let retired_identities = [
                ["track", "@agentplugins"].concat(),
                ["unrelated-predecessor", "/agentplugins"].concat(),
                historical.clone(),
                format!("\"name\": \"{historical}\""),
            ];
            for retired in retired_identities {
                if lower.contains(&retired) {
                    return Err(format!(
                        "{} contains retired identity `{retired}`",
                        path.display()
                    ));
                }
            }
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
    scan(root)
}

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("checker is under the repository root")
        .to_path_buf();
    match check(&root) {
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
}
