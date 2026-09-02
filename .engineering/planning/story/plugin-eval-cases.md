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
revision: 3
---
# Story: Every agent and skill has an eval case beside it

## Outcome

A change to a critic, the decomposer, the `ess-schema` skill or the golden path can be run against a recorded case and judged by a trace specification, by anyone with the `aep` binary — and a change that breaks a charter turns a row red instead of being noticed by a reader.

## Context

`agentplugins-check` is structural (`crates/agentplugins-check/src/main.rs`, 196 lines at 0.3.1; the 0.4.0 validator adds the critic requirement). No skill has a behavioural check. `aep/conformance/eval/` holds 5 `eval-case/1` cases (`decomposer-charter`, `plan-reviewer-charter`, `development-honest`, `development-tests-after-the-code`, `release-progressive-honest`) run by `aep eval run --case <dir> --arm plugin --harness claude`, and `--corpus <DIR>` lets a corpus live elsewhere. bdfinst keeps 279 evals and 8,891 tests beside its plugin. The cases belong beside the subject they judge; `aep` `epic:self-evaluation` owns the runner and the two planning-agent cases already there.

## Acceptance

- `evals/` in this repository holds one `eval-case/1` case per agent and per user-facing skill: the four critics (each verdict is a `review-result`, none moves an artifact), the decomposer's relation enumeration (an undecided relation becomes a `decision-blocker`, not a story), `ess-schema` on a new entity, the golden path end to end, and the wave's adversary (tests only, no implementation edit).
- `aep eval run --corpus evals --workflow adp/default --arm plugin --harness claude --out <dir>` runs them; each case's expectations file is a `trace-spec/1` document.
- Each case records at least one real run's transcript so the offline replay (`--stream`) judges without spending.
- `README.md` § Evals names the command and the cost of one full live run.

## Out of Scope

Asking a model whether the agent behaved reasonably. Refused for the reason `aep` `epic:self-evaluation` gives.

## Ambiguities

- `inferable` — the case shape and the runner's flags: `aep eval run --help` at 0.42.0.
- `requires-stakeholder-input` — the budget per live run. Decides: operator. Default: `--budget-usd 5` per case.

## Open Questions

None.
