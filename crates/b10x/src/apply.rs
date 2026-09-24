//! Applying a plan: refuse a stale one, snapshot what can be restored, run each action in order,
//! stop at the first failure, then read the state again and say whether it converged.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::inventory::{home, Inventory};
use crate::plan::{digest, Action, Plan};

/// What a snapshot holds: original path → copy inside the snapshot directory.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    /// Files that existed, and their copies.
    pub files: Vec<(String, String)>,
    /// Files that did not exist and are removed on undo.
    pub absent: Vec<String>,
}

/// Where snapshots live.
#[must_use]
pub fn snapshots() -> PathBuf {
    home().join(".local/state/b10x/setup")
}

fn touched(plan: &Plan) -> Vec<PathBuf> {
    let home = home();
    let mut files = vec![
        home.join(".claude/settings.json"),
        home.join(".claude/plugins/known_marketplaces.json"),
        home.join(".claude/plugins/installed_plugins.json"),
        home.join(".codex/config.toml"),
    ];
    for action in &plan.actions {
        match action {
            Action::RemoveSetting { file, .. } | Action::Unpin { file, .. } => {
                files.push(PathBuf::from(file));
            }
            Action::InstallBinary {
                name, directory, ..
            } => files.push(Path::new(directory).join(name)),
            Action::Command { cwd: Some(cwd), .. } => {
                files.push(Path::new(cwd).join(".claude/settings.json"));
                files.push(Path::new(cwd).join(".claude/settings.local.json"));
            }
            Action::Command { .. } | Action::Refresh { .. } => {}
        }
    }
    files.sort();
    files.dedup();
    files
}

/// Copy every file the plan can change into a new snapshot directory.
pub fn snapshot(plan: &Plan) -> Result<PathBuf, String> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let directory = snapshots().join(stamp.to_string());
    std::fs::create_dir_all(&directory)
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    let mut manifest = Manifest::default();
    for (index, file) in touched(plan).into_iter().enumerate() {
        if file.is_file() {
            let copy = format!("{index:03}");
            std::fs::copy(&file, directory.join(&copy))
                .map_err(|error| format!("{}: {error}", file.display()))?;
            manifest
                .files
                .push((file.to_string_lossy().into_owned(), copy));
        } else {
            manifest.absent.push(file.to_string_lossy().into_owned());
        }
    }
    let text = serde_json::to_string_pretty(&manifest).map_err(|error| error.to_string())?;
    std::fs::write(directory.join("manifest.json"), text).map_err(|error| error.to_string())?;
    Ok(directory)
}

/// Restore a snapshot.
pub fn undo(directory: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(directory.join("manifest.json"))
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    let manifest: Manifest = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    let mut restored = Vec::new();
    for (original, copy) in &manifest.files {
        std::fs::copy(directory.join(copy), original)
            .map_err(|error| format!("{original}: {error}"))?;
        restored.push(original.clone());
    }
    for absent in &manifest.absent {
        if Path::new(absent).exists() {
            std::fs::remove_file(absent).map_err(|error| format!("{absent}: {error}"))?;
            restored.push(format!("{absent} (removed)"));
        }
    }
    Ok(restored)
}

fn edit_json(file: &str, edit: impl FnOnce(&mut serde_json::Value)) -> Result<(), String> {
    let text = std::fs::read_to_string(file).unwrap_or_else(|_| "{}".to_owned());
    let mut value: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| format!("{file}: {error}"))?;
    edit(&mut value);
    let mut out = serde_json::to_string_pretty(&value).map_err(|error| error.to_string())?;
    out.push('\n');
    let incoming = format!("{file}.b10x-new");
    std::fs::write(&incoming, out).map_err(|error| format!("{incoming}: {error}"))?;
    std::fs::rename(&incoming, file).map_err(|error| format!("{file}: {error}"))
}

/// Run one action.
pub fn run(action: &Action) -> Result<(), String> {
    match action {
        Action::Refresh { argv, .. }
        | Action::Command {
            argv, cwd: None, ..
        } => command(argv, None),
        Action::Command {
            argv,
            cwd: Some(cwd),
            ..
        } => command(argv, Some(cwd)),
        Action::RemoveSetting { file, key, .. } => edit_json(file, |value| {
            if let Some(enabled) = value
                .get_mut("enabledPlugins")
                .and_then(serde_json::Value::as_object_mut)
            {
                enabled.remove(key);
            }
        }),
        Action::Unpin {
            file,
            name,
            repository,
            ..
        } => edit_json(file, |value| {
            if !value.is_object() {
                *value = serde_json::json!({});
            }
            let markets = value
                .as_object_mut()
                .map(|object| {
                    object
                        .entry("extraKnownMarketplaces")
                        .or_insert_with(|| serde_json::json!({}))
                })
                .expect("an object");
            if let Some(markets) = markets.as_object_mut() {
                markets.insert(
                    name.clone(),
                    serde_json::json!({"source": {"source": "github", "repo": repository}}),
                );
            }
        }),
        Action::InstallBinary {
            name,
            tag,
            install,
            directory,
            ..
        } => crate::install::install(name, tag, install, Path::new(directory)).map(|_| ()),
    }
}

fn command(argv: &[String], cwd: Option<&str>) -> Result<(), String> {
    let (program, arguments) = argv.split_first().ok_or("empty command")?;
    let mut command = Command::new(program);
    command.args(arguments);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command
        .output()
        .map_err(|error| format!("{program}: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!(
            "`{}` exited {}: {}",
            argv.join(" "),
            output.status.code().unwrap_or(-1),
            if stderr.trim().is_empty() {
                stdout.trim()
            } else {
                stderr.trim()
            }
        ))
    }
}

/// Refuse a plan made from another inventory.
pub fn fresh(plan: &Plan, inventory: &Inventory) -> Result<(), String> {
    if digest(inventory) == plan.inventory_digest {
        Ok(())
    } else {
        Err(
            "the installed state changed since this plan was made; run `b10x setup plan` again"
                .to_owned(),
        )
    }
}
