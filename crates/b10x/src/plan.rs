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

use crate::catalog::{Catalog, Install, Method};
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
        /// Which way.
        method: Method,
        /// The ways it can be installed.
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
                method,
                directory,
                reason,
                ..
            } => format!(
                "{reason}: install {name} {tag} into {directory} ({})",
                match method {
                    Method::Prebuilt => "prebuilt archive",
                    Method::Cargo => "cargo install",
                }
            ),
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
    /// What the user wants to do, as the onboarding question offers it.
    #[serde(default)]
    pub intent: String,
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
    /// What to do after applying: the next skill or command for the user.
    #[serde(default)]
    pub next: Vec<String>,
    /// Planned for the selected products only (`init`, `upgrade`).
    #[serde(default)]
    pub only: bool,
    /// The install method asked for, if any.
    #[serde(default)]
    pub method: Option<Method>,
    /// Planned as an upgrade: only what is installed is updated.
    #[serde(default)]
    pub upgrade: bool,
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
    /// Plan only the selected products (`init`, `upgrade`): nothing else is uninstalled, migrated or
    /// removed. `false` makes the selection the whole desired state (`setup plan`).
    pub only: bool,
    /// How to install binaries; `None` picks cargo when it is on `PATH`, else prebuilt.
    pub method: Option<Method>,
    /// Update only what is installed: no new plugins, and a host with nothing Beyond10x on it is
    /// left alone (`upgrade`).
    pub upgrade: bool,
}

/// Make the plan.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn plan(context: &Context<'_>, inventory: &Inventory) -> Plan {
    let catalog = context.catalog;
    let present = present(catalog, inventory);
    let selected: BTreeSet<String> = context.selection.clone().unwrap_or_else(|| {
        present
            .iter()
            .filter(|id| {
                context.only || catalog.product(id).is_some_and(|product| !product.optional)
            })
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
            intent: product.intent.clone(),
        })
        .collect();
    let mut findings = Vec::new();
    let mut actions = Vec::new();
    for host in &context.hosts {
        let state = match host {
            Host::Claude => inventory.claude.as_ref(),
            Host::Codex => inventory.codex.as_ref(),
        };
        let untouched = state.is_some_and(|state| {
            !state.plugins.iter().any(|plugin| {
                plugin.marketplace == catalog.marketplace.name
                    || catalog.retired_marketplace(&plugin.marketplace)
                    || catalog.retired_plugins.contains_key(&plugin.name)
            })
        });
        if context.upgrade && untouched {
            findings.push(Finding {
                level: Level::Note,
                host: Some(*host),
                subject: host.program().to_owned(),
                detail: format!(
                    "nothing from Beyond10x is installed here; `b10x init <products> --host {}` adds it",
                    host.program()
                ),
            });
            continue;
        }
        match state {
            Some(state) => plan_host(
                context,
                *host,
                state,
                &selected,
                &mut findings,
                &mut actions,
            ),
            None => match inventory.broken.iter().find(|(broken, _)| broken == host) {
                Some((_, error)) => findings.push(Finding {
                    level: Level::Warn,
                    host: Some(*host),
                    subject: host.program().to_owned(),
                    detail: format!(
                        "{error}; skipped. Repair it with `{}`, then plan again",
                        match host {
                            Host::Claude => "claude plugin marketplace update",
                            Host::Codex => "codex plugin marketplace upgrade",
                        }
                    ),
                }),
                None => findings.push(Finding {
                    level: Level::Note,
                    host: Some(*host),
                    subject: host.program().to_owned(),
                    detail: format!("`{}` is not installed here; skipped", host.program()),
                }),
            },
        }
    }
    if context.hosts.contains(&Host::Claude) && inventory.claude.is_some() {
        plan_settings(context, &selected, inventory, &mut findings, &mut actions);
    }
    plan_binaries(context, inventory, &selected, &mut findings, &mut actions);
    let mut next = Vec::new();
    for product in &catalog.products {
        if selected.contains(&product.id) {
            for plugin in &product.plugins {
                next.push(format!(
                    "/{plugin}:init starts {plugin} here (this session, before a restart: `b10x skill {plugin}:init`)"
                ));
            }
        }
    }
    next.push("/b10x:upgrade checks everything later; /b10x:init adds a product".to_owned());
    Plan {
        format: FORMAT.to_owned(),
        inventory_digest: digest(inventory),
        hosts: context.hosts.clone(),
        offers,
        resolved: context.resolved.clone(),
        findings,
        actions,
        next,
        only: context.only,
        method: context.method,
        upgrade: context.upgrade,
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
            None if context.upgrade && !catalog.base.iter().any(|base| base == plugin) => {
                findings.push(finding(
                    Level::Note,
                    &id,
                    "not installed; `b10x init` adds it".to_owned(),
                ));
            }
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
        if managed && !desired.contains(plugin) && !context.only {
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
    // Two retired plugins can share one replacement (`aep-plan` and `aep-drive` are both `aep`):
    // install it once per scope and project.
    let mut placed = BTreeSet::new();
    let mut kept = BTreeSet::new();
    for plugin in &state.plugins {
        let retired_market = catalog.retired_marketplace(&plugin.marketplace);
        let retired_name = catalog.retired_plugins.contains_key(&plugin.name);
        if !retired_market && !retired_name {
            continue;
        }
        if !catalog.knows(&plugin.name) && !retired_market {
            continue;
        }
        if context.only && !wanted(catalog, selected, catalog.current_name(&plugin.name)) {
            kept.insert(plugin.marketplace.clone());
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
            let first = placed.insert((
                replacement.clone(),
                plugin.scope.clone(),
                plugin.project.clone(),
            ));
            if !already && first && catalog.knows(&replacement) {
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
        if kept.contains(&market) {
            continue;
        }
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

/// Whether a plugin belongs to what this plan is for: the base, or a selected product.
fn wanted(catalog: &Catalog, selected: &BTreeSet<String>, plugin: &str) -> bool {
    catalog.base.iter().any(|base| base == plugin)
        || catalog
            .product_of(plugin)
            .is_some_and(|product| selected.contains(&product.id))
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
    selected: &BTreeSet<String>,
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
        if context.only && !wanted(catalog, selected, catalog.current_name(plugin)) {
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

/// The install method for one binary: the explicit choice when it can be honoured, else cargo when
/// it is on `PATH`, else the prebuilt archive when the release has one.
fn method(context: &Context<'_>, inventory: &Inventory, install: &Install) -> Option<Method> {
    let archive = install.archive.is_some()
        && context
            .resolved
            .archives
            .get(install.repository())
            .copied()
            .unwrap_or(false);
    let cargo = install.cargo.is_some() && inventory.cargo;
    match context.method {
        Some(Method::Prebuilt) if archive => Some(Method::Prebuilt),
        Some(Method::Cargo) if install.cargo.is_some() => Some(Method::Cargo),
        _ if cargo => Some(Method::Cargo),
        _ if archive => Some(Method::Prebuilt),
        _ if install.cargo.is_some() => Some(Method::Cargo),
        _ => None,
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
    let mut methods = BTreeSet::new();
    for product in &catalog.products {
        if !selected.contains(&product.id) {
            continue;
        }
        for binary in &product.binaries {
            let subject = binary.name.clone();
            let Some(tag) = context
                .resolved
                .latest
                .get(binary.install.repository())
                .cloned()
            else {
                findings.push(Finding {
                    level: Level::Warn,
                    host: None,
                    subject,
                    detail: "its newest release could not be read (offline?); not checked"
                        .to_owned(),
                });
                continue;
            };
            let copies = inventory
                .binaries
                .iter()
                .find(|state| state.name == binary.name)
                .map(|state| state.copies.as_slice())
                .unwrap_or_default();
            let current = copies.first();
            if current
                .is_some_and(|first| version::same(first.version.as_deref().unwrap_or(""), &tag))
            {
                let first = current.expect("checked above");
                findings.push(Finding {
                    level: Level::Ok,
                    host: None,
                    subject: subject.clone(),
                    detail: format!("{tag} at {} is the newest release", first.path),
                });
            } else if current.is_none() && binary.optional {
                findings.push(Finding {
                    level: Level::Note,
                    host: None,
                    subject: subject.clone(),
                    detail: format!(
                        "optional, not installed (newest {tag}); `b10x install {}` adds it",
                        binary.name
                    ),
                });
            } else {
                let Some(method) = method(context, inventory, &binary.install) else {
                    findings.push(Finding {
                        level: Level::Warn,
                        host: None,
                        subject: subject.clone(),
                        detail: format!("{tag} has no prebuilt archive and `cargo` is not on PATH; install a Rust toolchain (https://rustup.rs) and plan again"),
                    });
                    continue;
                };
                methods.insert(method);
                let default_directory = match method {
                    Method::Prebuilt => context.home.join(".local/bin"),
                    Method::Cargo => context.home.join(".cargo/bin"),
                };
                let directory = current
                    .and_then(|first| Path::new(&first.path).parent().map(Path::to_path_buf))
                    .filter(|parent| parent.starts_with(context.home))
                    .unwrap_or(default_directory);
                let (verb, detail) = match current {
                    None => ("install", format!("not on PATH; install {tag}")),
                    Some(first) => (
                        "upgrade",
                        format!(
                            "{} at {} is not the newest release {tag}; replace it",
                            first
                                .version
                                .clone()
                                .unwrap_or_else(|| "unknown".to_owned()),
                            first.path
                        ),
                    ),
                };
                findings.push(Finding {
                    level: Level::Change,
                    host: None,
                    subject: subject.clone(),
                    detail,
                });
                actions.push(Action::InstallBinary {
                    name: binary.name.clone(),
                    tag: tag.clone(),
                    method,
                    install: binary.install.clone(),
                    directory: directory.to_string_lossy().into_owned(),
                    reason: format!("{verb} `{}`", binary.name),
                });
            }
            for shadowed in copies.iter().skip(1) {
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
    if !methods.is_empty() {
        let chosen: Vec<&str> = methods
            .iter()
            .map(|method| match method {
                Method::Prebuilt => "prebuilt archives",
                Method::Cargo => "cargo install",
            })
            .collect();
        findings.push(Finding {
            level: Level::Note,
            host: None,
            subject: "install method".to_owned(),
            detail: format!(
                "{} ({}); choose with --method cargo or --method prebuilt",
                chosen.join(" and "),
                if inventory.cargo {
                    "cargo is on PATH"
                } else {
                    "cargo is not on PATH"
                }
            ),
        });
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
                ("aep".to_owned(), "0.13.0".to_owned()),
                ("ess".to_owned(), "0.30.0".to_owned()),
                ("worktree".to_owned(), "0.6.0".to_owned()),
            ]),
            latest: BTreeMap::from([
                ("beyond10x/aep".to_owned(), "0.57.0".to_owned()),
                ("beyond10x/ess".to_owned(), "0.30.0".to_owned()),
                ("beyond10x/worktree".to_owned(), "0.7.0".to_owned()),
                ("beyond10x/metaharness".to_owned(), "0.7.0".to_owned()),
            ]),
            archives: BTreeMap::from([
                ("beyond10x/aep".to_owned(), true),
                ("beyond10x/ess".to_owned(), true),
                ("beyond10x/worktree".to_owned(), true),
                ("beyond10x/metaharness".to_owned(), false),
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
            only: false,
            method: None,
            upgrade: false,
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
            "claude plugin install aep@b10x --scope user",
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
        assert!(commands.contains(&"codex plugin add aep@b10x".to_owned()));
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
    fn two_retired_plugins_in_one_project_become_one_install() {
        let mut state = HostState::default();
        for name in ["aep-plan", "aep-drive"] {
            state.plugins.push(Installed {
                name: name.to_owned(),
                marketplace: "b10x".to_owned(),
                version: Some("0.12.0".to_owned()),
                scope: "local".to_owned(),
                project: Some("/work/p".to_owned()),
                enabled: true,
            });
        }
        let inventory = Inventory {
            claude: Some(state),
            ..Inventory::default()
        };
        let plan = run(&inventory, Some(&[]), &[Host::Claude]);
        let commands = commands(&plan);
        let installs = commands
            .iter()
            .filter(|c| *c == "claude plugin install aep@b10x --scope local")
            .count();
        assert_eq!(installs, 1, "{commands:#?}");
        assert!(
            commands.contains(&"claude plugin uninstall aep-plan@b10x --scope local".to_owned())
        );
        assert!(
            commands.contains(&"claude plugin uninstall aep-drive@b10x --scope local".to_owned())
        );
    }

    fn run_only(inventory: &Inventory, products: &[&str], method: Option<Method>) -> Plan {
        let catalog = Catalog::embedded();
        let resolved = resolved();
        let context = Context {
            catalog: &catalog,
            resolved: &resolved,
            selection: Some(products.iter().map(|id| (*id).to_owned()).collect()),
            hosts: vec![Host::Claude],
            home: Path::new("/opt/b10x-home"),
            only: true,
            method,
            upgrade: false,
        };
        plan(&context, inventory)
    }

    #[test]
    fn init_of_one_product_leaves_every_other_install_alone() {
        let inventory = Inventory {
            claude: Some(legacy_claude()),
            ..Inventory::default()
        };
        let plan = run_only(&inventory, &["ess"], None);
        let commands = commands(&plan);
        assert!(commands.contains(&"claude plugin install ess@b10x --scope user".to_owned()));
        assert!(commands.contains(&"claude plugin uninstall ess@ess --scope user".to_owned()));
        assert!(
            !commands.iter().any(|c| c.contains("aep")),
            "aep untouched: {commands:#?}"
        );
        assert!(
            !commands
                .iter()
                .any(|c| c.ends_with("marketplace remove beyond10x")),
            "beyond10x still serves aep installs"
        );
        assert!(plan.next.iter().any(|line| line.starts_with("/ess:init")));
    }

    #[test]
    fn cargo_is_the_default_when_present_and_prebuilt_otherwise() {
        let method_for = |cargo: bool, asked: Option<Method>| {
            let inventory = Inventory {
                claude: Some(HostState::default()),
                cargo,
                ..Inventory::default()
            };
            run_only(&inventory, &["worktree"], asked)
                .actions
                .iter()
                .find_map(|action| match action {
                    Action::InstallBinary { method, .. } => Some(*method),
                    _ => None,
                })
        };
        assert_eq!(method_for(true, None), Some(Method::Cargo));
        assert_eq!(method_for(false, None), Some(Method::Prebuilt));
        assert_eq!(
            method_for(true, Some(Method::Prebuilt)),
            Some(Method::Prebuilt)
        );
    }

    #[test]
    fn a_release_without_archives_and_no_cargo_is_a_warning_not_an_action() {
        let inventory = Inventory {
            claude: Some(HostState::default()),
            binaries: vec![BinaryState {
                name: "metaharness".to_owned(),
                copies: vec![Copy {
                    path: "/opt/b10x-home/.local/bin/metaharness".to_owned(),
                    version: Some("0.6.0".to_owned()),
                }],
            }],
            ..Inventory::default()
        };
        let plan = run_only(&inventory, &["aep"], Some(Method::Prebuilt));
        let metaharness = plan.actions.iter().find(
            |action| matches!(action, Action::InstallBinary { name, .. } if name == "metaharness"),
        );
        assert!(
            matches!(metaharness, Some(Action::InstallBinary { method: Method::Cargo, .. })),
            "no archive for metaharness: asking for prebuilt falls back to cargo, which the catalog has"
        );
    }

    #[test]
    fn upgrade_updates_what_exists_and_leaves_an_empty_host_alone() {
        let mut claude = HostState::default();
        claude.marketplaces.push(Market {
            name: "b10x".to_owned(),
            source: "beyond10x/agentplugins".to_owned(),
            reference: None,
            location: None,
        });
        for name in ["b10x", "ess"] {
            claude.plugins.push(Installed {
                name: name.to_owned(),
                marketplace: "b10x".to_owned(),
                version: Some("0.12.0".to_owned()),
                scope: "user".to_owned(),
                project: None,
                enabled: true,
            });
        }
        let inventory = Inventory {
            claude: Some(claude),
            codex: Some(HostState::default()),
            ..Inventory::default()
        };
        let catalog = Catalog::embedded();
        let resolved = resolved();
        let context = Context {
            catalog: &catalog,
            resolved: &resolved,
            selection: None,
            hosts: vec![Host::Claude, Host::Codex],
            home: Path::new("/opt/b10x-home"),
            only: true,
            method: None,
            upgrade: true,
        };
        let plan = plan(&context, &inventory);
        let commands = commands(&plan);
        assert!(
            !commands.iter().any(|c| c.starts_with("codex")),
            "{commands:#?}"
        );
        assert!(commands.contains(&"claude plugin update ess@b10x --scope user".to_owned()));
        assert!(
            !commands.iter().any(|c| c.contains("install aep")),
            "{commands:#?}"
        );
        assert!(plan
            .findings
            .iter()
            .any(|f| f.host == Some(Host::Codex) && f.detail.contains("b10x init")));
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
