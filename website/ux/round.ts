// The round command (skill § 3.2): node ux/round.ts R-nn [--label <text>]. Reads the gate records
// in ux/out/gates (a run with UX_ALL=1 measures every size) and writes, into ux/rounds/R-nn/:
// `round.json` (the ratchet metrics, the problems per page and size, the baseline they were compared
// with), `report.md` (the metric table and the problems), `shots/` (the anchor pages at the ratchet
// size, both themes) and a row of ux/rounds/trend.md.
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { RATCHET_VIEWPORT, scenarios, slug, THEMES } from './adapter.ts';
import { metrics, readGates } from './metrics.ts';
import type { Baseline } from './ratchet-lib.ts';

const here = dirname(fileURLToPath(import.meta.url));
const id = process.argv[2];
if (!id || !/^R-\d{2}$/.test(id)) {
  console.error('usage: node ux/round.ts R-nn [--label <text>]');
  process.exit(2);
}
const at = process.argv.indexOf('--label');
const label = at > 0 ? process.argv[at + 1] : '';
const out = join(here, 'out');
const gates = readGates(out);
if (!gates.length) {
  console.error('round: no gate records in ux/out/gates; run `UX_ALL=1 playwright test` first');
  process.exit(1);
}
const dir = join(here, 'rounds', id);
mkdirSync(join(dir, 'shots'), { recursive: true });
const baselineFile = join(here, 'baseline.json');
const baseline = existsSync(baselineFile) ? (JSON.parse(readFileSync(baselineFile, 'utf8')) as Baseline) : undefined;
const m = metrics(gates);
const problems = gates
  .filter((g) => g.problems.length)
  .map((g) => ({ page: g.page, viewport: g.viewport, theme: g.theme, gating: g.gating, problems: g.problems }));
const sizes = [...new Set(gates.map((g) => g.viewport))];
const name = `round${label ? `-${label}` : ''}`;
writeFileSync(join(dir, `${name}.json`), JSON.stringify({ id, label, pages: new Set(gates.map((g) => g.page)).size, sizes, metrics: m, problems }, null, 2) + '\n');

const keys = [...new Set(Object.values(m).flatMap((k) => Object.keys(k)))];
const cell = (sc: string, k: string): string => {
  const v = m[sc]?.[k];
  const b = baseline?.scenarios[sc]?.[k]?.value;
  return v === undefined ? '–' : b === undefined || b === v ? String(v) : `${v} (baseline ${b})`;
};
const lines = [
  `# ${id} ${label ? `(${label}) ` : ''}measurement`,
  '',
  `${new Set(gates.map((g) => g.page)).size} pages, sizes ${sizes.join(', ')}, themes ${THEMES.join(', ')}. Metrics at ${RATCHET_VIEWPORT}, both themes summed; lower is better.`,
  '',
  `| scenario | ${keys.join(' | ')} |`,
  `|---|${keys.map(() => '---').join('|')}|`,
  ...Object.keys(m).sort().map((sc) => `| ${sc} | ${keys.map((k) => cell(sc, k)).join(' | ')} |`),
  '',
  `## Problems (${problems.length} page loads)`,
  '',
  ...problems.flatMap((p) => [`- \`${p.page || '/'}\` ${p.viewport} ${p.theme}${p.gating ? '' : ' (measured only)'}:`, ...p.problems.map((x) => `  - ${x}`)]),
  '',
];
writeFileSync(join(dir, `report${label ? `-${label}` : ''}.md`), lines.join('\n'));

for (const s of scenarios().filter((s) => s.anchor)) {
  for (const t of THEMES) {
    const shot = `${slug(s.route)}@${RATCHET_VIEWPORT}-${t}.png`;
    const from = join(out, 'gates', 'shots', shot);
    if (existsSync(from)) copyFileSync(from, join(dir, 'shots', `${label ? `${label}-` : ''}${shot}`));
  }
}

const trend = join(here, 'rounds', 'trend.md');
const site = m['site:all'] ?? {};
const head = '| round | pages | doc_hscroll | past_box | scroll_boxes | always_visible.hidden | axe.serious | axe.serious_nodes | layout_shift.per_load |\n|---|---|---|---|---|---|---|---|---|\n';
const row = `| ${id}${label ? ` ${label}` : ''} | ${new Set(gates.map((g) => g.page)).size} | ${site['layout.doc_hscroll']} | ${site['layout.past_box']} | ${site['layout.scroll_boxes']} | ${site['always_visible.hidden']} | ${site['axe.serious']} | ${site['axe.serious_nodes']} | ${site['layout_shift.per_load']} |\n`;
writeFileSync(trend, (existsSync(trend) ? readFileSync(trend, 'utf8') : `# Trend: site:all at ${RATCHET_VIEWPORT}, both themes summed\n\n${head}`) + row);
console.log(`round: wrote ${dir}`);
