---
format: aep.planning-md/3
id: task:ess-synthesized-implementation
kind: task
status: active
title: The ess skills route implementation through synthesized code, so a hand-transcribed model cannot drift
owner: human:timo
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-09-29T16:35:13Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-09-29T16:35:13Z", actor: "human:timo", revision: 3}
---
## Outcome
The ess skills tell an implementer to build behind `ess generate synthesize` output and never hand-transcribe the model, and say what to do while synthesis refuses a specification.

## Scope
`plugins/ess/skills/specifying/SKILL.md` (a section beside "Deterministic projections") and `plugins/ess/skills/testing-conformance/SKILL.md` (what a green suite does not prove). No CLI, agent or eval-runner change.

## Acceptance
Both sections are present in the released ess plugin; `task check` and `agentplugins-check tools` pass; the wording names no downstream product.

## Authorization
Interactive user approved the plan on 2026-09-29: skill notes, PR, merge and release.
