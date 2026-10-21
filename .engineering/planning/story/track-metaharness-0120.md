---
format: aep.planning-md/3
id: story:track-metaharness-0120
kind: story
status: implemented
title: Track Metaharness 0.12.0, the Claude Code it pins and AEP 0.76.0
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T20:23:32Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T20:23:32Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:59Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

Metaharness 0.12.0 (2026-10-21) is newer than the 0.10.1 that `story:track-metaharness-0101` moved to.
0.11.0 and 0.12.0 carry breaking changes (`--context` needs `--cwd`, `--strict-version` refuses an
off-pin binary, `--catalogue` on `run claude`, scratch roots must be UTF-8) and pin Claude Code 2.1.304
and 2.1.305 (`crates/metaharness-claude/src/lib.rs:183` at tag 0.12.0). The release links aep 0.68.1
(its `Cargo.lock`), so `AEP_VERSION` stays 0.68.1.

AEP 0.76.0 (2026-10-21) is newer than `verified.json` (0.75.0); it fixes repository protocol sources on
macOS and changes no command the skills spell.

## Acceptance

`verified.json` names metaharness 0.12.0 and aep 0.76.0; `.github/workflows/eval.yml` names metaharness
0.12.0, `AEP_VERSION` the aep that release links and `CLAUDE_VERSION` a version that release pins;
skill text that quotes a changed metaharness command matches its `--help`; `agentplugins-check tools`
reports no metaharness or aep problem.

## Scope

`verified.json`, `.github/workflows/eval.yml`, metaharness spellings in `plugins/**`.
