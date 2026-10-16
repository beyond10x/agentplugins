---
format: aep.planning-md/3
id: task:refresh-aep-074-connectors-052
kind: task
status: active
title: Track AEP 0.74.0 and Connectors v0.52.0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-16T10:03:44Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-16T10:03:44Z", actor: "human:timo", revision: 3}
---
## Context

AEP 0.74.0 (2026-10-16) and Connectors v0.52.0 (2026-10-16) were published after release 0.23.0;
`verified.json` names aep 0.73.0 and connectors v0.51.0, and the scheduled Tools check is red on
both. ESS 0.60.0, Worktree 0.18.2 and Metaharness 0.9.3 are still the newest.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at aep 0.74.0 and connectors v0.52.0. Each
consumer-visible change in the aep 0.74.0 and connectors v0.52.0 release notes is reflected in the
owning skill or recorded under Classification. The isolated `aep-backlog` trial measures no worse
than `trials/baseline.json`. The pull request's `Gate` and `Tools` checks pass.

## Scope

verified.json; `.github/workflows/eval.yml`; `plugins/aep/skills/planning/SKILL.md`;
`plugins/connectors/skills/integrating/references/setup.md` and `writes-and-services.md`;
`plugins/connectors/skills/upgrade/SKILL.md`.

## Classification

| Change | Resource | Decision |
|---|---|---|
| aep 0.74.0: `aep plan artifact spend` and `spend.<kind>.<figure>` in `project.yaml` | `aep:planning` | described beside `task_spend`, checked against `aep plan artifact spend --help` of 0.74.0 |
| aep 0.74.0: `concurrent: true` step map states, `bugfix/standard` workflow and step map, `releases/source` driver | none | no skill authors step maps; drives run through `metaharness aep drive` |
| aep 0.74.0: `release.source` no longer requires `verify-after-action` | none | no skill names profile principles |
| aep 0.74.0: signed calendar date parts refused | none | no skill writes such a date |
| connectors v0.52.0: approval clock is `roughtime-quorum/1`; `roughtime-clock/1` refused | `writes-and-services.md`, `setup.md`, `connectors:upgrade` | the table's shape, the refusal and `clock-check --adapter` before the next approval |
| connectors v0.52.0: Slack read with `"ok": false` is a refusal by name | `writes-and-services.md` | named; write semantics unchanged per the release's `docs/catalog-slack.md` |
| connectors v0.52.0: `confluence-v1` provider, `slack.user` history reads, Kubernetes statefulsets, nodes, custom resources and exec credential plugin | `setup.md` | listed with the configuration-revision consequence; the release's guides are the contract |
| connectors v0.52.0: `connectors-build live-read-loop` | none | a build tool no skill invokes |
| eval workflow connectors pin | `.github/workflows/eval.yml` | v0.52.0 |

## Verification

- `agentplugins-check tools` with `verified.json` at aep 0.74.0 and connectors v0.52.0: exit 0; aep 0.74.0 223 spelled commands and the `aep.project/1` refusal; connectors v0.52.0 24 spelled commands against the released source contract, runtime help verified on the release binary (archive checked against `SHA256SUMS`); ESS, Worktree 0.18.2 and Metaharness 0.9.3 unchanged.
- `aep plan artifact spend --help` and a run on this store read on the 0.74.0 release binary.
- Isolated trial `aep-backlog` on 0.23.1 with aep 0.74.0: 261 tool calls, isolated (plugins aep, b10x, ess from the sandbox), no measure worse than `trials/baseline.json`.
- `task check`: exit 0 (118, 2 and 77 tests passed; 22 eval cases; marketplace valid).
