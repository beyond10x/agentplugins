---
format: aep.planning-md/1
id: story:non-interactive-golden-path
kind: story
status: draft
title: The golden path runs without an operator, and says so at every stop
summary: Every operator stop becomes a recorded approval-record with the reason non-interactive; the P1 golden-path case ended after step 1 with 10 gaps because nobody was there to continue.
owner: plugins
tags:
- evals
- golden-path
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 1
---
# Story: The golden path runs without an operator, and says so at every stop

## Outcome

A single headless session told to walk the golden path reaches the first implemented story: every place the page stops for an operator (approve the wave, read the critics, clear a blocker) becomes a recorded decision with the reason "non-interactive", instead of the end of the session.

## Context

P1's `golden-path-end-to-end` case (2026-09-03, $0.80, 173 events) adopted and scanned the repository, wrote "Step 1 — adopted, scanned" and ended: 7 rows ok, 10 gap (no wave, ESS or drive skill offered, no fan-out, no blocker with an edge, no review-results, store never validated). The page is written for an operator across turns; a headless run has nobody to pause for. The compared plugin records the same situation as an audit line (`/plan` and `/build` non-interactive bypass records); this store already holds the `approval-record` kind for it.

## Acceptance

- The planning and wave skills detect a non-interactive run (no operator turn available, or an explicit "run without stopping" instruction) and continue through each stop, recording an `approval-record` (or the closest kind the lifecycle admits) naming the stop and the reason.
- The golden-path page gains a "non-interactive" section with the exact instruction to give, and the eval case's task uses it.
- The P1 golden-path case re-run reaches the wave with a filed blocker and four review-results; the ten gap rows drop to the ones a headless run cannot satisfy, each named.
- Nothing is auto-approved silently: every bypass is a record `aep artifact list` shows.

## Out of Scope

Making `/drive` the entry (that is `story:drive-entry-skill`, blocked on the aep walk).

## Ambiguities

- `inferable` — the stop points: golden path steps 4–8 at 0.5.1; wave skill § propose → stop → run.

## Open Questions

None.
