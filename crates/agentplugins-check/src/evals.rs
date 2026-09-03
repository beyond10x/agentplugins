//! Validates the eval corpus under `evals/`, and replays whatever transcripts are recorded there.
//!
//! Additive to the marketplace check beside it and reached from one line of [`crate::check`], so
//! that a plugin edit and a corpus edit do not have to be made in the same place.
//!
//! # What this is allowed to say, and what it is not
//!
//! `eval-case/1` and `trace-spec/1` are **`aep`'s formats**, and their authoritative readers live in
//! that repository. So this module deliberately checks two different classes of thing and keeps them
//! apart:
//!
//! * **What this repository owns** — that a case's `id` is its directory name, that its
//!   `expectations:` file is there, that its `subject:` names an agent or a skill that exists *here*,
//!   and that a `recorded/` directory exists to put a transcript in. None of that is `aep`'s to know.
//! * **The shape of somebody else's document** — that the format claim is right, that every
//!   expectation carries an id and exactly one kind, that no two ids collide. Shape only: the **kind
//!   vocabulary is not restated here**, because a copy of it is a second list to update and the first
//!   symptom of it going stale is a corpus that this gate calls valid and the runner refuses. The
//!   vocabulary is checked where it is owned, by `aep` — in the replay below when a transcript
//!   exists, and in `.github/workflows/eval.yml` on every live run.
//!
//! # Absent tooling is a skip, never a red gate
//!
//! `task check` is the offline gate (`AGENTS.md` § *Gate*), and the replay needs a binary this
//! repository does not ship. A missing `aep`, and an empty `recorded/`, are both **printed notices**
//! and exit 0. The alternative is a gate that goes red on a developer's machine for a reason that is
//! not about this repository — which is how a gate stops being run.
//!
//! It is the position `aep eval run` itself takes when `metaharness` is absent, and the position is
//! the same one for the same reason: an unrecorded case is a case nobody has run, which is a true
//! thing to report and not a broken check.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

/// The format claim a case carries.
const CASE_FORMAT: &str = "eval-case/1";

/// The format claim an expectations document carries.
const SPEC_FORMAT: &str = "trace-spec/1";

/// The arms a case may declare, which are `aep eval run --arm`'s own four words.
const ARMS: [&str; 4] = ["raw", "plugin", "driven", "native"];

/// The severities `trace-spec/1` declares.
const SEVERITIES: [&str; 2] = ["gate", "advisory"];

/// What an expectation says an undecidable verdict means.
const ON_UNKNOWN: [&str; 2] = ["unknown", "gap"];

/// How a recorded event stream is named, so that a stream and its provenance find each other.
const STREAM_SUFFIX: &str = ".events.jsonl";

/// How the run manifest beside a recorded stream is named.
const MANIFEST_SUFFIX: &str = ".manifest.yaml";

/// What this case is about: the agents, skills and paths a change to which should re-run it.
///
/// Not read by `aep eval run`, which needs five fields and ignores the rest. It is read here, so a
/// case cannot outlive the agent it judges, and by `.github/workflows/eval.yml`, which scopes a paid
/// run to the cases whose subject the pull request touched.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Subject {
    /// Agents, qualified the way a harness lists them: `<plugin>:<name>`.
    #[serde(default)]
    agents: Vec<String>,
    /// Skills, qualified the same way.
    #[serde(default)]
    skills: Vec<String>,
    /// Anything else the case is about, as a repository-relative path.
    #[serde(default)]
    paths: Vec<String>,
}

/// A case as it is written.
///
/// `deny_unknown_fields`, unlike `aep`'s own reader: that one is deliberately permissive because the
/// corpus it reads has a denier elsewhere. This corpus's denier is here, so a misspelt key is refused
/// by name rather than read as a default.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    /// The shape the document says it is.
    format: String,
    /// The id, which must be the directory's name.
    id: String,
    /// A human sentence.
    title: String,
    /// The workflow it is a run of.
    workflow: String,
    /// The workflow states it exercises.
    states: Vec<String>,
    /// Which arm it is written for.
    arm: String,
    /// What must have held.
    verdict: String,
    /// What the case is about.
    subject: Subject,
    /// What the agent is asked to do.
    task: String,
    /// The `trace-spec/1` document it is judged by, relative to the case directory.
    expectations: String,
    /// Where a recorded transcript goes, relative to the case directory.
    recorded: String,
    /// Expectations that are expected to gap, each with the observation that explains it.
    #[serde(default)]
    advisory_gaps: Vec<serde_yaml::Value>,
    /// Expectations a `violated` case must gap on, each with the reason.
    #[serde(default)]
    violated: Vec<serde_yaml::Value>,
}

/// One expectation as written, in the fields this gate reads.
#[derive(Debug, Deserialize)]
struct RawExpectation {
    /// The id a verdict is reported under.
    id: String,
    /// The author's own sentence.
    #[serde(default)]
    statement: Option<String>,
    /// Whether a gap moves the exit code.
    #[serde(default)]
    severity: Option<String>,
    /// What an undecidable verdict means.
    #[serde(default)]
    on_unknown: Option<String>,
    /// What it claims: exactly one kind, whose parameters are `aep`'s to check.
    expect: serde_yaml::Mapping,
}

/// An expectations document as written.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Spec {
    /// The shape the document says it is.
    format: String,
    /// What the specification is about.
    id: String,
    /// A human sentence for a report's heading.
    #[serde(default)]
    #[allow(dead_code)]
    title: Option<String>,
    /// The expectations it declares.
    expectations: Vec<RawExpectation>,
}

/// Reads and deserializes a YAML document, naming the file in every failure.
///
/// Through `serde_yaml` 0.9 because that is the reader `aep` parses both of these formats with
/// (`aep/Cargo.toml`). A document that this gate accepts and that reader refuses would be the worst
/// outcome available here, and one parser across both repositories is how that is avoided.
fn yaml<T: serde::de::DeserializeOwned>(path: &Path, label: &str) -> Result<T, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_yaml::from_str(&text)
        .map_err(|error| format!("{} is not {label}: {error}", path.display()))
}

/// The name a `SKILL.md` or an agent file declares in its own frontmatter.
///
/// Read rather than inferred from the directory: `plugins/ess-schema/skills/ess-schema/`
/// declares `name: ess-schema`, so a resolver that trusted the directory would look for a skill that
/// does not exist under a name nothing uses.
fn declared_name(path: &Path) -> Result<String, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    text.lines()
        .take_while(|line| !line.starts_with("---") || line.trim() == "---")
        .find_map(|line| {
            line.strip_prefix("name:")
                .map(|rest| rest.trim().to_owned())
        })
        .filter(|name| !name.is_empty())
        .ok_or_else(|| format!("{} declares no `name:` in its frontmatter", path.display()))
}

/// Splits `<plugin>:<name>` as a harness qualifies an agent or a skill.
fn qualified<'a>(reference: &'a str, kind: &str, case: &str) -> Result<(&'a str, &'a str), String> {
    reference
        .split_once(':')
        .filter(|(plugin, name)| !plugin.is_empty() && !name.is_empty())
        .ok_or_else(|| {
            format!(
                "case `{case}` names the {kind} `{reference}`, which is not written \
                 `<plugin>:<name>` as a harness qualifies one"
            )
        })
}

/// Resolves an agent reference to the file that declares it.
fn resolve_agent(root: &Path, reference: &str, case: &str) -> Result<(), String> {
    let (plugin, name) = qualified(reference, "agent", case)?;
    let path = root
        .join("plugins")
        .join(plugin)
        .join("agents")
        .join(format!("{name}.md"));
    if !path.is_file() {
        return Err(format!(
            "case `{case}` names the agent `{reference}`, and there is no {}",
            path.display()
        ));
    }
    let declared = declared_name(&path)?;
    if declared != name {
        return Err(format!(
            "case `{case}` names the agent `{reference}`, but {} declares `name: {declared}`",
            path.display()
        ));
    }
    Ok(())
}

/// Resolves a skill reference to the `SKILL.md` that declares it.
///
/// By the declared name across every skill directory of the plugin, and not by directory: the two
/// agree for four of this repository's five skills and not for the fifth, and a harness lists what
/// the document declares.
fn resolve_skill(root: &Path, reference: &str, case: &str) -> Result<(), String> {
    let (plugin, name) = qualified(reference, "skill", case)?;
    let skills = root.join("plugins").join(plugin).join("skills");
    let mut entries = std::fs::read_dir(&skills)
        .map_err(|error| {
            format!(
                "case `{case}` names the skill `{reference}`; reading {}: {error}",
                skills.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("reading {}: {error}", skills.display()))?
        .into_iter()
        .map(|entry| entry.path().join("SKILL.md"))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    entries.sort();
    for path in &entries {
        if declared_name(path)? == name {
            return Ok(());
        }
    }
    Err(format!(
        "case `{case}` names the skill `{reference}`, and no SKILL.md under {} declares \
         `name: {name}`",
        skills.display()
    ))
}

/// Checks one case directory, and returns the streams it has recorded.
/// Every surface a case says it is about resolves to something this repository ships.
///
/// A case that names an agent nobody ships goes on passing its own document while judging nothing,
/// which is the one failure a corpus cannot report about itself.
fn subject(root: &Path, document: &Case, name: &str) -> Result<(), String> {
    if document.subject.agents.is_empty() && document.subject.skills.is_empty() {
        return Err(format!(
            "case `{name}` names no agent and no skill, so nothing ties it to a surface this \
             repository ships"
        ));
    }
    for reference in &document.subject.agents {
        resolve_agent(root, reference, name)?;
    }
    for reference in &document.subject.skills {
        resolve_skill(root, reference, name)?;
    }
    for relative in &document.subject.paths {
        if !root.join(relative).exists() {
            return Err(format!(
                "case `{name}` names the path `{relative}`, which is not in this repository"
            ));
        }
    }
    Ok(())
}

/// The expectations document, by shape.
///
/// Its **kinds are `aep`'s to check** — see the module header for why the vocabulary is not restated
/// here. What is checked is that the document is one, that every expectation carries an id a verdict
/// can be reported under, that no two ids collide, and that each claims exactly one thing.
fn spec(expectations: &Path) -> Result<(), String> {
    let spec: Spec = yaml(expectations, &format!("a `{SPEC_FORMAT}` document"))?;
    if spec.format != SPEC_FORMAT {
        return Err(format!(
            "{} claims `{}` and not `{SPEC_FORMAT}`",
            expectations.display(),
            spec.format
        ));
    }
    if spec.id.trim().is_empty() {
        return Err(format!("{} states no id", expectations.display()));
    }
    if spec.expectations.is_empty() {
        return Err(format!(
            "{} declares no expectation, so the case gates nothing",
            expectations.display()
        ));
    }
    let mut seen = BTreeSet::new();
    for expectation in &spec.expectations {
        if expectation.id.trim().is_empty() {
            return Err(format!(
                "{} carries an expectation with no id, and a verdict is reported under one",
                expectations.display()
            ));
        }
        if !seen.insert(expectation.id.clone()) {
            return Err(format!(
                "{} declares `{}` twice, so one of the two verdicts is unreachable",
                expectations.display(),
                expectation.id
            ));
        }
        if expectation
            .statement
            .as_ref()
            .is_some_and(|line| line.trim().is_empty())
        {
            return Err(format!(
                "{} gives `{}` an empty statement",
                expectations.display(),
                expectation.id
            ));
        }
        if expectation.expect.len() != 1 {
            return Err(format!(
                "{} gives `{}` {} kinds under `expect:`; an expectation claims exactly one thing",
                expectations.display(),
                expectation.id,
                expectation.expect.len()
            ));
        }
        if let Some(severity) = &expectation.severity {
            if !SEVERITIES.contains(&severity.as_str()) {
                return Err(format!(
                    "{} gives `{}` the severity `{severity}`, which is not {}",
                    expectations.display(),
                    expectation.id,
                    SEVERITIES.join(" or ")
                ));
            }
        }
        if let Some(policy) = &expectation.on_unknown {
            if !ON_UNKNOWN.contains(&policy.as_str()) {
                return Err(format!(
                    "{} gives `{}` `on_unknown: {policy}`, which is not {}",
                    expectations.display(),
                    expectation.id,
                    ON_UNKNOWN.join(" or ")
                ));
            }
        }
    }
    Ok(())
}

fn case(root: &Path, directory: &Path) -> Result<Vec<PathBuf>, String> {
    let name = directory
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| format!("{} is not a readable directory name", directory.display()))?;
    let manifest = directory.join("case.yaml");
    if !manifest.is_file() {
        return Err(format!(
            "{} holds no case.yaml; `aep eval run --corpus evals` refuses a directory that is not \
             a case, so this one cannot sit here",
            directory.display()
        ));
    }
    let document: Case = yaml(&manifest, &format!("an `{CASE_FORMAT}` document"))?;

    if document.format != CASE_FORMAT {
        return Err(format!(
            "{} claims `{}` and not `{CASE_FORMAT}`",
            manifest.display(),
            document.format
        ));
    }
    if document.id != name {
        return Err(format!(
            "{} carries the id `{}`, which is not its directory's name `{name}`",
            manifest.display(),
            document.id
        ));
    }
    for (field, value) in [
        ("title", &document.title),
        ("workflow", &document.workflow),
        ("verdict", &document.verdict),
        ("task", &document.task),
    ] {
        if value.trim().is_empty() {
            return Err(format!("case `{name}` states no `{field}`"));
        }
    }
    if document.states.is_empty() {
        return Err(format!(
            "case `{name}` names no state, so nothing says which part of `{}` it measures",
            document.workflow
        ));
    }
    if !ARMS.contains(&document.arm.as_str()) {
        return Err(format!(
            "case `{name}` declares the arm `{}`, which is not one of {}",
            document.arm,
            ARMS.join(", ")
        ));
    }
    if document.verdict == "held" && !document.violated.is_empty() {
        return Err(format!(
            "case `{name}` declares `verdict: held` and lists expectations under `violated:`"
        ));
    }
    if document.verdict == "violated" && document.violated.is_empty() {
        return Err(format!(
            "case `{name}` declares `verdict: violated` and names nothing under `violated:`; a \
             violation case whose gapping set is unstated cannot be told from a broken one"
        ));
    }
    let _ = &document.advisory_gaps;

    subject(root, &document, name)?;
    spec(&directory.join(&document.expectations))?;

    // Where a transcript goes, and whatever is already there.
    let recorded = directory.join(&document.recorded);
    if !recorded.is_dir() {
        return Err(format!(
            "case `{name}` names `{}` and there is no such directory; a case with nowhere to put a \
             transcript is a case nobody can record",
            document.recorded
        ));
    }
    streams(&recorded)
}

/// The recorded streams in a case's `recorded/` directory, each with the manifest that dates it.
///
/// A stream with no manifest beside it is **refused**, not skipped. The manifest carries the
/// observation date, the harness version and the model the run resolved, and `aep eval run --stream`
/// needs the first of those to assemble a document that reproduces byte for byte. A transcript
/// without one is a file, not evidence.
fn streams(recorded: &Path) -> Result<Vec<PathBuf>, String> {
    let mut found = Vec::new();
    let mut entries = std::fs::read_dir(recorded)
        .map_err(|error| format!("reading {}: {error}", recorded.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("reading {}: {error}", recorded.display()))?
        .into_iter()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        let Some(name) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
            continue;
        };
        if !name.ends_with(STREAM_SUFFIX) {
            continue;
        }
        let stem = &name[..name.len() - STREAM_SUFFIX.len()];
        let manifest = recorded.join(format!("{stem}{MANIFEST_SUFFIX}"));
        if !manifest.is_file() {
            return Err(format!(
                "{} has no {stem}{MANIFEST_SUFFIX} beside it; a recorded transcript carries the \
                 manifest its run left, which is where its observation date comes from",
                path.display()
            ));
        }
        found.push(path);
    }
    Ok(found)
}

/// The `observed_at:` a run manifest states.
fn observed_at(manifest: &Path) -> Result<String, String> {
    let text = std::fs::read_to_string(manifest)
        .map_err(|error| format!("reading {}: {error}", manifest.display()))?;
    text.lines()
        .find_map(|line| line.strip_prefix("observed_at:"))
        .map(|rest| rest.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{} states no `observed_at`", manifest.display()))
}

/// Replays one recorded stream through `aep eval run --stream`, which spends nothing.
fn replay(binary: &Path, root: &Path, stream: &Path, out: &Path) -> Result<(), String> {
    let directory = stream
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| format!("{} is not inside a case", stream.display()))?;
    let name = stream
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .expect("a stream has a readable name");
    let stem = &name[..name.len() - STREAM_SUFFIX.len()];
    let manifest = stream
        .parent()
        .expect("a stream has a directory")
        .join(format!("{stem}{MANIFEST_SUFFIX}"));
    let arm: Case = yaml(&directory.join("case.yaml"), "a case")?;

    let output = Command::new(binary)
        .arg("eval")
        .arg("run")
        .args(["--case".as_ref(), directory.as_os_str()])
        .args(["--arm", &arm.arm])
        .args(["--harness", "claude"])
        .args(["--stream".as_ref(), stream.as_os_str()])
        .args(["--out".as_ref(), out.as_os_str()])
        .args(["--observed-at", &observed_at(&manifest)?])
        .arg("--redact")
        .current_dir(root)
        // Nothing here may spawn anything, whatever the caller exported. `--stream` never reaches
        // the spawn, and removing these makes that a property of the call rather than of the path
        // taken through somebody else's binary.
        .env_remove("METAHARNESS_LIVE")
        .env_remove("METAHARNESS_BIN")
        .output()
        .map_err(|error| format!("running {}: {error}", binary.display()))?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "replaying {} exited {}:\n{}{}",
        stream.display(),
        output
            .status
            .code()
            .map_or_else(|| "on a signal".to_owned(), |code| code.to_string()),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

/// Finds the `aep` binary, under either of the two names it is published as.
///
/// `aep` is the spelling this repository's instructions use (`AGENTS.md` § *Invariants*) and
/// `protocol` is what the binary prints in its own usage lines; a machine may carry either.
fn tool() -> Option<PathBuf> {
    for name in ["aep", "protocol"] {
        if let Ok(found) = Command::new(name).arg("--version").output() {
            if found.status.success() {
                return Some(PathBuf::from(name));
            }
        }
    }
    None
}

/// Validates every case under `evals/` and replays whatever transcripts are recorded beside them.
///
/// The whole free half of `story:eval-ci-gates`. It prints its own summary rather than returning
/// one, so that wiring it in moved exactly one line of [`crate::check`] — a signature this
/// repository's release path also calls.
pub(crate) fn evals(root: &Path) -> Result<(), String> {
    let corpus = root.join("evals");
    let mut directories = std::fs::read_dir(&corpus)
        .map_err(|error| format!("reading {}: {error}", corpus.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("reading {}: {error}", corpus.display()))?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    directories.sort();
    if directories.is_empty() {
        return Err(format!(
            "{} holds no case, and a corpus with nothing in it reports a clean sheet",
            corpus.display()
        ));
    }

    let mut recorded = Vec::new();
    for directory in &directories {
        recorded.extend(case(root, directory)?);
    }

    let cases = directories.len();
    if recorded.is_empty() {
        println!(
            "notice: {cases} eval case(s) validated; no transcript is recorded under any \
             `recorded/`, so nothing was replayed. Each case's `recorded/README.md` carries the \
             live command that would produce one."
        );
        return Ok(());
    }
    let Some(binary) = tool() else {
        println!(
            "notice: {cases} eval case(s) validated; {} recorded transcript(s) were not replayed \
             because neither `aep` nor `protocol` is on PATH. Install the CLI from \
             https://github.com/beyond10x/aep/releases to replay them.",
            recorded.len()
        );
        return Ok(());
    };

    let out = std::env::temp_dir().join(format!("agentplugins-evals-{}", std::process::id()));
    let mut replayed = 0_usize;
    let mut failure = None;
    for stream in &recorded {
        if let Err(error) = replay(&binary, root, stream, &out) {
            failure = Some(error);
            break;
        }
        replayed += 1;
    }
    let _ = std::fs::remove_dir_all(&out);
    if let Some(error) = failure {
        return Err(error);
    }
    println!("valid: {cases} eval case(s), {replayed} recorded transcript(s) replayed");
    Ok(())
}

/// Which case directories a set of changed paths is the subject of.
///
/// The diff scope `.github/workflows/eval.yml` runs a paid arm over. It lives here rather than as
/// `grep` in a workflow for two reasons: `AGENTS.md` § *Invariants* says anything executable in this
/// repository is Rust, and a scope computed by matching substrings against YAML is a scope that
/// silently returns nothing the first time a field is reindented.
///
/// A path matches a case when it *is* one of the case's `paths:`, when it is the file that declares
/// one of its `agents:`, or when it is the `SKILL.md` — or any file beside it — of one of its
/// `skills:`. The last of those is why a reference document under a skill's `references/` re-runs the
/// cases that skill covers: it is instruction the agent reads, and a change to it changes behaviour
/// exactly as a change to the `SKILL.md` does.
///
/// Each match carries the plugin directory an `arm: plugin` run of it installs — the plugin of its
/// first agent, or of its first skill. `aep eval run --plugin-dir` takes exactly one at 0.42.0, and
/// which one a case needs is a fact about the case rather than something a workflow should guess.
fn scope(root: &Path, changed: &[String]) -> Result<Vec<(String, String)>, String> {
    let corpus = root.join("evals");
    let mut directories = std::fs::read_dir(&corpus)
        .map_err(|error| format!("reading {}: {error}", corpus.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("reading {}: {error}", corpus.display()))?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    directories.sort();

    let mut matched = Vec::new();
    for directory in directories {
        let document: Case = yaml(&directory.join("case.yaml"), "a case")?;
        let mut surfaces: Vec<String> = document.subject.paths.clone();
        for reference in &document.subject.agents {
            let (plugin, name) = qualified(reference, "agent", &document.id)?;
            surfaces.push(format!("plugins/{plugin}/agents/{name}.md"));
        }
        for reference in &document.subject.skills {
            let (plugin, name) = qualified(reference, "skill", &document.id)?;
            surfaces.push(skill_directory(root, plugin, name, &document.id)?);
        }
        let hit = changed.iter().any(|path| {
            surfaces
                .iter()
                .any(|surface| path == surface || path.starts_with(&format!("{surface}/")))
        });
        if hit {
            matched.push((format!("evals/{}", document.id), plugin_of(&document)?));
        }
    }
    Ok(matched)
}

/// The plugin directory an `arm: plugin` run of a case installs.
fn plugin_of(document: &Case) -> Result<String, String> {
    let first = document
        .subject
        .agents
        .first()
        .or_else(|| document.subject.skills.first())
        .ok_or_else(|| format!("case `{}` names no agent and no skill", document.id))?;
    let (plugin, _) = qualified(first, "agent or skill", &document.id)?;
    Ok(format!("plugins/{plugin}"))
}

/// The directory of the skill a reference names, repository-relative.
fn skill_directory(root: &Path, plugin: &str, name: &str, case: &str) -> Result<String, String> {
    let skills = root.join("plugins").join(plugin).join("skills");
    let mut entries = std::fs::read_dir(&skills)
        .map_err(|error| format!("reading {}: {error}", skills.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("reading {}: {error}", skills.display()))?
        .into_iter()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        let document = path.join("SKILL.md");
        if document.is_file() && declared_name(&document)? == name {
            let directory = path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .expect("a skill directory has a readable name");
            return Ok(format!("plugins/{plugin}/skills/{directory}"));
        }
    }
    Err(format!(
        "case `{case}` names the skill `{plugin}:{name}`, and no SKILL.md under {} declares it",
        skills.display()
    ))
}

/// The corpus subcommands, or [`None`] when these arguments are not one.
///
/// Handled before the marketplace check's own argument match and never inside it, so that adding
/// this surface moved three lines of `main` and touched no existing arm. The `Ok` arm there prints
/// *marketplace beyond10x* and would be a false sentence under either verb below.
pub(crate) fn cli(root: &Path, arguments: &[String]) -> Option<std::process::ExitCode> {
    let verdict = match arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["evals"] => evals(root),
        ["evals", "scope", changed @ ..] => {
            let changed: Vec<String> = changed.iter().map(|path| (*path).to_owned()).collect();
            scope(root, &changed).map(|matched| {
                for (case, plugin) in matched {
                    println!("{case}\t{plugin}");
                }
            })
        }
        _ => return None,
    };
    Some(match verdict {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            std::process::ExitCode::from(1)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The repository root.
    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under the repository root")
            .to_path_buf()
    }

    #[test]
    fn the_committed_corpus_is_valid() {
        evals(&root()).expect("the committed eval corpus validates");
    }

    /// Every case names a surface this repository ships, and the resolver reads the frontmatter
    /// rather than the directory — which is the whole reason `ess-schema:ess-schema` resolves at all.
    #[test]
    fn a_skill_is_resolved_by_the_name_its_document_declares() {
        let root = root();
        resolve_skill(&root, "ess-schema:ess-schema", "t")
            .expect("the ESS skill declares `name: ess-schema` under `skills/ess-schema/`");
        let error = resolve_skill(&root, "ess-schema:schema-validation", "t")
            .expect_err("the directory name is not what a harness lists");
        assert!(error.contains("declares"), "{error}");
    }

    /// A case that names an agent nobody ships is the failure this check exists for: the case would
    /// go on passing its own document while judging nothing.
    #[test]
    fn a_case_naming_a_missing_agent_is_refused() {
        let error = resolve_agent(&root(), "aep-planning:plan-critic-security", "t")
            .expect_err("an agent this repository does not ship must be refused");
        assert!(error.contains("plan-critic-security"), "{error}");
    }

    /// A reference that is not `<plugin>:<name>` is refused by shape, before anything touches a
    /// filesystem — a bare `decomposer` is not what a harness lists and would resolve by accident.
    #[test]
    fn an_unqualified_reference_is_refused() {
        let error =
            qualified("decomposer", "agent", "t").expect_err("a bare name is not a qualified one");
        assert!(error.contains("<plugin>:<name>"), "{error}");
    }

    /// A stream with no manifest beside it has no observation date, so nothing can replay it into a
    /// document that reproduces. Refused rather than skipped.
    #[test]
    fn a_stream_without_its_manifest_is_refused() {
        let sandbox =
            std::env::temp_dir().join(format!("agentplugins-streams-{}", std::process::id()));
        std::fs::create_dir_all(&sandbox).expect("the sandbox is writable");
        std::fs::write(sandbox.join(format!("run{STREAM_SUFFIX}")), "").expect("writable");
        let error = streams(&sandbox).expect_err("a stream with no provenance must be refused");
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        assert!(error.contains(MANIFEST_SUFFIX), "{error}");
    }

    /// A change to one critic runs that critic's case and not the other three, and a change to the
    /// rubric they share runs all four — which is the whole point of scoping a paid run to a diff.
    #[test]
    fn the_diff_scope_follows_the_subject_and_not_the_directory() {
        let root = root();
        let one = scope(
            &root,
            &["plugins/aep-planning/agents/plan-critic-scope.md".to_owned()],
        )
        .expect("the corpus scopes");
        assert_eq!(
            one,
            vec![(
                "evals/plan-critic-scope-verdict".to_owned(),
                "plugins/aep-planning".to_owned()
            )]
        );

        let rubric = scope(
            &root,
            &["plugins/aep-planning/skills/planning/references/critic-rubric.md".to_owned()],
        )
        .expect("the corpus scopes");
        // The four critic cases name the rubric directly, and the golden path names the planning
        // skill the rubric lives inside — its step 5 is the panel.
        assert_eq!(
            rubric.len(),
            5,
            "the rubric is every critic's instruction: {rubric:?}"
        );
        assert!(
            rubric
                .iter()
                .any(|(case, _)| case == "evals/golden-path-end-to-end"),
            "{rubric:?}"
        );

        let unrelated = scope(&root, &["README.md".to_owned()]).expect("the corpus scopes");
        assert!(unrelated.is_empty(), "{unrelated:?}");
    }

    /// A file beside a `SKILL.md` is instruction the agent reads, so it re-runs that skill's cases.
    #[test]
    fn a_reference_beside_a_skill_is_in_that_skills_scope() {
        let matched = scope(
            &root(),
            &["plugins/adp/skills/wave/references/unit-brief.md".to_owned()],
        )
        .expect("the corpus scopes");
        // The adversary's case, and the golden path — whose step 6 is the wave. Both are right.
        assert_eq!(
            matched,
            vec![
                (
                    "evals/adversary-tests-only".to_owned(),
                    "plugins/adp".to_owned()
                ),
                (
                    "evals/golden-path-end-to-end".to_owned(),
                    "plugins/aep-planning".to_owned()
                ),
            ]
        );
    }

    /// Every case declares an arm `aep eval run --arm` knows, so a corpus cannot be validated here
    /// and refused there over a word.
    #[test]
    fn every_committed_case_declares_a_known_arm() {
        let corpus = root().join("evals");
        let mut checked = 0_usize;
        for entry in std::fs::read_dir(&corpus).expect("the corpus is readable") {
            let path = entry.expect("a readable entry").path();
            if !path.is_dir() {
                continue;
            }
            let document: Case = yaml(&path.join("case.yaml"), "a case").expect("the case parses");
            assert!(
                ARMS.contains(&document.arm.as_str()),
                "{} declares the arm `{}`",
                path.display(),
                document.arm
            );
            checked += 1;
        }
        assert!(checked >= 8, "the corpus lost cases: {checked} found");
    }
}
