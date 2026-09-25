//! `agentplugins-check tools` (network): the skills describe the CLIs as they are released now.
//!
//! Every plugin lives in this repository and names no CLI version (website/docs/structure.md R1,
//! R5), so the only thing that can drift is a product release renaming or removing a command the
//! skills still spell. This check downloads each product's newest release — the prebuilt archive,
//! checked against its `SHA256SUMS` — and runs `<cli> <subcommands> --help` for every command a
//! code span or code block in that plugin spells. ESS's syntax example must also still validate.
//! It runs on every pull request, every `main` push and daily; a red run is fixed by a skill edit.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Plugin, the CLI its skills drive, and the repository that releases it.
const TOOLS: &[(&str, &str, &str)] = &[
    ("aep", "aep", "beyond10x/aep"),
    ("ess", "ess", "beyond10x/ess"),
    ("worktree", "worktree", "beyond10x/worktree"),
];

/// Most subcommand words taken from one spelled command.
const DEPTH: usize = 4;

fn run(program: &str, arguments: &[&str]) -> Result<String, String> {
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

fn latest(repository: &str) -> Result<String, String> {
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
pub fn spelled(text: &str, cli: &str) -> BTreeSet<Vec<String>> {
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
            let path: Vec<String> = words[index + 1..]
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
    Ok(())
}

/// Check every plugin's spelled commands against its CLI's newest release.
pub fn verify(root: &Path) -> Result<(), String> {
    let scratch =
        std::env::temp_dir().join(format!("agentplugins-check-tools-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let result = (|| {
        let mut problems = Vec::new();
        for (plugin, cli, repository) in TOOLS {
            let (tag, binary) = fetch(cli, repository, &scratch)?;
            let mut checked = 0;
            for file in markdown(&root.join("plugins").join(plugin)) {
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
            if *cli == "ess" {
                syntax(root, &binary, &scratch)?;
            }
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{} spelled command(s) the newest releases do not have:\n  {}",
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
