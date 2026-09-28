---
format: aep.planning-md/3
id: story:ess-tutorial-and-onboarding
kind: story
status: active
title: A public tutorial takes a developer from nothing to a conforming ESS specification; onboarding pages match b10x
summary: New tutorials/first-ess-specification page recorded with real output, held to the newest CLIs by the checker and a trial; onboarding pages corrected; released as 0.17.0.
relations:
- informed_by: epic:ahead-of-the-alternative
scope:
- confidence: cited
  path: .github/workflows/tools.yml
- confidence: cited
  path: README.md
- confidence: cited
  path: SETUP.md
- confidence: cited
  path: b10x.docs.yaml
- confidence: cited
  path: crates/agentplugins-check
- confidence: cited
  path: plugins/ess
- confidence: cited
  path: trials/ess-tutorial
- confidence: cited
  path: verified.json
- confidence: cited
  path: website/docs
revision: 12
transitions:
- {from: "draft", to: "proposed", at: "2026-09-28T09:14:02Z", actor: "human:timo", revision: 11}
- {from: "proposed", to: "active", at: "2026-09-28T09:14:02Z", actor: "human:timo", revision: 12}
---
# Story: a public ESS tutorial, and onboarding pages that match b10x

## Outcome
A developer who reads the agentplugins docs can go from nothing to a validated ESS specification
and a passing Go conformance run, with an agent doing the writing, and every step shows the output
it really printed. The onboarding pages name the five plugins that ship and install them the way
`b10x` does.

## Context
Surveyed on `origin/main` d6c2ae1 (2026-09-28):
- No page in agentplugins, ess or aep walks from zero to a conformance run with real output; ESS
  `getting-started.md` stops at the built-in `billing` target.
- `website/docs/install.md:39-77` hand-installs AEP 0.55.0 and expects `protocol 0.55.0`;
  `golden-path.md:28` installs the `ess` plugin "from the ESS repository".
- `trust-and-scope.md:12-16` and `choose-a-plugin.md:12,32,50` name retired plugins ("AEP Plan",
  "AEP Drive", "ESS Specify", "Beyond10x"); the retired-name check matches hyphenated ids only
  (`crates/agentplugins-check/src/main.rs:332-453`).
- connectors is missing from `intro.md`, the docusaurus footer and `src/pages/index.tsx`.
- ess 0.38.0 is out and `verified.json` pins 0.37.0, so the Tools check fails on `main`.

## Acceptance
- `website/docs/tutorials/first-ess-specification.md` exists, recorded with ess 0.38.0; its
  committed specification validates, and `agentplugins-check tools` validates it and checks every
  command the page spells against the newest releases.
- `trials/ess-tutorial`: a fresh isolated agent given only the page reaches `validate: valid` and 0
  failed scenarios under `go test`.
- `agentplugins-check` refuses the retired display names in `website/` and `README.md`.
- `task check`, `task site-build` and `tools` pass; release 0.17.0 publishes its assets.

## Out of Scope
The AEP tutorial (later); the `website` repository's `experiences.json`; the ess repository's
own guides.
