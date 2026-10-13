---
format: aep.planning-md/3
id: task:refresh-worktree-018-connectors-048
kind: task
status: implemented
title: Track Worktree 0.18.0 and Connectors v0.48.0
summary: the skills and b10x follow Worktree 0.18.0 and Connectors v0.48.0, and verified.json moves
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-13T13:36:59Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-13T13:36:59Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-13T14:46:04Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":4}}}
---
## Context

Worktree 0.16.0 to 0.18.0 and Connectors v0.46.0 to v0.48.0 were published after release 0.22.8,
which verified the skills against Worktree 0.15.0 and Connectors v0.45.0. The scheduled Tools check
is red on both. AEP 0.71.2, ESS 0.59.0 and Metaharness 0.9.3 are still the newest releases.
`agentplugins-check upstream` also reports the eval AEP pin 0.68.0 behind 0.71.2, the eval
Worktree pin, the generated docs-system and website workflow pins, the gates workflow pin behind
gates `main`, and https://github.com/beyond10x/aep/issues/60 closed.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at worktree 0.18.0 and connectors v0.48.0,
with connectors runtime help verified against the released `connectors` 0.48.0 binary. Every
consumer-visible entry of the Worktree 0.16.0 to 0.18.0 and Connectors v0.46.0 to v0.48.0
changelogs is reflected in its owning resource or recorded under Classification as needing no
change. `b10x` plans a prebuilt archive for a Connectors release that carries one, and still
plans `cargo install` for an older source-only release. The isolated `ess-tutorial` trial measures
no worse than `trials/baseline.json`. The pull request's `Gate` and `Tools` checks pass.

## Scope

Cited: verified.json, catalog.json, crates/b10x/**, .github/workflows/eval.yml, plugins/worktree/**,
plugins/connectors/**, plugins/b10x/**, website/docs/**, SETUP.md, AGENTS.md, CHANGELOG.md,
Cargo.toml, Cargo.lock, every plugin manifest and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| Worktree 0.18.0 every ignored entry disposable unless `.worktreekeep` keeps it; `ignored-output`; `keep-file-invalid`; `cache-path-not-utf8` | `worktree:managing-worktrees`, `aep:implementing` `wave.md`, governed-plan tutorial and its trial fixture, worktree plugin page | revised; the old text said finish deletes only recognised cache and keeps records under `target/` |
| Worktree 0.17.1 and 0.18.0 nested repositories, mount points and unreadable entries retained; a directory holding one falls to its entries | `worktree:managing-worktrees` | added |
| Worktree 0.16.0 and 0.17.0 Flutter `build/` and `.dart_tool/` as build output | `worktree:managing-worktrees`, plugin page | added |
| Worktree 0.18.0 `archive` reads the keep file; `prune-archives --strip-build-output` strips ignored output | `worktree:managing-worktrees` | added |
| Worktree `ScannedPatch`, classifier and re-observation internals; `cache-state-changed`; protocol 4 and archive format 3 unchanged | none | internal, or no agent action |
| This repository's own records in ignored paths | none | trials write under `/var/tmp`, the checker under the temporary directory; no `.worktreekeep` needed |
| Connectors v0.48.0 prebuilt archives for four targets with `SHA256SUMS` | `catalog.json` archive route, `b10x` plan tests, `agentplugins-check tools`, eval workflow, connectors init, upgrade and integrating skills, install and plugin pages, AGENTS.md | revised; a source-only release still plans cargo |
| Connectors v0.48.0 CLI and host on macOS, weaker platform guarantees, unsigned binary and quarantine | connectors skills, plugin page | added |
| Connectors v0.46.0 custody backends per platform; `setup check` `os` and `custody_backend` | integrating skill, `setup.md`, plugin page | added |
| Connectors v0.47.0 Slack `chat.postMessage` and `conversations.open` under approval | `setup.md`, `writes-and-services.md` | added; the configuration revision change is inferred from earlier selection growth |
| Connectors v0.47.0 `{"boolean": …}` guards, `body_nonempty`; a `{"literal": "true"}` guard refused at load | `setup.md`, plugin page | added with the migration |
| Connectors v0.46.0 ESS 0.59.0 crate bump; macOS CI; Linux-only tests | none | upstream internals |
| eval AEP 0.68.0 behind 0.71.2 | none | Metaharness 0.9.3 (newest) links AEP 0.68.0; the pair moves together |
| gates workflow pin 1a5ede1 behind gates `main` | none | `.github/workflows/common.yml` is unchanged in that range |
| workflow pins docs-system, website | none | generated; Atlas reconciliation owns them |
| https://github.com/beyond10x/aep/issues/60 closed | none | cited only in the dated tutorial |

## Verification

- worktree 0.18.0 and connectors 0.48.0 from their release archives, each checked against its
  `SHA256SUMS`; nothing installed.
- `agentplugins-check tools`: exit 0; aep 0.71.2 222 spelled commands, ess 0.59.0 94 (the tutorial
  regenerated and its cargo test 2 passed), worktree 0.18.0 52, metaharness 0.9.3 13, connectors
  v0.48.0 21 against the released source contract at 3fc5001 with the release binary's runtime
  help verified.
- `cargo test -p b10x`: 56 passed (the archive-route test failed first with `Cargo` for
  `Prebuilt`); `cargo test -p agentplugins-check`: 118 and 2 passed; clippy with `-D warnings`
  clean.
- `task trial:run TRIAL=ess-tutorial NAME=ess-tutorial-0229`: isolated (1 session, b10x and ess
  0.22.9); 59 tool calls (baseline 67), validate valid, 17 scenarios 0 refusals, 5/5 outputs, cargo
  test 2 passed; no measure worse than `trials/baseline.json`.
- `task trial:run TRIAL=worktree-onboarding NAME=worktree-onboarding-0229`: isolated (b10x and
  worktree 0.22.9, sandbox `worktree` 0.18.0 installed by `b10x`); 11 tool calls; no measure worse
  than the baseline.
- Candidates from the trials, not acted on: the ESS tutorial does not show `domains/lending.yaml`,
  stays on `ess/23` while `ess/24` is newest, does not say the adapter trait needs
  `redeliver_event` and `configure_external_outcome` stubs, and does not say a `ScenarioContext`
  needs a well-formed scenario id; `worktree:managing-worktrees` step 1 leaves the choice between
  archive and hand-off open when a task stops at a local commit.
