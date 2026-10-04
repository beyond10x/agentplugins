//! `agentplugins-check tools` (network): the skills describe the CLIs as they are released now.
//!
//! Every plugin lives in this repository and names no CLI version (website/docs/structure.md R1,
//! R5), so the only thing that can drift is a product release renaming or removing a command the
//! skills still spell. This check acquires every catalogued CLI's newest release: a prebuilt archive
//! checked against `SHA256SUMS`, or a locked Cargo build from its release tag when the catalog has
//! no archive route. Every spelled command in plugin code is checked with `--help`. AEP migration
//! and ESS semantic source/pin upgrades are exercised against disposable repositories. The separate
//! `docs --tools` check validates and executes the public tutorial with its committed implementation.
//! It runs on every pull request, every `main` push and daily; a red run is fixed by a skill edit.
//!
//! It also fails when a CLI's newest release is newer than `verified.json`, the release its skills
//! were last verified against: every product release is re-verified (this check and an ESS trial
//! round) before `verified.json` moves. The offline gate only checks that file's shape.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(serde::Deserialize)]
pub(crate) struct Tool {
    pub name: String,
    #[serde(default)]
    platforms: Vec<String>,
    install: Installation,
}

#[derive(serde::Deserialize)]
struct Installation {
    archive: Option<Source>,
    cargo: Option<Source>,
}

#[derive(serde::Deserialize)]
struct Source {
    repository: String,
    package: Option<String>,
}

impl Tool {
    pub(crate) fn repository(&self) -> &str {
        self.install
            .archive
            .as_ref()
            .or(self.install.cargo.as_ref())
            .map_or("", |source| source.repository.as_str())
    }

    fn supported(&self, os: &str) -> bool {
        self.platforms.is_empty() || self.platforms.iter().any(|platform| platform == os)
    }
}

/// Read the catalog on disk, so adding a binary also adds its compatibility obligation.
pub(crate) fn catalog_tools(root: &Path) -> Result<Vec<Tool>, String> {
    #[derive(serde::Deserialize)]
    struct Catalog {
        products: Vec<Product>,
    }
    #[derive(serde::Deserialize)]
    struct Product {
        binaries: Vec<Tool>,
    }
    let text = std::fs::read_to_string(root.join("catalog.json")).map_err(|e| e.to_string())?;
    let catalog: Catalog = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let tools: Vec<Tool> = catalog
        .products
        .into_iter()
        .flat_map(|p| p.binaries)
        .collect();
    let mut names = BTreeSet::new();
    for tool in &tools {
        if tool.repository().is_empty() || !names.insert(&tool.name) {
            return Err(format!(
                "catalog: {} has no installation source or duplicates a CLI",
                tool.name
            ));
        }
    }
    Ok(tools)
}

/// Most subcommand words taken from one spelled command.
const DEPTH: usize = 4;

/// The file naming, per CLI, the release the skills were last verified against.
pub const VERIFIED: &str = "verified.json";

/// A key for an `x.y.z` version or tag.
pub(crate) fn key(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.trim_start_matches('v').split('.');
    let triple = (
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    );
    parts.next().is_none().then_some(triple)
}

/// Offline: `verified.json` names exactly the CLIs the skills drive, each at an `x.y.z` release.
pub fn verified(root: &Path) -> Result<std::collections::BTreeMap<String, String>, String> {
    let path = root.join(VERIFIED);
    let text = std::fs::read_to_string(&path).map_err(|error| format!("{VERIFIED}: {error}"))?;
    let map: std::collections::BTreeMap<String, String> = serde_json::from_str(&text)
        .map_err(|error| format!("{VERIFIED}: an object of CLI → `x.y.z` release: {error}"))?;
    let tools = catalog_tools(root)?;
    let expected: BTreeSet<&str> = tools.iter().map(|tool| tool.name.as_str()).collect();
    let named: BTreeSet<&str> = map.keys().map(String::as_str).collect();
    if named != expected {
        return Err(format!(
            "{VERIFIED}: names {named:?}; it must name exactly {expected:?}"
        ));
    }
    for (cli, release) in &map {
        if key(release).is_none() {
            return Err(format!(
                "{VERIFIED}: `{cli}` is `{release}`, not an `x.y.z` release"
            ));
        }
    }
    Ok(map)
}

/// The line for a CLI whose newest release is newer than the one its skills were verified against.
#[must_use]
pub fn unverified(cli: &str, newest: &str, verified: &str) -> Option<String> {
    (key(newest)? > key(verified)?).then(|| {
        format!(
            "{cli} {newest} is newer than {VERIFIED} ({verified}): re-verify the skills (`agentplugins-check tools` and an ESS trial round), then set `{cli}` to {newest} in {VERIFIED}"
        )
    })
}

pub(crate) fn run(program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| format!("running {program}: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "{program} {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn target() -> Result<&'static str, String> {
    match (std::env::consts::ARCH, std::env::consts::OS) {
        ("x86_64", "linux") => Ok("x86_64-unknown-linux-gnu"),
        ("aarch64", "linux") => Ok("aarch64-unknown-linux-gnu"),
        ("x86_64", "macos") => Ok("x86_64-apple-darwin"),
        ("aarch64", "macos") => Ok("aarch64-apple-darwin"),
        (arch, os) => Err(format!("no release archive for {arch}-{os}")),
    }
}

pub(crate) fn latest(repository: &str) -> Result<String, String> {
    let location = run(
        "curl",
        &[
            "-fsS",
            "-o",
            "/dev/null",
            "-w",
            "%{redirect_url}",
            &format!("https://github.com/{repository}/releases/latest"),
        ],
    )?;
    location
        .rsplit_once("/releases/tag/")
        .map(|(_, tag)| tag.trim().to_owned())
        .filter(|tag| !tag.is_empty())
        .ok_or_else(|| format!("{repository} has no release"))
}

/// Download, verify and unpack a CLI's newest release; returns its tag and the binary.
fn fetch(tool: &Tool, scratch: &Path, build_cache: &Path) -> Result<(String, PathBuf), String> {
    let cli = &tool.name;
    let repository = tool.repository();
    let tag = latest(repository)?;
    if tool.install.archive.is_none() {
        let source = tool.install.cargo.as_ref().ok_or("no Cargo source")?;
        let package = source.package.as_deref().ok_or("no Cargo package")?;
        let dir = scratch.join(cli);
        run(
            "cargo",
            &[
                "install",
                "--locked",
                "--jobs",
                "2",
                "--target-dir",
                &build_cache.to_string_lossy(),
                "--git",
                &format!("https://github.com/{}", source.repository),
                "--tag",
                &tag,
                "--root",
                &dir.to_string_lossy(),
                package,
            ],
        )?;
        return Ok((tag, dir.join("bin").join(cli)));
    }
    let target = target()?;
    let version = tag.trim_start_matches('v');
    let archive = format!("{cli}-{version}-{target}.tar.gz");
    let base = format!("https://github.com/{repository}/releases/download/{tag}");
    let dir = scratch.join(cli);
    std::fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    for file in [archive.as_str(), "SHA256SUMS"] {
        run(
            "curl",
            &[
                "-fsSL",
                "--retry",
                "2",
                "-o",
                &dir.join(file).to_string_lossy(),
                &format!("{base}/{file}"),
            ],
        )?;
    }
    let listed =
        std::fs::read_to_string(dir.join("SHA256SUMS")).map_err(|error| error.to_string())?;
    let line = listed
        .lines()
        .find(|line| line.ends_with(&archive))
        .ok_or_else(|| format!("{repository} {tag}: SHA256SUMS does not list {archive}"))?;
    let expected = line.split_whitespace().next().unwrap_or_default();
    let actual = run("sha256sum", &[&dir.join(&archive).to_string_lossy()])?;
    if !actual.starts_with(expected) {
        return Err(format!(
            "{repository} {tag}: {archive} does not match SHA256SUMS"
        ));
    }
    run(
        "tar",
        &[
            "-xzf",
            &dir.join(&archive).to_string_lossy(),
            "-C",
            &dir.to_string_lossy(),
        ],
    )?;
    Ok((
        tag.clone(),
        dir.join(format!("{cli}-{version}-{target}")).join(cli),
    ))
}

/// Subcommand words of every command spelled for `cli` in `text`: code spans and code blocks only,
/// Skip known options and their values, stopping at a placeholder, positional path,
/// or an option whose arity is not known; at most [`DEPTH`] command words.
#[must_use]
pub fn spelled(text: &str, cli: &str) -> BTreeSet<Vec<String>> {
    let mut code = Vec::new();
    let mut fenced = false;
    let text = text.replace("\\\n", " ");
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            code.push(line.trim_start().trim_start_matches("$ ").to_owned());
        } else {
            let mut parts = line.split('`');
            parts.next();
            while let Some(span) = parts.next() {
                code.push(span.to_owned());
                parts.next();
            }
        }
    }
    let mut commands = BTreeSet::new();
    for snippet in code {
        let Some(tokens) = shlex::split(&snippet) else {
            continue;
        };
        let words: Vec<&str> = tokens.iter().map(String::as_str).collect();
        for (index, word) in words.iter().enumerate() {
            let starts =
                index == 0 || matches!(words[index - 1], "&&" | "||" | "|" | ";" | "then" | "do");
            if *word != cli || !starts {
                continue;
            }
            let mut path = Vec::new();
            let mut remaining = words[index + 1..].iter();
            while let Some(word) = remaining.next() {
                if word.starts_with('-') {
                    if word.contains('=')
                        || matches!(
                            *word,
                            "--json"
                                | "--strict-requires"
                                | "--dry-run"
                                | "--verify"
                                | "--verbose"
                                | "--quiet"
                                | "--help"
                                | "-h"
                        )
                    {
                        continue;
                    }
                    // These options take values even when the value looks like a command.
                    // Unknown options stop extraction rather than inventing their arity.
                    if matches!(
                        *word,
                        "--store"
                            | "--path"
                            | "--engineering"
                            | "--root"
                            | "--repo"
                            | "--cwd"
                            | "--format"
                            | "--output"
                            | "--state-dir"
                            | "--profile"
                            | "--config"
                    ) {
                        if remaining
                            .next()
                            .is_some_and(|value| value.starts_with('<') && !value.ends_with('>'))
                        {
                            for part in remaining.by_ref() {
                                if part.ends_with('>') {
                                    break;
                                }
                            }
                        }
                        continue;
                    }
                    break;
                }
                if !word
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                    || !word.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
                {
                    break;
                }
                path.push((*word).to_owned());
                if path.len() == DEPTH {
                    break;
                }
            }
            if !path.is_empty() {
                commands.insert(path);
            }
        }
    }
    commands
}

fn markdown(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Validate the three YAML blocks of ESS's syntax reference with the released `ess`.
fn syntax(root: &Path, ess: &Path, scratch: &Path) -> Result<(), String> {
    let reference = root.join("plugins/ess/skills/specifying/references/syntax.md");
    let text = std::fs::read_to_string(&reference).map_err(|error| error.to_string())?;
    let blocks: Vec<&str> = text
        .split("```yaml\n")
        .skip(1)
        .filter_map(|rest| rest.split("```").next())
        .collect();
    let [system, components, domain] = blocks.as_slice() else {
        return Err(format!(
            "{}: expected three yaml blocks, found {}",
            reference.display(),
            blocks.len()
        ));
    };
    let dir = scratch.join("syntax");
    std::fs::create_dir_all(dir.join("domains")).map_err(|error| error.to_string())?;
    std::fs::write(dir.join("system.yaml"), system).map_err(|error| error.to_string())?;
    std::fs::write(dir.join("components.yaml"), components).map_err(|error| error.to_string())?;
    std::fs::write(dir.join("domains/list.yaml"), domain).map_err(|error| error.to_string())?;
    let out = run(
        &ess.to_string_lossy(),
        &["specify", "validate", "--path", &dir.to_string_lossy()],
    )
    .map_err(|error| {
        format!(
            "{}: the example no longer validates: {error}",
            reference.display()
        )
    })?;
    println!("tools `ess`: syntax example — {}", out.trim());
    let ess = ess.to_string_lossy();
    let path = dir.to_string_lossy();
    let suite = scratch.join("syntax-suite.json");
    let generated = scratch.join("syntax-generated");
    let generated = generated.to_string_lossy();
    let synthesized = run(
        &ess,
        &[
            "verify",
            "conform",
            "synthesize",
            "--path",
            &path,
            "--out",
            &suite.to_string_lossy(),
        ],
    )
    .map_err(|error| format!("{}: synthesize failed: {error}", reference.display()))?;
    if !synthesized.contains(" 0 refusal(s)") {
        return Err(format!(
            "{}: the example synthesizes with refusals: {}",
            reference.display(),
            synthesized.trim()
        ));
    }
    println!("tools `ess`: syntax example — {}", synthesized.trim());
    for arguments in [
        &["generate", "--kind", "schema"][..],
        &["generate", "project", "openapi"][..],
    ] {
        let mut arguments = arguments.to_vec();
        arguments.extend(["--path", &path, "--out", &generated]);
        run(&ess, &arguments).map_err(|error| {
            format!(
                "{}: `ess {}` failed on the example: {error}",
                reference.display(),
                arguments[..arguments.len() - 4].join(" ")
            )
        })?;
    }
    ess_upgrade(Path::new(ess.as_ref()), &dir, scratch)
}

fn snapshot(path: &Path) -> Result<std::collections::BTreeMap<PathBuf, Vec<u8>>, String> {
    let mut files = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_name() == ".git" {
            continue;
        }
        if entry.path().is_dir() {
            files.extend(snapshot(&entry.path())?);
        } else {
            files.insert(
                entry.path(),
                std::fs::read(entry.path()).map_err(|e| e.to_string())?,
            );
        }
    }
    Ok(files)
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn refusal(dir: &Path, binary: &Path, args: &[&str], contains: &str) -> Result<(), String> {
    match run_in(dir, &binary.to_string_lossy(), args) {
        Err(error) if error.contains(contains) => Ok(()),
        result => Err(format!(
            "expected refusal containing {contains:?}, got {result:?}"
        )),
    }
}

/// Reproduce the documented /1 -> /20 semantic decision: sparse emitted identity
/// ownership is no longer implicit. This is a compatibility fixture, not a migrator.
fn ess_upgrade(ess: &Path, original: &Path, scratch: &Path) -> Result<(), String> {
    let dir = scratch.join("ess-upgrade");
    copy(original, &dir)?;
    let system = std::fs::read_to_string(dir.join("system.yaml")).map_err(|e| e.to_string())?;
    write(
        &dir.join("system.yaml"),
        &system.replace("format: ess/1", "format: ess/20"),
    )?;
    refusal(
        &dir,
        ess,
        &["specify", "validate", "--path", "."],
        "emitted payload field has no source",
    )?;
    let domain =
        std::fs::read_to_string(dir.join("domains/list.yaml")).map_err(|e| e.to_string())?;
    let domain = domain
        .replace(
            "          library.lending.BranchOpened:\n",
            "          library.lending.BranchOpened:\n            branch_id: {generated: true}\n",
        )
        .replace(
            "          library.lending.CopyAdded:\n",
            "          library.lending.CopyAdded:\n            copy_id: {generated: true}\n",
        );
    write(&dir.join("domains/list.yaml"), &domain)?;
    run_in(
        &dir,
        &ess.to_string_lossy(),
        &["specify", "validate", "--path", "."],
    )?;
    ess_pin_upgrade(ess, &dir, scratch)?;
    for args in [
        vec!["generate", "--kind", "schema"],
        vec!["generate", "project", "openapi"],
    ] {
        let mut args = args;
        args.extend(["--path", ".", "--out", "../ess-upgrade-generated"]);
        run_in(&dir, &ess.to_string_lossy(), &args)?;
    }
    let result = run_in(
        &dir,
        &ess.to_string_lossy(),
        &[
            "verify",
            "conform",
            "synthesize",
            "--path",
            ".",
            "--out",
            "suite.json",
        ],
    )?;
    let suite: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.join("suite.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if !result.contains(" 0 refusal(s)")
        || suite["scenarios"]
            .as_object()
            .is_none_or(serde_json::Map::is_empty)
    {
        return Err(format!(
            "upgraded ESS suite has missing scenarios or refusals: {result}"
        ));
    }
    if !std::fs::read_to_string(dir.join("system.yaml"))
        .map_err(|e| e.to_string())?
        .contains("version: v1")
    {
        return Err("source format upgrade changed product version".into());
    }
    println!("tools `ess`: /1 -> /20 refuses missing ownership, accepts explicit generated identities, regenerates schema/OpenAPI and a nonempty conformance suite; product v1 preserved (suite execution belongs to docs --tools)");
    Ok(())
}

/// Distinguish the dispatcher from the effective exact pin, without changing the
/// active installation. The historical pin is an explicit compatibility fixture.
fn ess_pin_upgrade(ess: &Path, dir: &Path, scratch: &Path) -> Result<(), String> {
    let manifest = dir.join("ess-inputs.yaml");
    let old = "format: ess-inputs/2\nrequires: ess 0.51.0\nspecification: [system.yaml, components.yaml, domains/list.yaml]\nscenarios: []\n";
    write(&manifest, old)?;
    let effective = |args: &[&str]| -> Result<String, String> {
        let output = Command::new(ess)
            .args(args)
            .current_dir(dir)
            .env_remove("ESS_TOOLCHAIN_DELEGATED")
            .env_remove("ESS_TOOLCHAIN")
            .env_remove("ESS_TOOLCHAIN_BASE_URL")
            .env("ESS_TOOLCHAIN_DIR", scratch.join("ess-toolchains"))
            .output()
            .map_err(|e| e.to_string())?;
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if output.status.success() {
            Ok(text)
        } else {
            Err(text)
        }
    };
    let selected = effective(&["specify", "toolchain", "which"])?;
    if !selected.starts_with("ess 0.51.0\n") {
        return Err(format!("exact pin was not selected: {selected}"));
    }
    effective(&["--strict-requires", "specify", "validate", "--path", "."])?;
    refusal(
        dir,
        ess,
        &["--strict-requires", "specify", "validate", "--path", "."],
        "requires ess 0.51.0",
    )?;
    if std::fs::read_to_string(&manifest).map_err(|e| e.to_string())? != old {
        return Err("ESS discovery or validation rewrote the existing pin".into());
    }
    let current = run(&ess.to_string_lossy(), &["--version"])?;
    let current = current
        .trim()
        .strip_prefix("ess ")
        .ok_or("ESS version unavailable")?;
    write(
        &manifest,
        &old.replace("ess 0.51.0", &format!("ess {current}")),
    )?;
    let selected = effective(&["specify", "toolchain", "which"])?;
    if !selected.starts_with(&format!("ess {current}\n")) {
        return Err(format!("accepted pin update was not effective: {selected}"));
    }
    effective(&["--strict-requires", "specify", "validate", "--path", "."])?;
    println!("tools `ess`: exact older pin delegated in isolated cache, unchanged until explicit fixture update; effective toolchain now {current}");
    Ok(())
}

/// Exercise migration with real released binaries, including non-Git preservation.
fn aep_upgrade(aep: &Path, tag: &str, scratch: &Path) -> Result<(), String> {
    let protocols = scratch.join("aep-protocols");
    run(
        "git",
        &[
            "clone",
            "--quiet",
            "--depth",
            "1",
            "--branch",
            tag,
            "https://github.com/beyond10x/aep",
            &protocols.to_string_lossy(),
        ],
    )?;
    let dir = scratch.join("aep-upgrade");
    let engineering = dir.join(".engineering");
    let planning = engineering.join("planning/story");
    std::fs::create_dir_all(&planning).map_err(|e| e.to_string())?;
    let project = "version: aep.project/1\nprotocol: adp/1\nprofile: development.standard\nprotocols: ../../aep-protocols\nsummary: upgrade fixture\n";
    write(&engineering.join("project.yaml"), project)?;
    write(&planning.join("retained.md"), "---\nformat: aep.planning-md/1\nid: story:retained\nkind: story\nstatus: draft\ntitle: Retained\nrevision: 1\n---\n\nRetain this body.\n")?;
    write(&engineering.join("planning/journal.jsonl"), "")?;
    run_in(&dir, "git", &["init", "--quiet"])?;
    run_in(&dir, "git", &["add", ".engineering"])?;
    run_in(
        &dir,
        "git",
        &[
            "-c",
            "user.name=compatibility-fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "--quiet",
            "-m",
            "fixture",
        ],
    )?;
    refusal(&dir, aep, &["plan", "artifact", "list"], "aep.project/1")?;
    let before = snapshot(&engineering)?;
    run_in(
        &dir,
        &aep.to_string_lossy(),
        &["plan", "store", "migrate", "git", "--dry-run"],
    )?;
    if snapshot(&engineering)? != before {
        return Err("AEP dry-run wrote the store".into());
    }
    let migrated = run_in(
        &dir,
        &aep.to_string_lossy(),
        &["plan", "store", "migrate", "git", "--verify"],
    )?;
    if !migrated.contains("verified 1 artifact(s)") {
        return Err(format!("AEP migration lacks verification: {migrated}"));
    }
    let listed = run_in(&dir, &aep.to_string_lossy(), &["plan", "artifact", "list"])?;
    if !listed.contains("retained")
        || !std::fs::read_to_string(planning.join("retained.md"))
            .map_err(|e| e.to_string())?
            .contains("Retain this body.")
    {
        return Err("AEP migration lost the artifact or its body".into());
    }
    let modern =
        std::fs::read_to_string(engineering.join("project.yaml")).map_err(|e| e.to_string())?;
    aep_database_selectors(aep, &dir, &modern)?;
    println!("tools `aep`: legacy refusal, read-only dry-run, verified Git migration");
    Ok(())
}

fn aep_database_selectors(aep: &Path, dir: &Path, modern: &str) -> Result<(), String> {
    let engineering = dir.join(".engineering");
    let postgres = std::env::var("AGENTPLUGINS_TEST_POSTGRES_URL").ok();
    for (backend, locator, field) in [
        ("sqlite", "planning.sqlite3", "path"),
        (
            "postgres",
            postgres
                .as_deref()
                .unwrap_or("postgres://localhost/compatibility"),
            "url",
        ),
    ] {
        let mut manifest: serde_yaml::Value =
            serde_yaml::from_str(modern).map_err(|e| e.to_string())?;
        manifest["store"] = serde_yaml::from_str(&format!("{backend}:\n  {field}: {locator}"))
            .map_err(|e| e.to_string())?;
        let corrected = serde_yaml::to_string(&manifest).map_err(|e| e.to_string())?;
        let project = engineering.join("project.yaml");
        write(&project, &corrected)?;
        // Seed a real SQLite row. The fixture then regresses only the project
        // selector: its correction must not migrate or replace database bytes.
        let live = backend == "sqlite" || postgres.is_some();
        let inventory = if live {
            run_in(
                dir,
                &aep.to_string_lossy(),
                &[
                    "plan",
                    "artifact",
                    "new",
                    "story",
                    "database-retained",
                    "--title",
                    "Retained database row",
                ],
            )?;
            Some(run_in(
                dir,
                &aep.to_string_lossy(),
                &["plan", "artifact", "list"],
            )?)
        } else {
            None
        };
        let sqlite_before = if backend == "sqlite" {
            Some(std::fs::read(engineering.join(locator)).map_err(|e| e.to_string())?)
        } else {
            None
        };
        manifest["version"] = "aep.project/1".into();
        manifest["store"] =
            serde_yaml::from_str(&format!("{backend}: {locator}")).map_err(|e| e.to_string())?;
        write(
            &project,
            &serde_yaml::to_string(&manifest).map_err(|e| e.to_string())?,
        )?;
        let before = snapshot(&engineering)?;
        refusal(dir, aep, &["plan", "artifact", "list"], "aep.project/1")?;
        refusal(
            dir,
            aep,
            &["plan", "store", "migrate", "git", "--verify"],
            "other than markdown",
        )?;
        if snapshot(&engineering)? != before {
            return Err(format!("migration changed legacy {backend} state"));
        }
        // The released refusal explicitly prescribes this manifest correction,
        // with the same backend/locator. There is no database migration command.
        write(&project, &corrected)?;
        if let Some(bytes) = sqlite_before {
            if std::fs::read(engineering.join(locator)).map_err(|e| e.to_string())? != bytes {
                return Err("SQLite bytes changed during project format correction".into());
            }
        }
        if let Some(inventory) = inventory {
            let rows = run_in(dir, &aep.to_string_lossy(), &["plan", "artifact", "list"])?;
            if rows != inventory || !rows.contains("database-retained") {
                return Err(format!(
                    "{backend} inventory changed across project correction"
                ));
            }
            println!("tools `aep`: {backend} populated inventory retained across legacy selector correction");
        }
    }
    println!("tools `aep`: legacy SQLite/PostgreSQL selectors refuse Git migration without writes; documented /5 correction preserves SQLite bytes");
    if postgres.is_none() {
        println!("tools `aep`: PostgreSQL live preservation NOT VERIFIED; requires separate released_postgres_backend_preservation test with an isolated AGENTPLUGINS_TEST_POSTGRES_URL");
    }
    Ok(())
}

/// The public tutorial's committed specification and Go implementation.
pub const TUTORIAL: &str = "website/docs/tutorials/first-ess-specification";

/// Tutorial pages, whose spelled commands are held to the newest releases like the skills'.
pub const TUTORIALS: &str = "website/docs/tutorials";

/// Network documentation gate; kept separate from plugin compatibility.
pub fn verify_docs(root: &Path) -> Result<(), String> {
    let scratch =
        std::env::temp_dir().join(format!("agentplugins-check-docs-{}", std::process::id()));
    let result = (|| {
        for tool in catalog_tools(root)? {
            let cli = tool.name.as_str();
            let commands: BTreeSet<_> = markdown(&root.join(TUTORIALS))
                .into_iter()
                .map(|file| std::fs::read_to_string(file).map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .flat_map(|text| spelled(text, cli))
                .collect();
            if commands.is_empty() && cli != "ess" {
                continue;
            }
            if !tool.supported(std::env::consts::OS) {
                return Err(format!(
                    "docs: cannot execute {cli} on {}",
                    std::env::consts::OS
                ));
            }
            let (_, binary) = fetch(&tool, &scratch, &root.join("target/external-tools"))?;
            for command in commands {
                let mut args: Vec<_> = command.iter().map(String::as_str).collect();
                args.push("--help");
                run(&binary.to_string_lossy(), &args)?;
            }
            if cli == "ess" {
                tutorial(root, &binary, &scratch)?;
            }
        }
        Ok(())
    })();
    let _ = std::fs::remove_dir_all(scratch);
    result
}

fn copy(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|error| format!("{}: {error}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|error| format!("{}: {error}", from.display()))? {
        let path = entry.map_err(|error| error.to_string())?.path();
        let target = to.join(path.file_name().unwrap_or_default());
        if path.is_dir() {
            copy(&path, &target)?;
        } else {
            std::fs::copy(&path, &target)
                .map_err(|error| format!("{}: {error}", path.display()))?;
        }
    }
    Ok(())
}

/// Run in `dir` with `ESS_TOOLCHAIN_DELEGATED=1`, so a `requires:` pin in the tutorial's
/// `ess-inputs.yaml` does not hand the command to the pinned release: the newest one is under test.
fn run_in(dir: &Path, program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(dir)
        .env("ESS_TOOLCHAIN_DELEGATED", "1")
        .env("ESS_REPORT_FORMAT", "2")
        .output()
        .map_err(|error| format!("running {program}: {error}"))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        Ok(text)
    } else {
        Err(format!(
            "{program} {} failed: {}",
            arguments.join(" "),
            text.trim()
        ))
    }
}

/// The tutorial, as a reader runs it: its specification validates and synthesizes a Go suite with
/// no refusals, and its implementation passes that suite under `go test`.
fn tutorial(root: &Path, ess: &Path, scratch: &Path) -> Result<(), String> {
    let dir = scratch.join("tutorial");
    copy(&root.join(TUTORIAL), &dir)?;
    let ess = ess.to_string_lossy();
    let fail = |step: &str, error: String| format!("{TUTORIAL}: {step}: {error}");
    let validated = run_in(&dir, &ess, &["specify", "validate", "--path", "spec"])
        .map_err(|error| fail("the specification no longer validates", error))?;
    println!("tools `ess`: tutorial — {}", validated.trim());
    let synthesized = run_in(
        &dir,
        &ess,
        &[
            "verify",
            "conform",
            "synthesize",
            "--path",
            "spec",
            "--target",
            "go",
            "--out",
            "impl",
        ],
    )
    .map_err(|error| fail("synthesize failed", error))?;
    if !synthesized.contains(" 0 refusal(s)") {
        return Err(fail(
            "the specification synthesizes with refusals",
            synthesized.trim().to_owned(),
        ));
    }
    println!("tools `ess`: tutorial — {}", synthesized.trim());
    let tested = run_in(&dir.join("impl"), "go", &["test", "./..."])
        .map_err(|error| fail("the implementation fails its suite", error))?;
    println!(
        "tools `ess`: tutorial — go test: {}",
        tested.lines().next().unwrap_or_default().trim()
    );
    Ok(())
}

/// Check every plugin's spelled commands against its CLI's newest release.
pub fn verify(root: &Path) -> Result<(), String> {
    let scratch =
        std::env::temp_dir().join(format!("agentplugins-check-tools-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let result = (|| {
        let verified = verified(root)?;
        let mut problems = Vec::new();
        for tool in catalog_tools(root)? {
            let cli = tool.name.as_str();
            if !tool.supported(std::env::consts::OS) {
                println!("tools `{cli}`: unavailable on {}; supported platforms: {:?}; no executable verification claimed", std::env::consts::OS, tool.platforms);
                continue;
            }
            let (tag, binary) = fetch(&tool, &scratch, &root.join("target/external-tools"))?;
            // Even a catalogued tool with no command example must actually run.
            run(&binary.to_string_lossy(), &["--help"])?;
            let identity = run(&binary.to_string_lossy(), &["--version"])?;
            if identity.split_whitespace().last().and_then(key) != key(&tag) {
                return Err(format!(
                    "{cli} release {tag} reports a different identity: {}",
                    identity.trim()
                ));
            }
            println!(
                "tools `{cli}`: released binary identity — {}",
                identity.trim()
            );
            if let Some(line) = verified
                .get(cli)
                .and_then(|release| unverified(cli, &tag, release))
            {
                problems.push(line);
            }
            let mut checked = 0;
            let files = markdown(&root.join("plugins"));
            for file in files {
                let text = std::fs::read_to_string(&file).map_err(|error| error.to_string())?;
                for path in spelled(&text, cli) {
                    checked += 1;
                    let mut arguments: Vec<&str> = path.iter().map(String::as_str).collect();
                    arguments.push("--help");
                    let status = Command::new(&binary)
                        .args(&arguments)
                        .output()
                        .map_err(|error| error.to_string())?;
                    if !status.status.success() {
                        problems.push(format!(
                            "{}: `{cli} {}` is not a command of {cli} {tag}",
                            file.strip_prefix(root).unwrap_or(&file).display(),
                            path.join(" ")
                        ));
                    }
                }
            }
            println!("tools `{cli}` {tag}: {checked} spelled command(s) checked");
            if cli == "ess" {
                syntax(root, &binary, &scratch)?;
            }
            if cli == "aep" {
                aep_upgrade(&binary, &tag, &scratch)?;
            }
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{} problem(s) against the newest releases:\n  {}",
                problems.len(),
                problems.join("\n  ")
            ))
        }
    })();
    let _ = std::fs::remove_dir_all(&scratch);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_newer_release_than_verified_is_named_with_the_step() {
        let line = unverified("ess", "0.32.2", "0.32.1").unwrap();
        assert!(line.starts_with("ess 0.32.2 is newer than verified.json (0.32.1)"));
        assert!(line.contains("ESS trial round"));
        assert_eq!(unverified("ess", "0.32.1", "0.32.1"), None);
        assert_eq!(unverified("ess", "v0.32.0", "0.32.1"), None);
    }

    #[test]
    fn the_committed_verified_file_has_its_shape() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let map = verified(&root).unwrap();
        assert_eq!(map.len(), catalog_tools(&root).unwrap().len());
    }

    #[test]
    fn commands_are_read_from_code_only() {
        let text = "Run `ess specify validate --path <spec>` then prose ess binary.\n\n```console\n$ ess generate project openapi --out x\nb10x skill ess:init\n```\n`ess` alone, `ess specify|generate <verb>`\n";
        let found: Vec<Vec<String>> = spelled(text, "ess").into_iter().collect();
        assert_eq!(
            found,
            [
                vec![
                    "generate".to_owned(),
                    "project".to_owned(),
                    "openapi".to_owned()
                ],
                vec!["specify".to_owned(), "validate".to_owned()],
            ]
        );
    }

    #[test]
    fn options_before_subcommands_do_not_hide_them() {
        for command in [
            "aep --store planning plan artifact list",
            "aep --store=planning plan artifact list",
            "aep --json --engineering .engineering plan artifact list",
            "aep --store 'plan with spaces' plan artifact list",
            "aep --store planning \\\n plan artifact list",
        ] {
            assert!(spelled(&format!("`{command}`"), "aep").contains(&vec![
                "plan".into(),
                "artifact".into(),
                "list".into()
            ]));
        }
        assert_eq!(
            spelled("`aep plan --store plan artifact list`", "aep"),
            BTreeSet::from([vec!["plan".into(), "artifact".into(), "list".into()]])
        );
        assert_eq!(
            spelled(
                "`ess --strict-requires specify validate --path spec`",
                "ess"
            ),
            BTreeSet::from([vec!["specify".into(), "validate".into()]])
        );
        assert_eq!(
            spelled(
                "`ess specify validate --path <directory holding it>`",
                "ess"
            ),
            BTreeSet::from([vec!["specify".into(), "validate".into()]])
        );
    }

    #[test]
    fn catalog_includes_cargo_only_and_platform_restricted_tools() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let tools = catalog_tools(&root).unwrap();
        assert_eq!(tools.len(), 6);
        let connectors = tools.iter().find(|tool| tool.name == "connectors").unwrap();
        assert!(connectors.install.archive.is_none());
        assert_eq!(
            connectors
                .install
                .cargo
                .as_ref()
                .unwrap()
                .package
                .as_deref(),
            Some("connectors")
        );
        let harness = tools
            .iter()
            .find(|tool| tool.name == "b10x-harness")
            .unwrap();
        assert!(harness.supported("linux"));
        assert!(!harness.supported("macos"));
    }

    #[test]
    fn connectors_global_options_do_not_hide_valid_or_retired_commands() {
        for operation in ["list", "describe", "invoke"] {
            let code = format!("`connectors --state-dir <local state directory> --output json operations {operation} --endpoint provider`");
            assert_eq!(
                spelled(&code, "connectors"),
                BTreeSet::from([vec!["operations".into(), operation.into()]])
            );
        }
        // The extractor must also carry a retired command to the released binary,
        // where help exits nonzero. An empty set would silently bypass the gate.
        assert_eq!(
            spelled("`connectors --output json inspect providers`", "connectors"),
            BTreeSet::from([vec!["inspect".into(), "providers".into()]])
        );
        assert!(spelled(
            "prose connectors --output json operations list",
            "connectors"
        )
        .is_empty());
    }

    #[test]
    #[ignore = "network and released AEP/ESS binaries required; core tools runs these on downloaded releases"]
    fn released_upgrade_behaviors() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let scratch = root.join(".scratch/released-upgrades");
        if scratch.exists() {
            std::fs::remove_dir_all(&scratch).unwrap();
        }
        std::fs::create_dir_all(&scratch).unwrap();
        let versions = verified(&root).unwrap();
        for cli in ["aep", "ess"] {
            assert_eq!(
                run(cli, &["--version"]).unwrap().trim(),
                format!("{cli} {}", versions[cli])
            );
        }
        aep_upgrade(Path::new("aep"), &versions["aep"], &scratch).unwrap();
        syntax(&root, Path::new("ess"), &scratch).unwrap();
        std::fs::remove_dir_all(scratch).unwrap();
    }

    #[test]
    #[ignore = "downloads and executes every catalogued current stable release"]
    fn released_catalog_compatibility() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        verify(&root).unwrap();
    }

    #[test]
    #[ignore = "requires an isolated disposable AGENTPLUGINS_TEST_POSTGRES_URL and released AEP"]
    fn released_postgres_backend_preservation() {
        assert!(
            std::env::var("AGENTPLUGINS_TEST_POSTGRES_URL").is_ok(),
            "live database evidence is required"
        );
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let scratch = root.join(".scratch/released-postgres");
        if scratch.exists() {
            std::fs::remove_dir_all(&scratch).unwrap();
        }
        std::fs::create_dir_all(&scratch).unwrap();
        let version = verified(&root).unwrap()["aep"].clone();
        assert_eq!(
            run("aep", &["--version"]).unwrap().trim(),
            format!("aep {version}")
        );
        aep_upgrade(Path::new("aep"), &version, &scratch).unwrap();
        std::fs::remove_dir_all(scratch).unwrap();
    }
}
