---
format: aep.planning-md/1
id: story:plugin-eval-cases
kind: story
status: active
title: Every agent and skill has an eval case beside it
summary: evals/ holds one eval-case/1 per critic, the decomposer, ess-schema, the golden path and the adversary, run by aep eval run --corpus evals and replayable offline.
owner: plugins
tags:
- evals
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 4
---
# Story: Every agent and skill has an eval case beside it

## Outcome

A change to a critic, the decomposer, the `ess-schema` skill or the golden path can be run against a recorded case and judged by a trace specification, by anyone with the `aep` binary — and a change that breaks a charter turns a row red instead of being noticed by a reader.

## Context

`agentplugins-check` is structural (`crates/agentplugins-check/src/main.rs`, 196 lines at 0.3.1; the 0.4.0 validator adds the critic requirement). No skill has a behavioural check. `aep/conformance/eval/` holds 5 `eval-case/1` cases (`decomposer-charter`, `plan-reviewer-charter`, `development-honest`, `development-tests-after-the-code`, `release-progressive-honest`) run by `aep eval run --case <dir> --arm plugin --harness claude`, and `--corpus <DIR>` lets a corpus live elsewhere. bdfinst keeps 279 evals and 8,891 tests beside its plugin. The cases belong beside the subject they judge; `aep` `epic:self-evaluation` owns the runner and the two planning-agent cases already there.

## Acceptance

The nine cases under `evals/` name, in their `case.yaml` `subject:` fields, 7 of this repository's 10 agents and 5 of its 9 skills: the agents `aep-drive:adversary`, `aep-drive:story-scoper`, `aep-plan:decomposer`, `aep-plan:plan-critic-acceptance`, `aep-plan:plan-critic-design`, `aep-plan:plan-critic-parallel-safety` and `aep-plan:plan-critic-scope`, and the skills `aep-drive:drive`, `aep-drive:wave`, `aep-plan:planning`, `connectors:connectors` and `ess-specify:specify`. That half is done and is not what remains.

What remains is this story's scope — one `eval-case/1` case naming each of the seven subjects no case names today:

- the agent `aep-drive:implementor`
- the agent `aep-plan:plan-reviewer`
- the agent `aep-plan:reverse-engineer`
- the skill `aep-plan:story-migration`
- the skill `beyond10x:beyond10x`
- the skill `beyond10x:plugin-creator`
- the skill `workspace-hygiene:worktree`

Each new case satisfies what the existing nine already satisfy:

- `aep drive eval run --corpus evals --workflow adp/default --arm plugin --harness claude --out <dir>` runs it, and its expectations file is a `trace-spec/1` document.
- It records at least one real run's transcript so the offline replay (`--stream`) judges without spending.
- Its `subject:` names an agent or skill that exists here, written as the harness qualifies it, so `agentplugins-check` resolves it.

Coverage is counted from `evals/*/case.yaml` `subject:` fields and from nothing else; `README.md` § Evals and `evals/README.md` state the same counted numbers and say so (org-state review 2026-09-15, decision 14). `README.md` § Evals names the command and the cost of one full live run.

## Out of Scope

Asking a model whether the agent behaved reasonably. Refused for the reason `aep` `epic:self-evaluation` gives.

## Ambiguities

- `inferable` — the case shape and the runner's flags: `aep eval run --help` at 0.42.0.
- `requires-stakeholder-input` — the budget per live run. Decides: operator. Default: `--budget-usd 5` per case.

## Open Questions

None.
