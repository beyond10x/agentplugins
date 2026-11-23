---
format: aep.planning-md/3
id: story:stale-ess-versions-in-docs
kind: story
status: draft
title: Docs no longer present an old ESS version as the current one
summary: Pages that state ESS 0.56 or 0.60.0 beside ESS 0.90.0 are corrected or marked as dated evidence.
scope:
- confidence: cited
  path: website/docs/tutorials/first-ess-specification-2026-09-28.md
- confidence: cited
  path: website/docs/tutorials/first-governed-plan.md
revision: 4
---
# Story: docs no longer present an old ESS version as the current one

## Outcome
A reader of the website docs is never told that an old ESS version is the current one: a version
stated as current matches `verified.json`, and an old version appears only as dated evidence.

## Context
Found 2026-11-23 by the unslop pass (PR #110). Each claim checked with `git grep` on `origin/main`
(7d184a1):
- `website/docs/tutorials/first-governed-plan.md:105` says "In ESS 0.56, an existence-only
  input-related guard cannot accompany `wrong_state`", while `:19` of the same page targets ESS
  0.90.0. The sentence states a limitation for an old version, with no date.
- `website/docs/tutorials/first-ess-specification-2026-09-28.md:11` says the current Rust tutorial
  is "for ESS 0.60.0". `website/docs/tutorials/first-ess-specification.md:11` says ESS 0.90.0, and
  `verified.json` has `"ess": "0.90.0"`. The recording itself (ESS 0.38.0, Go) is labelled
  historical at `:9`; only the pointer sentence is wrong.
- `website/docs/golden-path.md:18` states that its recorded outputs use AEP `0.44.0` and ESS
  `0.5.1` and that they "remain dated evidence of that run"; the pasted output sits at `:43` and
  `:45`, and `:201` repeats ESS `0.5.1`. This page is already labelled and needs no change; it is
  listed so the acceptance check covers it.

## Acceptance
Every ESS or AEP version that a docs page states as current matches `verified.json`, or the
sentence is marked as dated evidence; dated recordings keep their pasted output byte for byte.

## Out of Scope
Rewriting the recorded output of any dated recording, including the console blocks of
`golden-path.md` and of the 2026-09-28 recording.

## Ambiguities
- Whether the `wrong_state` and `unknown_instance` limitation at
  `first-governed-plan.md:105-107` still holds in ESS 0.90.0: `requires-stakeholder-input`, the
  ESS maintainers decide; until answered the sentence is dated or reworded, not dropped.

## Open Questions
None.
