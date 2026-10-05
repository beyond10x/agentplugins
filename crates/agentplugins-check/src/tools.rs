//! `agentplugins-check tools` (network): the skills describe the CLIs as they are released now.
//!
//! Every plugin lives in this repository and names no CLI version (website/docs/structure.md R1,
//! R5), so the only thing that can drift is a product release renaming or removing a command the
//! skills still spell. This check downloads each product's newest release — the prebuilt archive,
//! checked against its `SHA256SUMS` — and runs `<cli> <subcommands> --help` for every command a
//! code span or code block in plugins, public docs or root guidance spells. Connectors has no
//! binary assets: its exact release's CLI source contract verifies paths and long flags, with
//! explicit source-only provenance. This is not a runtime invocation check.
//! ESS's syntax example must also still validate, and the public tutorial's specification must
//! validate, synthesize an IR suite with no refusals and pass it with its Rust implementation.
//! It runs on every pull request, every `main` push and daily; a red run is fixed by a skill edit.
//!
//! It also fails when a CLI's newest release is newer than `verified.json`, the release its skills
//! were last verified against: every product release is re-verified (this check and an ESS trial
//! round) before `verified.json` moves. The offline gate only checks that file's shape.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Plugin, the CLI its skills drive, and the repository that releases it.
const TOOLS: &[(&str, &str, &str)] = &[
    ("aep", "aep", "beyond10x/aep"),
    ("ess", "ess", "beyond10x/ess"),
    ("worktree", "worktree", "beyond10x/worktree"),
    ("aep", "metaharness", "beyond10x/metaharness"),
    ("connectors", "connectors", "beyond10x/connectors"),
];

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
    let expected: BTreeSet<&str> = TOOLS.iter().map(|(_, cli, _)| *cli).collect();
    let named: BTreeSet<&str> = map.keys().map(String::as_str).collect();
    let required = BTreeSet::from(["aep", "ess", "worktree"]);
    if !required.is_subset(&named) || !named.is_subset(&expected) {
        return Err(format!(
            "{VERIFIED}: names {named:?}; requires {required:?}, permits {expected:?}"
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
fn fetch(cli: &str, repository: &str, scratch: &Path) -> Result<(String, PathBuf), String> {
    let tag = latest(repository)?;
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
/// words up to the first flag, placeholder or path, at most [`DEPTH`].
#[must_use]
#[cfg(test)]
pub fn spelled(text: &str, cli: &str) -> BTreeSet<Vec<String>> {
    invocations(text, cli)
        .into_iter()
        .map(|(path, _)| path)
        .collect()
}

fn invocations(text: &str, cli: &str) -> BTreeSet<(Vec<String>, BTreeSet<String>)> {
    let text = text.replace("\\\n", " ");
    let mut code = Vec::new();
    let mut fenced = false;
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
        let words: Vec<&str> = snippet.split_whitespace().collect();
        for (index, word) in words.iter().enumerate() {
            let starts =
                index == 0 || matches!(words[index - 1], "&&" | "||" | "|" | ";" | "then" | "do");
            if *word != cli || !starts {
                continue;
            }
            let mut start = index + 1;
            while words.get(start).is_some_and(|word| {
                matches!(*word, "--config" | "--state-dir" | "--output" | "--store")
            }) {
                start += 2;
            }
            let path: Vec<String> = words
                .get(start..)
                .unwrap_or_default()
                .iter()
                .take_while(|word| {
                    word.bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                        && !word.starts_with('-')
                        && word.bytes().next().is_some_and(|b| b.is_ascii_lowercase())
                })
                .take(DEPTH)
                .map(|word| (*word).to_owned())
                .collect();
            if !path.is_empty() {
                let flags = words[index + 1..]
                    .iter()
                    .take_while(|word| !matches!(**word, "&&" | "||" | "|" | ";"))
                    .filter(|word| word.starts_with("--"))
                    .map(|word| {
                        word.split('=')
                            .next()
                            .unwrap_or(word)
                            .trim_end_matches(['`', ',', ';'])
                            .to_owned()
                    })
                    .filter(|word| {
                        word.len() > 2
                            && word[2..]
                                .bytes()
                                .all(|b| b.is_ascii_lowercase() || b == b'-')
                    })
                    .collect();
                commands.insert((path, flags));
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
    Ok(())
}

/// The public tutorial's committed specification and Rust implementation.
pub const TUTORIAL: &str = "website/docs/tutorials/first-ess-specification";

fn copy(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|error| format!("{}: {error}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|error| format!("{}: {error}", from.display()))? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path
            .file_name()
            .is_some_and(|name| matches!(name.to_str(), Some("target" | "node_modules" | ".git")))
        {
            continue;
        }
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

/// The tutorial, as a reader runs it: its specification validates and synthesizes an IR suite with
/// no refusals, and its Rust implementation passes that suite under `cargo test`.
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
            "ir",
            "--out",
            "impl/suite.json",
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
    let tested = run_in(
        &dir,
        "cargo",
        &[
            "test",
            "--locked",
            "--manifest-path",
            "impl/Cargo.toml",
            "--",
            "--nocapture",
        ],
    )
    .map_err(|error| fail("the implementation fails its suite", error))?;
    println!("tools `ess`: tutorial — cargo test:\n{}", tested.trim());
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
        eval_pair(root)?;
        for (_, cli, repository) in TOOLS {
            let tag = latest(repository)?;
            if *cli == "connectors" {
                verify_connectors(root, repository, &tag, &mut problems)?;
                if let Some(release) = verified.get(*cli) {
                    problems.extend(unverified(cli, &tag, release));
                } else {
                    problems.push(format!("{VERIFIED}: missing `{cli}` verification record"));
                }
                continue;
            }
            let (tag, binary) = fetch(cli, repository, &scratch)?;
            if !verified.contains_key(*cli) {
                problems.push(format!("{VERIFIED}: missing `{cli}` verification record"));
            }
            if let Some(line) = verified
                .get(*cli)
                .and_then(|release| unverified(cli, &tag, release))
            {
                problems.push(line);
            }
            let mut checked = 0;
            let files = command_files(root);
            for file in files {
                let text = std::fs::read_to_string(&file).map_err(|error| error.to_string())?;
                for (path, flags) in invocations(&text, cli) {
                    checked += 1;
                    let mut arguments: Vec<&str> = path.iter().map(String::as_str).collect();
                    arguments.push("--help");
                    let status = Command::new(&binary)
                        .args(&arguments)
                        .output()
                        .map_err(|error| error.to_string())?;
                    if status.status.success() {
                        let help = String::from_utf8_lossy(&status.stdout);
                        for flag in flags {
                            if !help
                                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                                .any(|word| word == flag)
                            {
                                problems.push(format!(
                                    "{}: `{cli} {}` has no {flag} option in {tag}",
                                    file.strip_prefix(root).unwrap_or(&file).display(),
                                    path.join(" ")
                                ));
                            }
                        }
                    } else {
                        problems.push(format!(
                            "{}: `{cli} {}` is not a command of {cli} {tag}",
                            file.strip_prefix(root).unwrap_or(&file).display(),
                            path.join(" ")
                        ));
                    }
                }
            }
            println!("tools `{cli}` {tag}: {checked} spelled command(s) checked");
            if *cli == "ess" {
                syntax(root, &binary, &scratch)?;
                tutorial(root, &binary, &scratch)?;
                examples(root, &binary, &scratch)?;
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

fn command_files(root: &Path) -> Vec<PathBuf> {
    let mut files = markdown(&root.join("plugins"));
    files.extend(markdown(&root.join("website/docs")));
    for name in ["README.md", "SETUP.md", "AGENTS.md"] {
        if root.join(name).exists() {
            files.push(root.join(name));
        }
    }
    files
}

fn eval_pair(root: &Path) -> Result<(), String> {
    let workflow = std::fs::read_to_string(root.join(".github/workflows/eval.yml"))
        .map_err(|error| error.to_string())?;
    let pin = |name: &str| {
        crate::upstream::pinned_in(&workflow, name)
            .ok_or_else(|| format!("eval.yml: missing {name}"))
    };
    let aep = pin("AEP_VERSION: '")?;
    let metaharness = pin("METAHARNESS_VERSION: '")?;
    let commit = crate::upstream::release_commit("beyond10x/aep", &aep)?;
    let manifest = run("curl", &["-fsSL", "--retry", "2", &format!("https://raw.githubusercontent.com/beyond10x/metaharness/{metaharness}/crates/metaharness-aep/Cargo.toml")])?;
    verify_eval_pair(&manifest, &commit)?;
    println!(
        "tools eval: AEP {aep} matches Metaharness {metaharness} embedded AEP commit {commit}"
    );
    Ok(())
}

fn verify_eval_pair(manifest: &str, commit: &str) -> Result<(), String> {
    let revisions: BTreeSet<_> = manifest
        .lines()
        .filter(|line| line.contains("https://github.com/beyond10x/aep\""))
        .filter_map(|line| crate::upstream::pinned_in(line, "rev = \""))
        .collect();
    if revisions != BTreeSet::from([commit.to_owned()]) {
        return Err(format!("eval.yml: AEP release commit {commit} differs from Metaharness embedded revisions {revisions:?}"));
    }
    Ok(())
}

fn connector_commands(text: &str) -> Result<BTreeMap<Vec<String>, BTreeSet<String>>, String> {
    let schema: serde_yaml::Value =
        serde_yaml::from_str(text).map_err(|error| error.to_string())?;
    let commands = schema["commands"]
        .as_sequence()
        .ok_or("Connectors contract has no commands")?;
    let mut globals = BTreeSet::from(["--help".to_owned()]);
    if let Some(mapping) = schema["globals"].as_mapping() {
        globals.extend(
            mapping
                .values()
                .filter_map(serde_yaml::Value::as_str)
                .map(|flag| format!("--{flag}")),
        );
    }
    let mut paths = BTreeMap::new();
    for command in commands {
        let path = command["path"]
            .as_sequence()
            .ok_or("Connectors command has no path")?
            .iter()
            .map(|word| {
                word.as_str()
                    .map(str::to_owned)
                    .ok_or("Connectors command word is not a string")
            })
            .collect::<Result<Vec<_>, _>>()?;
        for length in 1..path.len() {
            paths
                .entry(path[..length].to_vec())
                .or_insert_with(|| globals.clone());
        }
        let mut flags = globals.clone();
        if let Some(arguments) = command["arguments"].as_sequence() {
            for argument in arguments {
                for name in ["long", "inline", "file", "stdin"] {
                    if let Some(flag) = argument["source"][name].as_str() {
                        flags.insert(format!("--{flag}"));
                    }
                }
            }
        }
        paths.insert(path, flags);
    }
    if paths.is_empty() {
        return Err("Connectors contract has no command paths".to_owned());
    }
    Ok(paths)
}

fn verify_connectors(
    root: &Path,
    repository: &str,
    tag: &str,
    problems: &mut Vec<String>,
) -> Result<(), String> {
    let commit = crate::upstream::release_commit(repository, tag)?;
    let url = format!(
        "https://raw.githubusercontent.com/{repository}/{commit}/apps/connectors/spec/cli.yaml"
    );
    let text = run("curl", &["-fsSL", "--retry", "2", &url])?;
    let commands = connector_commands(&text)?;
    let route_url = format!(
        "https://raw.githubusercontent.com/{repository}/{commit}/apps/connectors/src/main.rs"
    );
    let routes = run("curl", &["-fsSL", "--retry", "2", &route_url])?;
    let service_help = connector_service_routes(&routes);
    let binary = std::env::var_os("AGENTPLUGINS_CONNECTORS_BINARY").map(PathBuf::from);
    if let Some(binary) = &binary {
        let version = run(&binary.to_string_lossy(), &["--version"])?;
        if version.trim() != format!("connectors {}", tag.trim_start_matches('v')) {
            return Err(format!(
                "{}: expected connectors {tag}, got {}",
                binary.display(),
                version.trim()
            ));
        }
        println!(
            "tools `connectors`: additional runtime help verification with {} (version {})",
            binary.display(),
            version.trim()
        );
    }
    let mut checked = 0;
    for file in command_files(root) {
        let text = std::fs::read_to_string(&file).map_err(|error| error.to_string())?;
        for (path, flags) in invocations(&text, "connectors") {
            checked += 1;
            if let Some(binary) = &binary {
                let mut arguments: Vec<&str> = path.iter().map(String::as_str).collect();
                arguments.push("--help");
                let help = run(&binary.to_string_lossy(), &arguments)?;
                for flag in &flags {
                    if !help
                        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                        .any(|word| word == flag)
                    {
                        problems.push(format!(
                            "{}: runtime `connectors {}` has no {flag} in {tag}",
                            file.strip_prefix(root).unwrap_or(&file).display(),
                            path.join(" ")
                        ));
                    }
                }
            }
            if let Some(allowed) = commands.get(&path) {
                for flag in flags.difference(allowed) {
                    problems.push(format!(
                        "{}: `connectors {}` has no {flag} in {tag} CLI contract",
                        file.strip_prefix(root).unwrap_or(&file).display(),
                        path.join(" ")
                    ));
                }
            } else if !(path.len() == 1
                && service_help.contains(&path[0])
                && flags == BTreeSet::from(["--help".to_owned()]))
            {
                problems.push(format!(
                    "{}: `connectors {}` is absent from {tag} CLI contract",
                    file.strip_prefix(root).unwrap_or(&file).display(),
                    path.join(" ")
                ));
            }
        }
    }
    let runtime = if binary.is_some() {
        "runtime help additionally verified"
    } else {
        "source contract only; runtime not executed (release has no binary assets)"
    };
    println!("tools `connectors` {tag}: {checked} spelled command(s) checked against released source contract {url} and service help routes {route_url}; {runtime}");
    Ok(())
}

fn connector_service_routes(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter(|line| line.contains("=> return true"))
        .filter_map(|line| {
            line.trim()
                .strip_prefix("Some(")?
                .split_once(')')
                .map(|(names, _)| names)
        })
        .flat_map(|names| names.split('|'))
        .filter_map(|name| {
            name.trim()
                .strip_prefix('"')?
                .strip_suffix('"')
                .map(str::to_owned)
        })
        .collect()
}

fn examples(root: &Path, ess: &Path, scratch: &Path) -> Result<(), String> {
    let dir = root.join("plugins/ess/skills/specifying/references/examples");
    conformance_examples(&dir, ess, scratch)?;
    advanced_examples(&dir, ess, scratch)
}

fn conformance_examples(dir: &Path, ess: &Path, scratch: &Path) -> Result<(), String> {
    for name in ["related-guard", "set-effects"] {
        let path = dir.join(format!("{name}.yaml"));
        let suite = scratch.join(format!("{name}-suite.json"));
        run(
            &ess.to_string_lossy(),
            &["specify", "validate", "--path", &path.to_string_lossy()],
        )?;
        let output = run(
            &ess.to_string_lossy(),
            &[
                "verify",
                "conform",
                "synthesize",
                "--path",
                &path.to_string_lossy(),
                "--out",
                &suite.to_string_lossy(),
            ],
        )?;
        if !output.contains(" 0 refusal(s)") {
            return Err(format!(
                "{name} example synthesized with refusals: {output}"
            ));
        }
        println!("tools `ess`: {name} example — {}", output.trim());
    }
    Ok(())
}

fn advanced_examples(dir: &Path, ess: &Path, scratch: &Path) -> Result<(), String> {
    let related = dir
        .join("related-guard.yaml")
        .to_string_lossy()
        .into_owned();
    let transport = dir.join("transport.yaml").to_string_lossy().into_owned();
    let protocol = dir
        .join("terminal-response.yaml")
        .to_string_lossy()
        .into_owned();
    let actions = dir
        .join("terminal-response.actions.json")
        .to_string_lossy()
        .into_owned();
    let trace = scratch
        .join("protocol-trace.json")
        .to_string_lossy()
        .into_owned();
    let client = scratch.join("client").to_string_lossy().into_owned();
    let commands: Vec<Vec<&str>> = vec![
        vec![
            "specify",
            "transport",
            "validate",
            "--path",
            &transport,
            "--spec",
            &related,
        ],
        vec![
            "generate",
            "client",
            "--path",
            &related,
            "--component",
            "library-service",
            "--transport",
            &transport,
            "--target",
            "rust",
            "--package",
            "library-events",
            "--out",
            &client,
        ],
        vec!["specify", "protocol", "validate", "--path", &protocol],
        vec![
            "verify",
            "protocol",
            "run",
            "--path",
            &protocol,
            "--actions",
            &actions,
            "--out",
            &trace,
        ],
        vec![
            "verify", "protocol", "replay", "--path", &protocol, "--trace", &trace,
        ],
        vec!["verify", "protocol", "explore", "--path", &protocol],
        vec![
            "verify",
            "diff",
            "--from",
            &related,
            "--to",
            &related,
            "--compatibility",
            "--fail-on",
            "breaking-or-unknown",
            "--format",
            "json",
        ],
    ];
    for command in commands {
        let output = run(&ess.to_string_lossy(), &command)?;
        println!(
            "tools `ess`: example {} — {}",
            command[..3].join(" "),
            output.trim()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_commands_include_global_options_and_continuations() {
        let found = invocations("```bash\nconnectors --config config.toml --state-dir state setup check \\\n --output json\n```", "connectors");
        assert_eq!(
            found,
            BTreeSet::from([(
                vec!["setup".to_owned(), "check".to_owned()],
                BTreeSet::from([
                    "--config".to_owned(),
                    "--state-dir".to_owned(),
                    "--output".to_owned()
                ])
            )])
        );
    }

    #[test]
    fn eval_pair_requires_every_embedded_aep_revision_to_match() {
        let manifest = "aep-cli = { git = \"https://github.com/beyond10x/aep\", rev = \"aaa\" }\naep-engine = { git = \"https://github.com/beyond10x/aep\", rev = \"bbb\" }";
        assert!(verify_eval_pair(manifest, "aaa").is_err());
        assert!(verify_eval_pair(&manifest.replace("bbb", "aaa"), "aaa").is_ok());
        assert!(verify_eval_pair("", "aaa").is_err());
    }

    #[test]
    fn explicit_connector_service_help_routes_follow_released_source() {
        assert_eq!(
            connector_service_routes(
                "Some(\"describe\" | \"invoke\" | \"serve\") => return true,\n_ => return false,"
            ),
            BTreeSet::from(["describe", "invoke", "serve"].map(str::to_owned))
        );
        assert_eq!(
            connector_service_routes("Some(\"retired\") => return false,"),
            BTreeSet::new()
        );
    }

    #[test]
    fn connector_release_contract_admits_groups_and_rejects_removed_commands() {
        let paths = connector_commands(
            "globals: {config: config, state: state-dir}\ncommands:\n  - path: [setup, check]\n  - path: [operations, describe]\n    arguments:\n      - source: {kind: option, long: operation}\n      - source: {kind: document, inline: input-json, file: input-file, stdin: input-stdin}\n",
        )
        .unwrap();
        assert!(paths.contains_key(&vec!["setup".to_owned()]));
        assert!(paths.contains_key(&vec!["operations".to_owned(), "describe".to_owned()]));
        assert!(!paths.contains_key(&vec!["inspect".to_owned(), "doctor".to_owned()]));
        let flags = &paths[&vec!["operations".to_owned(), "describe".to_owned()]];
        assert_eq!(
            flags,
            &BTreeSet::from(
                [
                    "--help",
                    "--config",
                    "--state-dir",
                    "--operation",
                    "--input-json",
                    "--input-file",
                    "--input-stdin"
                ]
                .map(str::to_owned)
            )
        );
        assert!(!paths[&vec!["setup".to_owned(), "check".to_owned()]].contains("--operation"));
        assert!(connector_commands("commands: []").is_err());
    }

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
        assert!(map.len() >= 3 && map.len() <= TOOLS.len());
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
}
