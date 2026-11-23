// The adapter of the ui-iteration-loop skill for the agentplugins docs site (skill § 1): the base
// path, the viewports, the themes, the always-visible elements and the anchor scenarios. The
// generic files (`probes.ts`, `settle.ts`, `ratchet-lib.ts`, `serve.ts`, `deadline.ts`,
// `slowest.ts`, `port.ts`) take these values as arguments and import nothing from the site.
import { readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));

/** `baseUrl` of `docusaurus.config.ts`. */
export const BASE_PATH = '/agentplugins/';

export interface Viewport {
  width: number;
  height: number;
  gating: boolean;
}

/** Every merge holds the gates at the gating sizes; a round (UX_ALL=1) also measures the others. */
export const VIEWPORTS: Viewport[] = [
  { width: 1366, height: 768, gating: true },
  { width: 1280, height: 720, gating: true },
  { width: 1440, height: 900, gating: false },
  { width: 1280, height: 800, gating: false },
  { width: 1024, height: 768, gating: false },
  { width: 768, height: 1024, gating: false },
  { width: 390, height: 844, gating: false },
];

export const THEMES = ['light', 'dark'] as const;
export type Theme = (typeof THEMES)[number];

/** The size the ratchet compares; the other sizes are reported in a round. */
export const RATCHET_VIEWPORT = '1366x768';

/** Docusaurus moves the navbar links, the sidebar and the theme toggle into a drawer below this width. */
export const DESKTOP_MIN_WIDTH = 997;

export const vpName = (v: Viewport): string => `${v.width}x${v.height}`;

/**
 * The sizes each page load of a gate test measures, in order. The gating sizes share one load (the
 * ratchet size first, so its layout shift is that of the load); a size only a round measures has a
 * load of its own.
 */
export const pageLoads = (all = Boolean(process.env.UX_ALL)): Viewport[][] => {
  const first = (v: Viewport): number => (vpName(v) === RATCHET_VIEWPORT ? 0 : 1);
  const gating = VIEWPORTS.filter((v) => v.gating).sort((a, b) => first(a) - first(b));
  return [gating, ...(all ? VIEWPORTS.filter((v) => !v.gating).map((v) => [v]) : [])];
};

/** A label and a CSS selector. */
export type Target = [string, string];

/** The navbar's colour mode toggle. */
export const THEME_TOGGLE = '.navbar button[title*="mode" i], .navbar button[aria-label*="mode" i]';

/** In view at every scroll position on a desktop-sized window: navbar, search, theme toggle. */
export const ALWAYS_VISIBLE: Target[] = [
  ['navbar', 'nav.navbar'],
  ['search', '.navbar [role="search"], .navbar input[type="search"], .navbar input[aria-label*="search" i], .navbar .DocSearch-Button'],
  ['theme toggle', THEME_TOGGLE],
];

/** Below the desktop width: the controls that replace the navbar links and the sidebar. */
export const ALWAYS_VISIBLE_NARROW: Target[] = [
  ['navbar', 'nav.navbar'],
  ['menu button', 'button.navbar__toggle'],
  ['search', '.navbar [role="search"], .navbar input[type="search"], .navbar input[aria-label*="search" i], .navbar .DocSearch-Button'],
];

/** On doc pages at desktop sizes: the sidebar menu (its top; it is taller than the window). */
export const SIDEBAR: Target = ['sidebar', 'aside.theme-doc-sidebar-container nav.menu'];

export const isDocPage = (route: string): boolean => route.startsWith('docs');

/** The always-visible elements of `route` at a window `width` wide. */
export const alwaysVisible = (route: string, width: number): Target[] =>
  width >= DESKTOP_MIN_WIDTH ? [...ALWAYS_VISIBLE, ...(isDocPage(route) ? [SIDEBAR] : [])] : ALWAYS_VISIBLE_NARROW;

/**
 * Gate problems held open by a finding that this repository's UI files cannot fix. The gate does
 * not fail on a problem one of these matches; the ratchet still counts it, so it cannot grow, and
 * `findings.test.ts` fails when the finding named is closed or missing. Remove the entry in the
 * change that fixes the finding.
 */
export interface KnownProblem {
  finding: string;
  /** Routes it applies to; every route when absent. */
  route?: RegExp;
  problem: RegExp;
}

export const KNOWN_PROBLEMS: KnownProblem[] = [];

export const knownFinding = (route: string, problem: string): string | undefined =>
  KNOWN_PROBLEMS.find((k) => (!k.route || k.route.test(route)) && k.problem.test(problem))?.finding;

/** The built site the gates measure; a round may measure another build with UX_BUILD. */
export const buildDir = (): string => resolve(here, '..', process.env.UX_BUILD ?? 'build');

/** Every route in the built site's sitemap, relative to the base path, `''` for the landing page. */
export function sitemapPages(): string[] {
  const xml = readFileSync(join(buildDir(), 'sitemap.xml'), 'utf8');
  const routes = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => new URL(m[1]).pathname);
  return [...new Set(routes.map((p) => p.replace(BASE_PATH, '').replace(/\/$/, '')))].sort();
}

/** A file-name-safe id for a route. */
export const slug = (route: string): string => (route === '' ? 'landing' : route.replace(/\//g, '_'));

/** An anchor scenario (`ux/scenarios/<name>/scenario.json`): a page every round re-measures and a reader's task on it. */
export interface Scenario {
  name: string;
  route: string;
  anchor: boolean;
  persona: string;
  task: string;
  /** The text on the page that answers the task: the truth of its end state. */
  truth: string;
}

export function scenarios(): Scenario[] {
  const dir = join(here, 'scenarios');
  return readdirSync(dir, { withFileTypes: true })
    .filter((d) => d.isDirectory())
    .map((d) => ({ name: d.name, ...(JSON.parse(readFileSync(join(dir, d.name, 'scenario.json'), 'utf8')) as Omit<Scenario, 'name'>) }))
    .sort((a, b) => a.name.localeCompare(b.name));
}
