//! The `connectors-readiness` command contract must accept the diagnosis the skill now spells.
//!
//! `connectors:integrating` step 2 (and `connectors:init`) tell an agent to run
//! `connectors --output json setup check`, then `connectors --output json instances list`, and
//! step 3 inspects with `instances describe` / `instances status`. The live eval
//! (`.github/workflows/eval.yml`) runs `agentplugins-check evals check-stream` on every stream of
//! `evals/connectors-readiness`, whose contract checks each `connectors` command a run requested.
//! A run that follows the skill must not be refused as a non-diagnostic command.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate is under the repository root")
        .to_path_buf()
}

/// Run `evals check-stream evals/connectors-readiness` on a stream of the given commands and
/// return its standard error.
fn check_stream(name: &str, commands: &[&str]) -> String {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("readiness-skill-inventory");
    std::fs::create_dir_all(&dir).unwrap();
    let stream = dir.join(format!("{name}.events.jsonl"));
    let lines: Vec<String> = commands
        .iter()
        .enumerate()
        .map(|(seq, command)| {
            serde_json::json!({
                "format": "metaharness.event/1",
                "seq": seq,
                "event": "tool.requested",
                "name": "Bash",
                "input": {"command": command},
            })
            .to_string()
        })
        .collect();
    std::fs::write(&stream, lines.join("\n")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_agentplugins-check"))
        .current_dir(root())
        .args(["evals", "check-stream", "evals/connectors-readiness"])
        .arg(&stream)
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stderr).into_owned()
}

const REFUSED: &str = "non-diagnostic connectors command requested";

#[test]
fn the_contract_refuses_a_real_write_so_the_assertion_below_can_fail() {
    let stderr = check_stream(
        "control-write",
        &[
            "connectors --output json setup check",
            "connectors --output json connections revoke --connector c --instance i",
        ],
    );
    assert!(stderr.contains(REFUSED), "{stderr}");
}

#[test]
fn the_skill_s_instance_inventory_is_accepted_as_diagnosis() {
    for (name, command) in [
        ("list", "connectors --output json instances list"),
        (
            "describe",
            "connectors --output json instances describe --connector 'c' --instance 'i'",
        ),
        (
            "status",
            "connectors --output json instances status --connector 'c' --instance 'i'",
        ),
    ] {
        let stderr = check_stream(
            &format!("skill-{name}"),
            &["connectors --output json setup check", command],
        );
        assert!(
            !stderr.contains(REFUSED),
            "`{command}` (spelled by connectors:integrating) is refused by the readiness contract: {stderr}"
        );
    }
}
