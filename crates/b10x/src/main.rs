//! `b10x`: installs, migrates and checks the Beyond10x agent plugins and the binaries they drive.

mod apply;
mod catalog;
mod check;
mod install;
mod inventory;
mod plan;
mod resolve;
mod skill;
mod version;

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

use crate::catalog::Catalog;
use crate::inventory::Host;
use crate::plan::{Level, Plan};
use crate::resolve::Source;

/// Install, migrate and check the Beyond10x agent plugins and their binaries.
#[derive(Debug, Parser)]
#[command(name = "b10x", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Top,
}

#[derive(Debug, Subcommand)]
enum Top {
    /// Plan the named products (`aep,ess`): their plugins and CLIs, installed or brought current.
    /// Nothing else is touched. Writes nothing but the plan file; apply it after confirmation.
    Init {
        /// Products, comma separated: aep, ess, worktree, connectors.
        #[arg(value_delimiter = ',')]
        products: Vec<String>,
        /// How to install CLIs; default: cargo when it is on PATH, else prebuilt.
        #[arg(long, value_enum)]
        method: Option<MethodArg>,
        /// Hosts to plan for.
        #[arg(long, value_enum, default_value_t = Hosts::All)]
        host: Hosts,
        /// Print the plan as JSON.
        #[arg(long)]
        json: bool,
        /// Write the JSON plan to this file, for `setup apply --plan`.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Check installed products (or the named ones) against what is newest and plan the upgrade.
    Upgrade {
        /// Products, comma separated; default: every installed product.
        #[arg(value_delimiter = ',')]
        products: Vec<String>,
        /// How to install CLIs; default: cargo when it is on PATH, else prebuilt.
        #[arg(long, value_enum)]
        method: Option<MethodArg>,
        /// Hosts to plan for.
        #[arg(long, value_enum, default_value_t = Hosts::All)]
        host: Hosts,
        /// Print the plan as JSON.
        #[arg(long)]
        json: bool,
        /// Write the JSON plan to this file, for `setup apply --plan`.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Plan and apply plugin and binary changes.
    Setup {
        #[command(subcommand)]
        command: Setup,
    },
    /// Offline drift check for a session-start hook; prints only problems; always exits 0.
    Check,
    /// Print an installed skill or agent (`ess:specifying`), or list a plugin's (`ess`). A host loads
    /// new plugins only in a new session; this works in the session that installed them.
    Skill {
        /// `<plugin>` or `<plugin>:<skill-or-agent>`.
        id: String,
    },
    /// Install one catalog binary at an exact release.
    Install {
        /// Binary name, e.g. `ess`.
        name: String,
        /// Release tag; defaults to the newest release.
        #[arg(long)]
        tag: Option<String>,
        /// Target directory; defaults to `~/.local/bin` (prebuilt) or `~/.cargo/bin` (cargo).
        #[arg(long)]
        dir: Option<PathBuf>,
        /// How to install; default: cargo when it is on PATH, else prebuilt.
        #[arg(long, value_enum)]
        method: Option<MethodArg>,
    },
}

#[derive(Debug, Subcommand)]
enum Setup {
    /// Read the installed state and print what setup would change. Writes nothing.
    Plan {
        /// Products to end up with, comma separated (`aep,ess,worktree`), or `none`.
        /// Default: the products present now.
        #[arg(long, value_delimiter = ',')]
        products: Option<Vec<String>>,
        /// Hosts to plan for.
        #[arg(long, value_enum, default_value_t = Hosts::All)]
        host: Hosts,
        /// How to install CLIs; default: cargo when it is on PATH, else prebuilt.
        #[arg(long, value_enum)]
        method: Option<MethodArg>,
        /// Print the plan as JSON.
        #[arg(long)]
        json: bool,
        /// Also write the JSON plan to this file, for `apply --plan`.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Apply a plan made by `setup plan --out`.
    Apply {
        /// The plan file.
        #[arg(long)]
        plan: PathBuf,
        /// Confirm that the user approved every listed action.
        #[arg(long)]
        yes: bool,
    },
    /// Restore the settings and binaries a snapshot holds (default: the newest).
    Undo {
        /// Snapshot directory or its timestamp name.
        snapshot: Option<String>,
    },
    /// Print the setup instructions an agent follows (the `b10x:init` skill).
    Guide,
}

/// The `init` skill, printed by `b10x setup guide` so an agent without the plugin reads the same text.
const GUIDE: &str = include_str!("../../../plugins/b10x/skills/init/SKILL.md");

#[derive(Debug, Clone, Copy, ValueEnum)]
enum MethodArg {
    /// The release's checksummed prebuilt archive.
    Prebuilt,
    /// `cargo install` from the release tag.
    Cargo,
}

impl MethodArg {
    fn method(self) -> catalog::Method {
        match self {
            MethodArg::Prebuilt => catalog::Method::Prebuilt,
            MethodArg::Cargo => catalog::Method::Cargo,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Hosts {
    Claude,
    Codex,
    All,
}

impl Hosts {
    fn list(self) -> Vec<Host> {
        match self {
            Hosts::Claude => vec![Host::Claude],
            Hosts::Codex => vec![Host::Codex],
            Hosts::All => vec![Host::Claude, Host::Codex],
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Top::Check => {
            check::run(&Catalog::embedded());
            Ok(ExitCode::SUCCESS)
        }
        Top::Install {
            name,
            tag,
            dir,
            method,
        } => install_one(&name, tag, dir, method),
        Top::Init {
            products,
            method,
            host,
            json,
            out,
        } => {
            if products.is_empty() {
                Err("name the products: aep, ess, worktree, connectors (the /b10x:init skill asks the user which)".to_owned())
            } else {
                emit(
                    make_plan(
                        Some(clean(products)),
                        host.list(),
                        true,
                        method.map(MethodArg::method),
                        false,
                    ),
                    json,
                    out.as_deref(),
                )
            }
        }
        Top::Upgrade {
            products,
            method,
            host,
            json,
            out,
        } => {
            let selection = (!products.is_empty()).then(|| clean(products));
            emit(
                make_plan(
                    selection,
                    host.list(),
                    true,
                    method.map(MethodArg::method),
                    true,
                ),
                json,
                out.as_deref(),
            )
        }
        Top::Skill { id } => skill::text(
            &inventory::home(),
            &Catalog::embedded().marketplace.name,
            &id,
        )
        .map(|text| {
            print!("{text}");
            ExitCode::SUCCESS
        }),
        Top::Setup { command } => match command {
            Setup::Plan {
                products,
                host,
                method,
                json,
                out,
            } => setup_plan(products, host, method, json, out.as_deref()),
            Setup::Apply { plan, yes } => setup_apply(&plan, yes),
            Setup::Undo { snapshot } => setup_undo(snapshot),
            Setup::Guide => {
                print!("{GUIDE}");
                Ok(ExitCode::SUCCESS)
            }
        },
    };
    match result {
        Ok(code) => code,
        Err(error) => {
            eprintln!("b10x: {error}");
            ExitCode::from(1)
        }
    }
}

/// `B10X_MARKETPLACE`: register and read the marketplace from this source (a local checkout or
/// `owner/repo`) instead of the catalog's repository. For testing a marketplace before it is
/// published; users never need it.
fn marketplace_override() -> Option<String> {
    std::env::var("B10X_MARKETPLACE")
        .ok()
        .filter(|value| !value.is_empty())
}

fn clean(products: Vec<String>) -> BTreeSet<String> {
    products
        .into_iter()
        .map(|id| id.trim().to_owned())
        .filter(|id| !id.is_empty() && id != "none")
        .collect()
}

fn make_plan(
    selection: Option<BTreeSet<String>>,
    hosts: Vec<Host>,
    only: bool,
    method: Option<catalog::Method>,
    upgrade: bool,
) -> Result<Plan, String> {
    let overridden = marketplace_override();
    let mut embedded = Catalog::embedded();
    if let Some(source) = &overridden {
        embedded.marketplace.repository.clone_from(source);
    }
    let inventory = inventory::collect(&embedded, &[Host::Claude, Host::Codex]);
    let local = overridden
        .as_deref()
        .map(std::path::Path::new)
        .filter(|path| path.is_dir());
    let source = match (local, resolve::clone_of(&inventory, &embedded)) {
        (Some(path), _) | (None, Some(path)) => Source::Clone(path),
        (None, None) => Source::Remote(&embedded.marketplace.repository),
    };
    let mut catalog = resolve::catalog(&source);
    if let Some(source) = &overridden {
        catalog.marketplace.repository.clone_from(source);
    }
    if let Some(selection) = &selection {
        for id in selection {
            if catalog.product(id).is_none() {
                let known: Vec<&str> = catalog.products.iter().map(|p| p.id.as_str()).collect();
                return Err(format!(
                    "unknown product `{id}`; choose from {}",
                    known.join(", ")
                ));
            }
        }
    }
    let resolved = resolve::resolve(&catalog, &source);
    let home = inventory::home();
    resolve::remember(&home, &resolved);
    let context = plan::Context {
        catalog: &catalog,
        resolved: &resolved,
        selection,
        hosts,
        home: &home,
        only,
        method,
        upgrade,
    };
    Ok(plan::plan(&context, &inventory))
}

fn setup_plan(
    products: Option<Vec<String>>,
    host: Hosts,
    method: Option<MethodArg>,
    json: bool,
    out: Option<&std::path::Path>,
) -> Result<ExitCode, String> {
    emit(
        make_plan(
            products.map(clean),
            host.list(),
            false,
            method.map(MethodArg::method),
            false,
        ),
        json,
        out,
    )
}

/// Print a plan (text, or JSON with `--json`) and write it to `--out`.
fn emit(
    plan: Result<Plan, String>,
    json: bool,
    out: Option<&std::path::Path>,
) -> Result<ExitCode, String> {
    let plan = plan?;
    let text = serde_json::to_string_pretty(&plan).map_err(|error| error.to_string())?;
    if let Some(out) = out {
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        std::fs::write(out, &text).map_err(|error| format!("{}: {error}", out.display()))?;
    }
    if json {
        println!("{text}");
    } else {
        print_plan(&plan);
        if !plan.converged() {
            match out {
                Some(out) => println!(
                    "\nApply after the user confirms this list: b10x setup apply --plan {} --yes",
                    out.display()
                ),
                None => println!(
                    "\nWrite the plan with --out <file>, then apply it after the user confirms."
                ),
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn level(level: Level) -> &'static str {
    match level {
        Level::Ok => "ok",
        Level::Note => "note",
        Level::Change => "change",
        Level::Warn => "WARN",
    }
}

fn print_plan(plan: &Plan) {
    let hosts: Vec<&str> = plan.hosts.iter().map(|host| host.program()).collect();
    println!("b10x setup plan (hosts: {})", hosts.join(", "));
    println!("\nProducts:");
    for offer in &plan.offers {
        println!(
            "  [{}] {:<11} {}{}{}",
            if offer.selected { "x" } else { " " },
            offer.id,
            offer.summary,
            if offer.present { "  (present)" } else { "" },
            if offer.optional { "  (optional)" } else { "" },
        );
    }
    println!("\nFindings:");
    for finding in &plan.findings {
        let host = finding.host.map_or("-", Host::program);
        println!(
            "  {:<6} {:<6} {:<32} {}",
            level(finding.level),
            host,
            finding.subject,
            finding.detail
        );
    }
    let changes: Vec<&plan::Action> = plan.actions.iter().filter(|a| a.changes()).collect();
    if changes.is_empty() {
        println!("\nNothing to change.");
    } else {
        println!("\nActions ({}):", changes.len());
        for (index, action) in changes.iter().enumerate() {
            println!("  {:>2}. {}", index + 1, action.describe());
        }
    }
    let refreshes: Vec<&plan::Action> = plan.actions.iter().filter(|a| !a.changes()).collect();
    if !refreshes.is_empty() {
        println!("\nAlso runs (refreshes the marketplace snapshot; changes no version):");
        for action in refreshes {
            println!("   - {}", action.describe());
        }
    }
    if !plan.next.is_empty() {
        println!("\nNext:");
        for line in &plan.next {
            println!("  {line}");
        }
    }
}

fn setup_apply(path: &PathBuf, yes: bool) -> Result<ExitCode, String> {
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let plan: Plan =
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
    if plan.format != plan::FORMAT {
        return Err(format!(
            "{} is not a `{}` document",
            path.display(),
            plan::FORMAT
        ));
    }
    let embedded = Catalog::embedded();
    let inventory = inventory::collect(&embedded, &[Host::Claude, Host::Codex]);
    apply::fresh(&plan, &inventory)?;
    if !yes {
        print_plan(&plan);
        eprintln!(
            "\nb10x: nothing applied. Confirm every action with the user, then rerun with --yes."
        );
        return Ok(ExitCode::from(2));
    }
    let snapshot = apply::snapshot(&plan)?;
    println!("snapshot: {}", snapshot.display());
    let total = plan.actions.len();
    for (index, action) in plan.actions.iter().enumerate() {
        match apply::run(action) {
            Ok(()) => println!("  ok   {}/{total} {}", index + 1, action.describe()),
            Err(error) => {
                println!(
                    "  FAIL {}/{total} {}\n       {error}",
                    index + 1,
                    action.describe()
                );
                for rest in &plan.actions[index + 1..] {
                    println!("  skip {}", rest.describe());
                }
                println!("Undo with: b10x setup undo {}", snapshot.display());
                return Ok(ExitCode::from(1));
            }
        }
    }
    let selection: BTreeSet<String> = plan
        .offers
        .iter()
        .filter(|offer| offer.selected)
        .map(|offer| offer.id.clone())
        .collect();
    let after = make_plan(
        Some(selection),
        plan.hosts.clone(),
        plan.only,
        plan.method,
        plan.upgrade,
    )?;
    println!("\nAfter:");
    print_plan(&after);
    if after.converged() {
        println!(
            "\nConverged. New plugins load in a new session: restart Claude Code (or /reload-plugins) or start a new Codex thread. In this session, `b10x skill <plugin>` lists a plugin's skills and `b10x skill <plugin>:<skill>` prints one."
        );
        Ok(ExitCode::SUCCESS)
    } else {
        println!("\nNot converged: run `b10x setup plan` again and review the remaining actions.");
        Ok(ExitCode::from(1))
    }
}

fn setup_undo(snapshot: Option<String>) -> Result<ExitCode, String> {
    let root = apply::snapshots();
    let directory = match snapshot {
        Some(given) if given.contains('/') => PathBuf::from(given),
        Some(stamp) => root.join(stamp),
        None => {
            let mut stamps: Vec<u64> = std::fs::read_dir(&root)
                .map_err(|error| format!("{}: {error}", root.display()))?
                .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
                .collect();
            stamps.sort_unstable();
            root.join(stamps.last().ok_or("no snapshot to restore")?.to_string())
        }
    };
    for restored in apply::undo(&directory)? {
        println!("restored {restored}");
    }
    // The restored settings name marketplaces whose snapshots apply removed; each host rebuilds them.
    for argv in [
        ["claude", "plugin", "marketplace", "update"],
        ["codex", "plugin", "marketplace", "upgrade"],
    ] {
        match std::process::Command::new(argv[0])
            .args(&argv[1..])
            .output()
        {
            Ok(output) if output.status.success() => println!("refreshed: {}", argv.join(" ")),
            Ok(output) => println!(
                "could not refresh ({}): {}; run it yourself",
                argv.join(" "),
                String::from_utf8_lossy(&output.stderr)
                    .lines()
                    .next()
                    .unwrap_or("no output")
            ),
            Err(_) => {}
        }
    }
    println!("Restart Claude Code and start a new Codex thread.");
    Ok(ExitCode::SUCCESS)
}

fn install_one(
    name: &str,
    tag: Option<String>,
    dir: Option<PathBuf>,
    method: Option<MethodArg>,
) -> Result<ExitCode, String> {
    let catalog = Catalog::embedded();
    let (_, binary) = catalog
        .binary(name)
        .ok_or_else(|| format!("`{name}` is not a catalog binary"))?;
    let repository = binary.install.repository();
    let tag = match tag {
        Some(tag) => tag,
        None => resolve::latest_tag(repository)
            .ok_or_else(|| format!("no release found for {repository}"))?,
    };
    let method = method.map_or_else(
        || {
            if inventory::copies_on_path("cargo").is_empty() {
                catalog::Method::Prebuilt
            } else {
                catalog::Method::Cargo
            }
        },
        MethodArg::method,
    );
    let home = inventory::home();
    let directory = dir.unwrap_or_else(|| match method {
        catalog::Method::Prebuilt => home.join(".local/bin"),
        catalog::Method::Cargo => home.join(".cargo/bin"),
    });
    let path = install::install(name, &tag, method, &binary.install, &directory)?;
    println!("installed {name} {tag} at {}", path.display());
    Ok(ExitCode::SUCCESS)
}
