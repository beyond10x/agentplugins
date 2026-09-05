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
use std::sync::atomic::{AtomicUsize, Ordering};

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

/// How many times this process has replayed a corpus, so two calls never share a directory.
static REPLAYS: AtomicUsize = AtomicUsize::new(0);

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
    /// Optional repository-owned semantic assertions beyond AEP's trace vocabulary.
    #[serde(default)]
    command_contract: Option<String>,
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
/// Read rather than inferred from the directory: nothing requires a skill's directory to be spelt
/// the way its frontmatter is, a harness lists what the document declares, and a resolver that
/// trusted the directory would look for a skill that does not exist under a name nothing uses.
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
/// By the declared name across every skill directory of the plugin, and not by directory: a harness
/// lists what the document declares, and nothing makes the two agree. One of this repository's
/// skills carried a directory spelt differently from its frontmatter for four releases, and a
/// resolver that trusted the directory was wrong about it for all four.
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
    if document
        .command_contract
        .as_deref()
        .is_some_and(|contract| contract != "connectors-readiness")
    {
        return Err(format!(
            "case `{name}` declares an unknown command_contract"
        ));
    }
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

/// The `plugin:` spellings a run manifest's `plugins:` list names, in order.
///
/// One `- plugin: <repo>@<name>@<pin>` line each. Read as text rather than through a typed reader
/// because this gate needs one field of a document `aep` owns, and a struct here would be a second
/// definition of that document to keep in step.
fn pinned_plugins(manifest: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(manifest)
        .map_err(|error| format!("reading {}: {error}", manifest.display()))?;
    Ok(text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- plugin:"))
        .map(|rest| rest.trim().to_owned())
        .filter(|value| !value.is_empty())
        .collect())
}

/// The plugin arguments a replay must repeat, because a run's treatment is not in its stream.
///
/// `aep eval run --stream` refuses `EVAL-STREAM-009` when a stream attests more than one installed
/// plugin and nothing on the command line says which of them was the treatment — *"which of them
/// was the treatment is not this reader's guess to make"*. It is right to: the manifest carries one
/// `plugin_digest`, and a reader that picked would be authoring the experiment.
///
/// The two documents beside the stream do say. The manifest's `plugins:` names every pin, verbatim,
/// which is what `--plugin` took. The case's `subject` names every plugin the case is about, as the
/// `<plugin>:` prefix of each agent and skill; the one that is not pinned is the one that arrived as
/// `--plugin-dir`, and its directory is `plugins/<name>` by this repository's own layout.
///
/// A case whose manifest pins none needs no arguments at all: a single-plugin stream is unambiguous
/// and `aep` reads the treatment out of it.
///
/// **Nothing here refuses.** `--plugin-dir` reaches only the spawn a replay never performs —
/// `aep`'s `ingest_recorded` resolves the treatment from `--plugin` alone — so a subject whose
/// remainder is not exactly one plugin is not a reason to fail the gate. The pins go, and `aep`
/// answers with `EVAL-STREAM-009` if it genuinely cannot decide. A gate that refused here would
/// stop a replay that would have succeeded, over an argument the reader discards.
fn treatment_args(case: &Case, pins: Vec<String>) -> Vec<String> {
    if pins.is_empty() {
        return Vec::new();
    }
    let mut named: Vec<String> = case
        .subject
        .agents
        .iter()
        .chain(case.subject.skills.iter())
        .filter_map(|qualified| qualified.split(':').next())
        .map(str::to_owned)
        .collect();
    named.sort();
    named.dedup();

    let pinned_names: Vec<&str> = pins
        .iter()
        .filter_map(|pin| pin.rsplit_once('@').map(|(head, _)| head))
        .filter_map(|head| head.rsplit_once('@').map(|(_, name)| name))
        .collect();
    let directories: Vec<&String> = named
        .iter()
        .filter(|name| !pinned_names.contains(&name.as_str()))
        .collect();
    // The pins are what `aep` actually reads back: `ingest_recorded` resolves the treatment from
    // `--plugin` alone, and `--plugin-dir` reaches only the spawn this path never performs. So an
    // ambiguous remainder is not a reason to refuse a replay — it is a reason to send the pins and
    // let `aep` answer, which it does with `EVAL-STREAM-009` if it genuinely cannot.
    let mut args = Vec::new();
    if let [directory] = directories.as_slice() {
        args.push("--plugin-dir".to_owned());
        args.push(format!("plugins/{directory}"));
    }
    for pin in pins {
        args.push("--plugin".to_owned());
        args.push(pin);
    }
    args
}

/// How the trace report a replay writes beside its stream is named.
const REPORT_SUFFIX: &str = ".report.json";

/// The format claim the report a replay writes carries.
const REPORT_FORMAT: &str = "trace-report/1";

/// The gating expectations a replay's own report says the run contradicted.
///
/// **The exit status of `aep eval run --stream` does not carry this.** Measured on 0.44.0,
/// 2026-09-03: a replay of `golden-path-end-to-end` against a specification whose first two rows the
/// transcript contradicts prints *"not conformant: the run contradicted 2 expectation(s) …
/// (exit 1)"* — and exits **0**. So a gate that reads only the status, which this one did until
/// 0.6.2, reports *"1 recorded transcript(s) replayed"* over a corpus that no longer describes its
/// own recording. Nothing else in this repository would notice; the rename that found it found it by
/// a person running the replay by hand.
///
/// Read from `trace-report/1` rather than from the printed sentence: the record is the document
/// `aep` writes for a reader, and a gate that scraped prose would go quiet the first time the
/// wording changed.
///
/// # The rule is the document's own verdict, not a row predicate this gate invents
///
/// A first version of this read *"a row that says `gap` and does not say `advisory`"*, on the belief
/// that `on_unknown:` had already resolved an `unk` by the time a row's verdict was written. **It
/// has not.** Measured on `protocol 0.50.0`, 2026-09-03: a replay against a specification holding
/// one gating `order` row that the transcript cannot decide prints *"undecided: nothing was
/// contradicted and 1 expectation(s) could not be judged from this transcript … (exit 3)"*, exits
/// **0**, and leaves the row at `verdict: "unknown"`, `severity: "gate"` — the same fail-open, one
/// word further along. A gate reading only `gap` calls that a replayed transcript.
///
/// So the rule is `verdict == "ok"` at the top of the document, which is `aep`'s own arithmetic over
/// severity and `on_unknown:` rather than a second copy of it here. It is exactly *"every gating row
/// held"*: the same probe reported `"ok"` for a document carrying one advisory gap and one advisory
/// unknown and nothing else. The rows are then listed for the message, and a document that refuses
/// itself while naming no row is refused too, so an unexplained verdict cannot pass as an empty one.
///
/// # And the format claim is checked, like every other document this module reads
///
/// The module header's own rule for somebody else's document is *"that the format claim is right"*.
/// This one was exempt from it: a `trace-report` whose rows carried their outcome under another key
/// read as a report with nothing contradicted, which relocates the fail-open rather than removing
/// it. A report that does not claim `trace-report/1` is refused unread.
fn contradicted(report: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(report).map_err(|error| {
        format!(
            "reading {}: {error}; a replay that left no report is not evidence that it held",
            report.display()
        )
    })?;
    let document: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| format!("parsing {}: {error}", report.display()))?;

    let format = document
        .get("format")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("{} states no `format`", report.display()))?;
    if format != REPORT_FORMAT {
        return Err(format!(
            "{} claims `{format}` and not `{REPORT_FORMAT}`; this gate reads the fields \
             `{REPORT_FORMAT}` defines, and reading them out of a document that says it is \
             something else is how a report with nothing contradicted gets manufactured",
            report.display()
        ));
    }

    let verdict = document
        .get("verdict")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            format!(
                "{} states no `verdict`; a report that does not say whether the run held is not \
                 evidence that it did",
                report.display()
            )
        })?;
    if verdict == "ok" {
        return Ok(Vec::new());
    }

    let rows = document
        .get("expectations")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{} carries no `expectations`", report.display()))?;
    let named: Vec<String> = rows
        .iter()
        .filter(|row| {
            row.get("verdict").and_then(serde_json::Value::as_str) != Some("ok")
                && row.get("severity").and_then(serde_json::Value::as_str) != Some("advisory")
        })
        .map(|row| {
            row.get("id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("<a row with no id>")
                .to_owned()
        })
        .collect();
    if named.is_empty() {
        return Err(format!(
            "{} states `verdict: {verdict}` and names no gating row that explains it; an \
             unexplained refusal is not a pass",
            report.display()
        ));
    }
    Ok(named)
}

/// Replays one recorded stream through `aep eval run --stream`, which spends nothing.
///
/// **The flat `eval run` here is deliberate and stays.** AEP 0.52.0 groups the verb as
/// `aep drive eval run` and keeps `eval run` as a hidden alias with identical output, but this is a
/// call and not a lesson: the binary it reaches is whichever `aep` is on the machine's `PATH`, and
/// the published release the install page pins does not have the grouped spelling. `main.rs`'s
/// `flat_spellings` sweep reads documents and not Rust for the same reason.
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
    check_commands(&arm, stream)?;

    let output = Command::new(binary)
        .arg("eval")
        .arg("run")
        .args(["--case".as_ref(), directory.as_os_str()])
        .args(["--arm", &arm.arm])
        .args(["--harness", "claude"])
        .args(["--stream".as_ref(), stream.as_os_str()])
        .args(["--out".as_ref(), out.as_os_str()])
        .args(["--observed-at", &observed_at(&manifest)?])
        .args(treatment_args(&arm, pinned_plugins(&manifest)?))
        .arg("--redact")
        .current_dir(root)
        // Nothing here may spawn anything, whatever the caller exported. `--stream` never reaches
        // the spawn, and removing these makes that a property of the call rather than of the path
        // taken through somebody else's binary.
        .env_remove("METAHARNESS_LIVE")
        .env_remove("METAHARNESS_BIN")
        .output()
        .map_err(|error| format!("running {}: {error}", binary.display()))?;
    if !output.status.success() {
        return Err(format!(
            "replaying {} exited {}:\n{}{}",
            stream.display(),
            output
                .status
                .code()
                .map_or_else(|| "on a signal".to_owned(), |code| code.to_string()),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let contradicted = contradicted(&out.join(format!("{stem}{REPORT_SUFFIX}")))?;
    if contradicted.is_empty() {
        return Ok(());
    }
    Err(format!(
        "replaying {} left {} gating expectation(s) unheld — {}. The transcript is the record of a \
         run that happened; a gating row it contradicts, or that it cannot decide, is the corpus \
         describing a run other than the one recorded. Re-record the case, or say in the document \
         why the recorded run is what the row means.\n{}",
        stream.display(),
        contradicted.len(),
        contradicted.join(", "),
        String::from_utf8_lossy(&output.stdout)
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

    // Per **call**, not per process. Two of this crate's own tests reach here — the corpus one and
    // the marketplace one — and `cargo test` runs them on threads of one process, so a directory
    // named for the pid is one directory shared by both: whichever finishes first runs the
    // `remove_dir_all` below while the other is still writing into it. That was invisible until a
    // transcript was actually committed, because before that the replay never ran.
    let out = std::env::temp_dir().join(format!(
        "agentplugins-evals-{}-{}",
        std::process::id(),
        REPLAYS.fetch_add(1, Ordering::Relaxed)
    ));
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
        ["evals", "check-stream", case, stream] => {
            check_stream(root, Path::new(case), Path::new(stream))
        }
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

fn check_commands(case: &Case, stream: &Path) -> Result<(), String> {
    match case.command_contract.as_deref() {
        None => Ok(()),
        Some("connectors-readiness") => crate::readiness::check(stream),
        Some(other) => Err(format!("unknown command_contract `{other}`")),
    }
}

/// Check the repository's extra command contract and the AEP report from a live run.
fn check_stream(root: &Path, directory: &Path, stream: &Path) -> Result<(), String> {
    let directory = root.join(directory);
    case(root, &directory)?;
    let document: Case = yaml(&directory.join("case.yaml"), "a case")?;
    check_commands(&document, stream)?;
    let name = stream
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("invalid stream name")?;
    let stem = name
        .strip_suffix(STREAM_SUFFIX)
        .ok_or("expected an .events.jsonl stream")?;
    let report = stream.with_file_name(format!("{stem}{REPORT_SUFFIX}"));
    let failures = contradicted(&report)?;
    if !failures.is_empty() {
        return Err(format!(
            "{}: unheld expectations: {}",
            report.display(),
            failures.join(", ")
        ));
    }
    println!("valid: {} command and trace contracts", document.id);
    Ok(())
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

    /// A case carrying only the two fields `treatment_args` reads.
    fn case_about(plugins: &[&str]) -> Case {
        Case {
            format: CASE_FORMAT.to_owned(),
            id: "a-case".to_owned(),
            title: String::new(),
            workflow: String::new(),
            states: Vec::new(),
            arm: "plugin".to_owned(),
            verdict: "held".to_owned(),
            subject: Subject {
                agents: plugins.iter().map(|p| format!("{p}:an-agent")).collect(),
                skills: Vec::new(),
                paths: Vec::new(),
            },
            task: String::new(),
            expectations: String::new(),
            recorded: String::new(),
            command_contract: None,
            advisory_gaps: Vec::new(),
            violated: Vec::new(),
        }
    }

    #[test]
    fn readiness_contract_is_required_even_when_the_trace_report_passes() {
        let directory =
            std::env::temp_dir().join(format!("readiness-contract-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let stream = directory.join("synthetic.events.jsonl");
        let report = directory.join(format!("synthetic{REPORT_SUFFIX}"));
        std::fs::write(&report, r#"{"format":"trace-report/1","verdict":"ok"}"#).unwrap();
        std::fs::write(&stream, "").unwrap();
        let error =
            check_stream(&root(), Path::new("evals/connectors-readiness"), &stream).unwrap_err();
        assert!(error.contains("doctor-ran"), "{error}");
        std::fs::write(&stream, r#"{"format":"metaharness.event/1","event":"tool.requested","name":"exec_command","input":{"cmd":"connectors inspect doctor && connectors serve local --help"}}"#).unwrap();
        check_stream(&root(), Path::new("evals/connectors-readiness"), &stream).unwrap();
        std::fs::remove_dir_all(&directory).unwrap();
    }

    #[test]
    fn the_treatment_a_replay_repeats_is_the_subject_minus_the_pins() {
        // The golden path's shape: three plugins in the subject, two of them pinned by the
        // manifest, so the third is what arrived as `--plugin-dir`. The committed golden-path
        // manifest still pins the pre-0.6.2 spellings, because it is the record of a run that
        // happened under them — so that replay takes the branch the third test covers, and this one
        // is about the rule rather than about today's corpus.
        let case = case_about(&["aep-plan", "aep-drive", "ess-specify"]);
        let pins = vec![
            "beyond10x/agentplugins@aep-drive@0.6.1".to_owned(),
            "beyond10x/agentplugins@ess-specify@0.6.1".to_owned(),
        ];
        assert_eq!(
            treatment_args(&case, pins),
            vec![
                "--plugin-dir",
                "plugins/aep-plan",
                "--plugin",
                "beyond10x/agentplugins@aep-drive@0.6.1",
                "--plugin",
                "beyond10x/agentplugins@ess-specify@0.6.1",
            ]
        );
    }

    #[test]
    fn a_case_whose_manifest_pins_nothing_repeats_nothing() {
        // A single-plugin stream is unambiguous and `aep` reads the treatment out of it, so a
        // replay that added arguments would be inventing the experiment.
        assert!(treatment_args(&case_about(&["aep-drive"]), Vec::new()).is_empty());
    }

    #[test]
    fn an_ambiguous_remainder_sends_the_pins_rather_than_failing_the_gate() {
        // `--plugin-dir` reaches only the spawn a replay never performs. Refusing here would stop
        // a replay that would have succeeded, over an argument `aep` discards.
        let two_left = case_about(&["aep-plan", "aep-drive", "ess-specify", "extra"]);
        let pins = vec!["beyond10x/agentplugins@aep-drive@0.6.1".to_owned()];
        let args = treatment_args(&two_left, pins.clone());
        assert!(!args.iter().any(|a| a == "--plugin-dir"), "{args:?}");
        assert_eq!(
            args,
            vec!["--plugin", "beyond10x/agentplugins@aep-drive@0.6.1"]
        );

        // And when every subject plugin is pinned, the remainder is empty rather than wrong.
        let all_pinned = case_about(&["aep-drive"]);
        assert_eq!(
            treatment_args(&all_pinned, pins),
            vec!["--plugin", "beyond10x/agentplugins@aep-drive@0.6.1"]
        );
    }

    #[test]
    fn the_committed_corpus_is_valid() {
        evals(&root()).expect("the committed eval corpus validates");
    }

    /// A replay is judged by the report it wrote, because the process status does not carry the
    /// verdict — `aep eval run --stream` exits 0 on a run it has just called *not conformant*. A
    /// gate reading the status alone calls a corpus green while its own recording contradicts it.
    #[test]
    fn a_replay_that_contradicts_a_gating_row_is_not_a_green_replay() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-report-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&sandbox).expect("the sandbox is writable");
        let report = sandbox.join(format!("run{REPORT_SUFFIX}"));
        let write = |verdict: &str, rows: &str| {
            std::fs::write(
                &report,
                format!(
                    "{{\"format\":\"{REPORT_FORMAT}\",\"verdict\":\"{verdict}\",\
                     \"expectations\":[{rows}]}}"
                ),
            )
            .expect("the sandbox is writable");
        };

        // `aep`'s own arithmetic: a document whose only gaps are advisory is `verdict: ok`, which
        // is measured rather than assumed — see [`contradicted`].
        write(
            "ok",
            "{\"id\":\"a\",\"verdict\":\"ok\",\"severity\":\"gate\"},\
             {\"id\":\"b\",\"verdict\":\"gap\",\"severity\":\"advisory\"}",
        );
        assert!(
            contradicted(&report).expect("the report reads").is_empty(),
            "an advisory gap is what `severity: advisory` declares it to be"
        );

        write(
            "gap",
            "{\"id\":\"a\",\"verdict\":\"ok\",\"severity\":\"gate\"},\
             {\"id\":\"the-decomposer-was-offered\",\"verdict\":\"gap\",\"severity\":\"gate\"}",
        );
        assert_eq!(
            contradicted(&report).expect("the report reads"),
            vec!["the-decomposer-was-offered"]
        );

        // A document that refuses itself and names no gating row does not pass as an empty list.
        write(
            "gap",
            "{\"id\":\"a\",\"verdict\":\"ok\",\"severity\":\"gate\"}",
        );
        let error = contradicted(&report).expect_err("an unexplained refusal is not a pass");
        assert!(error.contains("unexplained refusal"), "{error}");

        // A report that does not say whether the run held is refused rather than read for the
        // fields it does carry.
        std::fs::write(
            &report,
            format!("{{\"format\":\"{REPORT_FORMAT}\",\"expectations\":[]}}"),
        )
        .expect("the sandbox is writable");
        let error = contradicted(&report).expect_err("a report with no verdict must be refused");
        assert!(error.contains("states no `verdict`"), "{error}");

        // A replay that wrote no report is not evidence that it held.
        std::fs::remove_file(&report).expect("the sandbox is writable");
        let error = contradicted(&report).expect_err("a missing report must be refused");
        assert!(error.contains("not evidence"), "{error}");

        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
    }

    /// A replay `aep` itself calls **undecided** is not a green replay either, and this gate says
    /// it is.
    ///
    /// [`contradicted`] counts a row only when its verdict is `gap`, on the stated ground that
    /// *"an `unk` is `aep`'s to resolve through `on_unknown:`, which it has already done by the
    /// time it writes a row's verdict"*. Measured on `protocol 0.50.0`, 2026-09-03, replaying the
    /// committed golden-path transcript against a specification holding one **gating** `order` row
    /// nothing in that stream decides:
    ///
    /// ```text
    /// claude-plugin-golden-path-end-to-end — undecided: nothing was contradicted and 1
    /// expectation(s) could not be judged from this transcript — somebody should look at the
    /// format, not at the agent (exit 3)
    /// ```
    ///
    /// The process exited **0** — the same fail-open this gate was added in 0.6.2 to close for
    /// `gap`. The row is left at `verdict: "unknown"`, `severity: "gate"`; nothing resolved it. The
    /// document below is that run's report, verbatim.
    ///
    /// The same run's document-level `verdict` is `"unknown"`, and a probe with one advisory gap
    /// and one advisory unknown and nothing else reported `verdict: "ok"` — so `verdict == "ok"`
    /// is exactly *"every gating row held"*, and reading it would close both halves at once.
    #[test]
    fn a_replay_aep_calls_undecided_is_not_a_green_replay() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-undecided-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&sandbox).expect("the sandbox is writable");
        let report = sandbox.join(format!("run{REPORT_SUFFIX}"));
        std::fs::write(
            &report,
            r#"{"format":"trace-report/1","spec_id":"eval-case/golden-path-end-to-end",
               "spec_title":"One gating ordering row that the transcript cannot decide",
               "adapter":{"name":"metaharness/event-stream"},"redacted":true,
               "advisory_overrides":[],
               "summary":{"total":1,"ok":0,"gap":0,"unknown":1,"advisory_gap":0,
                          "advisory_unknown":0},
               "verdict":"unknown",
               "expectations":[{"id":"a-gating-ordering-nothing-decides",
                 "statement":"a command nobody ran preceded another command nobody ran",
                 "kind":"order","severity":"gate",
                 "outcome":{"outcome":"undecidable","reason":"never_occurred"},
                 "verdict":"unknown"}]}"#,
        )
        .expect("the sandbox is writable");

        let judged = contradicted(&report).expect("the report reads");
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        assert_eq!(
            judged,
            vec!["a-gating-ordering-nothing-decides"],
            "`aep` called this replay undecided and exited 0; a gate that reads only `gap` calls \
             it a replayed transcript"
        );
    }

    /// The one document this crate reads without checking the format claim it carries.
    ///
    /// Every other document here is refused when its `format:` is not the one this gate was
    /// written against — that is the module's own stated rule, *"that the format claim is right"* —
    /// and [`contradicted`] instead reaches straight for `expectations`, `verdict` and `severity`.
    /// A `trace-report` whose rows carry their outcome anywhere else therefore reads as a report
    /// with nothing contradicted, which is the fail-open this gate exists to remove rather than to
    /// relocate. The row below is a gating `gap` written the way the same run's report already
    /// spells it under `outcome.outcome`.
    #[test]
    fn a_report_whose_format_this_gate_does_not_know_is_not_read_as_green() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-report-format-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&sandbox).expect("the sandbox is writable");
        let report = sandbox.join(format!("run{REPORT_SUFFIX}"));
        std::fs::write(
            &report,
            r#"{"format":"trace-report/2",
               "expectations":[{"id":"the-decomposer-was-offered","severity":"gate",
                 "outcome":{"outcome":"gap"}}]}"#,
        )
        .expect("the sandbox is writable");

        let judged = contradicted(&report);
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        let error = judged.expect_err(
            "a report claiming a format this gate was not written against must be refused, not \
             read for the fields it happens to share",
        );
        assert!(error.contains("trace-report/1"), "{error}");
    }

    /// Every case names a surface this repository ships, and the resolver reads the frontmatter
    /// rather than the directory — so a reference is refused when no document declares it, however
    /// plausible the directory that would have carried it.
    #[test]
    fn a_skill_is_resolved_by_the_name_its_document_declares() {
        let root = root();
        resolve_skill(&root, "ess-specify:specify", "t")
            .expect("the ESS skill declares `name: specify` under `skills/specify/`");
        let error = resolve_skill(&root, "ess-specify:schema-validation", "t")
            .expect_err("a name no SKILL.md declares is not what a harness lists");
        assert!(error.contains("declares"), "{error}");
    }

    /// A case that names an agent nobody ships is the failure this check exists for: the case would
    /// go on passing its own document while judging nothing.
    #[test]
    fn a_case_naming_a_missing_agent_is_refused() {
        let error = resolve_agent(&root(), "aep-plan:plan-critic-security", "t")
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
            &["plugins/aep-plan/agents/plan-critic-scope.md".to_owned()],
        )
        .expect("the corpus scopes");
        assert_eq!(
            one,
            vec![(
                "evals/plan-critic-scope-verdict".to_owned(),
                "plugins/aep-plan".to_owned()
            )]
        );

        let rubric = scope(
            &root,
            &["plugins/aep-plan/skills/planning/references/critic-rubric.md".to_owned()],
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
            &["plugins/aep-drive/skills/wave/references/unit-brief.md".to_owned()],
        )
        .expect("the corpus scopes");
        // The adversary's case, and the golden path — whose step 6 is the wave. Both are right.
        assert_eq!(
            matched,
            vec![
                (
                    "evals/adversary-tests-only".to_owned(),
                    "plugins/aep-drive".to_owned()
                ),
                (
                    "evals/golden-path-end-to-end".to_owned(),
                    "plugins/aep-plan".to_owned()
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
