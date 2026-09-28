---
format: aep.planning-md/3
id: story:aep-tutorial
kind: story
status: implemented
title: A public tutorial takes the ESS tutorial's library to a governed, critiqued plan and one implemented story with AEP
summary: 'tutorials/first-governed-plan: adopt, a new noun modelled in ESS first, decompose, scope, critic panel, one wave; real output; held true by tools and a trial.'
relations:
- informed_by: story:ess-tutorial-and-onboarding
scope:
- confidence: cited
  path: trials/aep-tutorial
- confidence: cited
  path: website/docs/tutorials
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-09-28T13:12:54Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-09-28T13:12:54Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-09-28T15:54:13Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1}}}
---
# Story: a public AEP tutorial

## Outcome
A developer who finished *Your first ESS specification* continues on the same library repository:
adopts an AEP planning store, asks for a feature that introduces a new noun, and gets a modelled
noun, a decomposed and scoped plan argued with by the critic panel, and one story implemented in a
wave with its evidence recorded. Every output on the page is from a real run.

## Context
- `website/docs/golden-path.md` records AEP 0.44.0 and ESS 0.5.1 output on an `aep.project/1`
  store; new stores are `aep.project/5` since aep 0.62.
- The ESS tutorial (0.17.0) ends with "a tutorial like this one for AEP follows".

## Acceptance
- `website/docs/tutorials/first-governed-plan.md` exists, recorded with the newest aep and ess.
- `agentplugins-check tools` checks every command the page spells against the newest releases.
- A trial gives a fresh agent only the page and reaches a valid store with the planned stories and
  one story implemented.
- `task check` and `task site-build` pass.

## Out of Scope
Driving a story with `metaharness aep drive` (blocked, see `story:drive-entry-skill`).
