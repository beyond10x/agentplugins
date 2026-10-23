---
format: aep.planning-md/3
id: story:track-releases-1022
kind: story
status: active
title: Track ESS 0.68.0, Metaharness 0.14.0 and Connectors v0.56.0
scope:
- confidence: cited
  path: .github/workflows/eval.yml
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: fixtures/library-reservations-drafted
- confidence: cited
  path: plugins/connectors/skills
- confidence: cited
  path: plugins/ess/skills
- confidence: cited
  path: trials/aep-tutorial/fixture
- confidence: cited
  path: trials/ess-tutorial/fixture
- confidence: cited
  path: verified.json
- confidence: cited
  path: website/docs
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-23T12:18:10Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-23T12:18:10Z", actor: "human:timo", revision: 3}
---
## Context

The scheduled Tools workflow on `main` has been red since 2026-10-22 (runs 39760742953, 39766472361,
39935994395; last green 39676693335 on 2026-10-21). Its step "Skills match the newest CLI releases"
reports three releases newer than `verified.json`: ESS 0.68.0 (0.67.0), Metaharness 0.14.0 (0.12.0) and
Connectors v0.56.0 (v0.55.0). `agentplugins-check tools` on unchanged `main` found every spelled command
and flag of the three releases present; only the three version problems failed.

## Acceptance

The ESS, Metaharness and Connectors skills describe each consumer-visible change of the three releases or
the unit report says why none applies; `verified.json` and `.github/workflows/eval.yml` name ess 0.68.0,
metaharness 0.14.0 and connectors v0.56.0 (eval AEP stays 0.68.1, the revision Metaharness 0.14.0
links); the ESS tutorial set moves to 0.68.0 together; `agentplugins-check tools` exits 0; the
`ess-tutorial` trial measures no worse than `trials/baseline.json`; `task check` exits 0; the Tools
workflow on `main` is green after the merge.

## Scope

`plugins/ess/skills/**`, `plugins/connectors/skills/**`, `verified.json`,
`.github/workflows/eval.yml`, `website/docs/plugins/{ess,connectors}.md`,
`website/docs/tutorials/first-ess-specification*`, `trials/{ess,aep}-tutorial/fixture/**`,
`fixtures/library-reservations-drafted/**`, `CHANGELOG.md`.
