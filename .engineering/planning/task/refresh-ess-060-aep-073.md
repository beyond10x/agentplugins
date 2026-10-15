---
format: aep.planning-md/3
id: task:refresh-ess-060-aep-073
kind: task
status: active
title: Track ESS 0.60.0 and AEP 0.73.0
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-15T05:35:16Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-15T05:35:16Z", actor: "human:timo", revision: 3}
---
## Context

ESS 0.60.0 (2026-10-15T01:42Z) and AEP 0.73.0 (2026-10-15T04:15Z) were published while the 0.23.0
wave was open; `verified.json` named ess 0.59.0 and aep 0.72.1. Worktree 0.18.2, Metaharness 0.9.3
and Connectors v0.51.0 are still the newest.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at ess 0.60.0 and aep 0.73.0. Each
consumer-visible change in their release notes is reflected in the owning skill or recorded under
Classification. The tutorial and trial-fixture implementations pass their conformance tests against
0.60.0 suites. The pull request's `Gate` and `Tools` checks pass.

## Scope

verified.json; `plugins/ess/skills/**`; `plugins/aep/skills/planning/SKILL.md`; the ESS tutorials,
the aep-tutorial and ess-tutorial trial fixtures, `fixtures/library-reservations-drafted`;
`.github/workflows/eval.yml`; `crates/agentplugins-check/src/tools.rs` (the `devices` example).

## Classification

| Change | Resource | Decision |
|---|---|---|
| ess/25 create-or-update with `when_subject:` beside a creating `unknown_instance:` | specifying `later-formats.md`, `examples/devices.yaml` | added; validated and synthesized on 0.60.0 (16 scenarios, 0 refusals) |
| ess/25 `trimmed:` text guards | `later-formats.md` | added; `empty_declaration` on a padded literal reproduced |
| ess/25 per-row `emits:` (`each: changed`, `when_state:`) | `later-formats.md` | added |
| ess/25 `before_row: true` on an `external:` refusal | `later-formats.md`, hardening `spec-diff.md` | added |
| ess/25 view `absent: {row: …}`; `ess-diff/19` `absent-changed` | `later-formats.md`, `spec-diff.md`, upgrade | added; diff reproduced |
| `ess-http/1` `request:` encoding, `outcomes:`, `body: single`, `refusals:` (ESS-HTTP-020 to -031) | specifying `current-features.md` | added; binding validated and compiled, HTTP-020, -024, -030 reproduced |
| `ess-composition/4` `roles:` | `current-features.md`, upgrade | added; compose and `role_input_drift` reproduced |
| read-only command may declare `unknown_instance:` | `later-formats.md` | added; 0.59.0 refusal and 0.60.0 acceptance reproduced |
| newtype literals refused as `type_mismatch` | `later-formats.md`, upgrade | added; reproduced |
| mutation report and manifest `/6`, guard mutants, `--scenarios` | hardening `SKILL.md`, `techniques.md` | added; a `when_subject` guard mutant killed |
| explorers: `refused` field, `preserves:` and `Decimal` drawn, selected-row effects | hardening | added from the notes |
| `one_time_response:` disclosure scenarios send the caller | testing-conformance, upgrade | added from the notes |
| null `conformances` in `ess-composition/1` refused; `serde_yaml` removed | upgrade | added; no `serde_yaml` in the three refreshed lockfiles |
| tutorials, fixtures, eval pin | tutorial pages, trial fixtures, drafted fixture, eval.yml | require 0.60.0 with crates at tag 0.60.0; tutorial and aep-tutorial impls 2 of 2 passed on 17-scenario suites; the drafted fixture keeps 10 passed, 45 unsupported, as on 0.59.0 |
| aep 0.73.0 `expect: failure`, `output:`, `drive run --input` | none | no skill authors step maps; drives run through `metaharness aep drive` |
| aep 0.73.0 `evidence --kind task_spend --from` | `aep:planning` | one paragraph, checked against `aep plan artifact evidence --help` of the 0.73.0 release binary |
| aep 0.73.0 drive model in its ESS specification | none | internal to aep |
