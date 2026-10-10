---
format: aep.planning-md/3
id: task:refresh-ess-058-connectors-045
kind: task
status: active
title: Track ESS 0.58.0 and Connectors v0.45.0
summary: the skills follow ESS 0.58.0 and Connectors v0.45.0, and verified.json moves
revision: 3
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
