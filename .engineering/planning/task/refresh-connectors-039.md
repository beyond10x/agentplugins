---
format: aep.planning-md/3
id: task:refresh-connectors-039
kind: task
status: implemented
title: Track Connectors v0.39.0
summary: the connectors skills follow v0.39.0, its credential-selection compatibility is named, and verified.json moves
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-09T16:11:23Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-09T16:11:23Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-10T11:20:48Z", actor: "human:timo", revision: 6}
---
## Context

The scheduled Tools check on `main` (29a4c3e) failed: Connectors v0.39.0 (2026-10-09) is newer
than `verified.json`'s v0.38.0. `agentplugins-check upstream` also reports the eval AEP pin 0.68.0
behind AEP 0.71.0 and the generated docs-system and website workflow pins.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at connectors v0.39.0, with runtime help
verified against a `connectors` 0.39.0 binary built from the tag. Every connectors skill and the
plugin page name v0.39.0 where they named the verified release. Every consumer-visible entry of
the v0.39.0 changelog is reflected in its owning resource or recorded under Classification as
needing no change. The isolated `ess-tutorial` trial passes. The pull request's `Gate` and
`Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, plugins/connectors/**,
website/docs/plugins/connectors.md, CHANGELOG.md, Cargo.toml, Cargo.lock, every plugin manifest
and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| verified release named in prose (was v0.37.0) | `connectors:integrating`, `setup.md`, `connectors:init`, `connectors:upgrade`, plugin page | v0.39.0; Rust 1.91 unchanged (`apps/connectors/Cargo.toml` at the tag) |
| catalog selection `credential`; older readers refuse it | `setup.md`, plugin page | upgrade every binary that loads the adapter first |
| `connectors-grafana` adapter, Loki through Grafana | none | adapter-specific; the skill defers to each adapter's guide and `adapters describe` |
| Jira and Slack catalog reads, `slack.user` profile | none | discovered through `operations list`/`describe`; no command change |
| documentation served at its own address | none | the skills link release-tagged source documents, not the site |
| eval AEP 0.68.0 behind 0.71.0 | none | Metaharness 0.9.3 (newest) links AEP 0.68.0; the pair moves together |
| workflow pins docs-system, website | none | generated; Atlas reconciliation owns them |
| beyond10x/aep#60 closed | none | cited only in the dated tutorial, which already says AEP 0.65.0 fixed it |

## Verification

- connectors v0.39.0 built from tag commit 1bfb2d2 with `cargo build --locked --release -p
  connectors`; `--version` prints `connectors 0.39.0`.
- `AGENTPLUGINS_CONNECTORS_BINARY=<that binary> agentplugins-check tools`: exit 0; connectors
  v0.39.0 21 spelled commands checked against the released source contract at 1bfb2d2, runtime
  help additionally verified.
- `task trial:run TRIAL=ess-tutorial`: isolated (1 session, plugins b10x and ess at 0.22.4 from the
  sandbox); 85 tool calls (baseline 67), validate valid, 17 scenarios 0 refusals, 5/5 outputs,
  cargo test 2 passed 0 failed; no measure worse than `trials/baseline.json`.
- `task check` and `task site-build`: exit 0 on 2f2ccc5.

## Delivered

- https://github.com/beyond10x/agentplugins/pull/77 (merge 58d14a9)
- https://github.com/beyond10x/agentplugins/releases/tag/0.22.5
