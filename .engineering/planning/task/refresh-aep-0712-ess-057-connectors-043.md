---
format: aep.planning-md/3
id: task:refresh-aep-0712-ess-057-connectors-043
kind: task
status: implemented
title: Track AEP 0.71.2, ESS 0.57.0 and Connectors v0.43.0
summary: the skills follow AEP 0.71.2, ESS 0.57.0 and Connectors v0.43.0, and verified.json moves
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-10T11:20:49Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-10T11:20:49Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-10T18:37:19Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":3}}}
---
## Context

The scheduled Tools check on `main` (58d14a9) failed: AEP 0.71.2, ESS 0.57.0 and Connectors
v0.43.0 are newer than `verified.json` (aep 0.71.0, ess 0.56.0, connectors v0.39.0).
`agentplugins-check upstream` also reports the eval AEP pin 0.68.0, the eval ESS pin 0.56.0, the
eval Connectors pin v0.39.0, the generated docs-system and website workflow pins, and
beyond10x/aep#60 closed.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at aep 0.71.2, ess 0.57.0 and connectors
v0.43.0, with runtime help verified against a `connectors` 0.43.0 binary built from the tag. Every
consumer-visible entry of the AEP 0.71.1 and 0.71.2, ESS 0.57.0 and Connectors v0.40.0 to v0.43.0
changelogs is reflected in its owning resource or recorded under Classification as needing no
change. The isolated `ess-tutorial` trial passes against `trials/baseline.json`. The pull
request's `Gate` and `Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, plugins/aep/**, plugins/ess/**,
plugins/connectors/**, website/docs/**, trials/**/fixture/**, fixtures/**, CHANGELOG.md,
Cargo.toml, Cargo.lock, every plugin manifest and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| AEP 0.71.1: `evidence --from <report/2> --suite` records an ordinary suite (no `coverage`) | `aep:planning` | added; reproduced: 0.71.0 refuses an ESS 0.57.0 ordinary suite with `MissingField at $suite.coverage`, 0.71.2 records `ess_conformance_v2` |
| AEP 0.71.2: report with `producer_profile: go-scenario-status/2` | `aep:planning` | added from the changelog; no Go or Node runner was run |
| AEP 0.71.1: `store migrate git` repairs a repeated transition on `aep.project/5` | `aep:upgrade` | added; reproduced on a scratch store (`--dry-run`, `--verify`, then `validate` valid) |
| AEP 0.71.1: migration of a journal holding a move twice carries it once | none | the migration route is unchanged |
| ESS 0.57.0 `ess-diff/15`–`/17` | `ess:hardening` `spec-diff.md` | added; each reproduced with exit codes |
| ESS 0.57.0 `ess-conformance/46`, `/47` constrained `String` responses | `later-formats.md`, `ess:testing-conformance`, `ess:upgrade` | the old limit removed; the record-invariant refusal kept; `/46` reproduced, `/47` not |
| ESS 0.57.0 `path_segment_wire_name`, `when: true` refusal (`ESS-COMMAND-004`) | `syntax.md`, `ess:upgrade` | added; reproduced |
| ESS 0.57.0 interpreted target decides `now` guards; Go/TS runners admit `json` | `ess:testing-conformance` | added; the `now` guard reproduced, `json` admission not |
| ESS 0.57.0 `ess-mutation-report/5` `identical_answer`, `when_subject:` equivalents | `ess:hardening`, `techniques.md` | added; the class name `outcome-order-flip` corrected to `precedence-swap` |
| ESS 0.57.0 `ess-cli/2`, `--component --scenarios` `outside:`, synthesis fixes, internal changes | none | no skill teaches these surfaces or documented a workaround for them |
| ESS 0.57.0 tutorials and fixtures | ESS and AEP tutorial pages, trial fixtures, drafted fixture | require ESS 0.57.0; crates at tag 0.57.0 |
| Connectors v0.40.0–v0.43.0 verified release named in prose | `connectors:integrating`, `setup.md`, `connectors:init`, `connectors:upgrade`, plugin page, eval pin | v0.43.0; Rust 1.91 unchanged; source only (no release assets) |
| Connectors v0.41.0–v0.43.0 selection members and the GitLab amendment `correct_path` | `setup.md`, plugin page | upgrade every binary that builds or loads the adapter first (refusal inferred from v0.39.0's strict field parsing) |
| Connectors GitLab, Jira and SQL operations added; guide configuration revisions change | `setup.md`, plugin page | permit new operations only when authorized; recover a new revision as step 4 says |
| Connectors MySQL, Prometheus through Grafana, catalog operations | none | adapter-specific; the skill defers to each adapter's guide and `adapters describe` |
| Connectors CLI contract | none | `apps/connectors/spec/cli.yaml` unchanged from v0.39.0 to v0.43.0 |
| eval AEP 0.68.0 behind 0.71.2 | none | Metaharness 0.9.3 (newest) links AEP 0.68.0; the pair moves together |
| planning `protocols` pin | `.engineering/project.yaml` | moves to the 0.71.2 tag commit 2b840f7; no protocol document changed |
| Worktree 0.15.0: archives leave cargo build layout out (`worktree.archive/3`), `prune-archives --strip-build-output` | `worktree:managing-worktrees`, plugin page, eval pin | the two changed paragraphs of `worktree skill` 0.15.0 merged by hand (the skill is curated); `prune-archives --help` of the 0.15.0 binary checked |
| workflow pins docs-system, website | none | generated; Atlas reconciliation owns them |
| beyond10x/aep#60 closed | none | cited only in the dated tutorial |

## Verification

- aep 0.71.2 and ess 0.57.0 from their release archives, checked against `SHA256SUMS`; connectors
  0.43.0 built from tag commit 0e5fd63 with `cargo build --locked --release -p connectors`;
  `--version` prints `connectors 0.43.0`.
- `AGENTPLUGINS_CONNECTORS_BINARY=<that binary> agentplugins-check tools`: exit 0; aep 0.71.2 222
  spelled commands, ess 0.57.0 91, worktree 0.14.0 49, metaharness 0.9.3 13, connectors v0.43.0 21
  against the released source contract at 0e5fd63, runtime help additionally verified, on 6c5f97d.
- `task trial:run TRIAL=ess-tutorial`: isolated (1 session, plugins b10x and ess from the
  sandbox, sandbox `ess` 0.57.0); 73 tool calls (baseline 67), validate valid, 17 scenarios 0
  refusals, 5/5 outputs, cargo test 2 passed 0 failed; no measure worse than
  `trials/baseline.json`.
- The ESS tutorial's planted step-7 defect fails 1 of 17 scenarios on the 0.57.0 crates and passes
  17 of 17 restored.
- `task check` and `task site-build`: exit 0 on 8e10638.
- Worktree 0.15.0, released during the first CI run: `agentplugins-check tools` exit 0 on dd1c6ed
  with worktree 52 spelled commands; isolated `worktree-onboarding` trial (sandbox `worktree`
  0.15.0): 10 tool calls, no measure worse than `trials/baseline.json`.

## Delivered

- https://github.com/beyond10x/agentplugins/pull/78 (merge 5b567be)
- https://github.com/beyond10x/agentplugins/releases/tag/0.22.6
