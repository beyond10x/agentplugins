---
format: aep.planning-md/3
id: story:waves-from-accepted-stories
kind: story
status: implemented
title: Waves are derived up front from accepted stories; findings wait in the backlog
summary: The wave skill runs predefined waves of accepted stories and files pre-existing findings as backlog drafts no wave takes before the plan is delivered
scope:
- confidence: cited
  path: evals/command-wave-agent-turn
- confidence: cited
  path: plugins/aep/skills/implementing
- confidence: cited
  path: plugins/aep/skills/wave
- confidence: cited
  path: trials/aep-tutorial/fixture/tutorial.md
- confidence: cited
  path: website/docs/plugins/aep.md
- confidence: cited
  path: website/docs/tutorials/first-governed-plan.md
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-11-02T21:17:55Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-11-02T21:17:55Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-11-02T22:29:01Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":1}}}
---
## Context

`plugins/aep/skills/implementing/references/wave.md` routed an adversary finding with origin
`pre-existing` to "file a story. It does not block the unit", selected a wave's candidates from every
`draft` and `proposed` story (`aep plan artifact waves --kind story --status draft`), and told the
coordinator to move each story out of `draft` itself. Coordinators then accepted the finding drafts
they had filed and waved them: over three days one repository filed 23 stories from findings at wave
close and implemented 35 stories, and 0 of its 29 issues were closed (operator report, 2026-11-02).

`aep plan artifact waves` at 0.79.0 sequences every story of the kind when `--status` is absent
(`crates/edge/aep-cli/src/planning.rs:4460`), so the skill selects accepted stories only by passing
the accepted status.

## Acceptance

`wave.md`, `SKILL.md`, the `wave` command, `adversary.md`, `security-reviewer.md`,
`adversary-panel.md` and `branch-and-merge.md` state one model: waves are derived up front, once,
from accepted stories with `aep plan artifact waves --status <accepted>`; a coordinator runs the
wave it is given, adds no story and accepts none; a finding the unit introduced or its acceptance
needs goes back to the implementor; a pre-existing finding is a row in the `review-result` and one
`draft` story in the backlog; no backlog story enters a wave before every predefined wave is
delivered. No text under `plugins/` routes a pre-existing finding to "file a story" or selects from
draft stories, and `task check` exits 0.

## Scope

`plugins/aep/skills/implementing/**`, `plugins/aep/skills/wave/**`, `evals/command-wave-agent-turn`,
`website/docs/plugins/aep.md`, `website/docs/tutorials/first-governed-plan.md`,
`trials/aep-tutorial/fixture/tutorial.md`.
