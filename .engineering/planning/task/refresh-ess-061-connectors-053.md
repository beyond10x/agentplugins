---
format: aep.planning-md/3
id: task:refresh-ess-061-connectors-053
kind: task
status: active
title: Track ESS 0.62.0 and Connectors v0.53.1
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-17T11:47:43Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-17T11:47:43Z", actor: "human:timo", revision: 3}
---
## Context

ESS 0.61.0 (2026-10-16), ESS 0.62.0 (2026-10-17) and Connectors v0.53.0 and v0.53.1 (2026-10-17) were published after
release 0.23.1; `verified.json` names ess 0.60.0 and connectors v0.52.0, and the scheduled Tools
check is red on both. AEP 0.74.0, Worktree 0.18.2 and Metaharness 0.9.3 are still the newest.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at ess 0.62.0 and connectors v0.53.1. Each
consumer-visible change in the ess 0.61.0 and 0.62.0 and connectors v0.53.0 and v0.53.1 release notes is
reflected in the owning skill or recorded under Classification. The isolated `ess-tutorial` trial
measures no worse than `trials/baseline.json`. The pull request's `Gate` and `Tools` checks pass.

## Scope

verified.json; `.github/workflows/eval.yml`; `plugins/ess/skills/specifying/SKILL.md` and
`references/syntax.md`; `plugins/ess/skills/retrofitting/SKILL.md`;
`plugins/ess/skills/hardening/SKILL.md` and `references/techniques.md`; `plugins/ess/skills/testing-conformance/SKILL.md`; `plugins/ess/skills/upgrade/SKILL.md`; `plugins/ess/skills/specifying/references/later-formats.md`; `plugins/connectors/skills/integrating/references/setup.md`
and `writes-and-services.md`; `plugins/connectors/skills/upgrade/SKILL.md`.

## Classification

| Change | Resource | Decision |
|---|---|---|
| ess 0.61.0: `--rust-raw-json <type>[.<field>]` on `generate types --target rust` | `ess:specifying` | described beside `exact-numbers`, checked against `ess generate types --help` of 0.61.0 |
| ess 0.61.0: `ess-history/3` input, events, arranged | `ess:hardening` | described under several clients, checked against `check-history --help` of 0.61.0 |
| ess 0.61.0: ESS-SYNTH-005 names a skipped read and keeps the scenario | syntax reference, `ess:retrofitting` | a probe view filtered on the identity field synthesized with 0 refusals on 0.61.0 (and 0.60.0); the text that said such a view is refused is replaced |
| ess 0.61.0: one-time response guide, concurrent explorer fixes, rustls, CI | none | upstream documentation and fixes; no skill text states the old behaviour |
| connectors v0.53.0: OpenRouter provider, `key.create` with credential custody | `writes-and-services.md`, `setup.md` | custody contract and recovery named; the release's guides are the contract |
| connectors v0.53.0: Runpod `billing.pods`, GitLab `file.raw`, `project.languages` | `setup.md` | listed |
| connectors v0.53.0: owner exits when its state directory goes | none | internal lifecycle |
| connectors v0.53.1: Runpod capacity refusal; provider SHA-256, bootstrap and `connections revalidate` | `writes-and-services.md`, `connectors:upgrade` | named; commands checked against the v0.53.1 binaries' `--help` |
| eval workflow connectors pin | `.github/workflows/eval.yml` | v0.53.1 |
| ess 0.62.0: `hold_now` step, `ess-conformance/50` and `/51`, `ConformanceTarget::hold_now`, boundary scenarios | `ess:testing-conformance`, `ess:upgrade` | described; a target without `hold_now` reports only those scenarios unsupported without failing |
| ess 0.62.0: `ESS-MUTATE-004` for an unchanged suite, `ess-mutation-report/7`, `guard-boundary` inconclusive on the interpreted target | `ess:hardening` techniques | described, checked against `ess verify conform mutate --help` of 0.62.0 |
| ess 0.62.0: explorers draw `Timestamp` inputs and perform `deletes:` | `ess:hardening` | described |
| ess 0.62.0: `ess/26` external refusal leaves the addressed row unchanged | later-formats table, syntax reference range | row added, checked against `ess specify formats` of 0.62.0 |
| ess 0.62.0: Rust newtype decoders enforce alphabet, prefix and invariants | `ess:specifying` | one sentence beside the type libraries |
| ess 0.62.0: `ess-runtime/2` schedules and pod settings | `ess:upgrade` | named as a format older readers refuse; no skill authors runtime documents |
| ess 0.62.0: synthesis arranging fixes, caller-swap fresh values, `serde_json` number fix, `ess-deployment` types | none | fixes and library types no skill quotes |

## Verification

- `agentplugins-check tools` with `verified.json` at ess 0.62.0 and connectors v0.53.1: exit 0; aep 0.74.0 223, ess 0.62.0 100, worktree 0.18.2 52, metaharness 0.9.3 13 and connectors v0.53.1 25 spelled commands; connectors runtime help verified on the release binary (archives checked against `SHA256SUMS`).
- A probe view filtered on the identity field synthesizes with 0 refusals on ess 0.60.0, 0.61.0 and 0.62.0.
- Isolated trial `ess-tutorial` on ess 0.61.0 (58 tool calls) and on ess 0.62.0 (74 tool calls, plugins b10x and ess at 0.23.2 from the sandbox): specification valid, 17 scenarios, 0 refusals, outputs 5/5, `cargo test` 2 passed, no measure worse than `trials/baseline.json`.
- `task check`: exit 0 (118, 2 and 77 tests passed; 22 eval cases; marketplace valid).
