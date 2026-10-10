---
format: aep.planning-md/3
id: task:refresh-aep-0712-ess-057-connectors-043
kind: task
status: active
title: Track AEP 0.71.2, ESS 0.57.0 and Connectors v0.43.0
summary: the skills follow AEP 0.71.2, ESS 0.57.0 and Connectors v0.43.0, and verified.json moves
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-10T11:20:49Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-10T11:20:49Z", actor: "human:timo", revision: 3}
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

To be filled as each changelog entry is reviewed.

## Verification

To be filled from the runs.
