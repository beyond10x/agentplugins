---
format: aep.planning-md/1
id: story:decomposer-names-undecided-relations
kind: story
status: implemented
title: The decomposer names every undecided domain relation and stops there
relations:
- decomposes: epic:second-adopter-feedback
revision: 4
---
# Story: The decomposer names every undecided domain relation and stops there

## Outcome

An epic that introduces a new noun is decomposed into stories only for the parts whose domain
relations are decided; every undecided relation becomes a named `decision-blocker` with a `blocks`
edge, so the operator reads a question instead of a plan that improvised the answer.

## Context

The second adopter's plan was inconsistent about the relation between a new entity and an existing
one. The decomposer's contract already says undecided parts must be left out and the blocking
question named (its report's part 3), but nothing makes it enumerate the relations first, and nothing
classifies a gap as inferable-with-evidence versus needs-a-decision. The third-party plugin's
"ambiguity log" does exactly that classification and is the mechanism most likely to have surfaced
the adopter's gap. Derived from the epic this decomposes.

## Acceptance

- `plugins/aep-planning/agents/decomposer.md` gains a step, before any story is drafted, that lists
  every domain relation the epic implies (entity A to entity B, cardinality, ownership, lifecycle
  coupling) and classifies each as `inferable` — with the `path:line` or artifact that settles it —
  or `requires-stakeholder-input`.
- For each `requires-stakeholder-input` relation the agent files a `decision-blocker` through
  `aep artifact new decision-blocker … --relate blocks:<epic or story>` and drafts no story that
  depends on the answer; the report's third section lists them.
- An `inferable` relation is written into the story body that depends on it, with its citation.
- The agent remains draft-only: it moves nothing and edits nothing it did not create.
- The skill validator and `task check` pass.

## Out of Scope

Changing the artifact templates in the protocol repository (a sibling story there adds an
`## Ambiguities` section); any change to the planning skill's rules (the critic-panel story owns
that file).

## Open Questions

None.
