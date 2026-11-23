# R-01 triage

Rule: `priority = severity weight × frequency × confidence ÷ cost` (S1 = 9, S2 = 3, S3 = 1;
cost S = 1, M = 2, L = 4). With no recorded sessions, frequency is pages affected ÷ 16 pages
measured, at any measured size. Confidence is 1 for every finding: one source each (the gates);
the review agents and people did not run this round.

## Caveats

- The before-measurement (`report-before.md`, `round-before.json`) ran on `main` at 7d184a1. The
  mid and after measurements ran on `main` at dfc04b0, after the prose pull request #110 rewrote
  ten pages. Counts that depend on page text (links in text, layout shift) are not comparable
  one to one across that line.
- The before-measurement loaded each theme separately; mid and after load the first theme and
  switch to the second with the navbar toggle (one load per page, to keep the step inside its
  budget on a loaded host). `layout_shift.per_load` for the second theme is now the shift after
  the switch, not a load, so `site:all` and the anchors report the light load.
- `always_visible.hidden` 72 is UXF-002 (search, 64) and UXF-003 (sidebar on two pages, 8); both are
  held open by `KNOWN_PROBLEMS` and cannot grow.

## Did the last change sets move their metrics?

None: R-01 is the first round.

## Findings by priority

| finding | severity | frequency | cost | priority | in the change set |
|---|---|---|---|---|---|
| UXF-001 colours never applied, contrast fails | S2 | 16/16 | S | 3.00 | yes |
| UXF-002 no search | S2 | 16/16 | S | 3.00 | no: decision D1 (dependency refused by this session's permission check) |
| UXF-005 links told apart by colour alone | S2 | 13/16 | S | 2.44 | yes (exposed by the UXF-001 fix) |
| UXF-006 scrolling table not focusable | S2 | 1/16 | S | 0.19 | yes |
| UXF-004 layout shift on load | S3 | 5/16 | M | 0.16 | no: diagnose first (R-02) |
| UXF-003 no sidebar on two dated pages | S3 | 2/16 | S | 0.13 | no: needs `website/docs` front matter (prose owner) |
| UXF-007 landing action past its box at 390 px | S3 | 1/16 | S | 0.06 | yes |

## Change set (4 of at most 5)

1. UXF-001: `website/package.json` `browserslist` (native `@layer`). Target `axe.serious`
   `site:all` 32 → 0, `axe.serious_nodes` 287 → 0. Asset `ux/brand.spec.ts`.
2. UXF-005: `website/src/css/custom.css` underlines links in running text. Target
   `axe.serious` 52 page loads with `link-in-text-block` (mid build) → 0. Asset `ux/brand.spec.ts`.
3. UXF-006: `website/src/theme/MDXComponents.tsx` gives Markdown tables `tabIndex={0}`. Target
   `scrollable-region-focusable` at 1024x768 and 390x844 → 0. Asset `ux/narrow.spec.ts`.
4. UXF-007: `website/src/pages/index.module.css` lets the landing actions wrap. Target
   `layout.past_box` at 390x844 → 0. Asset `ux/narrow.spec.ts`.

Each asset was run red against the build before its fix and green after (UXF-001: computed
`--ifm-color-primary` `#3578e5` against the 7d184a1 build; UXF-005: the axe-core gate on the
mid build; UXF-006 and UXF-007: `ux/narrow.spec.ts` against the mid build).

## For the prose owner (`website/docs`)

- UXF-003: add `displayed_sidebar: docs` to the front matter of
  `docs/tutorials/first-ess-specification-2026-09-28.md` and
  `docs/tutorials/first-governed-plan-2026-09-28.md`, then remove the UXF-003 entry from
  `KNOWN_PROBLEMS` in `website/ux/adapter.ts` in the same change.
