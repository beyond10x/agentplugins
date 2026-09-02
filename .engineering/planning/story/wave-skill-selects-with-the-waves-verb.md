---
format: aep.planning-md/1
id: story:wave-skill-selects-with-the-waves-verb
kind: story
status: implemented
title: The wave skill selects on aep artifact waves when the binary has it
summary: The selection step pastes waves, collisions and unassessed from the verb; pairwise prose only as a stated fallback.
owner: plugins
tags:
- wave
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 4
---
# Story: The wave skill selects on `aep artifact waves` when the binary has it

## Outcome

A coordinator proposing a wave pastes the verb's output — waves, collisions, unassessed — instead of judging pairs, and the operator sees which units rest on cited scope and which on inferred.

## Context

`plugins/adp/skills/wave/SKILL.md:138-157` selects on non-overlap by reading `## Scope` sections per pair. `aep` `story:artifact-waves-verb` computes it. The skill keeps the prose path for a binary that predates the verb, and says which path it took.

## Acceptance

- The selection step runs `aep artifact waves --format json` first; on `unrecognized subcommand` it falls back to the pairwise reading and says so in the proposal.
- The proposal names every collision the verb excluded and every unassessed story, verbatim.
- The scoper is dispatched for unassessed stories before the proposal, and writes `scope` through `aep artifact scope` (the `aep` `story:scope-as-a-typed-field` verb) as well as the section.
- The skill's failure-mode section gains the case "the verb and the prose disagree", with the rule that the verb wins and the disagreement is reported.

## Out of Scope

Changing the branch topology or the evidence rule.

## Ambiguities

- `inferable` — waits on `aep` `story:artifact-waves-verb` and `story:scope-as-a-typed-field`.

## Open Questions

None.
