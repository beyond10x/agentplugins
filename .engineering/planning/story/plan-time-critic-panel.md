---
format: aep.planning-md/1
id: story:plan-time-critic-panel
kind: story
status: implemented
title: A plan-time critic panel with a bounded revision loop
relations:
- decomposes: epic:second-adopter-feedback
revision: 4
---
# Story: A plan-time critic panel with a bounded revision loop

## Outcome

After a decomposition is drafted, two to four read-only critics each return `approve` or
`needs-revision` with cited reasons; the planning skill revises at most twice; every verdict is a
`review-result` artifact. The operator reads a plan that has already been argued with.

## Context

The second adopter's explicit ask: something that "loops several times over a given task from
different perspectives (set of rules)". This stack has one on-demand plan auditor and one
post-implementation adversary; it has no plan-time panel and no revision loop. The third-party
plugin's `/plan` runs one to five critics by tier with at most two revision rounds, LLM-judged. Derived
from the epic this decomposes.

## Acceptance

- New read-only agents under `plugins/aep-planning/agents/`: `plan-critic-acceptance` (every story's
  acceptance is observable and covers state transitions), `plan-critic-design` (coupling, cycles,
  stories that share a surface), `plan-critic-scope` (the epic's outcome is covered, nothing outside
  it is drafted), `plan-critic-parallel-safety` (two stories on one file are named). Each has `Read,
  Grep, Glob, Bash` only, moves nothing, and returns exactly `approve` or `needs-revision` plus a
  list of `artifact — reason — citation` lines.
- One shared rubric under `plugins/aep-planning/skills/planning/references/critic-rubric.md` written
  as rules, not vocabulary: it names no kind, status or relation the CLI would answer for.
- The planning skill gains one step: after a decomposition is reported, dispatch the critics in
  parallel, record each verdict with `aep artifact new review-result … --from` (with `reviews:`
  edges to the artifacts it judged), revise drafts through `aep artifact body` on `needs-revision`,
  and stop after two revision rounds with the open findings listed in the report.
- The step is skipped, and says so, when the store has fewer than two stories under the epic.
- The skill validator, the plugin validator and `task check` pass.

## Out of Scope

Code review after implementation (the `adp` adversary owns that); a model pin per critic; changing
what `review-result` may hold (protocol repository).

## Open Questions

None.
