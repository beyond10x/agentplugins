---
format: aep.planning-md/3
id: task:refresh-worktree-018-connectors-048
kind: task
status: active
title: Track Worktree 0.18.0 and Connectors v0.48.0
summary: the skills and b10x follow Worktree 0.18.0 and Connectors v0.48.0, and verified.json moves
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-13T13:36:59Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-13T13:36:59Z", actor: "human:timo", revision: 3}
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

Filled in as the units land.

## Verification

Filled in as the checks run.
