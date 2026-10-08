---
format: aep.planning-md/3
id: task:refresh-connectors-033
kind: task
status: implemented
title: Track Connectors v0.33.0; operations by family needs an adapter
summary: connectors:integrating names --adapter on operations list --family; the skills follow v0.33.0 and verified.json moves
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T07:40:34Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-08T07:40:34Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-08T07:40:34Z", actor: "human:timo", revision: 5}
---
## Context

`agentplugins-check upstream` on 2026-10-08 reported Connectors v0.33.0 (pinned v0.32.0) and the
eval AEP pin 0.68.0 behind AEP 0.69.0. `connectors:integrating` showed
`connectors --output json operations list --family '<contract id>'` as finding operations across
adapters; on v0.33.0 that command exits 2 with `cli_parse`, because the family filter narrows one
adapter's cached description and requires `--adapter`. Entity Runtime 0.30.2 also shipped; no
skill, catalog entry or pin here names Entity Runtime.

## Acceptance

`connectors:integrating` spells `operations list --adapter '<alias>' --family '<contract id>'`,
and every command and flag the connectors skills spell exists in `connectors --help` of v0.33.0.
`agentplugins-check tools` exits 0 with `verified.json` at connectors v0.33.0. Every
consumer-visible entry of the v0.33.0 changelog is reflected in its owning resource or recorded
under Classification as needing no change. The pull request's `Gate` and `Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, plugins/connectors/**,
website/docs/plugins/connectors.md, CHANGELOG.md, Cargo.toml, Cargo.lock, every plugin manifest
and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| `operations list --family` requires `--adapter` | `connectors:integrating` | command corrected; per-adapter search named |
| v0.33.0 `setup init` creates checkpointed stores 0.32.0 refuses | `setup.md`, `connectors:upgrade`, plugin page | upgrade every binary that opens a store first |
| v0.33.0 `setup checkpoints-enable --confirm one-way` | `setup.md`, `connectors:upgrade`, plugin page | one-way; only on operator request; refuses while an owner runs |
| v0.33.0 owner-held store reads, expiry batches, memory | none | internal performance |
| v0.33.0 ESS 0.56.0, Entity Runtime 0.30.2, Eventlog 0.8.1 pins | none | internal |
| Entity Runtime 0.30.2 | none | no consumer in this repository |
| eval AEP 0.68.0 behind 0.69.0 | none | Metaharness 0.9.1 (newest) links AEP 0.68.0; the pair moves together |
| workflow pins docs-system, website | none | generated; Atlas reconciliation owns them |
| workflow pin gates | none | out of this task's scope |
| beyond10x/aep#60 closed | none | cited only in the dated tutorial, already says AEP 0.65.0 fixed it |

## Verification

- connectors v0.33.0 built from its tag with `cargo install --locked`; `connectors --version`
  prints `connectors 0.33.0`; `operations list --family x` exits 2 `cli_parse`.
- Every flag spelled in `plugins/connectors/skills` appears in the v0.33.0 help of its command.
- `agentplugins-check tools`: exit 0; connectors v0.33.0 21 spelled commands checked against the
  released source contract at d83e00f.
- No trial under `trials/` exercises Connectors; ESS, AEP and Worktree did not move, so no trial
  round is affected.
