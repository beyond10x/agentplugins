---
format: aep.planning-md/3
id: story:coordinator-returns-blockless-report
kind: story
status: implemented
title: The coordinator returns a review with no findings block to its author
relations:
- decomposes: epic:ahead-of-the-alternative
- depends_on: story:hardening-findings-block
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-09T01:19:34Z", actor: "human:timo", revision: 4}
- {from: "proposed", to: "active", at: "2026-10-09T01:19:34Z", actor: "human:timo", revision: 5}
- {from: "active", to: "implemented", at: "2026-10-09T01:23:08Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1}}}
---
# Story: the coordinator returns a review with no findings block to its author

## Outcome

In a store whose `.engineering/project.yaml` sets `findings_required_since`, a coordinator that
receives an adversary, security-review, critic or hardening report with no ` ```findings ` block
sends it back to its author and records only the corrected report, as it already does for a block
the CLI cannot parse. In every other store, and for a review recorded with `--prose-only`, a report
without a block is recorded as it is.

## Context

`plugins/aep/skills/implementing/references/wave.md` § recording a pass records each pass verbatim
through `new --from` and returns an unparsable block to its author; a report with no block at all
is recorded as it is. `plugins/aep/skills/planning/SKILL.md` § recording a critic round says the
same for critic rounds.

`aep` 0.71.0 (https://github.com/beyond10x/aep/releases/tag/0.71.0, 2026-10-09) ships both:

- `findings_required_since: YYYY-MM-DD` in `project.yaml`. Once the key is set, whatever the date,
  `aep plan artifact new review-result` refuses a body with no `findings` block, naming the two ways
  forward; `validate` counts an older block-less review as a problem unless it predates the date,
  was recorded `--prose-only`, or is superseded by a review with a block.
- `aep plan artifact new --prose-only <reason>`, which records a review with no block on purpose
  and writes the reason as front-matter `prose_only`. It works in every store.

The refusal, read from the 0.71.0 binary in a probe store: "`review-result:<id>` records no
`findings` block, and this store requires one on every review since <date>
(`findings_required_since` in project.yaml). Either give the findings — a fenced ```findings block
in the body, `[]` for a review that found nothing, or `--findings <file>` with a JSON array — or
record why there are none with `--prose-only <reason>`".

## Acceptance

- `wave.md` § recording a pass and `planning/SKILL.md` § recording a critic round say: where the
  store's project file sets `findings_required_since`, a report with no ` ```findings ` block goes
  back to its author with the refusal text, and only the author's corrected report is recorded.
- Both say a review that is prose by nature is recorded with `--prose-only <reason>`, and that a
  store without `findings_required_since` records a block-less report as it is.
- The text matches what `aep` 0.71.0 ships, checked against that release's notes and
  `aep plan artifact new --help` of the 0.71.0 binary. The skills name no CLI version (rule R5 of
  `website/docs/structure.md`); `CHANGELOG.md` names 0.71.0 as the release this text needs.
- `verified.json` pins `aep` 0.71.0 or a newer one.
- `task check` passes.

## Out of Scope

The hardening skill's block: `story:hardening-findings-block`. Setting `findings_required_since`
in this repository's own store.
