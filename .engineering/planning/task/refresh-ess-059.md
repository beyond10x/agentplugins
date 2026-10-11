---
format: aep.planning-md/3
id: task:refresh-ess-059
kind: task
status: active
title: Track ESS 0.59.0
summary: the skills follow ESS 0.59.0, and verified.json moves
revision: 5
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

## Verification

- ess 0.59.0 from its release archive, checked against `SHA256SUMS`; nothing installed.
- `agentplugins-check tools`: exit 0; aep 0.71.2 222 spelled commands, ess 0.59.0 94 (the tutorial
  regenerated and its cargo test 2 passed), worktree 0.15.0 52, metaharness 0.9.3 13, connectors
  v0.45.0 21 against the released source contract (no binary assets; runtime not executed).
- `task trial:run TRIAL=ess-tutorial`: isolated, sandbox `ess` 0.59.0, plugins at 0.22.8; 85 tool
  calls, 17 scenarios 0 refusals, cargo test 2 passed. Its agent lost 14 scenarios to two tutorial
  gaps (a refusal's consistency token; `Node::Null` for an absent optional), fixed in 97b9b71. The
  rerun on 97b9b71: 59 tool calls (baseline 67), validate valid, 17 scenarios 0 refusals, 5/5
  outputs, cargo test 2 passed; no measure worse than `trials/baseline.json`.
- `task check` and `task site-build`: exit 0.
- Candidates from the rerun, not acted on: the adapter guidance does not say a `ScenarioContext`
  needs a well-formed scenario id or that `Timestamp` has no `FromStr`; the printed cargo counts
  stop matching once a reader adds a unit test.
