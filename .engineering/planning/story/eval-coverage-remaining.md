---
format: aep.planning-md/3
id: story:eval-coverage-remaining
kind: story
status: draft
title: The remaining plugin subjects get an eval case
summary: 23 agents and skills no eval case names, and the review-outcome assertion
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 1
---
# Story: the remaining plugin subjects get an eval case

## Outcome

Every agent and skill this repository ships is named by the `subject:` of at least one case under `evals/`, and one case asserts that a critic round leaves no `review-result` without an outcome.

## Context

Counted on 2026-10-08 from `evals/*/case.yaml` `subject:` fields against `plugins/*/agents/*.md` and `plugins/*/skills/*/`: 22 cases name 19 subjects; these 23 are named by none.

- agents: `aep:implementor`, `aep:plan-reviewer`, `aep:reverse-engineer`, `ess:author`, `ess:conformance`, `ess:retrofitter`
- skills: `aep:init`, `aep:investigating`, `aep:migrating`, `aep:upgrade`, `b10x:init`, `b10x:routing`, `b10x:upgrade`, `connectors:init`, `connectors:upgrade`, `ess:hardening`, `ess:init`, `ess:retrofitting`, `ess:testing-conformance`, `ess:upgrade`, `worktree:init`, `worktree:managing-worktrees`, `worktree:upgrade`

It carries what `story:plugin-eval-cases` and `story:review-outcome-recorded` left open when they were archived.

## Acceptance

- Each subject above is named by a case that `agentplugins-check` validates and whose recorded transcript replays offline.
- One case asserts that after a critic revision round every `review-result` carries `fixed`, `no-op` or `escalated`.
- `README.md` and `evals/README.md` state the counted coverage.

## Out of Scope

Judging behaviour by a model's opinion.
