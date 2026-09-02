---
format: aep.planning-md/1
id: epic:ahead-of-the-alternative
kind: epic
status: draft
title: What the 2026-09-02 comparison left open, and the parts only this stack can do
summary: Model pins, the wave skill on aep artifact waves, eval cases beside every agent and skill with free and label-gated CI, a /drive entry, relations cited from ESS, and recorded review outcomes.
owner: plugins
tags:
- bench
- comparison
revision: 1
---
# Epic: What the 2026-09-02 comparison left open, and the parts only this stack can do

## Outcome

An adopter choosing between these plugins and `bdfinst/agentic-dev-team` finds nothing the alternative does at plan or review time that this stack does not, and finds three things it cannot follow: a plan derived from a validated store, reviews that leave computable facts, and a driven run whose bounds are enforced rather than instructed.

## Why Now

`epic:second-adopter-feedback` (implemented, 0.4.0) closed the six adoption gaps the second adopter hit. The full review (`company-brain` `docs/reviews/2026-09-02-agentic-dev-team-vs-beyond10x.md` §10, moving to `beyond10x/bench`) lists what is still open on this side: no `model:`/`effort:` on any of the 10 agents; wave selection by pairwise prose (`plugins/adp/skills/wave/SKILL.md:149`); a one-sentence attack budget (`:421`); no behavioural test of any skill (`agentplugins-check` is structural); no recorded review outcome; no session path that runs `aep drive`. `bdfinst` ships 100 `dev-team-v*` tags since 2026-03-02, so parity decays without work behind it.

## Scope

The plugin-side half of four `aep` epics (`epic:wave-derivation`, `epic:review-facts`, `epic:self-evaluation`, `epic:reference-driver`) and of `ess` `epic:entity-relations`: model pins, the wave skill reading `aep artifact waves`, eval cases beside every agent and skill, CI gates for them, a `/drive` entry, the `ess-schema` rule citing `relations:`, and outcome recording. `story:finding-signature-ledger` (draft, instruction form) joins this epic.

## Out of Scope

- Session hooks. The decision at `aep` `README.md@0.14.0:66-87` stands: enforcement lives in the CLI and in driven runs, not in plugin hooks.
- Twenty-eight review lenses. One security critic may join the panel when a case shows it finds something the four do not.
- The bench itself. `beyond10x/bench` holds the corpora and runs; this epic makes the plugins measurable there.

## Risks

- Cross-store dependencies: five of these stories wait on an `aep` or `ess` verb. Each names the artifact it waits on in its body; none can be sequenced before that artifact is `implemented`.
- Eval cost: a live gate on every PR is a bill. The CI story gates live runs behind a label and a budget.

## Ambiguities

- `inferable` — the four critics are `plan-critic-{acceptance,design,scope,parallel-safety}` under `plugins/aep-planning/agents/`.
- `inferable` — the eval case shape is `eval-case/1` (`aep/conformance/eval/*/case.yaml`, 5 cases at `a054945`).
- `requires-stakeholder-input` — which model the critics pin to before a measurement exists. Decides: operator. Default: `sonnet`, `effort: high`, the pairing bdfinst uses for its five critics.

## Done When

Every story here is implemented or superseded by an `aep`/`ess` change that made it unnecessary, and the bench's first side-by-side run of both stacks on one corpus reads these plugins' cases from `evals/`.
