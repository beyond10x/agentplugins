// Reduces the gate records in ux/out/gates to the ratchet's metric table: one `page:<slug>` scenario
// per anchor page and `site:all` over every page of the sitemap, at the ratchet size, both themes summed.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { RATCHET_VIEWPORT, scenarios, slug } from './adapter.ts';
import type { GateRecord } from './gates.spec.ts';
import type { Metrics } from './ratchet-lib.ts';

export const readGates = (out: string): GateRecord[] => {
  const dir = join(out, 'gates');
  return existsSync(dir)
    ? readdirSync(dir).filter((f) => f.endsWith('.json')).sort().map((f) => JSON.parse(readFileSync(join(dir, f), 'utf8')) as GateRecord)
    : [];
};

const empty = (): Record<string, number> => ({
  'layout.doc_hscroll': 0,
  'layout.past_box': 0,
  'layout.scroll_boxes': 0,
  'always_visible.hidden': 0,
  'axe.serious': 0,
  'axe.serious_nodes': 0,
  'layout_shift.per_load': 0,
});

function add(t: Record<string, number>, g: GateRecord): void {
  t['layout.doc_hscroll'] = Math.max(t['layout.doc_hscroll'], g.doc_hscroll);
  t['layout.past_box'] += g.past_box.length;
  t['layout.scroll_boxes'] += g.scroll_boxes.length;
  t['always_visible.hidden'] += g.hidden_always_visible.length;
  t['axe.serious'] += g.axe_serious.length;
  t['axe.serious_nodes'] += g.axe_serious.reduce((n, a) => n + Number(/\((\d+)\)/.exec(a)?.[1] ?? 1), 0);
  t['layout_shift.per_load'] = Math.max(t['layout_shift.per_load'], g.layout_shift);
}

export function metrics(gates: GateRecord[]): Metrics {
  const anchors = new Map(scenarios().filter((s) => s.anchor).map((s) => [s.route, `page:${slug(s.route)}`]));
  const m: Metrics = {};
  // An anchor that left the sitemap has no record; the ratchet reports its keys as missing.
  for (const g of gates.filter((g) => g.viewport === RATCHET_VIEWPORT)) {
    add((m['site:all'] ??= empty()), g);
    const k = anchors.get(g.page);
    if (k) add((m[k] ??= empty()), g);
  }
  return m;
}
