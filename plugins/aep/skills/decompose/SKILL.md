---
name: decompose
description: Decompose one epic into draft stories for the plan-critic panel; /aep:decompose <epic-id> or when asked to decompose an epic. Hands off to aep:planning.
argument-hint: "<epic-id>"
---

# Decompose an epic

Load `aep:planning` and follow its § 6 and § 7 for the one epic in `$ARGUMENTS`. Read the
`decomposer` role, [references/decomposer.md](../planning/references/decomposer.md), and the
[critic rubric](../planning/references/critic-rubric.md) in full before acting.

- With no epic id, or several, ask for exactly one and stop.
- Dispatch this plugin's `decomposer` agent for the draft stories, then the four plan critics at
  once, as § 7 says. In a host without subagents, run each role yourself from its reference.
- Record every critic verdict as § 7 says, revise at most twice, and report what is still open.
- Draft only: move no artifact, and relay every refusal from `aep` unedited.
