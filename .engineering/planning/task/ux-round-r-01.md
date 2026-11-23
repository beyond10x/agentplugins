---
format: aep.planning-md/3
id: task:ux-round-r-01
kind: task
status: draft
title: 'UX round R-01: adapter, baseline, gates and the first fixes on the docs site'
summary: Adapter, anchor scenarios, baseline, Playwright and axe-core gates, ratchet; findings UXF-001 to UXF-004.
tags:
- ux
relations:
- derived_from: story:docs-site-ux-loop
revision: 2
---
## Intent

Round R-01 of the docs-site UX loop: write the adapter, the anchor scenarios and the first
baseline; add the Playwright and axe-core gates and the ratchet, wired into `task site-build` and
the `Documentation validation` workflow; measure, file findings, and fix what the site's own files
can fix.

## Change set

1. UXF-001: `custom.css` colours never applied (Infima's polyfilled cascade layers beat them); a
   `browserslist` of browsers with native `@layer` support. Regression asset `website/ux/brand.spec.ts`.
2. UXF-005: links in running text told apart by colour alone once UXF-001 applied the site's
   purple; `custom.css` underlines them. Regression asset `website/ux/brand.spec.ts`.
3. UXF-006: a Markdown table that scrolls sideways takes no keyboard focus; `src/theme/MDXComponents.tsx`
   gives tables `tabIndex={0}`. Regression asset `website/ux/narrow.spec.ts`.
4. UXF-007: a landing page action 5 px past its row at 390 px; `index.module.css` lets the actions
   wrap. Regression asset `website/ux/narrow.spec.ts`.

## Left open

- UXF-002 search: waits for decision R-01 D1 (a local search plugin).
- UXF-003 no sidebar on the two dated tutorial recordings: front matter in `website/docs` (prose owner).
- UXF-004 layout shift on load: measured only; diagnose in R-02.

## Acceptance

`task site-ux` green; `website/ux/baseline.json` at or below the R-01 before-measurement on every key.
