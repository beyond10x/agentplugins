// Break tests of the layout contract: each check of `probes.ts` fails on a page built to break it,
// and passes on a clean one. No server and no site: the pages are set inline.
import { expect, test } from '@playwright/test';
import { axeSerious, layout } from './probes.ts';

const ALWAYS: Array<[string, string]> = [
  ['bar', '#bar'],
  ['search', '#search'],
];

const page = (body: string): string =>
  `<!doctype html><html lang="en"><head><title>probe</title><style>body{margin:0;font:16px sans-serif}</style></head><body>${body}</body></html>`;

const CLEAN = page(`<nav id="bar" style="position:sticky;top:0;height:40px;background:#fff"><input id="search" aria-label="Search"></nav>
<main><h1>Clean</h1><p>Text.</p><pre tabindex="0" style="overflow-x:auto;width:200px"><code>${'x'.repeat(200)}</code></pre></main>`);

test('a clean page passes every check; a code block that scrolls is measured, not failed', async ({ page: p }) => {
  await p.setViewportSize({ width: 800, height: 600 });
  await p.setContent(CLEAN);
  const l = await layout(p, ALWAYS);
  expect(l.doc_hscroll).toBe(0);
  expect(l.past_box).toEqual([]);
  expect(l.hidden_always_visible).toEqual([]);
  expect(l.scroll_boxes.length).toBe(1);
  expect(await axeSerious(p)).toEqual([]);
});

test('sideways page scroll and content past its box are found', async ({ page: p }) => {
  await p.setViewportSize({ width: 800, height: 600 });
  await p.setContent(page(`<nav id="bar"><input id="search" aria-label="Search"></nav>
<main><div id="box" style="width:300px"><table><tr><td style="white-space:nowrap">${'wide '.repeat(300)}</td></tr></table></div></main>`));
  const l = await layout(p, ALWAYS);
  expect(l.doc_hscroll).toBeGreaterThan(0);
  expect(l.past_box.join()).toContain('div#box');
});

test('an always-visible element missing, scrolled away or covered is found', async ({ page: p }) => {
  await p.setViewportSize({ width: 800, height: 600 });
  await p.setContent(page(`<nav id="bar" style="height:40px"><input aria-label="Search"></nav>
<div style="position:fixed;top:0;left:0;width:100%;height:50px;background:#000"></div><main style="height:3000px">long</main>`));
  let l = await layout(p, ALWAYS);
  expect(l.hidden_always_visible).toEqual(['bar: covered', 'search: missing']);
  await p.evaluate(() => window.scrollTo(0, 1000));
  l = await layout(p, ALWAYS);
  expect(l.hidden_always_visible).toContain('bar: outside the viewport');
});

test('a serious axe-core violation is found', async ({ page: p }) => {
  await p.setContent(page(`<nav id="bar"><input id="search"></nav><main><p style="color:#bbb;background:#fff">low contrast</p></main>`));
  const v = await axeSerious(p);
  expect(v.some((s) => s.startsWith('color-contrast'))).toBe(true);
  expect(v.some((s) => s.startsWith('label'))).toBe(true);
});
