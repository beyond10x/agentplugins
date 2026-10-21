---
format: aep.planning-md/3
id: story:track-ess-newest
kind: story
status: active
title: Track the newest ESS release in the skills and the tutorial set
relations:
- decomposes: epic:release-0-24-0
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:47Z", actor: "human:timo", revision: 3}
---
## Context

The ESS skills stop at the 0.62.0 formats (`ess/26`, `ess-conformance/50`). Newer: 0.63.0 `ess/27`,
`ess-conformance/52` and `/53`, `ess verify conform diff`, `synthesize --check`; 0.64.0 `ess/28`;
0.66.0 generated Go and TypeScript runners skip unsupported scenarios. The ESS tutorial set
(`website/docs/tutorials/first-ess-specification*`, `trials/aep-tutorial/fixture`,
`fixtures/library-reservations-drafted`) builds against ess 0.60.0.

## Acceptance

The ESS skills describe each consumer-visible change of ess 0.63.0 to the newest release (0.67.0 if
tagged, else 0.66.0) or record it under Classification; `verified.json` and `.github/workflows/eval.yml`
name that release; the tutorial set moves together to it; `agentplugins-check tools` reports no ess
problem and runs the executable examples green; the `ess-tutorial` trial measures no worse than
`trials/baseline.json`.

## Scope

`plugins/ess/skills/**`, `verified.json`, `.github/workflows/eval.yml`,
`website/docs/tutorials/first-ess-specification*`, `trials/aep-tutorial/fixture/**`,
`fixtures/library-reservations-drafted/**`, `CHANGELOG.md` (Unreleased).
