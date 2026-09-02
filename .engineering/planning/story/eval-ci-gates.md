---
format: aep.planning-md/1
id: story:eval-ci-gates
kind: story
status: implemented
title: CI validates the cases always and runs them live behind a label and a budget
summary: task check validates and replays for free; eval.yml runs live only with the run-eval label, diff-scoped, under a budget variable, with the bot secret.
owner: plugins
tags:
- ci
- evals
relations:
- decomposes: epic:ahead-of-the-alternative
- depends_on: story:plugin-eval-cases
revision: 4
---
# Story: CI validates the cases always and runs them live behind a label and a budget

## Outcome

Every PR proves the eval corpus is well-formed for free; a PR that touches an agent or skill can be run live by adding one label, within a budget the workflow refuses to exceed.

## Context

bdfinst's `agent-eval.yml` runs a model-free structural gate on every PR and a paid live gate only when the PR carries the `run-eval` label, diff-scoped to the agents the PR changed (`evals/README.md` at `dev-team-v13.0.0`). `task check` here must stay free (`AGENTS.md`). `aep eval run` refuses to spawn without `METAHARNESS_LIVE=1` and `--budget-usd`.

## Acceptance

- `task check` validates every case under `evals/` (schema and expectations parse; each names an existing agent or skill) and replays recorded streams with `--stream`.
- `.github/workflows/eval.yml` runs the live arm only with label `run-eval` or a manual dispatch, only for cases whose subject the diff touched, with `--budget-usd` from a repository variable, and posts the `aep eval matrix` table as a check summary.
- A live run that would exceed the budget is refused before spawning, and the check says so.
- The secret used is the bot's, never a personal key.

## Out of Scope

Blocking merges on a live result. The live gate informs; the structural gate blocks.

## Ambiguities

- `inferable` — `metaharness` must be on the runner's `PATH`; the workflow installs the pinned release.
- `requires-stakeholder-input` — the per-run budget variable. Decides: operator. Default: `EVAL_BUDGET_USD=20`.

## Open Questions

None.
