---
format: aep.planning-md/3
id: task:refresh-ess-059
kind: task
status: active
title: Track ESS 0.59.0
summary: the skills follow ESS 0.59.0, and verified.json moves
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-11T11:02:18Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-11T11:02:18Z", actor: "human:timo", revision: 3}
---
## Context

ESS 0.59.0 (https://github.com/beyond10x/ess/releases/tag/0.59.0) was published after release
0.22.7, which verified ESS 0.58.0. The scheduled Tools check turns red on it. AEP 0.71.2, Worktree
0.15.0, Metaharness 0.9.3 and Connectors v0.45.0 are still the newest releases.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at ess 0.59.0; the isolated ess-tutorial
trial measures no worse than `trials/baseline.json`; every consumer-visible entry of the ESS 0.59.0
changelog is either reflected in the owning skill or recorded below as needing no change.

## Classification

| change | resource | disposition |
|---|---|---|
| ESS 0.59.0 `ess-scenario/5` `caller:` on an authored act; `ESS-AUTHOR-013`, `-014`, `-015`; `ess-scenario/1`-`/4` refuse the key | `ess:testing-conformance`, `ess:upgrade` | added; reproduced from the release's own fixture (author 0 refusals, interpreted 1 passed) and each refusal |
| ESS 0.59.0 unsupported reason naming the caller attribute and `caller:` | `ess:testing-conformance` | added from the changelog |
| ESS 0.59.0 synthesis writes `when_subject:` scenarios with no covering view instead of `ESS-SYNTH-001` | `ess:specifying` `later-formats.md` limits table, `ess:upgrade` | revised; reproduced (orders without a view: 6 scenarios, 0 refusals, notes naming the unpublished field; ess 0.57.0 refused 4) |
| ESS 0.59.0 `when_related:` over a captured struct input member | `ess:upgrade` | suite bytes for that case can change; from the changelog |
| ESS 0.59.0 tutorials and fixtures | ESS and AEP tutorial pages, trial fixtures, drafted fixture, eval workflow | require ESS 0.59.0; crates at tag 0.59.0 |
