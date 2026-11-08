---
name: review-plan
description: Audit the AEP planning store for what `aep plan artifact validate` cannot see; /aep:review-plan or when asked to review the plan. Read-only, proposes moves and makes none. Hands off to aep:planning.
argument-hint: "[artifact-id...]"
---

# Review the plan

Load `aep:planning` and run its `plan-reviewer` role: read
[references/plan-reviewer.md](../planning/references/plan-reviewer.md) in full before acting.

- Scope: the artifact ids in `$ARGUMENTS`; with none, the whole store.
- Dispatch this plugin's `plan-reviewer` agent and name it with its plugin prefix. In a host
  without subagents, follow that reference yourself.
- Read only: write each proposed move out as a command and run none; change no file.
- End with the verbatim output of `aep plan artifact validate`.
