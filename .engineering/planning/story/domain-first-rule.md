---
format: aep.planning-md/1
id: story:domain-first-rule
kind: story
status: active
title: An epic that introduces a new noun models it first
relations:
- decomposes: epic:second-adopter-feedback
revision: 3
---
# Story: An epic that introduces a new noun models it first

## Outcome

When an epic or story introduces a new entity, the planning skill has the agent draft an `ess/1`
domain for it (or import one with `aep reverse openapi` where an OpenAPI document exists) and cite it
from the epic, so a relation has a typed home before stories are written around it.

## Context

Neither this stack nor the third-party plugin prompts for domain relations during planning; this
stack is the only one with a typed place to put them, and nothing routes a planner there — the
`ess-schema` skill triggers only when a `system.yaml` already exists. The second adopter's failure was
exactly a relation with no home. Derived from the epic this decomposes.

## Acceptance

- The planning skill gains one rule: an epic or story whose outcome introduces an entity that no
  `ess/1` document in the repository declares is not decomposed until a draft domain exists, cited
  from the artifact body by path.
- The draft is a proposal, never a silent completion: every relation the agent could not read from
  code, an OpenAPI document or an existing artifact is written with an `UNMAPPED:` marker, in line
  with "imports never guess".
- The `ess-schema` skill's trigger description is widened to "a story or epic introduces an entity",
  and its instructions say how to start a domain from nothing (the minimal valid document, then
  `ess validate`).
- The skill validator and `task check` pass.

## Out of Scope

Generating code from the domain; changing ESS itself.

## Open Questions

None.
