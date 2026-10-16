---
format: aep.planning-md/3
id: task:refresh-aep-0721
kind: task
status: implemented
title: Track AEP 0.72.1 and Connectors v0.51.0
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-14T22:49:50Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-14T22:49:50Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-16T10:03:32Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":1}}}
---
## Context

AEP 0.72.0 and 0.72.1 were published after release 0.22.11; `verified.json` names aep 0.71.3.
Connectors v0.51.0 followed on 2026-10-14 (`verified.json`: v0.50.0). ESS 0.59.0, Worktree 0.18.2
and Metaharness 0.9.3 are still the newest.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at aep 0.72.1 and connectors v0.51.0. Each
consumer-visible change in the aep 0.72.0 and 0.72.1 and connectors v0.51.0 release notes is
reflected in the owning skill or recorded under Classification. The isolated `aep-backlog` trial
measures no worse than `trials/baseline.json`. The pull request's `Gate` and `Tools` checks pass.

## Scope

verified.json; `plugins/connectors/skills/integrating/references/setup.md` and
`writes-and-services.md`; any other skill named under Classification.

## Classification

| Change | Resource | Decision |
|---|---|---|
| aep 0.72.0: an `llm` step of a driver step map carries `judgement:` | none | no skill authors step maps; drives run through `metaharness aep drive` |
| aep 0.72.0: published `driver-steps` schema refuses blank and zero values | none | no skill writes step maps |
| aep 0.72.0: `aep drive run/resume/status` print `llm steps <declared> declared, <run> run` | none | skills drive through `metaharness aep drive`, whose output they quote verbatim |
| aep 0.72.1: stores pinning a protocol tree older than 0.72.0 validate again; the `judgement:` rule moves to `aep.driver-steps/2` | none | no skill names a step-map format version |
| connectors v0.51.0: Slack bot selection gains `chat.update` and `chat.delete`, with named refusals | `integrating/references/setup.md`, `writes-and-services.md` | described: changed descriptor, recover, add only when authorized; refusals named |
| connectors v0.51.0: credential-only `slack` adapter moves to `slack-bot` | `integrating/references/setup.md` | points at the release's `docs/catalog-slack.md` migration section |
| connectors v0.51.0: archives carry `connectors-catalog-provider` | `integrating/references/setup.md` | says `b10x` installs `connectors` alone and the provider comes from the same archive |
| connectors v0.51.0: `approvals clock-check` names `clock_refusal` | `writes-and-services.md` | named where approvals need the clock |
| connectors v0.51.0: Opsgenie and Grafana IRM incident reads, SQL and Loki read profiles, mounted-secret custody, compatibility refusal codes | none | no skill lists adapters' operations or these refusal codes; the adapter guides are the contract |

## Verification

- `agentplugins-check tools` with `verified.json` at aep 0.72.1 and connectors v0.51.0: exit 0; aep 0.72.1 222 spelled commands; connectors v0.51.0 22 spelled commands against the released source contract, runtime help verified on the release binary (archive checked against `SHA256SUMS`).
- Isolated trial `aep-backlog` on 0.23.0: 284 tool calls, isolated (plugins aep, b10x, ess from the sandbox), no measure worse than `trials/baseline.json`.
