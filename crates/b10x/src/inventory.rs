//! What is installed right now: plugins and marketplaces per host, binaries on `PATH`, and plugin
//! entries in project settings files. Parsing is separate from collection so the parsers can be
//! tested on recorded host output.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::catalog::Catalog;
use crate::version;

/// An agent host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Host {
    /// Claude Code, `claude`.
    Claude,
    /// Codex, `codex`.
    Codex,
}

impl Host {
    /// The executable.
    #[must_use]
    pub fn program(self) -> &'static str {
        match self {
            Host::Claude => "claude",
            Host::Codex => "codex",
        }
    }
}

/// Everything setup reads before deciding.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Inventory {
    /// Claude Code, when `claude` runs.
    pub claude: Option<HostState>,
    /// Codex, when `codex` runs.
    pub codex: Option<HostState>,
    /// Every catalog binary, every copy on `PATH` in `PATH` order.
    pub binaries: Vec<BinaryState>,
    /// Plugin entries in settings files the host list does not report (other projects, orphans).
    pub settings: Vec<SettingsEntry>,
}

/// One host's plugins and marketplaces.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct HostState {
    /// Installed plugins.
    pub plugins: Vec<Installed>,
    /// Registered marketplaces.
    pub marketplaces: Vec<Market>,
}

/// One installed plugin at one scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Installed {
    /// Plugin name.
    pub name: String,
    /// Marketplace name.
    pub marketplace: String,
    /// Installed version, when the host reports one.
    pub version: Option<String>,
    /// `user`, `project` or `local` (Codex: `user`).
    pub scope: String,
    /// Project directory for `project` and `local` scope.
    pub project: Option<String>,
    /// Whether it is enabled.
    pub enabled: bool,
}

impl Installed {
    /// `name@marketplace`.
    #[must_use]
    pub fn id(&self) -> String {
        format!("{}@{}", self.name, self.marketplace)
    }
}

/// One registered marketplace.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Market {
    /// Marketplace name.
    pub name: String,
    /// `owner/repo`, a Git URL or a directory.
    pub source: String,
    /// Pinned ref, when the registration names one.
    pub reference: Option<String>,
    /// Local clone, when the host reports one.
    pub location: Option<String>,
}

/// Every copy of one binary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BinaryState {
    /// Executable name.
    pub name: String,
    /// Copies in `PATH` order; the first is the one that runs.
    pub copies: Vec<Copy>,
}

/// One copy of a binary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Copy {
    /// Absolute path.
    pub path: String,
    /// Version its `--version` printed.
    pub version: Option<String>,
}

/// A plugin entry in a Claude Code settings file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SettingsEntry {
    /// The settings file.
    pub file: String,
    /// `project` (`.claude/settings.json`, committed) or `local` or `user`.
    pub scope: String,
    /// The project directory, for project and local files.
    pub project: Option<String>,
    /// `name@marketplace`.
    pub plugin: String,
    /// The value under `enabledPlugins`.
    pub enabled: bool,
}

fn split_id(id: &str) -> Option<(String, String)> {
    let (name, marketplace) = id.rsplit_once('@')?;
    Some((name.to_owned(), marketplace.to_owned()))
}

fn string(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
}

/// Parse `claude plugin list --json`.
pub fn parse_claude_plugins(text: &str) -> Result<Vec<Installed>, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|error| format!("claude plugin list: {error}"))?;
    let entries = value.as_array().ok_or("claude plugin list: not an array")?;
    let mut plugins = Vec::new();
    for entry in entries {
        let id = string(entry, "id").ok_or("claude plugin list: entry without id")?;
        let Some((name, marketplace)) = split_id(&id) else {
            continue;
        };
        plugins.push(Installed {
            name,
            marketplace,
            version: string(entry, "version"),
            scope: string(entry, "scope").unwrap_or_else(|| "user".to_owned()),
            project: string(entry, "projectPath"),
            enabled: entry
                .get("enabled")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true),
        });
    }
    plugins.sort();
    Ok(plugins)
}

/// Parse `claude plugin marketplace list --json`.
pub fn parse_claude_marketplaces(text: &str) -> Result<Vec<Market>, String> {
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|error| format!("claude plugin marketplace list: {error}"))?;
    let entries = value
        .as_array()
        .ok_or("claude plugin marketplace list: not an array")?;
    let mut markets = Vec::new();
    for entry in entries {
        let name = string(entry, "name").ok_or("claude marketplace without name")?;
        let source = string(entry, "repo")
            .or_else(|| string(entry, "url"))
            .or_else(|| string(entry, "path"))
            .unwrap_or_default();
        markets.push(Market {
            name,
            source,
            reference: string(entry, "ref"),
            location: string(entry, "installLocation"),
        });
    }
    markets.sort();
    Ok(markets)
}

/// Parse `codex plugin list --json`.
pub fn parse_codex_plugins(text: &str) -> Result<Vec<Installed>, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|error| format!("codex plugin list: {error}"))?;
    let entries = value
        .get("installed")
        .and_then(serde_json::Value::as_array)
        .ok_or("codex plugin list: no `installed` array")?;
    let mut plugins = Vec::new();
    for entry in entries {
        let id = string(entry, "pluginId").ok_or("codex plugin list: entry without pluginId")?;
        let Some((name, marketplace)) = split_id(&id) else {
            continue;
        };
        plugins.push(Installed {
            name,
            marketplace,
            version: string(entry, "version"),
            scope: "user".to_owned(),
            project: None,
            enabled: entry
                .get("enabled")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true),
        });
    }
    plugins.sort();
    Ok(plugins)
}

/// Parse `codex plugin marketplace list --json`, taking pinned refs from `~/.codex/config.toml`
/// because the list does not report them.
pub fn parse_codex_marketplaces(text: &str, config: Option<&str>) -> Result<Vec<Market>, String> {
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|error| format!("codex plugin marketplace list: {error}"))?;
    let entries = value
        .get("marketplaces")
        .and_then(serde_json::Value::as_array)
        .ok_or("codex plugin marketplace list: no `marketplaces` array")?;
    let config: Option<toml::Table> = config.and_then(|text| text.parse().ok());
    let mut markets = Vec::new();
    for entry in entries {
        let name = string(entry, "name").ok_or("codex marketplace without name")?;
        let source = entry
            .get("marketplaceSource")
            .and_then(|source| string(source, "source"))
            .unwrap_or_default();
        let reference = config
            .as_ref()
            .and_then(|table| table.get("marketplaces"))
            .and_then(|markets| markets.get(&name))
            .and_then(|market| market.get("ref"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned);
        markets.push(Market {
            name,
            source,
            reference,
            location: string(entry, "root"),
        });
    }
    markets.sort();
    Ok(markets)
}

/// Plugin entries under `enabledPlugins` in one Claude Code settings file that the catalog knows or
/// that come from a marketplace the catalog retired.
pub fn parse_settings(
    text: &str,
    file: &str,
    scope: &str,
    project: Option<&str>,
    catalog: &Catalog,
) -> Vec<SettingsEntry> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
        return Vec::new();
    };
    let Some(enabled) = value
        .get("enabledPlugins")
        .and_then(serde_json::Value::as_object)
    else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for (plugin, state) in enabled {
        let Some((name, marketplace)) = split_id(plugin) else {
            continue;
        };
        let relevant = marketplace == catalog.marketplace.name
            || catalog.retired_marketplace(&marketplace)
            || catalog.retired_plugins.contains_key(&name);
        if relevant {
            entries.push(SettingsEntry {
                file: file.to_owned(),
                scope: scope.to_owned(),
                project: project.map(str::to_owned),
                plugin: plugin.clone(),
                enabled: state.as_bool().unwrap_or(false),
            });
        }
    }
    entries.sort();
    entries
}

/// Output of a command, or `None` when it cannot run or fails.
fn output(program: &str, arguments: &[&str]) -> Option<String> {
    let output = Command::new(program).args(arguments).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The user's home directory.
///
/// # Panics
/// When `HOME` is unset, which no supported host allows.
#[must_use]
pub fn home() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").expect("HOME is set"))
}

fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Every executable called `name` on `PATH`, in `PATH` order, each canonical path once.
#[must_use]
pub fn copies_on_path(name: &str) -> Vec<Copy> {
    let mut seen = BTreeSet::new();
    let mut copies = Vec::new();
    let Some(path) = std::env::var_os("PATH") else {
        return copies;
    };
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join(name);
        let Ok(metadata) = std::fs::metadata(&candidate) else {
            continue;
        };
        if !metadata.is_file() || !executable(&metadata) {
            continue;
        }
        let canonical = std::fs::canonicalize(&candidate).unwrap_or_else(|_| candidate.clone());
        if !seen.insert(canonical) {
            continue;
        }
        let shown = candidate.to_string_lossy().into_owned();
        let version = output(&shown, &["--version"]).and_then(|text| version::parse(&text));
        copies.push(Copy {
            path: shown,
            version,
        });
    }
    copies
}

#[cfg(unix)]
fn executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn executable(_: &std::fs::Metadata) -> bool {
    true
}

/// Read one host, or `None` when its executable does not run.
fn host_state(host: Host, home: &Path) -> Option<Result<HostState, String>> {
    let program = host.program();
    let plugins = output(program, &["plugin", "list", "--json"])?;
    let markets = output(program, &["plugin", "marketplace", "list", "--json"])?;
    Some(match host {
        Host::Claude => parse_claude_plugins(&plugins).and_then(|plugins| {
            Ok(HostState {
                plugins,
                marketplaces: parse_claude_marketplaces(&markets)?,
            })
        }),
        Host::Codex => {
            let config = read(&home.join(".codex/config.toml"));
            parse_codex_plugins(&plugins).and_then(|plugins| {
                Ok(HostState {
                    plugins,
                    marketplaces: parse_codex_marketplaces(&markets, config.as_deref())?,
                })
            })
        }
    })
}

/// Claude Code settings files that can enable a plugin: the user file and each known project's
/// shared and local file.
fn settings_entries(home: &Path, catalog: &Catalog) -> Vec<SettingsEntry> {
    let mut entries = Vec::new();
    let user = home.join(".claude/settings.json");
    if let Some(text) = read(&user) {
        entries.extend(parse_settings(
            &text,
            &user.to_string_lossy(),
            "user",
            None,
            catalog,
        ));
    }
    let projects: Vec<String> = read(&home.join(".claude.json"))
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|value| {
            value
                .get("projects")
                .and_then(serde_json::Value::as_object)
                .map(|projects| projects.keys().cloned().collect())
        })
        .unwrap_or_default();
    for project in projects {
        for (file, scope) in [
            ("settings.json", "project"),
            ("settings.local.json", "local"),
        ] {
            let path = Path::new(&project).join(".claude").join(file);
            if let Some(text) = read(&path) {
                entries.extend(parse_settings(
                    &text,
                    &path.to_string_lossy(),
                    scope,
                    Some(&project),
                    catalog,
                ));
            }
        }
    }
    entries.sort();
    entries.dedup();
    entries
}

/// Read everything.
pub fn collect(catalog: &Catalog, hosts: &[Host]) -> Result<Inventory, String> {
    let home = home();
    let mut inventory = Inventory::default();
    for host in hosts {
        let state = host_state(*host, &home).transpose()?;
        match host {
            Host::Claude => inventory.claude = state,
            Host::Codex => inventory.codex = state,
        }
    }
    let mut names: Vec<&str> = catalog
        .products
        .iter()
        .flat_map(|product| product.binaries.iter().map(|binary| binary.name.as_str()))
        .collect();
    names.push("b10x");
    names.sort_unstable();
    names.dedup();
    inventory.binaries = names
        .into_iter()
        .map(|name| BinaryState {
            name: name.to_owned(),
            copies: copies_on_path(name),
        })
        .collect();
    if inventory.claude.is_some() {
        inventory.settings = settings_entries(&home, catalog);
    }
    Ok(inventory)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLAUDE_LIST: &str = include_str!("../tests/fixtures/claude-legacy-list.json");
    const CLAUDE_MARKETS: &str = include_str!("../tests/fixtures/claude-legacy-marketplaces.json");
    const CODEX_LIST: &str = include_str!("../tests/fixtures/codex-legacy-list.json");
    const CODEX_MARKETS: &str = include_str!("../tests/fixtures/codex-legacy-marketplaces.json");
    const CODEX_CONFIG: &str = include_str!("../tests/fixtures/codex-legacy-config.toml");

    #[test]
    fn recorded_claude_output_parses() {
        let plugins = parse_claude_plugins(CLAUDE_LIST).unwrap();
        assert!(plugins
            .iter()
            .any(|p| p.id() == "workspace-hygiene@beyond10x"
                && p.version.as_deref() == Some("0.10.0")));
        let markets = parse_claude_marketplaces(CLAUDE_MARKETS).unwrap();
        let legacy = markets.iter().find(|m| m.name == "beyond10x").unwrap();
        assert_eq!(legacy.reference.as_deref(), Some("0.10.0"));
    }

    #[test]
    fn recorded_codex_output_parses_with_refs_from_config() {
        let plugins = parse_codex_plugins(CODEX_LIST).unwrap();
        assert!(plugins.iter().any(|p| p.id() == "aep-plan@beyond10x"));
        let markets = parse_codex_marketplaces(CODEX_MARKETS, Some(CODEX_CONFIG)).unwrap();
        assert_eq!(markets[0].reference.as_deref(), Some("0.10.0"));
    }

    #[test]
    fn settings_keep_only_entries_the_catalog_cares_about() {
        let catalog = Catalog::embedded();
        let text = r#"{"enabledPlugins":{"beyond10x@beyond10x":true,"ess-schema@beyond10x":false,"brain@org-brain":true,"worktree@b10x":true}}"#;
        let entries = parse_settings(
            text,
            "/p/.claude/settings.local.json",
            "local",
            Some("/p"),
            &catalog,
        );
        let names: Vec<&str> = entries.iter().map(|e| e.plugin.as_str()).collect();
        assert_eq!(
            names,
            [
                "beyond10x@beyond10x",
                "ess-schema@beyond10x",
                "worktree@b10x"
            ]
        );
    }
}
