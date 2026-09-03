---
format: aep.planning-md/1
id: story:p1-lens-findings
kind: story
status: draft
title: 'Three P1 findings on the lenses: the ESS skill trigger, the adversary order, the scope selector'
summary: ess-schema not offered on a new noun; the adversary ran the suite before writing its test; every case gaps on the scope selector.
owner: plugins
tags:
- evals
relations:
- decomposes: epic:ahead-of-the-alternative
- depends_on: story:plugin-eval-cases
revision: 1
---
# Story: Three P1 findings on the lenses: the ESS skill trigger, the adversary's order, the scope selector

## Outcome

The next recording of the eight cases has no gap that the plugin itself caused.

## Context

P1 (2026-09-03): `ess-schema-new-entity` — `the-skill-was-offered` gap: a story introducing a new entity did not trigger the `ess-schema` skill (the trigger names a repository that already holds `system.yaml`, widened in 0.4.0 to "a story or epic introduces an entity" but not matched by this prompt). `adversary-tests-only` — `the-test-was-written-before-the-suite-was-run` gap: the adversary ran the suite before writing its failing test. Every case — `the-scope-was-actually-tested` gap: the case's scope selector does not match what the run touched; a case defect, not a plugin one.

## Acceptance

- `ess-schema`'s trigger matches a task that names a new noun without a `system.yaml` present; the P1 prompt is the regression fixture.
- `adversary.md` states the order — the failing test is written and its red output captured before the suite is run — and the case's row goes `ok` on re-record.
- The eight `expectations.trace.yaml` files' scope rows select what the run touched; the eight reports re-checked offline go to 0 gap on that row.

## Out of Scope

The negative rows (`nothing-was-moved` …): those are aep `story:absent-rows-decide-on-a-closed-stream`.

## Ambiguities

- `inferable` — the exact prompts and reports live on the operator's machine (`~/.cache/p1-fixtures/<case>/out`), not in this repository, until aep `story:redact-covers-home-and-user` lets them in.

## Open Questions

None.
