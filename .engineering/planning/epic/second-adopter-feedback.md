---
format: aep.planning-md/1
id: epic:second-adopter-feedback
kind: epic
status: implemented
title: What the second adopter could not do from the front door
summary: Install block, golden path, undecided relations named by the decomposer, a plan-time critic panel, and a domain-first rule — the 2026-09-02 adopter report, ranked by an independent review.
revision: 4
---
# Epic: What the second adopter could not do from the front door

## Outcome

An engineer who reads the release announcement can install the plugins with one copy-paste block,
follow one page from "feature idea" to a decomposed and critiqued plan, and get a plan that names
every undecided domain relation instead of improvising it. They do not need a second plugin for
spec → plan → implement.

## Why Now

On 2026-09-02 a second adopter reported, in the announcement thread for the 0.14.0 release of the
protocol repository, that they set the plugins up, tried to plan a CRUD feature that introduces a new
entity with a relation to an existing one, got a plan that was inconsistent about that relation, and
fell back to a third-party plugin (`bdfinst/agentic-dev-team`) for spec, plan and implementation. They
asked for two things: a multi-perspective iteration over a plan, and straight-to-the-point
instructions with a golden-path example. They also said they were not sure they had used the
plugins properly. The install path they would have followed pointed, since the repository split on
2026-09-01, at a repository that no longer carries a marketplace file. An independent review of the
third-party plugin against this stack (held by the operator) ranked the fixes; the stories under
this epic are that ranking.

## Scope

The install page and README of this repository; one new golden-path page on the public site; the
`aep-planning` decomposer, planning skill and new critic agents; the `ess-schema` skill trigger.
Every change is markdown; the skill and plugin validators and `task check` gate it.

## Out of Scope

Session hooks that block or freeze scope — the reason hooks left the plugin is recorded in the
protocol repository's 0.14.0 README and still holds. Deterministic wave derivation and a preflight
verb are Rust and belong to the protocol repository's store. Vendoring any file from the third-party
plugin.

## Risks

A critic panel that restates the planning vocabulary in prose would violate the "rules only, discover
the rest" contract of the planning skill; the rubric must stay rules. A golden path written from the
authors' machines is the failure mode the adopter reported; it must be walked once on a tree that is
not ours before it ships.

## Done When

The five stories are implemented, `task check` and `task site-build` pass, a release carries them,
and the adopter has been pointed at the golden-path page in the thread that started this.
