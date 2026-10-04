---
name: review-plan
description: Audit the AEP planning store for what `aep plan artifact validate` cannot see, started by the operator as /aep:review-plan. Hands off to aep:planning and its plan-reviewer role; read-only, it proposes moves and makes none.
disable-model-invocation: true
argument-hint: "[artifact-id...]"
---

# Review the plan

Before acting, complete the [repository preflight](../planning/references/repository-preflight.md)
or reuse the coordinator’s matching completed record and user decision.

Load `aep:planning` and run its `plan-reviewer` role: read
[references/plan-reviewer.md](../planning/references/plan-reviewer.md) in full before acting.

- Scope: the artifact ids in `$ARGUMENTS`; with none, the whole store.
- Dispatch this plugin's `plan-reviewer` agent and name it with its plugin prefix. In a host
  without subagents, follow that reference yourself.
- Read only: write each proposed move out as a command and run none; change no file.
- End with the verbatim output of `aep plan artifact validate`.
