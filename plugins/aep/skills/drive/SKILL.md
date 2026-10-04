---
name: drive
description: Start one governed AEP run over a single story, started by the operator as /aep:drive <story-id>. Hands off to aep:implementing in drive mode.
disable-model-invocation: true
argument-hint: "<story-id>"
---

# Drive one story

Before acting, complete the [repository preflight](../planning/references/repository-preflight.md)
or reuse the coordinator’s matching completed record and user decision.

Load `aep:implementing` and run it in **drive** mode: read its
[references/drive.md](../implementing/references/drive.md) in full before acting.

- The story is the one id in `$ARGUMENTS`. With none or several, ask for exactly one and stop.
- Say the reference's one line on what a driven run costs and how it usually ends, before launching.
- Launch one run, print its run id and how to follow it, and stop.
- Move no artifact, and relay every refusal from `aep`, `metaharness` or the driver unedited.
