// UXF-001: the site's own colours (src/css/custom.css) apply in the built site, in both themes. They
// did not: the default browserslist made the build turn Infima's cascade layers into selectors two
// ids stronger than custom.css, so Infima's blue (3.0 to 4.2:1 on its backgrounds) won everywhere.
// UXF-005: links in running text carry an underline.
import { expect, test } from '@playwright/test';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { RATCHET_VIEWPORT } from './adapter.ts';
import { openPage } from './probes.ts';

const css = readFileSync(join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'css', 'custom.css'), 'utf8');

/** The custom properties `custom.css` declares in the block whose selector is `selector`. */
function declared(selector: string): Record<string, string> {
  const at = css.indexOf(`${selector} {`);
  if (at < 0) throw new Error(`custom.css has no ${selector} block`);
  const body = css.slice(css.indexOf('{', at) + 1, css.indexOf('}', at));
  return Object.fromEntries([...body.matchAll(/(--[\w-]+):\s*([^;]+);/g)].map((m) => [m[1], m[2].trim().toLowerCase()]));
}

const [width, height] = RATCHET_VIEWPORT.split('x').map(Number);

for (const [theme, selector] of [['light', ':root'], ['dark', "[data-theme='dark']"]] as const) {
  test(`custom.css colours apply in the ${theme} theme`, async ({ page }) => {
    const want = declared(selector);
    expect(Object.keys(want).length).toBeGreaterThan(0);
    await page.setViewportSize({ width, height });
    await openPage(page, 'docs/install', theme);
    const got = await page.evaluate(
      (names) => Object.fromEntries(names.map((n) => [n, getComputedStyle(document.documentElement).getPropertyValue(n).trim().toLowerCase()])),
      Object.keys(want),
    );
    expect(got).toEqual(want);

    // UXF-005: with the site's colours, a link in running text is told from the text by its
    // underline, not by colour alone (axe-core `link-in-text-block`).
    const plain = await page.evaluate(() =>
      [...document.querySelectorAll('.markdown p a:not(.hash-link), .markdown li a:not(.hash-link)')]
        .filter((a) => !getComputedStyle(a).textDecorationLine.includes('underline'))
        .map((a) => a.textContent?.trim() ?? ''),
    );
    expect(plain).toEqual([]);
  });
}
