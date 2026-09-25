//! Trial definitions: `trials/<name>/trial.yaml`, one per trial, with an optional fixture beside
//! it, and `trials/baseline.json`, the measures of the last accepted run of each.
//!
//! A trial prompt that lives in a throwaway script cannot be re-run next round or compared with the
//! last one. Here each is data the gate validates and `task trial:run TRIAL=<name>` executes:
//! [`prepare`] copies the fixture into a fresh sandbox, commits it, and writes the prompt and the
//! setup the Taskfile runs.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

/// The directory holding every trial, relative to the repository root.
pub const TRIALS: &str = "trials";

/// The committed baseline, relative to the repository root.
pub const BASELINE: &str = "trials/baseline.json";

/// What a trial exercises. A label for readers and for the round's coverage; it does not change
/// how a run is measured, which [`Definition::measures`] decides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// A new ESS specification from a sentence.
    EssNew,
    /// An ESS specification of an existing service (the fixture).
    EssRetrofit,
    /// Generation and a synthesized suite from an existing specification.
    EssPipeline,
    /// Every ESS output, and an implementation held to the synthesized Go suite.
    EssFullPackage,
    /// AEP planning from an existing backlog.
    AepBacklog,
    /// Setting up `worktree` in a repository.
    WorktreeOnboarding,
    /// Bringing a seeded older install current.
    Upgrade,
}

/// A number `agentplugins-check trial-report` reads from a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Measure {
    /// Tool calls the run made.
    ToolCalls,
    /// Whether `ess specify validate` ran and its last output said `valid`.
    Validate,
    /// Scenarios and refusals from the last `ess verify conform synthesize`.
    Synthesis,
    /// `UNMAPPED:` markers in the spec files the run wrote, read from disk.
    Unmapped,
    /// Which of [`Definition::outputs`] exist on disk.
    Outputs,
    /// Passed, failed and skipped tests of the last `go test`.
    GoTest,
}

/// One `trials/<name>/trial.yaml`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    /// The trial's name; equal to its directory name and the sandbox name.
    pub name: String,
    /// What it exercises.
    pub kind: Kind,
    /// What the user types.
    pub prompt: String,
    /// The subdirectory of the sandbox's `work/` the run starts in and the fixture is copied to.
    #[serde(default)]
    pub dir: Option<String>,
    /// A directory beside `trial.yaml` copied into the run's directory and committed there.
    #[serde(default)]
    pub fixture: Option<String>,
    /// Give the fixture a bare `origin` inside the sandbox, with `main` pushed.
    #[serde(default)]
    pub remote: bool,
    /// The sandbox is seeded with older plugins on purpose (an upgrade trial).
    #[serde(default)]
    pub seeded: bool,
    /// Shell lines run from the sandbox root with the sandbox's `env`, before the run: installing
    /// the plugins under test, or seeding an older install.
    #[serde(default)]
    pub setup: Vec<String>,
    /// The numbers the run must report.
    pub measures: Vec<Measure>,
    /// Output name → path relative to the run's directory, for [`Measure::Outputs`].
    #[serde(default)]
    pub outputs: BTreeMap<String, String>,
}

impl Definition {
    /// The directory the run starts in, relative to the sandbox.
    #[must_use]
    pub fn workdir(&self) -> PathBuf {
        let work = PathBuf::from("work");
        match self.dir.as_deref() {
            Some(dir) if !dir.is_empty() => work.join(dir),
            _ => work,
        }
    }

    /// Whether the run reports `measure`.
    #[must_use]
    pub fn measures(&self, measure: Measure) -> bool {
        self.measures.contains(&measure)
    }
}

/// A relative path that stays below the directory it is joined to.
fn contained(path: &str) -> bool {
    let path = Path::new(path);
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}

fn read(root: &Path, name: &str) -> Result<Definition, String> {
    let path = root.join(TRIALS).join(name).join("trial.yaml");
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_yaml::from_str(&text).map_err(|error| format!("parsing {}: {error}", path.display()))
}

/// Every trial name under `trials/`, sorted.
pub fn names(root: &Path) -> Result<Vec<String>, String> {
    let directory = root.join(TRIALS);
    let mut names = Vec::new();
    for entry in std::fs::read_dir(&directory)
        .map_err(|error| format!("reading {}: {error}", directory.display()))?
    {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                names.push(name.to_owned());
            }
        }
    }
    names.sort();
    Ok(names)
}

/// One trial's definition, checked.
pub fn load(root: &Path, name: &str) -> Result<Definition, String> {
    let known = names(root)?;
    if !known.iter().any(|known| known == name) {
        return Err(format!(
            "no trial `{name}` under {TRIALS}/; the trials are: {}",
            known.join(", ")
        ));
    }
    let definition = read(root, name)?;
    let problems = problems(root, &definition);
    if problems.is_empty() {
        Ok(definition)
    } else {
        Err(format!(
            "{TRIALS}/{name}/trial.yaml:\n  {}",
            problems.join("\n  ")
        ))
    }
}

fn problems(root: &Path, definition: &Definition) -> Vec<String> {
    let mut problems = Vec::new();
    let directory = root.join(TRIALS).join(&definition.name);
    if !directory.is_dir() {
        problems.push(format!(
            "`name: {}` is not the directory the definition sits in",
            definition.name
        ));
    }
    if definition.prompt.trim().is_empty() {
        problems.push("the prompt is empty".to_owned());
    }
    if let Some(dir) = definition.dir.as_deref() {
        if !contained(dir) {
            problems.push(format!("`dir: {dir}` is not a relative path below `work/`"));
        }
    }
    if let Some(fixture) = definition.fixture.as_deref() {
        if !contained(fixture) || !directory.join(fixture).is_dir() {
            problems.push(format!(
                "`fixture: {fixture}` is not a directory beside trial.yaml"
            ));
        } else if directory.join(fixture).join(".git").exists() {
            problems.push(format!(
                "`fixture: {fixture}` carries a `.git`; the sandbox commits it"
            ));
        }
    }
    if definition.remote && definition.fixture.is_none() {
        problems.push("`remote: true` needs a fixture to push".to_owned());
    }
    if definition.seeded && definition.setup.is_empty() {
        problems.push("`seeded: true` needs the `setup` that seeds the install".to_owned());
    }
    if !definition.measures(Measure::ToolCalls) {
        problems.push("every trial measures `tool_calls`".to_owned());
    }
    if definition.measures(Measure::Outputs) == definition.outputs.is_empty() {
        problems.push("`outputs` are listed exactly when `measures` has `outputs`".to_owned());
    }
    for (output, path) in &definition.outputs {
        if !contained(path) {
            problems.push(format!(
                "output `{output}: {path}` is not a relative path below the run's directory"
            ));
        }
    }
    problems
}

/// Every definition parses and holds together, and the baseline names only trials that exist.
pub fn check(root: &Path) -> Result<(), String> {
    let mut found = Vec::new();
    let names = names(root)?;
    for name in &names {
        match read(root, name) {
            Ok(definition) => {
                for problem in problems(root, &definition) {
                    found.push(format!("{TRIALS}/{name}/trial.yaml: {problem}"));
                }
            }
            Err(error) => found.push(error),
        }
    }
    let baseline = root.join(BASELINE);
    let text = std::fs::read_to_string(&baseline)
        .map_err(|error| format!("reading {BASELINE}: {error}"))?;
    match serde_json::from_str::<BTreeMap<String, serde_json::Value>>(&text) {
        Ok(entries) => {
            for trial in entries.keys() {
                if !names.contains(trial) {
                    found.push(format!("{BASELINE} records `{trial}`, which is no trial"));
                }
            }
        }
        Err(error) => found.push(format!("parsing {BASELINE}: {error}")),
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} trial definition problem(s):\n  {}",
            found.len(),
            found.join("\n  ")
        ))
    }
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    std::fs::create_dir_all(to).map_err(|error| format!("creating {}: {error}", to.display()))?;
    for entry in
        std::fs::read_dir(from).map_err(|error| format!("reading {}: {error}", from.display()))?
    {
        let path = entry.map_err(|error| error.to_string())?.path();
        let target = to.join(path.file_name().unwrap_or_default());
        if path.is_dir() {
            copy_tree(&path, &target)?;
        } else {
            std::fs::copy(&path, &target)
                .map_err(|error| format!("copying {}: {error}", path.display()))?;
        }
    }
    Ok(())
}

/// Run `git` in `directory` as the trial identity, reading no configuration of the operator's.
fn git(sandbox: &Path, directory: &Path, args: &[&str]) -> Result<(), String> {
    let output = Command::new("git")
        .args([
            "-c",
            "commit.gpgsign=false",
            "-c",
            "init.defaultBranch=main",
        ])
        .args(args)
        .current_dir(directory)
        .env("HOME", sandbox.join("home"))
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "trial")
        .env("GIT_AUTHOR_EMAIL", "trial@example.invalid")
        .env("GIT_COMMITTER_NAME", "trial")
        .env("GIT_COMMITTER_EMAIL", "trial@example.invalid")
        .output()
        .map_err(|error| format!("running git: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git {} in {}: {}",
            args.join(" "),
            directory.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Prepare a fresh sandbox (`task trial:sandbox`) for `name`: the fixture copied into the run's
/// directory and committed, and beside `env` the files `task trial:run` reads — `prompt.txt`,
/// `workdir`, `setup.sh` and, for an upgrade trial, `seeded`.
pub fn prepare(root: &Path, name: &str, sandbox: &Path) -> Result<String, String> {
    let definition = load(root, name)?;
    if !sandbox.join("env").is_file() {
        return Err(format!(
            "{} is not a trial sandbox (no `env`); create it with `task trial:sandbox`",
            sandbox.display()
        ));
    }
    let work = sandbox.join(definition.workdir());
    std::fs::create_dir_all(&work)
        .map_err(|error| format!("creating {}: {error}", work.display()))?;
    let mut done = Vec::new();
    if let Some(fixture) = definition.fixture.as_deref() {
        copy_tree(&root.join(TRIALS).join(name).join(fixture), &work)?;
        git(sandbox, &work, &["init", "--quiet"])?;
        git(sandbox, &work, &["add", "--all"])?;
        git(
            sandbox,
            &work,
            &["commit", "--quiet", "-m", "Initial import"],
        )?;
        done.push(format!("fixture committed in {}", work.display()));
        if definition.remote {
            let remote = sandbox.join("remote.git");
            git(
                sandbox,
                sandbox,
                &["init", "--quiet", "--bare", &remote.to_string_lossy()],
            )?;
            git(
                sandbox,
                &work,
                &["remote", "add", "origin", &remote.to_string_lossy()],
            )?;
            git(sandbox, &work, &["push", "--quiet", "-u", "origin", "main"])?;
            done.push(format!("origin {}", remote.display()));
        }
    }
    let write = |file: &str, text: &str| {
        std::fs::write(sandbox.join(file), text).map_err(|error| format!("writing {file}: {error}"))
    };
    write("prompt.txt", definition.prompt.trim_end())?;
    write("workdir", &definition.dir.clone().unwrap_or_default())?;
    if definition.setup.is_empty() {
        let _ = std::fs::remove_file(sandbox.join("setup.sh"));
    } else {
        write(
            "setup.sh",
            &format!("set -e\n{}\n", definition.setup.join("\n")),
        )?;
        done.push(format!("{} setup line(s)", definition.setup.len()));
    }
    if definition.seeded {
        write("seeded", "")?;
    } else {
        let _ = std::fs::remove_file(sandbox.join("seeded"));
    }
    Ok(format!(
        "prepared trial `{name}` ({:?}) in {}{}",
        definition.kind,
        sandbox.display(),
        if done.is_empty() {
            String::new()
        } else {
            format!(": {}", done.join("; "))
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "agentplugins-check-trials-{label}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("the scratch directory is writable");
        path
    }

    fn repository() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under the repository root")
            .to_path_buf()
    }

    #[test]
    fn the_committed_trials_hold_together() {
        check(&repository()).expect("every committed trial validates");
        let names = names(&repository()).expect("trials/ is readable");
        for expected in [
            "ess-new",
            "ess-retrofit",
            "ess-pipeline",
            "ess-full-package",
            "aep-backlog",
            "worktree-onboarding",
            "upgrade-seeded",
        ] {
            assert!(names.iter().any(|name| name == expected), "{expected}");
        }
    }

    #[test]
    fn a_definition_that_does_not_hold_together_is_refused() {
        let root = scratch("bad");
        let trial = root.join(TRIALS).join("broken");
        std::fs::create_dir_all(&trial).unwrap();
        std::fs::write(
            trial.join("trial.yaml"),
            "name: other\nkind: ess-new\nprompt: ' '\ndir: ../escape\nseeded: true\nmeasures: [outputs]\n",
        )
        .unwrap();
        std::fs::write(root.join(BASELINE), "{\"gone\": {}}").unwrap();
        let error = check(&root).unwrap_err();
        std::fs::remove_dir_all(&root).unwrap();
        for expected in [
            "not the directory",
            "prompt is empty",
            "not a relative path below `work/`",
            "needs the `setup`",
            "every trial measures `tool_calls`",
            "`outputs` are listed exactly",
            "records `gone`, which is no trial",
        ] {
            assert!(error.contains(expected), "{expected}: {error}");
        }
    }

    #[test]
    fn prepare_commits_the_fixture_and_writes_the_run_files() {
        let root = scratch("prepare");
        let trial = root.join(TRIALS).join("demo");
        std::fs::create_dir_all(trial.join("fixture/src")).unwrap();
        std::fs::write(trial.join("fixture/src/main.go"), "package main\n").unwrap();
        std::fs::write(
            trial.join("trial.yaml"),
            "name: demo\nkind: ess-retrofit\nprompt: |\n  Describe this service.\ndir: svc\nfixture: fixture\nremote: true\nsetup: [echo one]\nmeasures: [tool_calls, unmapped]\n",
        )
        .unwrap();
        std::fs::write(root.join(BASELINE), "{}").unwrap();
        let sandbox = root.join("sandbox");
        std::fs::create_dir_all(sandbox.join("home")).unwrap();
        std::fs::write(sandbox.join("env"), "").unwrap();

        let summary = prepare(&root, "demo", &sandbox).unwrap();
        assert!(summary.contains("fixture committed"), "{summary}");
        let work = sandbox.join("work/svc");
        assert!(work.join("src/main.go").is_file());
        let log = Command::new("git")
            .args(["log", "--format=%an %s", "origin/main"])
            .current_dir(&work)
            .output()
            .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&log.stdout),
            "trial Initial import\n"
        );
        assert_eq!(
            std::fs::read_to_string(sandbox.join("prompt.txt")).unwrap(),
            "Describe this service."
        );
        assert_eq!(
            std::fs::read_to_string(sandbox.join("workdir")).unwrap(),
            "svc"
        );
        assert_eq!(
            std::fs::read_to_string(sandbox.join("setup.sh")).unwrap(),
            "set -e\necho one\n"
        );
        assert!(!sandbox.join("seeded").exists());
        let unknown = prepare(&root, "missing", &sandbox).unwrap_err();
        std::fs::remove_dir_all(&root).unwrap();
        assert!(unknown.contains("the trials are: demo"), "{unknown}");
    }
}
