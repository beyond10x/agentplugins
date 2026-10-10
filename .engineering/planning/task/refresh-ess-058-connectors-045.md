---
format: aep.planning-md/3
id: task:refresh-ess-058-connectors-045
kind: task
status: active
title: Track ESS 0.58.0 and Connectors v0.45.0
summary: the skills follow ESS 0.58.0 and Connectors v0.45.0, and verified.json moves
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-10T18:37:07Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-10T18:37:07Z", actor: "human:timo", revision: 3}
---
## Context

ESS 0.58.0 and Connectors v0.44.0 and v0.45.0 were published after release 0.22.6, which verified
the skills against ESS 0.57.0 and Connectors v0.43.0. `agentplugins-check upstream` reports both
releases, the eval ESS pin 0.57.0, the eval Connectors pin v0.43.0, the eval AEP pin 0.68.0 behind
0.71.2, the generated docs-system and website workflow pins, and beyond10x/aep#60 closed.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at ess 0.58.0 and connectors v0.45.0, with
runtime help verified against a `connectors` 0.45.0 binary built from the tag. Every
consumer-visible entry of the ESS 0.58.0 and Connectors v0.44.0 and v0.45.0 changelogs is
reflected in its owning resource or recorded under Classification as needing no change. The
isolated `ess-tutorial` trial passes against `trials/baseline.json`. The pull request's `Gate` and
`Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, plugins/ess/**, plugins/connectors/**,
website/docs/**, trials/**/fixture/**, fixtures/**, CHANGELOG.md, Cargo.toml, Cargo.lock, every
plugin manifest and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| ESS 0.58.0 `ess/24` `undeclared_fields: ignored` on a struct or command response; its validate refusals | `ess:specifying` (SKILL, `syntax.md`, `later-formats.md`), `ess:retrofitting`, plugin page | added; reproduced (valid, `ESS-COMMAND-009`/`ESS-TYPE-009` under `ess/23`, `ESS-COMMAND-005`, `ESS-EVENT-009`, `ESS-ERROR-009`) |
| ESS 0.58.0 `ess generate` writes `additionalProperties: true` at opened objects | `later-formats.md` | added; schema and OpenAPI reproduced, AsyncAPI not |
| ESS 0.58.0 `ess-diff/18` change ids and ratings | `ess:hardening` `spec-diff.md`, `ess:upgrade` | added; opening (exit 4 under `--fail-on breaking-or-unknown`) and closing a response reproduced; struct input/stored closing from the changelog |
| ESS 0.58.0 `ess-conformance/48` (`/49`); observers admit undeclared keys only at opened objects | `ess:testing-conformance`, `ess:upgrade` | added; synthesis and an interpreted run reproduced |
| ESS 0.58.0 synthesis refuses an opened one-time response, retained-result replay, complete-subject snapshot | `later-formats.md` limits table | added; the first two reproduced (`ESS-SYNTH-001`), the snapshot from the changelog |
| ESS 0.58.0 authored literals compared by declared fields at an opened struct | none | runner internals; no instruction changes |
| ESS 0.58.0 `ESS-AUTHOR-041` for two acts the precedence plan answers otherwise; external witness change | `ess:upgrade` | added from the changelog, not reproduced |
| ESS 0.58.0 tutorials and fixtures | ESS and AEP tutorial pages, trial fixtures, drafted fixture | require ESS 0.58.0; crates at tag 0.58.0 (3323fa2) |
| Connectors v0.44.0 GitLab writes (42 to 51 operations, new revision); v0.45.0 Slack 7 to 10 and Jira 8 to 9 | `setup.md`, plugin page | re-copy the revision from `--print-local-bootstrap`; add operations only when authorized |
| Connectors v0.44.0 `correct_media_type`; v0.45.0 binary and `download` selections | `setup.md`, plugin page | upgrade every binary that builds or loads the adapter first (inferred from strict field parsing) |
| Connectors v0.45.0 catalog `hosts` | `setup.md`, plugin page | named; adding it changes the revision |
| Connectors v0.45.0 Kubernetes `pod_logs`, `kubeconfig`, `pod_exec` | `setup.md`, plugin page | named; pod exec is a write on `connectors-private/2` needing an approval policy |
| Connectors v0.44.0 Kubernetes reads and kinds; SDK `AuthenticatedWrite::upgrade` | none | additive or library-facing; the skill defers to each adapter's guide |
| Connectors CLI contract | none | nothing under `apps/connectors` changed from v0.43.0 to v0.45.0; Rust 1.91 unchanged; source only |
| eval AEP 0.68.0 behind 0.71.2 | none | Metaharness 0.9.3 (newest) links AEP 0.68.0; the pair moves together |
| workflow pins docs-system, website | none | generated; Atlas reconciliation owns them |
| beyond10x/aep#60 closed | none | cited only in the dated tutorial |

## Verification

- ess 0.58.0 from its release archive, checked against `SHA256SUMS`; connectors 0.45.0 built from
  tag commit 94e0805 with `cargo build --locked --release -p connectors`; `--version` prints
  `connectors 0.45.0`.
- `AGENTPLUGINS_CONNECTORS_BINARY=<that binary> agentplugins-check tools`: exit 0; aep 0.71.2 222
  spelled commands, ess 0.58.0 93, worktree 0.15.0 52, metaharness 0.9.3 13, connectors v0.45.0 21
  against the released source contract at 94e0805, runtime help additionally verified.
- `task trial:run TRIAL=ess-tutorial`: isolated (1 session, plugins b10x and ess 0.22.7 from the
  sandbox, sandbox `ess` 0.58.0); 73 tool calls (baseline 67, as in the previous round), validate
  valid, 17 scenarios 0 refusals, 5/5 outputs, cargo test 2 passed 0 failed; no measure worse than
  `trials/baseline.json`.
- `task site-build`: exit 0.
