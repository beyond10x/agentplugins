// Hard gates 1, 2 and 5 of the skill (§ 4) on every page of the built site's sitemap, at the
// gating sizes, in both themes: no sideways page scroll, nothing past its box, the always-visible
// elements in view and uncovered at the top and at the end of the content, and no serious or
// critical axe-core violation. One test loads a page once, measures each gating size in the first
// theme, switches to the other theme with the navbar toggle and measures again; axe-core runs once
// per theme, at the first size. Each size and theme writes `out/gates/<page>@<size>-<theme>.json`,
// which `ratchet.ts` and `round.ts` read.
import { expect, test } from '@playwright/test';
import type { Page } from '@playwright/test';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { alwaysVisible, knownFinding, pageLoads, RATCHET_VIEWPORT, sitemapPages, slug, THEME_TOGGLE, THEMES, vpName } from './adapter.ts';
import type { Theme, Viewport } from './adapter.ts';
import { axeSerious, layout, layoutShift, openPage, scrollToContentEnd } from './probes.ts';
import { settle } from './settle.ts';

const out = join(dirname(fileURLToPath(import.meta.url)), 'out', 'gates');
mkdirSync(join(out, 'shots'), { recursive: true });

export interface GateRecord {
  page: string;
  viewport: string;
  theme: string;
  gating: boolean;
  doc_hscroll: number;
  past_box: string[];
  scroll_boxes: string[];
  hidden_always_visible: string[];
  /** Measured once per theme and load, at the first size of the load (gate 5 holds per theme); empty at the other sizes. */
  axe_serious: string[];
  axe_measured: boolean;
  /** Since the load at the first size of a load; at a later size, since the page came to rest at it. */
  layout_shift: number;
  problems: string[];
}

async function measure(page: Page, route: string, vp: Viewport, theme: Theme, shiftBefore: number, runAxe: boolean): Promise<GateRecord> {
  const always = alwaysVisible(route, vp.width);
  const top = await layout(page, always);
  const cls = (await layoutShift(page)) - shiftBefore;
  const name = `${slug(route)}@${vpName(vp)}-${theme}`;
  // Shots are for a round's report; a merge gate does not take them.
  if (process.env.UX_ALL && (vpName(vp) === RATCHET_VIEWPORT || !vp.gating)) await page.screenshot({ path: join(out, 'shots', `${name}.png`) });
  const axe = runAxe ? await axeSerious(page) : [];
  await scrollToContentEnd(page);
  const bottom = await layout(page, always);
  const hidden = [
    ...new Set([...top.hidden_always_visible.map((h) => `top: ${h}`), ...bottom.hidden_always_visible.map((h) => `end: ${h}`)]),
  ];
  const problems = [
    ...(top.doc_hscroll ? [`page scrolls sideways by ${top.doc_hscroll}px`] : []),
    ...top.past_box.map((p) => `past its box: ${p}`),
    ...hidden.map((h) => `always-visible: ${h}`),
    ...axe.map((a) => `axe: ${a}`),
  ];
  const rec: GateRecord = {
    page: route, viewport: vpName(vp), theme, gating: vp.gating,
    doc_hscroll: top.doc_hscroll, past_box: top.past_box, scroll_boxes: top.scroll_boxes,
    hidden_always_visible: hidden, axe_serious: axe, axe_measured: runAxe, layout_shift: Math.round(cls * 1000) / 1000, problems,
  };
  writeFileSync(join(out, `${name}.json`), JSON.stringify(rec, null, 2));
  return rec;
}

/**
 * Switches the loaded page to `theme` the way a reader does, with the navbar's theme toggle (at
 * most three presses: the toggle may pass through "system"), so React-drawn parts such as code
 * blocks follow too. Below the desktop width the toggle is in the drawer; set the stored theme and
 * reload there.
 */
async function switchTheme(page: Page, route: string, theme: Theme): Promise<void> {
  await page.emulateMedia({ reducedMotion: 'reduce', colorScheme: theme });
  const toggle = page.locator(THEME_TOGGLE).first();
  if (!(await toggle.isVisible())) {
    await openPage(page, route, theme);
    return;
  }
  for (let i = 0; i < 3; i++) {
    if ((await page.evaluate(() => document.documentElement.dataset.theme)) === theme) break;
    await toggle.click();
  }
  await expect(page.locator('html')).toHaveAttribute('data-theme', theme);
  await settle(page);
}

// One test per page and load: the first theme is loaded, the second is switched to with the
// toggle, and each gating size of each theme writes its own record.
for (const route of sitemapPages()) {
  for (const sizes of pageLoads()) {
    test(`${slug(route)} @ ${sizes.map(vpName).join(' ')} ${THEMES.join('+')}`, async ({ page }) => {
      const failed: string[] = [];
      for (const [t, theme] of THEMES.entries()) {
        for (const [i, vp] of sizes.entries()) {
          await page.setViewportSize({ width: vp.width, height: vp.height });
          let shiftBefore = 0;
          if (t === 0 && i === 0) {
            await openPage(page, route, theme);
          } else {
            await page.evaluate(() => window.scrollTo(0, 0));
            if (i === 0) await switchTheme(page, route, theme);
            await settle(page);
            shiftBefore = await layoutShift(page);
          }
          const rec = await measure(page, route, vp, theme, shiftBefore, i === 0);
          if (vp.gating) failed.push(...rec.problems.filter((p) => !knownFinding(route, p)).map((p) => `${rec.viewport} ${theme}: ${p}`));
        }
      }
      // A round (UX_ALL) or a baseline measurement (UX_MEASURE) records every size and asserts nothing.
      if (!process.env.UX_ALL && !process.env.UX_MEASURE) expect(failed, failed.join('\n')).toEqual([]);
    });
  }
}
