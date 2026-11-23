// The ratchet (skill § 6): node ux/ratchet.ts [--round] [--update]. Exit 1 on a value worse than
// the baseline, an improvement not taken in, a missing value, or a baseline worse than main's
// without a decision id. Layout shift depends on the host's fonts and timing and is compared only in
// a round (--round). --update writes every improvement and every new key into the baseline and
// fails only on what it cannot take in.
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { metrics, readGates } from './metrics.ts';
import { compare, hardFailures, hostDependent, mainBaseline } from './ratchet-lib.ts';
import type { Baseline } from './ratchet-lib.ts';

const here = dirname(fileURLToPath(import.meta.url));
const file = join(here, 'baseline.json');
const update = process.argv.includes('--update');
const round = process.argv.includes('--round');

const EMPTY: Baseline = {
  about:
    'UX ratchet of the agentplugins docs site (ui-iteration-loop skill § 6) at 1366x768, light and dark summed: page:<anchor> per anchor scenario, site:all over every sitemap page. Lower is better for every key. Written by ux/ratchet.ts --update.',
  tolerances: {
    counts: '0',
    'layout_shift.per_load': '0.01, compared only in a round (host fonts and timing)',
  },
  scenarios: {},
};

const base = existsSync(file) ? (JSON.parse(readFileSync(file, 'utf8')) as Baseline) : EMPTY;
const main = mainBaseline(here, 'website/ux/baseline.json');
if (!main) console.log('ratchet: no baseline on main to compare with');
const current = metrics(readGates(join(here, 'out')));
if (!Object.keys(current).length) {
  console.log('ratchet: no gate records in ux/out/gates; run the Playwright gates first');
  process.exit(1);
}
const outcome = compare(base, current, main, round ? () => false : hostDependent);
if (update) {
  writeFileSync(file, JSON.stringify(outcome.next, null, 2) + '\n');
  const left = hardFailures(outcome);
  for (const f of outcome.failures) console.log(`${left.includes(f) ? 'FAIL' : 'updated'}: ${f}`);
  process.exit(left.length ? 1 : 0);
}
for (const f of outcome.failures) console.log(`FAIL: ${f}`);
if (!outcome.failures.length) console.log(`ratchet: ${Object.keys(base.scenarios).length} scenarios at or better than the baseline`);
process.exit(outcome.failures.length ? 1 : 0);
