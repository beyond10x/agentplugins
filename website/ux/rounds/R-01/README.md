# R-01: adapter, baseline, gates and the first fixes

The first round of the `ui-iteration-loop` skill on the agentplugins docs site (`website/`).
AEP: `task:ux-round-r-01` under `story:docs-site-ux-loop`.

## What this round set up

- Adapter `ux/adapter.ts` (skill § 1): base path `/agentplugins/`; gating viewports 1366x768 and
  1280x720; measured-only 1440x900, 1280x800, 1024x768, 768x1024, 390x844; themes light and dark;
  always visible: navbar, search, theme toggle, and the sidebar on doc pages at desktop sizes
  (below 997 px: navbar, menu button, search). Gate exceptions held open by a finding:
  `KNOWN_PROBLEMS`.
- Anchor scenarios `ux/scenarios/<name>/scenario.json`: `landing`, `install`, `choose-a-plugin`,
  `reference-aep`, `tutorial-first-ess`. Each names a route, a persona, a task and its truth (the
  text on the page that answers it). Every round re-measures them; `ux/scenarios.spec.ts` checks
  the truth is on the page and that the keyboard reaches the content through the skip link.
- Gates (fast loop, every merge, hermetic, no keys): `ux/gates.spec.ts` (every sitemap page, both
  gating sizes, both themes: no sideways scroll, nothing past its box, always-visible elements at
  the top and at the end of the content, axe-core with tags `wcag2a, wcag2aa, wcag21a, wcag21aa,
  wcag22aa` and no serious or critical violation, once per theme at 1366x768; one page load per
  page, the second theme switched to with the navbar toggle), `ux/probes.spec.ts` (break tests: each check
  fails on a page built to break it), `ux/brand.spec.ts` (UXF-001), `ux/ratchet.test.ts` and
  `ux/findings.test.ts`, then the ratchet `ux/ratchet.ts` against `ux/baseline.json`.
- Built once, served by `ux/serve.ts` (a static file server, no Docusaurus start-up); the step runs
  under `ux/deadline.ts 120` and Playwright kills a test at 60 s and prints its ten slowest tests
  (`ux/slowest.ts`).
- Wiring: `task site-build` ends with `task site-ux` (`npm run ux` in `website/`); the
  `Documentation validation` workflow (`.github/workflows/pages.yml`) installs Chromium and runs
  `npm run ux` after its build.
- Round command: `task site-ux-round -- R-nn` measures every size and writes `report*.md`,
  `round*.json`, `shots/` and a row of `../trend.md`.

## Out of scope this round

- Recorded user sessions (rrweb recordings, the 90-second questionnaire, L6 live rounds) and the
  screen-only user bot with a model, or any other model call (L4 keyed replay, L5 probe): none
  run, none recorded. The skill's people steps start in a later round.
- The deterministic screen-only bot (§ 7, gate 4) and the two review agents (§ 8) did not run;
  the gates of this round are layout, always-visible, accessibility and both themes.
- Completion states and a guidance cue do not apply to a docs site: no gate 3 or gate 6.

## Caveat that bounds every result here

The public pages at beyond10x.github.io/docs/agentplugins/ are rendered by the Website repository
with the shared docs-system (`b10x.docs.yaml` publishes `website/docs/**` and `website/static/**`
only). `website/docusaurus.config.ts`, `website/src/` and the fixes of this round change the
repository's own build (`task site-build`, local preview, the validation workflow), not the public
site. A finding that also shows on the public site belongs in the Website or docs-system
repository.
