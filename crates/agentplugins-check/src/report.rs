//! What a headless trial run achieved, in numbers, and whether they got worse than the baseline.
//!
//! Until this existed a run was read by hand and nothing compared one round with the last. Every
//! number here comes from the run's own stream-json (tool calls and the output of the commands it
//! ran) or from the files it left in the sandbox, never from what the agent's final prose claims:
//! a `UNMAPPED:` marker counts when it is in a spec file on disk.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::trials::{Definition, Measure};

/// A Bash call and what it printed.
struct Shell {
    command: String,
    output: Option<String>,
}

/// What the report reads from a stream-json run.
struct Run {
    tool_calls: usize,
    shells: Vec<Shell>,
    /// Paths given to `Write`, `Edit` and `MultiEdit`, in first-write order.
    written: Vec<PathBuf>,
}

fn result_text(content: &Value) -> String {
    match content {
        Value::String(text) => text.clone(),
        Value::Array(blocks) => blocks
            .iter()
            .filter_map(|block| block["text"].as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

fn parse(text: &str) -> Run {
    let mut seen = HashSet::new();
    let mut tool_calls = 0;
    let mut shells = Vec::new();
    let mut by_id = BTreeMap::new();
    let mut written = Vec::new();
    for line in text.lines() {
        let Ok(event) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        for block in event["message"]["content"].as_array().into_iter().flatten() {
            match block["type"].as_str() {
                Some("tool_use") => {
                    let id = block["id"].as_str().unwrap_or_default().to_owned();
                    if !id.is_empty() && !seen.insert(id.clone()) {
                        continue;
                    }
                    tool_calls += 1;
                    let input = &block["input"];
                    match block["name"].as_str() {
                        Some("Bash") => {
                            by_id.insert(id, shells.len());
                            shells.push(Shell {
                                command: input["command"].as_str().unwrap_or_default().to_owned(),
                                output: None,
                            });
                        }
                        Some("Write" | "Edit" | "MultiEdit") => {
                            if let Some(path) = input["file_path"].as_str() {
                                let path = PathBuf::from(path);
                                if !written.contains(&path) {
                                    written.push(path);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Some("tool_result") => {
                    let id = block["tool_use_id"].as_str().unwrap_or_default();
                    if let Some(&index) = by_id.get(id) {
                        shells[index].output = Some(result_text(&block["content"]));
                    }
                }
                _ => {}
            }
        }
    }
    Run {
        tool_calls,
        shells,
        written,
    }
}

/// Whether `ess specify validate` ran, and what its last output said.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Validate {
    /// Never ran.
    NotRun,
    /// Its last output was not `valid`.
    Invalid,
    /// Its last output said `valid`.
    Valid,
}

/// The counts of the last synthesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Synthesis {
    /// Scenarios synthesized.
    pub scenarios: u64,
    /// Refusals reported.
    pub refusals: u64,
}

/// The counts of the last `go test`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GoTest {
    /// Tests that passed.
    pub passed: u64,
    /// Tests that failed, and packages that did not build.
    pub failed: u64,
    /// Tests that were skipped.
    pub skipped: u64,
}

/// One run's numbers; a field is present exactly when the trial measures it. The same shape is a
/// trial's entry in `trials/baseline.json`.
///
/// `synthesis` and `go_test` have three states on purpose: absent (not measured), `null` (measured,
/// never ran) and counts; a baseline that had counts and a run with `null` got worse.
#[allow(clippy::option_option)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Measures {
    /// Tool calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<u64>,
    /// `ess specify validate`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validate: Option<Validate>,
    /// The last synthesis; `null` when none ran.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "measured"
    )]
    pub synthesis: Option<Option<Synthesis>>,
    /// `UNMAPPED:` markers in spec files the run wrote.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<u64>,
    /// The listed outputs that exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<BTreeSet<String>>,
    /// The last `go test`; `null` when none ran.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "measured"
    )]
    pub go_test: Option<Option<GoTest>>,
}

/// A field that is present is measured, even when its value is `null` (the command never ran);
/// an absent field is a measure the trial does not take.
#[allow(clippy::option_option)]
fn measured<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

fn is_validate(command: &str) -> bool {
    command.contains("specify validate") || command.contains("ess validate")
}

/// The verdict of one validate output: the last line that states one.
fn verdict(output: &str) -> Validate {
    let mut verdict = Validate::Invalid;
    for line in output.lines() {
        let line = line.trim();
        if line == "valid" || line.ends_with(", valid") || line.contains("\"valid\": true") {
            verdict = Validate::Valid;
        } else if line.contains(" was refused") || line.contains("\"valid\": false") {
            verdict = Validate::Invalid;
        }
    }
    verdict
}

/// The number right before `word` on `line`.
fn number_before(line: &str, word: &str) -> Option<u64> {
    let before = &line[..line.find(word)?];
    let digits: String = before
        .trim_end()
        .chars()
        .rev()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.chars().rev().collect::<String>().parse().ok()
}

/// `N scenario(s) … M refusal(s)` from the last synthesis that printed it.
fn synthesis(run: &Run) -> Option<Synthesis> {
    let mut last = None;
    for shell in run
        .shells
        .iter()
        .filter(|s| s.command.contains("synthesize"))
    {
        for line in shell.output.as_deref().unwrap_or_default().lines() {
            if let (Some(scenarios), Some(refusals)) = (
                number_before(line, "scenario(s)"),
                number_before(line, "refusal(s)"),
            ) {
                last = Some(Synthesis {
                    scenarios,
                    refusals,
                });
            }
        }
    }
    last
}

/// Counts of one `go test` output, from `-v` lines or `-json` events; `None` when it holds neither.
fn go_counts(output: &str) -> Option<GoTest> {
    let mut results: BTreeMap<String, &str> = BTreeMap::new();
    let mut broken = 0;
    for line in output.lines() {
        let trimmed = line.trim();
        if let Ok(event) = serde_json::from_str::<Value>(trimmed) {
            if let (Some(action), Some(test)) = (event["Action"].as_str(), event["Test"].as_str()) {
                if let Some(status) = ["pass", "fail", "skip"].into_iter().find(|s| *s == action) {
                    results.insert(test.to_owned(), status);
                }
            }
            continue;
        }
        for (prefix, status) in [
            ("--- PASS: ", "pass"),
            ("--- FAIL: ", "fail"),
            ("--- SKIP: ", "skip"),
        ] {
            if let Some(rest) = trimmed.strip_prefix(prefix) {
                let name = rest.rsplit_once(" (").map_or(rest, |(name, _)| name);
                results.insert(name.to_owned(), status);
            }
        }
        if trimmed.starts_with("FAIL")
            && (trimmed.ends_with("[build failed]") || trimmed.ends_with("[setup failed]"))
        {
            broken += 1;
        }
    }
    if results.is_empty() && broken == 0 {
        return None;
    }
    // A top-level test with subtests is their parent; its verdict repeats theirs.
    let parents: BTreeSet<&str> = results
        .keys()
        .filter_map(|name| name.split_once('/').map(|(top, _)| top))
        .collect();
    let mut counts = GoTest {
        failed: broken,
        ..GoTest::default()
    };
    for (name, status) in &results {
        if !name.contains('/') && parents.contains(name.as_str()) {
            continue;
        }
        match *status {
            "pass" => counts.passed += 1,
            "fail" => counts.failed += 1,
            _ => counts.skipped += 1,
        }
    }
    Some(counts)
}

fn go_test(run: &Run) -> Option<GoTest> {
    run.shells
        .iter()
        .filter(|shell| shell.command.contains("go test"))
        .rev()
        .find_map(|shell| go_counts(shell.output.as_deref()?))
}

/// `UNMAPPED:` markers in the YAML files the run wrote inside the sandbox, as they are on disk.
fn unmapped(run: &Run, sandbox: &Path, workdir: &Path) -> (u64, usize) {
    let mut markers = 0;
    let mut files = 0;
    for path in &run.written {
        let path = if path.is_absolute() {
            path.clone()
        } else {
            workdir.join(path)
        };
        let yaml = path
            .extension()
            .is_some_and(|extension| extension == "yaml" || extension == "yml");
        let inside = std::fs::canonicalize(&path).is_ok_and(|path| path.starts_with(sandbox));
        if !yaml || !inside {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(&path) {
            files += 1;
            markers += text.matches("UNMAPPED:").count() as u64;
        }
    }
    (markers, files)
}

fn present(path: &Path) -> bool {
    if path.is_dir() {
        std::fs::read_dir(path).is_ok_and(|mut entries| entries.next().is_some())
    } else {
        path.is_file()
    }
}

/// A run measured, with the lines that say so.
pub struct Report {
    /// The numbers.
    pub measures: Measures,
    /// One line per measure.
    pub lines: Vec<String>,
}

/// Measure a run. `definition` is `None` for an ad-hoc `PROMPT=` run, which gets every measure
/// that needs no definition.
pub fn measure(
    text: &str,
    sandbox: &Path,
    definition: Option<&Definition>,
) -> Result<Report, String> {
    let sandbox = std::fs::canonicalize(sandbox)
        .map_err(|error| format!("sandbox {}: {error}", sandbox.display()))?;
    let wants = |measure: Measure| {
        definition.map_or(measure != Measure::Outputs, |definition| {
            definition.measures(measure)
        })
    };
    let workdir = definition.map_or_else(|| sandbox.join("work"), |d| sandbox.join(d.workdir()));
    let run = parse(text);
    if run.tool_calls == 0 && !text.lines().any(|line| line.contains("\"type\"")) {
        return Err("no stream-json events: not a trial run".to_owned());
    }
    let mut measures = Measures::default();
    let mut lines = Vec::new();

    measures.tool_calls = Some(run.tool_calls as u64);
    lines.push(format!("tool calls: {}", run.tool_calls));

    if wants(Measure::Validate) {
        let runs: Vec<&Shell> = run
            .shells
            .iter()
            .filter(|s| is_validate(&s.command))
            .collect();
        let validate = runs.last().map_or(Validate::NotRun, |shell| {
            verdict(shell.output.as_deref().unwrap_or_default())
        });
        measures.validate = Some(validate);
        lines.push(match validate {
            Validate::NotRun => "validate: not run".to_owned(),
            Validate::Invalid => format!("validate: not valid (last of {} run(s))", runs.len()),
            Validate::Valid => format!("validate: valid (last of {} run(s))", runs.len()),
        });
    }
    if wants(Measure::Synthesis) {
        let synthesis = synthesis(&run);
        measures.synthesis = Some(synthesis);
        lines.push(synthesis.map_or_else(
            || "synthesis: not run".to_owned(),
            |s| {
                format!(
                    "synthesis: {} scenario(s), {} refusal(s)",
                    s.scenarios, s.refusals
                )
            },
        ));
    }
    if wants(Measure::Unmapped) {
        let (markers, files) = unmapped(&run, &sandbox, &workdir);
        measures.unmapped = Some(markers);
        lines.push(format!(
            "unmapped: {markers} `UNMAPPED:` marker(s) in {files} YAML file(s) the run wrote"
        ));
    }
    if let Some(definition) = definition.filter(|d| d.measures(Measure::Outputs)) {
        let mut have = BTreeSet::new();
        let mut missing = Vec::new();
        for (name, path) in &definition.outputs {
            if present(&workdir.join(path)) {
                have.insert(name.clone());
            } else {
                missing.push(format!("{name} ({path})"));
            }
        }
        let mut line = format!(
            "outputs: {}/{} present",
            have.len(),
            definition.outputs.len()
        );
        if !missing.is_empty() {
            let _ = write!(line, "; missing: {}", missing.join(", "));
        }
        lines.push(line);
        measures.outputs = Some(have);
    }
    if wants(Measure::GoTest) {
        let counts = go_test(&run);
        measures.go_test = Some(counts);
        lines.push(counts.map_or_else(
            || "go test: not run".to_owned(),
            |c| {
                format!(
                    "go test: {} passed, {} failed, {} skipped",
                    c.passed, c.failed, c.skipped
                )
            },
        ));
    }
    Ok(Report { measures, lines })
}

/// Every way `now` is worse than `then`. Only a measure both record is compared.
#[must_use]
pub fn worse(then: &Measures, now: &Measures) -> Vec<String> {
    let mut found = Vec::new();
    if let (Some(then), Some(now)) = (then.tool_calls, now.tool_calls) {
        if now * 2 > then * 3 {
            found.push(format!("tool calls: {then} → {now}, up by more than 50%"));
        }
    }
    if let (Some(Validate::Valid), Some(now)) = (then.validate, now.validate) {
        if now != Validate::Valid {
            found.push(format!("validate: was valid, now {now:?}"));
        }
    }
    if let (Some(Some(then)), Some(now)) = (then.synthesis, now.synthesis) {
        match now {
            None => found.push("synthesis: ran before, now did not".to_owned()),
            Some(now) if now.refusals > then.refusals => found.push(format!(
                "synthesis: refusals {} → {}",
                then.refusals, now.refusals
            )),
            Some(_) => {}
        }
    }
    if let (Some(then), Some(now)) = (&then.outputs, &now.outputs) {
        let lost: Vec<&str> = then.difference(now).map(String::as_str).collect();
        if !lost.is_empty() {
            found.push(format!("outputs: missing now: {}", lost.join(", ")));
        }
    }
    if let (Some(Some(then)), Some(now)) = (then.go_test, now.go_test) {
        match now {
            None => found.push("go test: ran before, now did not".to_owned()),
            Some(now) if now.failed > then.failed => found.push(format!(
                "go test: failures {} → {}",
                then.failed, now.failed
            )),
            Some(_) => {}
        }
    }
    found
}

/// Read `trials/baseline.json` (or another baseline file).
pub fn read_baseline(path: &Path) -> Result<BTreeMap<String, Measures>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("parsing {}: {error}", path.display()))
}

/// Record `measures` as `trial`'s baseline in `path`, keeping every other trial's entry.
pub fn write_baseline(path: &Path, trial: &str, measures: &Measures) -> Result<(), String> {
    let mut baseline = if path.exists() {
        read_baseline(path)?
    } else {
        BTreeMap::new()
    };
    baseline.insert(trial.to_owned(), measures.clone());
    let text = serde_json::to_string_pretty(&baseline).map_err(|error| error.to_string())?;
    std::fs::write(path, text + "\n")
        .map_err(|error| format!("writing {}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trials::Kind;

    fn tool(id: &str, name: &str, input: &str) -> String {
        format!(
            r#"{{"type":"assistant","message":{{"content":[{{"type":"tool_use","id":"{id}","name":"{name}","input":{input}}}]}}}}"#
        )
    }

    fn result(id: &str, text: &str) -> String {
        let content = serde_json::to_string(text).unwrap();
        format!(
            r#"{{"type":"user","message":{{"content":[{{"type":"tool_result","tool_use_id":"{id}","content":[{{"type":"text","text":{content}}}]}}]}}}}"#
        )
    }

    fn bash(id: &str, command: &str, output: &str) -> String {
        let input = serde_json::json!({ "command": command }).to_string();
        format!("{}\n{}", tool(id, "Bash", &input), result(id, output))
    }

    fn scratch(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "agentplugins-check-report-{label}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(path.join("work")).unwrap();
        std::fs::canonicalize(path).unwrap()
    }

    fn definition(measures: &[Measure], outputs: &[(&str, &str)]) -> Definition {
        Definition {
            name: "demo".to_owned(),
            kind: Kind::EssFullPackage,
            prompt: "p".to_owned(),
            dir: None,
            fixture: None,
            remote: false,
            seeded: false,
            setup: Vec::new(),
            measures: measures.to_vec(),
            outputs: outputs
                .iter()
                .map(|(name, path)| ((*name).to_owned(), (*path).to_owned()))
                .collect(),
        }
    }

    const INIT: &str = r#"{"type":"system","subtype":"init"}"#;

    #[test]
    fn validate_is_the_verdict_of_the_last_run() {
        let sandbox = scratch("validate");
        let valid = bash(
            "a",
            "ess specify validate --path spec",
            "garden v1 — 2 file(s), valid",
        );
        let refused = bash(
            "b",
            "ess specify validate --path spec",
            "spec was refused:\n  - domains/plot.yaml: missing field `type`",
        );
        let only = definition(&[Measure::ToolCalls, Measure::Validate], &[]);
        let run = [INIT, &refused, &valid].join("\n");
        let report = measure(&run, &sandbox, Some(&only)).unwrap();
        assert_eq!(report.measures.validate, Some(Validate::Valid));
        assert_eq!(report.lines[1], "validate: valid (last of 2 run(s))");
        let run = [INIT, &valid, &refused].join("\n");
        let report = measure(&run, &sandbox, Some(&only)).unwrap();
        assert_eq!(report.measures.validate, Some(Validate::Invalid));
        let json = bash(
            "c",
            "ess specify validate --format json",
            "{\n  \"valid\": true\n}",
        );
        let report = measure(&[INIT, &json].join("\n"), &sandbox, Some(&only)).unwrap();
        assert_eq!(report.measures.validate, Some(Validate::Valid));
        let report = measure(INIT, &sandbox, Some(&only)).unwrap();
        assert_eq!(report.measures.validate, Some(Validate::NotRun));
        std::fs::remove_dir_all(&sandbox).unwrap();
    }

    #[test]
    fn synthesis_counts_come_from_the_last_synthesize_output() {
        let sandbox = scratch("synthesis");
        let first = bash(
            "a",
            "ess verify conform synthesize --path spec --target go --out out/conformance",
            "refused: refusal[ESS-SYNTH-011]: …\n12 scenario(s) (0 authored), 5 refusal(s), 5 file(s) written to out",
        );
        let second = bash(
            "b",
            "cd spec && ess verify conform synthesize --target ir --out suite.json",
            "14 scenario(s) (0 authored), 2 refusal(s), 1 file(s) written to suite.json",
        );
        let unrelated = bash(
            "c",
            "echo '3 scenario(s), 9 refusal(s)'",
            "3 scenario(s), 9 refusal(s)",
        );
        let run = [INIT, &first, &second, &unrelated].join("\n");
        let report = measure(&run, &sandbox, None).unwrap();
        assert_eq!(
            report.measures.synthesis,
            Some(Some(Synthesis {
                scenarios: 14,
                refusals: 2
            }))
        );
        assert!(report
            .lines
            .contains(&"synthesis: 14 scenario(s), 2 refusal(s)".to_owned()));
        assert_eq!(report.measures.tool_calls, Some(3));
        std::fs::remove_dir_all(&sandbox).unwrap();
    }

    #[test]
    fn unmapped_markers_are_read_from_the_files_on_disk() {
        let sandbox = scratch("unmapped");
        let spec = sandbox.join("work/spec");
        std::fs::create_dir_all(&spec).unwrap();
        std::fs::write(
            spec.join("loans.yaml"),
            "# UNMAPPED: who may lend\n# UNMAPPED: overdue rule\ndomain: x\n",
        )
        .unwrap();
        std::fs::write(spec.join("system.yaml"), "format: ess/1\n").unwrap();
        std::fs::write(spec.join("notes.md"), "UNMAPPED: not a spec\n").unwrap();
        let write = |id: &str, file: &str| {
            let input = serde_json::json!({ "file_path": spec.join(file), "content": "UNMAPPED: in the prose only" });
            tool(id, "Write", &input.to_string())
        };
        let edit = tool(
            "d",
            "Edit",
            &serde_json::json!({ "file_path": spec.join("loans.yaml") }).to_string(),
        );
        let outside = tool(
            "e",
            "Write",
            &serde_json::json!({ "file_path": "/nonexistent/elsewhere.yaml" }).to_string(),
        );
        let run = [
            INIT,
            &write("a", "loans.yaml"),
            &write("b", "system.yaml"),
            &write("c", "notes.md"),
            &edit,
            &outside,
        ]
        .join("\n");
        let report = measure(&run, &sandbox, None).unwrap();
        assert_eq!(report.measures.unmapped, Some(2));
        assert!(report.lines.contains(
            &"unmapped: 2 `UNMAPPED:` marker(s) in 2 YAML file(s) the run wrote".to_owned()
        ));
        assert_eq!(report.measures.tool_calls, Some(5));
        std::fs::remove_dir_all(&sandbox).unwrap();
    }

    #[test]
    fn go_test_counts_leaf_tests_of_the_last_run() {
        let sandbox = scratch("gotest");
        let verbose = "=== RUN   TestConformance\n=== RUN   TestConformance/garden.plot.Plot/assign/assigned\n    --- PASS: TestConformance/garden.plot.Plot/assign/assigned (0.00s)\n    --- PASS: TestConformance/garden.plot.Plot (0.00s)\n    --- SKIP: TestConformance/garden.plot.Release/redeliver (0.00s)\n    --- FAIL: TestConformance/garden.plot.Plot/invariant (0.01s)\n--- FAIL: TestConformance (0.02s)\n--- PASS: TestStore (0.00s)\nFAIL\nFAIL\texample.com/garden/impl\t0.031s";
        let broken = bash(
            "a",
            "go test ./...",
            "# example.com/garden/impl\nimpl/target.go:9:2: undefined: essconform.Tagret\nFAIL\texample.com/garden/impl [build failed]",
        );
        let listed = bash(
            "c",
            "go test -list . ./impl",
            "TestConformance\nok  \texample.com/garden/impl\t0.002s",
        );
        let only = definition(&[Measure::ToolCalls, Measure::GoTest], &[]);
        let run = [
            INIT,
            &broken,
            &bash("b", "cd impl && go test -v ./...", verbose),
            &listed,
        ]
        .join("\n");
        let report = measure(&run, &sandbox, Some(&only)).unwrap();
        assert_eq!(
            report.measures.go_test,
            Some(Some(GoTest {
                passed: 3,
                failed: 1,
                skipped: 1
            }))
        );
        assert_eq!(report.lines[1], "go test: 3 passed, 1 failed, 1 skipped");
        let report = measure(&[INIT, &broken].join("\n"), &sandbox, Some(&only)).unwrap();
        assert_eq!(
            report.measures.go_test,
            Some(Some(GoTest {
                passed: 0,
                failed: 1,
                skipped: 0
            }))
        );
        let json = "{\"Action\":\"run\",\"Test\":\"TestConformance\"}\n{\"Action\":\"pass\",\"Test\":\"TestConformance/a\"}\n{\"Action\":\"skip\",\"Test\":\"TestConformance/b\"}\n{\"Action\":\"pass\",\"Test\":\"TestConformance\"}";
        let report = measure(
            &[INIT, &bash("d", "go test -json ./...", json)].join("\n"),
            &sandbox,
            Some(&only),
        )
        .unwrap();
        assert_eq!(
            report.measures.go_test,
            Some(Some(GoTest {
                passed: 1,
                failed: 0,
                skipped: 1
            }))
        );
        std::fs::remove_dir_all(&sandbox).unwrap();
    }

    #[test]
    fn outputs_are_checked_on_disk_and_only_when_listed() {
        let sandbox = scratch("outputs");
        let out = sandbox.join("work/out");
        std::fs::create_dir_all(out.join("schema")).unwrap();
        std::fs::write(out.join("schema/a.json"), "{}").unwrap();
        std::fs::create_dir_all(out.join("site")).unwrap();
        std::fs::write(out.join("openapi.yaml"), "openapi: 3.1.0").unwrap();
        let listed = definition(
            &[Measure::ToolCalls, Measure::Outputs],
            &[
                ("schema", "out/schema"),
                ("openapi", "out/openapi.yaml"),
                ("site", "out/site"),
                ("asyncapi", "out/asyncapi"),
            ],
        );
        let report = measure(INIT, &sandbox, Some(&listed)).unwrap();
        assert_eq!(
            report.measures.outputs,
            Some(["openapi".to_owned(), "schema".to_owned()].into())
        );
        assert_eq!(
            report.lines[1],
            "outputs: 2/4 present; missing: asyncapi (out/asyncapi), site (out/site)"
        );
        assert_eq!(report.lines.len(), 2, "unlisted measures print nothing");
        let adhoc = measure(INIT, &sandbox, None).unwrap();
        assert!(adhoc.measures.outputs.is_none());
        assert!(measure("not json", &sandbox, None).is_err());
        std::fs::remove_dir_all(&sandbox).unwrap();
    }

    #[test]
    fn a_duplicated_tool_use_is_counted_once() {
        let sandbox = scratch("dupes");
        let call = tool("same", "Read", "{}");
        let report = measure(&[INIT, &call, &call].join("\n"), &sandbox, None).unwrap();
        assert_eq!(report.measures.tool_calls, Some(1));
        std::fs::remove_dir_all(&sandbox).unwrap();
    }

    fn full() -> Measures {
        Measures {
            tool_calls: Some(40),
            validate: Some(Validate::Valid),
            synthesis: Some(Some(Synthesis {
                scenarios: 12,
                refusals: 2,
            })),
            unmapped: Some(0),
            outputs: Some(["go".to_owned(), "schema".to_owned()].into()),
            go_test: Some(Some(GoTest {
                passed: 10,
                failed: 0,
                skipped: 2,
            })),
        }
    }

    #[test]
    fn the_same_or_better_is_not_worse() {
        let mut better = full();
        better.tool_calls = Some(60);
        better.unmapped = Some(4);
        better.synthesis = Some(Some(Synthesis {
            scenarios: 9,
            refusals: 1,
        }));
        better.go_test = Some(Some(GoTest {
            passed: 1,
            failed: 0,
            skipped: 11,
        }));
        assert!(worse(&full(), &full()).is_empty());
        assert!(
            worse(&full(), &better).is_empty(),
            "{:?}",
            worse(&full(), &better)
        );
        assert!(worse(&Measures::default(), &full()).is_empty());
    }

    #[test]
    fn every_listed_regression_is_worse() {
        let now = Measures {
            tool_calls: Some(61),
            validate: Some(Validate::NotRun),
            synthesis: Some(Some(Synthesis {
                scenarios: 12,
                refusals: 3,
            })),
            unmapped: Some(0),
            outputs: Some(["schema".to_owned()].into()),
            go_test: Some(Some(GoTest {
                passed: 9,
                failed: 1,
                skipped: 2,
            })),
        };
        let found = worse(&full(), &now);
        assert_eq!(found.len(), 5, "{found:?}");
        for expected in [
            "tool calls: 40 → 61",
            "validate: was valid, now NotRun",
            "refusals 2 → 3",
            "missing now: go",
            "failures 0 → 1",
        ] {
            assert!(
                found.iter().any(|line| line.contains(expected)),
                "{expected}: {found:?}"
            );
        }
        let gone = Measures {
            synthesis: Some(None),
            go_test: Some(None),
            ..full()
        };
        let found = worse(&full(), &gone);
        assert_eq!(found.len(), 2, "{found:?}");
    }

    #[test]
    fn a_baseline_round_trips_and_keeps_other_trials() {
        let sandbox = scratch("baseline");
        let path = sandbox.join("baseline.json");
        std::fs::write(&path, "{\"other\": {\"tool_calls\": 3}}\n").unwrap();
        write_baseline(&path, "demo", &full()).unwrap();
        let read = read_baseline(&path).unwrap();
        assert_eq!(read["demo"], full());
        assert_eq!(read["other"].tool_calls, Some(3));
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("\"validate\": \"valid\""), "{text}");
        let never = Measures {
            synthesis: Some(None),
            go_test: Some(None),
            ..Measures::default()
        };
        write_baseline(&path, "never", &never).unwrap();
        assert_eq!(read_baseline(&path).unwrap()["never"], never);
        std::fs::remove_dir_all(&sandbox).unwrap();
    }
}
