---
format: aep.planning-md/3
id: story:commands-agent-invocable
kind: story
status: implemented
title: An agent can start every command skill; approvals live inside the skill
summary: drop disable-model-invocation and allow_implicit_invocation false from the five commands; R3, the gate and an eval per command follow
refs:
- provider: github
  reference: https://github.com/beyond10x/agentplugins/issues/41
scope:
- confidence: cited
  path: crates/agentplugins-check/src/concept.rs
- confidence: inferred
  path: evals
- confidence: cited
  path: plugins/aep/skills/decompose
- confidence: cited
  path: plugins/aep/skills/drive
- confidence: cited
  path: plugins/aep/skills/review-plan
- confidence: cited
  path: plugins/aep/skills/wave
- confidence: cited
  path: plugins/b10x/skills/authoring-plugins/SKILL.md
- confidence: cited
  path: plugins/b10x/skills/routing/SKILL.md
- confidence: cited
  path: plugins/worktree/skills/cleanup
- confidence: cited
  path: website/docs/plugins/aep.md
- confidence: cited
  path: website/docs/plugins/b10x.md
- confidence: cited
  path: website/docs/plugins/worktree.md
- confidence: cited
  path: website/docs/structure.md
revision: 20
transitions:
- {from: "draft", to: "proposed", at: "2026-10-07T08:06:23Z", actor: "human:timo", revision: 15}
- {from: "proposed", to: "active", at: "2026-10-07T08:06:23Z", actor: "human:timo", revision: 16}
- {from: "active", to: "implemented", at: "2026-10-07T09:18:25Z", actor: "human:timo", revision: 20, decided_on: {"recorded":{"test_result":1,"review_outcome":8,"verification":2}}}
---
## Context

https://github.com/beyond10x/agentplugins/issues/41. Five command skills set
`disable-model-invocation: true` and, in `agents/openai.yaml`, `policy.allow_implicit_invocation:
false`: `aep:wave`, `aep:drive`, `aep:decompose`, `aep:review-plan`, `worktree:cleanup`. Claude
Code's Skill tool refuses them to an agent and adds "Do not replicate this skill's workflow by other
means", so an agent asked in words for exactly what the command does has to stop and ask the operator
to type it. Structure rule R3 (`website/docs/structure.md`) requires the flag on every command and
`crates/agentplugins-check/src/concept.rs` enforces it; `plugins/b10x/skills/authoring-plugins/SKILL.md`
and `website/docs/plugins/b10x.md` teach it.

Each command's risky step is already gated inside its procedure: `aep:wave` stops at the stage-1
proposal for approval; `aep:decompose` writes drafts only; `aep:review-plan` is read-only;
`aep:drive` hands off to `aep:implementing` drive mode, which is model-invocable already;
`worktree:cleanup` dry-runs and applies exact reviewed ids, and `worktree:managing-worktrees`
already says an explicit cleanup request authorizes removal in its scope.

## Acceptance

No skill under `plugins/` sets `disable-model-invocation: true` or
`policy.allow_implicit_invocation: false`. R3 in `website/docs/structure.md`, the
`authoring-plugins` skill and `website/docs/plugins/b10x.md` say a command may be started by the
operator or by an agent acting on the operator's request, that any approval it needs is a step in
its body or its activity skill, and that a skill kept operator-only sets both flags and states the
reason in its description in a sentence starting `Operator-only:`. `agentplugins-check` refuses a
skill that sets either flag without that sentence, or sets one flag without the other, with unit
tests for each case. `worktree:cleanup` says that cleanup is authorized by the operator's request
(the command, or the same request in words that an agent acts on), and that without such a request
it stops after the dry-run. `evals/` holds one case per command whose task is an agent turn ("the
operator asked for …"), whose expectations require the Skill tool call for that command and either
the command's work or its documented approval stop, and never a refusal at invocation;
`agentplugins-check evals` validates them. `task check` passes.

## Scope

Confirmed by the implementor's table and the merged diff (merge `adb1764`):

Cited, held: plugins/aep/skills/{wave,drive,decompose,review-plan}/SKILL.md and agents/openai.yaml,
plugins/worktree/skills/cleanup/SKILL.md and agents/openai.yaml, website/docs/structure.md,
crates/agentplugins-check/src/concept.rs, plugins/b10x/skills/authoring-plugins/SKILL.md,
website/docs/plugins/b10x.md, plugins/b10x/skills/routing/SKILL.md.

Inferred, held: evals/ (five `command-*-agent-turn` cases and README.md).

Inferred, wrong (removed before dispatch): plugins/aep/skills/implementing/SKILL.md (its sentence
naming `/aep:wave` and `/aep:drive` stays true), README.md (no command text).

Not in the scope, found by grep and changed: website/docs/plugins/aep.md:36,63 and
website/docs/plugins/worktree.md:15,52-55 still taught that only the operator starts a command.

Left for later (adversary pass 1, finding 6, pre-existing): the git regexes in
evals/wave-claim-verdict, adversary-panel-one-family and adversary-tests-only miss
`git -C <dir> <verb>`.
