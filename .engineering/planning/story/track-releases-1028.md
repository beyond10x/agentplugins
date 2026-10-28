---
format: aep.planning-md/3
id: story:track-releases-1028
kind: story
status: active
title: Track ESS 0.75.0, Worktree 0.19.0 and Metaharness 0.22.0
summary: Move verified.json and the eval pins to the newest releases and re-verify the skills
scope:
- confidence: cited
  path: .github/workflows/eval.yml
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: fixtures/library-reservations-drafted
- confidence: cited
  path: plugins/aep/skills
- confidence: cited
  path: plugins/ess/skills
- confidence: cited
  path: plugins/worktree/skills
- confidence: cited
  path: trials
- confidence: cited
  path: verified.json
- confidence: cited
  path: website/docs
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-28T19:01:57Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-28T19:01:57Z", actor: "human:timo", revision: 3}
---
## Context

The scheduled Tools run 40678898507 on 2026-10-28 failed `agentplugins-check tools` on three releases
newer than `verified.json`: ESS 0.74.0 (0.69.2), Worktree 0.19.0 (0.18.2) and Metaharness 0.22.0
(0.19.0). ESS 0.75.0 followed the same day. ESS 0.70.0 adds `format: ess/32` (a named-row guard
beside a row-set guard, `fields: []`, a lifecycle state's `wire:` spelling), `ess generate types
--check`, Decimal view fields in `ess-conformance/58` and `/59`, and makes a variant's `code:` name its
generated identifier; 0.71.0 to 0.75.0 widen what synthesis arranges and admit seeds for caller
attributes. Worktree 0.19.0 honours a `.worktreekeep` in a subdirectory. Metaharness 0.20.0 to 0.22.0
report an off-pin vendor version as advisory, move the harness pin and add `metaharness::qualify`;
0.22.0 still links AEP 0.77.0 (its `Cargo.lock`), so the eval AEP pin stays.

## Acceptance

The ESS, Worktree and AEP drive skills describe each consumer-visible change of the releases or this
record says why none applies; `verified.json` names ess 0.75.0, worktree 0.19.0 and metaharness
0.22.0; `.github/workflows/eval.yml` names metaharness 0.22.0 with eval AEP 0.77.0, ess 0.75.0 and
worktree 0.19.0; the ESS tutorial set moves to 0.75.0 and `ess/32` together;
`agentplugins-check tools` exits 0; the `ess-tutorial`, `ess-full-package` and `worktree-onboarding`
trials measure no worse than `trials/baseline.json`; `task check` and `task site-build` exit 0.

## Scope

`plugins/{ess,worktree,aep}/skills/**`, `verified.json`, `.github/workflows/eval.yml`,
`website/docs/plugins/*.md`, `website/docs/tutorials/first-ess-specification*`,
`website/docs/tutorials/first-governed-plan.md`, `trials/{ess,aep}-tutorial/fixture/**`,
`fixtures/library-reservations-drafted/**`, `CHANGELOG.md`, the workspace and plugin versions.

## Verification

On the candidate tree, 2026-10-28: `task check` exit 0; `agentplugins-check tools` exit 0 (aep 0.78.0 230,
ess 0.75.0 107, worktree 0.19.0 52, metaharness 0.22.0 15 and connectors v0.59.0 32 spelled commands;
tutorial 17/17 scenarios passed); `task site-build` exit 0. On unchanged `main`, `tools` found every
spelled command present in the three releases; only the three version problems failed.

The `ess/32` constructs were probed on 0.75.0 with the wiring and job-queue fixtures of the ESS 0.75.0
tag (plus an open `fields: []` struct): 7 and 5 scenarios, 0 refusals, `ess-conformance/34`, the
`interpreted` target passed all 12. Under `ess/31` they are refused as `ESS-COMMAND-009`,
`ESS-TYPE-007` and `ESS-ENTITY-009`; ESS 0.69.2 refuses the header as `ESS-SPEC-009` and a state's
`wire:` at parse time. `metaharness doctor claude` on 0.22.0 still exits 1 off the pin.

Isolated trials at plugin version 0.25.1, each isolation check passed:

| trial | tool calls (baseline) | result |
|---|---|---|
| `worktree-onboarding` | 13 (12) | no measure worse |
| `ess-tutorial` | 58 (67) | valid, 17 scenarios 0 refusals, 5/5 outputs, cargo test 2/0; no measure worse |
| `ess-full-package` | 59 (67) | third run, after the fixes below: valid, 18 scenarios 0 refusals, 0 unmapped, 9/9 outputs, cargo test 4/0, every ESS scenario passed; no measure worse |

The first two `ess-full-package` runs (63 and 116 tool calls) passed every ESS scenario (24/24 and 26/26)
but wrote two Cargo tests where the baseline holds three, so `trial-report` read "cargo test: passes
3 → 2". Their findings were fixed here: `ess:specifying` now says `ess generate types` needs
`--package` for Rust and Go and `--module` for Go (the second run found them from the errors), and
`ess:testing-conformance` reconciles "the Rust runner has no skipped status" with the `hold_now`
boundary exception and says the mixed-caller synthesis note is not a refusal. Baselines are unchanged.

Left for the next tutorial wording round: the tutorial target's "cannot expose events: all
publications are direct" leaves the `<command>/grant/denied` scenarios unsupported once a model declares actor
grants (two `ess-full-package` runs copied it and added an event log).
