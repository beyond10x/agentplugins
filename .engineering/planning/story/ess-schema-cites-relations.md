---
format: aep.planning-md/1
id: story:ess-schema-cites-relations
kind: story
status: implemented
title: 'A domain relation is a relations: entry, and the planning skill cites it'
summary: 'ess-schema, guardrail 7, the decomposer and the golden path require a relation to be an ess/1 relations: entry once ESS ships the construct.'
owner: plugins
tags:
- ess
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 4
---
# Story: A domain relation is a `relations:` entry, and the planning skill cites it

## Outcome

When the decomposer or the domain-first guardrail says *this relation is decided*, the citation is a `relations:` entry in an `ess/1` document that `ess validate` accepted — not a sentence in a story.

## Context

0.4.0's decomposer classifies each relation `inferable` with a `path:line` or `requires-stakeholder-input`; guardrail 7 sends a new noun to ESS; `ess-schema` shows the minimal document. All three predate an ESS relation construct. `ess` `epic:entity-relations` adds one; this story makes the plugin require it once the construct exists.

## Acceptance

- `ess-schema`'s minimal document example carries one `owns` relation, and its refusal list gains the relation refusals.
- Guardrail 7 says a relation is modelled as a `relations:` entry, and `UNMAPPED:` covers a relation whose cardinality is not known.
- The decomposer's `inferable` citation for a relation must point at an `ess/1` document; a `path:line` into code is accepted only with the word `inferred`.
- The golden path's worked example gains the relation in its ESS step, with the CLI block produced by running it.

## Out of Scope

Requiring ESS for a repository with no domain model. The guardrail applies when a new noun appears.

## Ambiguities

- `inferable` — waits on `ess` `story:relations-in-the-domain-model` and the vocabulary decided by `decision-blocker:relation-vocabulary` there.

## Open Questions

None.
