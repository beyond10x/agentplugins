//! From an inventory and resolved versions to findings and exact actions. Pure: no I/O, so every
//! rule is tested on recorded host output.
//!
//! The user-scope result is declarative: the base plugins and the selected products' plugins end up
//! installed from the catalog marketplace at the version it serves now; a catalog plugin of an
//! unselected product is removed. Legacy installs at `project` or `local` scope are migrated in
//! place, whatever the selection, because they belong to a project rather than to the user.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::catalog::{Bind, Catalog, Install};
use crate::inventory::{Host, HostState, Installed, Inventory};
use crate::resolve::Resolved;
use crate::version;

/// The plan document format.
pub const FORMAT: &str = "b10x.setup-plan/1";

/// How much a finding matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    /// Already right.
    Ok,
    /// Worth knowing; nothing to do.
    Note,
    /// An action below changes it.
    Change,
    /// Needs the user's attention; setup does not change it.
    Warn,
}

/// One observation, in words.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Finding {
    /// How much it matters.
    pub level: Level,
    /// Host it concerns, if any.
    pub host: Option<Host>,
    /// Plugin, marketplace, binary or file.
    pub subject: String,
    /// What was found and what happens.
    pub detail: String,
}

/// One step `apply` performs. Commands are argument vectors, never shell text.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Action {
    /// Refresh the marketplace snapshot; changes nothing the user chose.
    Refresh {
        /// Host.
        host: Host,
        /// Program and arguments.
        argv: Vec<String>,
    },
    /// Run a host command.
    Command {
        /// Host.
        host: Host,
        /// Program and arguments.
        argv: Vec<String>,
        /// Directory to run in (project and local scope).
        cwd: Option<String>,
        /// Why.
        reason: String,
    },
    /// Remove one `enabledPlugins` key from a Claude Code settings file.
    RemoveSetting {
        /// Settings file.
        file: String,
        /// `name@marketplace`.
        key: String,
        /// Why.
        reason: String,
    },
    /// Replace a pinned marketplace declaration in Claude Code user settings with an unpinned one.
    Unpin {
        /// Settings file.
        file: String,
        /// Marketplace name.
        name: String,
        /// `owner/repo`.
        repository: String,
        /// Why.
        reason: String,
    },
    /// Install a binary at an exact release.
    InstallBinary {
        /// Executable name.
        name: String,
        /// Release tag.
        tag: String,
        /// How.
        install: Install,
        /// Directory it lands in.
        directory: String,
        /// Why.
        reason: String,
    },
}

impl Action {
    /// Whether this action changes something the user would notice.
    #[must_use]
    pub fn changes(&self) -> bool {
        !matches!(self, Action::Refresh { .. })
    }

    /// One line for a confirmation list.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Action::Refresh { argv, .. } => format!("refresh: {}", argv.join(" ")),
            Action::Command {
                argv, cwd, reason, ..
            } => match cwd {
                Some(cwd) => format!("{reason}: {} (in {cwd})", argv.join(" ")),
                None => format!("{reason}: {}", argv.join(" ")),
            },
            Action::RemoveSetting { file, key, reason } => {
                format!("{reason}: remove `{key}` from {file}")
            }
            Action::Unpin {
                file, name, reason, ..
            } => format!("{reason}: rewrite marketplace `{name}` in {file}"),
            Action::InstallBinary {
                name,
                tag,
                directory,
                reason,
                ..
            } => format!("{reason}: install {name} {tag} into {directory}"),
        }
    }
}

/// A product as offered to the user.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Offer {
    /// Product id.
    pub id: String,
    /// One line.
    pub summary: String,
    /// Offered but not preselected.
    pub optional: bool,
    /// Present now, current or legacy, on any host.
    pub present: bool,
    /// In this plan's selection.
    pub selected: bool,
}

/// The whole plan.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Plan {
    /// Always [`FORMAT`].
    pub format: String,
    /// Digest of the inventory the plan was made from; `apply` refuses another.
    pub inventory_digest: String,
    /// Hosts planned for.
    pub hosts: Vec<Host>,
    /// Products offered, with what is selected.
    pub offers: Vec<Offer>,
    /// Versions the plan resolved.
    pub resolved: Resolved,
    /// What was found.
    pub findings: Vec<Finding>,
    /// What `apply` does, in order.
    pub actions: Vec<Action>,
}

impl Plan {
    /// Whether applying changes anything.
    #[must_use]
    pub fn converged(&self) -> bool {
        !self.actions.iter().any(Action::changes)
    }
}

/// Digest of an inventory.
///
/// # Panics
/// Never: an inventory always serializes.
#[must_use]
pub fn digest(inventory: &Inventory) -> String {
    let bytes = serde_json::to_vec(inventory).expect("an inventory serializes");
    crate::install::hex(&Sha256::digest(bytes))
}

fn argv(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_owned()).collect()
}

/// Products present now, by current or legacy plugin name, on any host.
#[must_use]
pub fn present(catalog: &Catalog, inventory: &Inventory) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let hosts = [inventory.claude.as_ref(), inventory.codex.as_ref()];
    for plugin in hosts.into_iter().flatten().flat_map(|state| &state.plugins) {
        if let Some(product) = catalog.product_of(catalog.current_name(&plugin.name)) {
            ids.insert(product.id.clone());
        }
    }
    for entry in &inventory.settings {
        if let Some((name, _)) = entry.plugin.rsplit_once('@') {
            if let Some(product) = catalog.product_of(catalog.current_name(name)) {
                ids.insert(product.id.clone());
            }
        }
    }
    ids
}

/// Everything a plan needs besides the inventory.
pub struct Context<'a> {
    /// Catalog.
    pub catalog: &'a Catalog,
    /// Resolved versions.
    pub resolved: &'a Resolved,
    /// Selected product ids; `None` means "what is present now".
    pub selection: Option<BTreeSet<String>>,
    /// Hosts to plan for.
    pub hosts: Vec<Host>,
    /// The user's home, for default install directories and settings paths.
    pub home: &'a Path,
}

/// Make the plan.
#[must_use]
pub fn plan(context: &Context<'_>, inventory: &Inventory) -> Plan {
    let catalog = context.catalog;
    let present = present(catalog, inventory);
    let selected: BTreeSet<String> = context.selection.clone().unwrap_or_else(|| {
        present
            .iter()
            .filter(|id| catalog.product(id).is_some_and(|product| !product.optional))
            .cloned()
            .collect()
    });
    let offers = catalog
        .products
        .iter()
        .map(|product| Offer {
            id: product.id.clone(),
            summary: product.summary.clone(),
            optional: product.optional,
            present: present.contains(&product.id),
            selected: selected.contains(&product.id),
        })
        .collect();
    let mut findings = Vec::new();
    let mut actions = Vec::new();
    for host in &context.hosts {
        let state = match host {
            Host::Claude => inventory.claude.as_ref(),
            Host::Codex => inventory.codex.as_ref(),
        };
        match state {
            Some(state) => plan_host(
                context,
                *host,
                state,
                &selected,
                &mut findings,
                &mut actions,
            ),
            None => findings.push(Finding {
                level: Level::Note,
                host: Some(*host),
                subject: host.program().to_owned(),
                detail: format!("`{}` is not installed here; skipped", host.program()),
            }),
        }
    }
    if context.hosts.contains(&Host::Claude) && inventory.claude.is_some() {
        plan_settings(context, inventory, &mut findings, &mut actions);
    }
    plan_binaries(context, inventory, &selected, &mut findings, &mut actions);
    Plan {
        format: FORMAT.to_owned(),
        inventory_digest: digest(inventory),
        hosts: context.hosts.clone(),
        offers,
        resolved: context.resolved.clone(),
        findings,
        actions,
    }
}

fn scope_args(plugin: &Installed) -> Vec<String> {
    argv(&["--scope", &plugin.scope])
}

#[allow(clippy::too_many_lines)]
fn plan_host(
    context: &Context<'_>,
    host: Host,
    state: &HostState,
    selected: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
    actions: &mut Vec<Action>,
) {
    let catalog = context.catalog;
    let name = catalog.marketplace.name.as_str();
    let repository = catalog.marketplace.repository.as_str();
    let program = host.program();
    let finding = |level, subject: &str, detail: String| Finding {
        level,
        host: Some(host),
        subject: subject.to_owned(),
        detail,
    };

    // 1. The marketplace: registered, unpinned, fresh.
    match state.marketplaces.iter().find(|market| market.name == name) {
        None => {
            findings.push(finding(
                Level::Change,
                name,
                format!("marketplace `{name}` is not registered; add {repository}"),
            ));
            actions.push(Action::Command {
                host,
                argv: argv(&[program, "plugin", "marketplace", "add", repository]),
                cwd: None,
                reason: format!("register marketplace `{name}`"),
            });
        }
        Some(market) if market.reference.is_some() => {
            let pinned = market.reference.clone().unwrap_or_default();
            findings.push(finding(
                Level::Change,
                name,
                format!("marketplace `{name}` is pinned to `{pinned}`; follow the default branch instead"),
            ));
            match host {
                Host::Claude => {
                    actions.push(Action::Unpin {
                        file: context
                            .home
                            .join(".claude/settings.json")
                            .to_string_lossy()
                            .into_owned(),
                        name: name.to_owned(),
                        repository: repository.to_owned(),
                        reason: format!("unpin marketplace `{name}`"),
                    });
                    actions.push(Action::Command {
                        host,
                        argv: argv(&[program, "plugin", "marketplace", "add", repository]),
                        cwd: None,
                        reason: format!("re-register marketplace `{name}` unpinned"),
                    });
                }
                Host::Codex => {
                    actions.push(Action::Command {
                        host,
                        argv: argv(&[program, "plugin", "marketplace", "remove", name]),
                        cwd: None,
                        reason: format!("drop pinned marketplace `{name}`"),
                    });
                    actions.push(Action::Command {
                        host,
                        argv: argv(&[program, "plugin", "marketplace", "add", repository]),
                        cwd: None,
                        reason: format!("re-register marketplace `{name}` unpinned"),
                    });
                }
            }
        }
        Some(_) => actions.push(Action::Refresh {
            host,
            argv: match host {
                Host::Claude => argv(&[program, "plugin", "marketplace", "update", name]),
                Host::Codex => argv(&[program, "plugin", "marketplace", "upgrade", name]),
            },
        }),
    }

    // 2. User scope: exactly the base plus the selected products' plugins.
    let mut desired: Vec<&str> = catalog.base.iter().map(String::as_str).collect();
    for product in &catalog.products {
        if selected.contains(&product.id) {
            desired.extend(product.plugins.iter().map(String::as_str));
        }
    }
    let user: BTreeMap<&str, &Installed> = state
        .plugins
        .iter()
        .filter(|plugin| plugin.marketplace == name && plugin.scope == "user")
        .map(|plugin| (plugin.name.as_str(), plugin))
        .collect();
    for plugin in &desired {
        let id = format!("{plugin}@{name}");
        let target = context.resolved.plugins.get(*plugin);
        match user.get(plugin) {
            None => {
                findings.push(finding(
                    Level::Change,
                    &id,
                    format!(
                        "not installed; install {}",
                        target.map_or("the served version".to_owned(), Clone::clone)
                    ),
                ));
                actions.push(install(host, &id, "user", None));
            }
            Some(installed) => {
                let current = installed.version.clone().unwrap_or_default();
                match target {
                    Some(target) if !version::same(&current, target) => {
                        findings.push(finding(
                            Level::Change,
                            &id,
                            format!("installed {current}; upgrade to {target}"),
                        ));
                        actions.extend(upgrade(host, &id));
                    }
                    _ => findings.push(finding(Level::Ok, &id, format!("current ({current})"))),
                }
                if !installed.enabled {
                    findings.push(finding(
                        Level::Change,
                        &id,
                        "disabled; enable it".to_owned(),
                    ));
                    actions.push(Action::Command {
                        host,
                        argv: argv(&[program, "plugin", "enable", &id]),
                        cwd: None,
                        reason: format!("enable `{id}`"),
                    });
                }
            }
        }
    }
    for (plugin, installed) in &user {
        let managed = catalog.product_of(plugin).is_some();
        if managed && !desired.contains(plugin) {
            let id = installed.id();
            findings.push(finding(
                Level::Change,
                &id,
                "its product is not selected; uninstall".to_owned(),
            ));
            actions.push(uninstall(host, installed));
        }
    }

    // 3. Legacy installs: a retired name or a retired marketplace, at any scope.
    let mut retire_markets = BTreeSet::new();
    for plugin in &state.plugins {
        let retired_market = catalog.retired_marketplace(&plugin.marketplace);
        let retired_name = catalog.retired_plugins.contains_key(&plugin.name);
        if !retired_market && !retired_name {
            continue;
        }
        if !catalog.knows(&plugin.name) && !retired_market {
            continue;
        }
        let id = plugin.id();
        let replacement = catalog.current_name(&plugin.name).to_owned();
        let where_ = plugin
            .project
            .as_deref()
            .map_or(String::new(), |project| format!(" in {project}"));
        findings.push(finding(
            Level::Change,
            &id,
            format!(
                "legacy install ({} scope{where_}); replace with `{replacement}@{name}`",
                plugin.scope
            ),
        ));
        if plugin.scope != "user" {
            let already = state.plugins.iter().any(|other| {
                other.name == replacement
                    && other.marketplace == name
                    && other.scope == plugin.scope
                    && other.project == plugin.project
            });
            if !already && catalog.knows(&replacement) {
                actions.push(install(
                    host,
                    &format!("{replacement}@{name}"),
                    &plugin.scope,
                    plugin.project.clone(),
                ));
            }
        }
        actions.push(uninstall(host, plugin));
        if retired_market {
            retire_markets.insert(plugin.marketplace.clone());
        }
    }
    for market in &state.marketplaces {
        if catalog.retired_marketplace(&market.name) {
            retire_markets.insert(market.name.clone());
        }
    }
    for market in retire_markets {
        if state.marketplaces.iter().any(|known| known.name == market) {
            findings.push(finding(
                Level::Change,
                &market,
                format!("retired marketplace `{market}`; remove it"),
            ));
            actions.push(Action::Command {
                host,
                argv: argv(&[program, "plugin", "marketplace", "remove", &market]),
                cwd: None,
                reason: format!("remove retired marketplace `{market}`"),
            });
        }
    }
}

fn install(host: Host, id: &str, scope: &str, project: Option<String>) -> Action {
    let program = host.program();
    let argv = match host {
        Host::Claude => argv(&[program, "plugin", "install", id, "--scope", scope]),
        Host::Codex => argv(&[program, "plugin", "add", id]),
    };
    Action::Command {
        host,
        argv,
        cwd: project,
        reason: format!("install `{id}`"),
    }
}

fn upgrade(host: Host, id: &str) -> Vec<Action> {
    let program = host.program();
    match host {
        Host::Claude => vec![Action::Command {
            host,
            argv: argv(&[program, "plugin", "update", id, "--scope", "user"]),
            cwd: None,
            reason: format!("upgrade `{id}`"),
        }],
        Host::Codex => vec![
            Action::Command {
                host,
                argv: argv(&[program, "plugin", "remove", id]),
                cwd: None,
                reason: format!("remove `{id}` before reinstalling"),
            },
            Action::Command {
                host,
                argv: argv(&[program, "plugin", "add", id]),
                cwd: None,
                reason: format!("upgrade `{id}`"),
            },
        ],
    }
}

fn uninstall(host: Host, plugin: &Installed) -> Action {
    let program = host.program();
    let id = plugin.id();
    let argv = match host {
        Host::Claude => {
            let mut argv = argv(&[program, "plugin", "uninstall", &id]);
            argv.extend(scope_args(plugin));
            argv
        }
        Host::Codex => argv(&[program, "plugin", "remove", &id]),
    };
    Action::Command {
        host,
        argv,
        cwd: plugin.project.clone(),
        reason: format!("uninstall `{id}`"),
    }
}

/// Settings entries the host list does not report: enabled keys whose marketplace is gone or
/// retired. Their replacement is installed at the same scope when the catalog knows it.
fn plan_settings(
    context: &Context<'_>,
    inventory: &Inventory,
    findings: &mut Vec<Finding>,
    actions: &mut Vec<Action>,
) {
    let catalog = context.catalog;
    let Some(state) = inventory.claude.as_ref() else {
        return;
    };
    let name = catalog.marketplace.name.as_str();
    for entry in &inventory.settings {
        let Some((plugin, marketplace)) = entry.plugin.rsplit_once('@') else {
            continue;
        };
        let reported = state.plugins.iter().any(|installed| {
            installed.id() == entry.plugin
                && installed.scope == entry.scope
                && installed.project == entry.project
        });
        let registered = state
            .marketplaces
            .iter()
            .any(|market| market.name == marketplace);
        let retired = catalog.retired_marketplace(marketplace)
            || catalog.retired_plugins.contains_key(plugin);
        if reported || (registered && !retired) {
            continue;
        }
        let committed = entry.scope == "project";
        findings.push(Finding {
            level: if committed {
                Level::Warn
            } else {
                Level::Change
            },
            host: Some(Host::Claude),
            subject: entry.plugin.clone(),
            detail: format!(
                "orphan entry in {}{}; remove it",
                entry.file,
                if committed {
                    " (a committed file: review the diff)"
                } else {
                    ""
                }
            ),
        });
        actions.push(Action::RemoveSetting {
            file: entry.file.clone(),
            key: entry.plugin.clone(),
            reason: format!("remove orphan `{}`", entry.plugin),
        });
        let replacement = catalog.current_name(plugin);
        let target = format!("{replacement}@{name}");
        let already = inventory
            .settings
            .iter()
            .any(|other| other.plugin == target && other.file == entry.file)
            || state.plugins.iter().any(|installed| {
                installed.id() == target
                    && installed.scope == entry.scope
                    && installed.project == entry.project
            });
        if entry.enabled && entry.scope != "user" && !already && catalog.knows(replacement) {
            actions.push(install(
                Host::Claude,
                &target,
                &entry.scope,
                entry.project.clone(),
            ));
        }
    }
}

#[allow(clippy::too_many_lines)]
fn plan_binaries(
    context: &Context<'_>,
    inventory: &Inventory,
    selected: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
    actions: &mut Vec<Action>,
) {
    let catalog = context.catalog;
    for product in &catalog.products {
        if !selected.contains(&product.id) {
            continue;
        }
        for binary in &product.binaries {
            let subject = binary.name.clone();
            let target = match &binary.bind {
                Bind::Latest => context
                    .resolved
                    .latest
                    .get(binary.install.repository())
                    .cloned(),
                Bind::Plugin { plugin } => context.resolved.plugins.get(plugin).cloned(),
            };
            let Some(tag) = target else {
                findings.push(Finding {
                    level: Level::Warn,
                    host: None,
                    subject,
                    detail: "its version could not be resolved (offline?); not checked".to_owned(),
                });
                continue;
            };
            let copies = inventory
                .binaries
                .iter()
                .find(|state| state.name == binary.name)
                .map(|state| state.copies.as_slice())
                .unwrap_or_default();
            let default_directory = match binary.install {
                Install::ReleaseArchive { .. } => context.home.join(".local/bin"),
                Install::Cargo { .. } => context.home.join(".cargo/bin"),
            };
            let bound = match &binary.bind {
                Bind::Latest => "newest release".to_owned(),
                Bind::Plugin { plugin } => format!("the `{plugin}` plugin"),
            };
            match copies.first() {
                None if binary.optional => findings.push(Finding {
                    level: Level::Note,
                    host: None,
                    subject,
                    detail: format!("optional, not installed (newest {tag})"),
                }),
                None => {
                    findings.push(Finding {
                        level: Level::Change,
                        host: None,
                        subject: subject.clone(),
                        detail: format!("not on PATH; install {tag} to match {bound}"),
                    });
                    actions.push(Action::InstallBinary {
                        name: binary.name.clone(),
                        tag,
                        install: binary.install.clone(),
                        directory: default_directory.to_string_lossy().into_owned(),
                        reason: format!("install `{}`", binary.name),
                    });
                }
                Some(first) => {
                    let current = first
                        .version
                        .clone()
                        .unwrap_or_else(|| "unknown".to_owned());
                    if version::same(&current, &tag) {
                        findings.push(Finding {
                            level: Level::Ok,
                            host: None,
                            subject: subject.clone(),
                            detail: format!("{current} at {} matches {bound}", first.path),
                        });
                    } else {
                        let directory = Path::new(&first.path)
                            .parent()
                            .filter(|parent| parent.starts_with(context.home))
                            .map_or(default_directory.clone(), Path::to_path_buf);
                        findings.push(Finding {
                            level: Level::Change,
                            host: None,
                            subject: subject.clone(),
                            detail: format!(
                                "{current} at {} does not match {bound} ({tag}); replace it",
                                first.path
                            ),
                        });
                        actions.push(Action::InstallBinary {
                            name: binary.name.clone(),
                            tag: tag.clone(),
                            install: binary.install.clone(),
                            directory: directory.to_string_lossy().into_owned(),
                            reason: format!("upgrade `{}`", binary.name),
                        });
                    }
                    for shadowed in &copies[1..] {
                        let seen = shadowed
                            .version
                            .clone()
                            .unwrap_or_else(|| "unknown".to_owned());
                        if !version::same(&seen, &tag) {
                            findings.push(Finding {
                                level: Level::Warn,
                                host: None,
                                subject: subject.clone(),
                                detail: format!(
                                    "another copy, {seen} at {}, is later on PATH and never runs; remove it if nothing else uses it",
                                    shadowed.path
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{self, BinaryState, Copy, Market, SettingsEntry};

    fn legacy_claude() -> HostState {
        HostState {
            plugins: inventory::parse_claude_plugins(include_str!(
                "../tests/fixtures/claude-legacy-list.json"
            ))
            .unwrap(),
            marketplaces: inventory::parse_claude_marketplaces(include_str!(
                "../tests/fixtures/claude-legacy-marketplaces.json"
            ))
            .unwrap(),
        }
    }

    fn legacy_codex() -> HostState {
        HostState {
            plugins: inventory::parse_codex_plugins(include_str!(
                "../tests/fixtures/codex-legacy-list.json"
            ))
            .unwrap(),
            marketplaces: inventory::parse_codex_marketplaces(
                include_str!("../tests/fixtures/codex-legacy-marketplaces.json"),
                Some(include_str!("../tests/fixtures/codex-legacy-config.toml")),
            )
            .unwrap(),
        }
    }

    fn resolved() -> Resolved {
        Resolved {
            plugins: BTreeMap::from([
                ("b10x".to_owned(), "0.12.0".to_owned()),
                ("aep-plan".to_owned(), "0.12.0".to_owned()),
                ("aep-drive".to_owned(), "0.12.0".to_owned()),
                ("ess".to_owned(), "0.30.0".to_owned()),
                ("worktree".to_owned(), "0.6.0".to_owned()),
            ]),
            latest: BTreeMap::from([
                ("beyond10x/aep".to_owned(), "0.57.0".to_owned()),
                ("beyond10x/metaharness".to_owned(), "0.7.0".to_owned()),
            ]),
        }
    }

    fn binaries(ess: &[(&str, &str)]) -> Vec<BinaryState> {
        vec![BinaryState {
            name: "ess".to_owned(),
            copies: ess
                .iter()
                .map(|(path, version)| Copy {
                    path: (*path).to_owned(),
                    version: Some((*version).to_owned()),
                })
                .collect(),
        }]
    }

    fn run(inventory: &Inventory, selection: Option<&[&str]>, hosts: &[Host]) -> Plan {
        let catalog = Catalog::embedded();
        let resolved = resolved();
        let context = Context {
            catalog: &catalog,
            resolved: &resolved,
            selection: selection.map(|ids| ids.iter().map(|id| (*id).to_owned()).collect()),
            hosts: hosts.to_vec(),
            home: Path::new("/opt/b10x-home"),
        };
        plan(&context, inventory)
    }

    fn commands(plan: &Plan) -> Vec<String> {
        plan.actions
            .iter()
            .filter(|action| action.changes())
            .map(|action| match action {
                Action::Command { argv, .. } => argv.join(" "),
                other => other.describe(),
            })
            .collect()
    }

    #[test]
    fn the_legacy_claude_install_migrates_to_b10x() {
        let inventory = Inventory {
            claude: Some(legacy_claude()),
            ..Inventory::default()
        };
        let plan = run(&inventory, None, &[Host::Claude]);
        let selected: Vec<&str> = plan
            .offers
            .iter()
            .filter(|offer| offer.selected)
            .map(|offer| offer.id.as_str())
            .collect();
        assert_eq!(
            selected,
            ["aep", "ess", "worktree"],
            "legacy products are preselected"
        );
        let commands = commands(&plan);
        for expected in [
            "claude plugin marketplace add beyond10x/agentplugins",
            "claude plugin install b10x@b10x --scope user",
            "claude plugin install aep-plan@b10x --scope user",
            "claude plugin install ess@b10x --scope user",
            "claude plugin install worktree@b10x --scope user",
            "claude plugin uninstall workspace-hygiene@beyond10x --scope user",
            "claude plugin uninstall ess@ess --scope user",
            "claude plugin marketplace remove beyond10x",
            "claude plugin marketplace remove ess",
        ] {
            assert!(
                commands.iter().any(|c| c == expected),
                "missing `{expected}` in {commands:#?}"
            );
        }
        let add = commands
            .iter()
            .position(|c| c.ends_with("marketplace add beyond10x/agentplugins"));
        let remove = commands
            .iter()
            .position(|c| c.ends_with("marketplace remove beyond10x"));
        assert!(add < remove, "new marketplace first, retired one last");
    }

    #[test]
    fn codex_legacy_install_migrates_and_unpins_nothing_it_does_not_own() {
        let inventory = Inventory {
            codex: Some(legacy_codex()),
            ..Inventory::default()
        };
        let plan = run(&inventory, Some(&["aep"]), &[Host::Codex]);
        let commands = commands(&plan);
        assert!(commands.contains(&"codex plugin add aep-drive@b10x".to_owned()));
        assert!(commands.contains(&"codex plugin remove workspace-hygiene@beyond10x".to_owned()));
        assert!(
            !commands.iter().any(|c| c.contains("worktree@b10x")),
            "worktree not selected"
        );
        assert!(commands.contains(&"codex plugin marketplace remove beyond10x".to_owned()));
    }

    #[test]
    fn a_pinned_b10x_registration_is_unpinned() {
        let mut state = HostState::default();
        state.marketplaces.push(Market {
            name: "b10x".to_owned(),
            source: "https://github.com/beyond10x/agentplugins.git".to_owned(),
            reference: Some("0.11.0".to_owned()),
            location: None,
        });
        let inventory = Inventory {
            claude: Some(state),
            ..Inventory::default()
        };
        let plan = run(&inventory, Some(&[]), &[Host::Claude]);
        assert!(matches!(plan.actions[0], Action::Unpin { .. }));
    }

    #[test]
    fn current_state_converges() {
        let mut state = HostState::default();
        state.marketplaces.push(Market {
            name: "b10x".to_owned(),
            source: "beyond10x/agentplugins".to_owned(),
            reference: None,
            location: None,
        });
        for (name, version) in [("b10x", "0.12.0"), ("ess", "0.30.0")] {
            state.plugins.push(Installed {
                name: name.to_owned(),
                marketplace: "b10x".to_owned(),
                version: Some(version.to_owned()),
                scope: "user".to_owned(),
                project: None,
                enabled: true,
            });
        }
        let inventory = Inventory {
            claude: Some(state),
            binaries: binaries(&[("/opt/b10x-home/.local/bin/ess", "0.30.0")]),
            ..Inventory::default()
        };
        let plan = run(&inventory, None, &[Host::Claude]);
        assert!(plan.converged(), "{:#?}", plan.actions);
    }

    #[test]
    fn a_binary_behind_its_plugin_is_replaced_where_it_runs_and_shadows_are_named() {
        let inventory = Inventory {
            claude: Some(HostState::default()),
            binaries: binaries(&[
                ("/opt/b10x-home/.cargo/bin/ess", "0.26.0"),
                ("/opt/b10x-home/.local/bin/ess", "0.26.0"),
            ]),
            ..Inventory::default()
        };
        let plan = run(&inventory, Some(&["ess"]), &[Host::Claude]);
        let install = plan
            .actions
            .iter()
            .find_map(|action| match action {
                Action::InstallBinary {
                    name,
                    tag,
                    directory,
                    ..
                } if name == "ess" => Some((tag.clone(), directory.clone())),
                _ => None,
            })
            .expect("ess is replaced");
        assert_eq!(
            install,
            ("0.30.0".to_owned(), "/opt/b10x-home/.cargo/bin".to_owned())
        );
        assert!(plan
            .findings
            .iter()
            .any(|f| f.level == Level::Warn && f.detail.contains("/opt/b10x-home/.local/bin/ess")));
    }

    #[test]
    fn deselecting_a_product_uninstalls_its_current_plugins() {
        let mut state = HostState::default();
        state.plugins.push(Installed {
            name: "worktree".to_owned(),
            marketplace: "b10x".to_owned(),
            version: Some("0.6.0".to_owned()),
            scope: "user".to_owned(),
            project: None,
            enabled: true,
        });
        let inventory = Inventory {
            claude: Some(state),
            ..Inventory::default()
        };
        let plan = run(&inventory, Some(&[]), &[Host::Claude]);
        assert!(commands(&plan)
            .contains(&"claude plugin uninstall worktree@b10x --scope user".to_owned()));
    }

    #[test]
    fn an_orphan_in_a_project_file_is_removed_and_replaced_in_place() {
        let mut state = HostState::default();
        state.marketplaces.push(Market {
            name: "b10x".to_owned(),
            source: "beyond10x/agentplugins".to_owned(),
            reference: None,
            location: None,
        });
        let inventory = Inventory {
            claude: Some(state),
            settings: vec![SettingsEntry {
                file: "/work/acd/.claude/settings.local.json".to_owned(),
                scope: "local".to_owned(),
                project: Some("/work/acd".to_owned()),
                plugin: "beyond10x@beyond10x".to_owned(),
                enabled: true,
            }],
            ..Inventory::default()
        };
        let plan = run(&inventory, Some(&[]), &[Host::Claude]);
        assert!(plan.actions.iter().any(
            |a| matches!(a, Action::RemoveSetting { key, .. } if key == "beyond10x@beyond10x")
        ));
        assert!(plan.actions.iter().any(|a| matches!(a,
            Action::Command { argv, cwd: Some(cwd), .. }
                if argv.join(" ") == "claude plugin install b10x@b10x --scope local" && cwd == "/work/acd")));
    }

    #[test]
    fn the_digest_changes_with_the_inventory() {
        let empty = Inventory::default();
        let other = Inventory {
            claude: Some(HostState::default()),
            ..Inventory::default()
        };
        assert_ne!(digest(&empty), digest(&other));
    }
}
