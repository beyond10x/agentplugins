// The anchor scenarios (ux/scenarios/<name>/scenario.json): each anchor page is in the sitemap, its
// truth is on the page and in view once scrolled to, and the keyboard reaches the content: the
// first Tab lands on the skip link, and taking it moves focus past the navbar.
import { expect, test } from '@playwright/test';
import { RATCHET_VIEWPORT, scenarios, sitemapPages } from './adapter.ts';
import { openPage } from './probes.ts';

const [width, height] = RATCHET_VIEWPORT.split('x').map(Number);
const pages = new Set(sitemapPages());

for (const s of scenarios().filter((s) => s.anchor)) {
  test(`scenario ${s.name}: truth on the page and keyboard reaches the content`, async ({ page }) => {
    expect(pages.has(s.route), `${s.route} is not in the sitemap`).toBe(true);
    await page.setViewportSize({ width, height });
    await openPage(page, s.route, 'light');
    const truth = page.getByText(s.truth, { exact: false }).first();
    await truth.scrollIntoViewIfNeeded();
    await expect(truth).toBeInViewport();

    await page.evaluate(() => window.scrollTo(0, 0));
    await page.keyboard.press('Tab');
    const first = await page.evaluate(() => document.activeElement?.textContent?.trim() ?? '');
    expect(first).toMatch(/skip to main content/i);
    await page.keyboard.press('Enter');
    await page.keyboard.press('Tab');
    const inNavbar = await page.evaluate(() => Boolean(document.activeElement?.closest('nav.navbar')));
    expect(inNavbar, 'after the skip link, Tab is still in the navbar').toBe(false);
  });
}
