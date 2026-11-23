// Findings at sizes a merge does not gate (the round measures them), held by their own tests.
import { expect, test } from '@playwright/test';
import { alwaysVisible } from './adapter.ts';
import { axeSerious, layout, openPage } from './probes.ts';

test('UXF-006: a table that scrolls sideways is reachable by keyboard (docs/structure at 1024x768)', async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  await openPage(page, 'docs/structure', 'light');
  const scrolling = await page.evaluate(() =>
    [...document.querySelectorAll('.markdown table')].filter((t) => t.scrollWidth > t.clientWidth + 1).map((t) => t.getAttribute('tabindex')),
  );
  expect(scrolling.length, 'the page has no table that scrolls sideways at this size').toBeGreaterThan(0);
  expect(scrolling.every((tabindex) => tabindex === '0')).toBe(true);
  expect((await axeSerious(page)).filter((v) => v.startsWith('scrollable-region-focusable'))).toEqual([]);
});

test('UXF-007: the landing page actions stay inside their box (390x844)', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await openPage(page, '', 'light');
  const l = await layout(page, alwaysVisible('', 390));
  expect(l.doc_hscroll).toBe(0);
  expect(l.past_box).toEqual([]);
});
