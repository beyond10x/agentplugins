---
format: aep.planning-md/1
id: story:review-outcome-recorded
kind: story
status: draft
title: The critic step and the wave record each review outcome
summary: no-op, fixed or escalated written by the step that acted on a review-result; structured findings emitted by the rubric and the adversary.
owner: plugins
tags:
- critics
- review
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 1
---
# Story: The critic step and the wave record each review's outcome

## Outcome

Every `review-result` the plugins create later carries a `no-op | fixed | escalated` record, written by the step that acted on it, so `aep artifact review-value` has data.

## Context

The critic step (planning skill §7 at 0.4.0) creates one immutable `review-result` per verdict; the wave's adversary findings are handled by the route table (`wave/SKILL.md:375-380`). Neither records what became of a finding. `aep` `story:review-outcome-field` adds the record kind; this story writes it.

## Acceptance

- After a revision round, the critic step records `fixed` for each finding the revision addressed, `no-op` for each it did not need to, and `escalated` for each reported open to the operator.
- The wave records the outcome of each adversary finding when the route table's row is taken.
- The critic rubric and the adversary file emit the structured findings block that `aep` `story:structured-findings-on-review-result` parses.
- A case under `evals/` asserts that a critic round leaves no `review-result` without an outcome.

## Out of Scope

Deciding the outcome by a model's opinion; the outcome is what the step did.

## Ambiguities

- `inferable` — waits on the two `aep` stories named above.

## Open Questions

None.
