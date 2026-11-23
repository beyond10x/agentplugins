// UXF-002: the navbar has a search box at every gating size, and a query finds the page it names.
// UXF-008: the box's shortcut hint passes axe-core contrast. The index is the one the build wrote
// (`@easyops-cn/docusaurus-search-local`), served with the site.
import { expect, test } from '@playwright/test';
import { ALWAYS_VISIBLE, VIEWPORTS, vpName } from './adapter.ts';
import { axeSerious, openPage } from './probes.ts';

const SEARCH = ALWAYS_VISIBLE.find(([label]) => label === 'search')![1];

for (const vp of VIEWPORTS.filter((v) => v.gating)) {
  test(`search at ${vpName(vp)}: the box is in view and "worktree" finds the Worktree reference`, async ({ page }) => {
    await page.setViewportSize({ width: vp.width, height: vp.height });
    await openPage(page, 'docs/install', 'light');
    const box = page.locator(SEARCH).first();
    await expect(box).toBeInViewport({ ratio: 1 });
    // UXF-008: the box's shortcut hint (`kbd`) is readable; with the plugin's colours it failed contrast in the light theme.
    expect((await axeSerious(page)).filter((v) => v.includes('kbd'))).toEqual([]);
    await box.click();
    await page.keyboard.type('worktree');
    // The index loads on the first focus; the options appear once it is there.
    const hit = page.getByRole('option').filter({ hasText: /worktree/i }).first();
    await expect(hit).toBeVisible({ timeout: 15_000 });
    await hit.click();
    await expect(page).toHaveURL(/\/docs\/plugins\/worktree/);
  });
}
