---
format: aep.planning-md/3
id: story:docs-site-ux-loop
kind: story
status: draft
title: The docs site improves in measured UX rounds with gates on every merge
summary: 'ui-iteration-loop on website/: layout, always-visible and axe-core gates in both themes, a lower-is-better ratchet, one task per round.'
tags:
- ux
- website
revision: 1
---
## Intent

The repository's docs site (`website/`) improves in measured rounds of the `ui-iteration-loop`
skill: every merge passes the UX gates (no sideways scroll, nothing past its box, navbar, search,
theme toggle and sidebar in view, no serious or critical axe-core violation, both themes, 1366x768
and 1280x720), and the ratchet in `website/ux/baseline.json` never gets worse.

Each round is one task under this story; its change set holds at most 5 changes, each with a
regression asset.

## Acceptance

- `task site-ux` (and the `Documentation validation` workflow) runs the gates and the ratchet
  against the built site within 120 s, no test over 60 s, and fails on a gate problem or a
  worse ratchet value.
- Every closed S1 or S2 finding in `website/ux/findings/` names a regression asset that exists
  (`website/ux/findings.test.ts`).
- A round writes `website/ux/rounds/R-nn/` and a row of `website/ux/rounds/trend.md`.
