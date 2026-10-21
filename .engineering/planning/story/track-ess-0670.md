---
format: aep.planning-md/3
id: story:track-ess-0670
kind: story
status: implemented
title: Track ESS 0.67.0 in the skills and the tutorial set
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T20:23:32Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T20:23:32Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:59Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

ESS 0.67.0 (2026-10-21) is newer than the 0.66.0 that `story:track-ess-newest` moved the skills,
`verified.json`, `.github/workflows/eval.yml` and the tutorial set to. New in 0.67.0: `format: ess/29`
(`external:` with `when_subject:`, guards with `coalesce(x, d)`, refused out-of-order branches),
`ess-conformance/56` and `/57` for one-time read cells, and several fixes that need regenerated suites.

## Acceptance

The ESS skills describe each consumer-visible change of ess 0.67.0 or record it under Classification in
the unit report; `verified.json` and `.github/workflows/eval.yml` name 0.67.0; the tutorial set moves
together to it; `agentplugins-check tools` reports no ess problem and runs the executable examples
green; the `ess-tutorial` trial measures no worse than `trials/baseline.json`.

## Scope

`plugins/ess/skills/**`, `verified.json`, `.github/workflows/eval.yml`,
`website/docs/tutorials/first-ess-specification*`, `trials/aep-tutorial/fixture/**`,
`fixtures/library-reservations-drafted/**`.
