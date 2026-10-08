---
format: aep.planning-md/3
id: story:coordinator-returns-blockless-report
kind: story
status: draft
title: The coordinator returns a review with no findings block to its author
relations:
- decomposes: epic:ahead-of-the-alternative
- depends_on: story:hardening-findings-block
revision: 2
---
# Story: the coordinator returns a review with no findings block to its author

## Outcome

In a store whose `.engineering/project.yaml` sets `findings_required_since`, a coordinator that
receives an adversary, security-review, critic or hardening report with no ` ```findings ` block
sends it back to its author and records only the corrected report, as it already does for a block
the CLI cannot parse. In every other store, and for a review recorded with `--prose-only`, a report
without a block is recorded as it is.

## Context

`plugins/aep/skills/implementing/references/wave.md:495-504` records each pass verbatim through
`new --from` and returns an unparsable block to its author; a report with no block at all is
recorded as it is. `plugins/aep/skills/planning/SKILL.md:502` says the same for critic rounds.
The planned `aep` release requires the block only in a store that opts in through
`findings_required_since` in its project file, and accepts a review marked `--prose-only`. That
release is not out (`aep` 0.70.0, 2026-10-08, does not ship it).

## Acceptance

- `wave.md` § recording a pass and `planning/SKILL.md` § recording a critic round say: where the
  store's project file sets `findings_required_since`, a report with no ` ```findings ` block goes
  back to its author with the refusal text, and only the author's corrected report is recorded.
- Both say a review that is prose by nature is recorded with `--prose-only`, and that a store
  without `findings_required_since` records a block-less report as it is.
- Both name the `aep` version that ships `findings_required_since` and `--prose-only`, checked
  against that release's notes and `aep plan artifact new --help`.
- `verified.json` pins that `aep` version or a newer one.
- `task check` passes.

## Out of Scope

The hardening skill's block: `story:hardening-findings-block`. Setting `findings_required_since`
in this repository's own store.
