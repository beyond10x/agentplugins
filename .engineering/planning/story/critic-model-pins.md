---
format: aep.planning-md/1
id: story:critic-model-pins
kind: story
status: implemented
title: The four critics declare their model and effort
summary: 'model: and effort: on plan-critic-*.md, refused by agentplugins-check when absent; default sonnet/high until the review-value table exists.'
owner: plugins
tags:
- critics
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 4
---
# Story: The four critics declare their model and effort

## Outcome

A plan critique costs what it was chosen to cost, and the validator refuses a critic that forgot to say.

## Context

`grep '^(model|effort):'` over the 10 agent files at 0.4.0 (`21147b7`) matches nothing. bdfinst pins `model: sonnet`, `effort: high` on its five plan critics (`plan-review-*.md:5-6` at `dev-team-v13.0.0`) and reports the routing cost it measured. Until `aep` `story:review-value-table` exists there is no local number; the pin is the precondition for getting one.

## Acceptance

- `plan-critic-acceptance`, `-design`, `-scope`, `-parallel-safety` carry `model:` and `effort:` in frontmatter.
- `agentplugins-check` refuses a file under `plugins/aep-planning/agents/plan-critic-*.md` without both keys.
- The critic rubric states the pin and says it is a default awaiting the review-value table.
- `CHANGELOG.md` records it.

## Out of Scope

Pinning the decomposer, the adversary or the implementor. Those are the expensive, judgement-heavy roles and stay on the session's model until a table says otherwise.

## Ambiguities

- `requires-stakeholder-input` — the model. Decides: operator. Default: `sonnet`, `effort: high`.

## Open Questions

None.
