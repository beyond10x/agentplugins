// The ratchet's four failure rules (skill § 6), each shown failing, and a pass.
import assert from 'node:assert/strict';
import { test } from 'node:test';
import { compare, hardFailures } from './ratchet-lib.ts';
import type { Baseline } from './ratchet-lib.ts';

const base = (value: number, decision?: string): Baseline => ({
  about: 'test',
  tolerances: {},
  scenarios: { 'page:x': { 'axe.serious': { value, tolerance: 0, ...(decision ? { decision } : {}) } } },
});

test('equal values pass', () => {
  assert.deepEqual(compare(base(2), { 'page:x': { 'axe.serious': 2 } }).failures, []);
});

test('a worse value fails and --update cannot take it in', () => {
  const o = compare(base(2), { 'page:x': { 'axe.serious': 3 } });
  assert.match(o.failures.join(), /worse than the baseline/);
  assert.equal(hardFailures(o).length, 1);
});

test('an improvement not taken in fails; --update takes it in', () => {
  const o = compare(base(2), { 'page:x': { 'axe.serious': 0 } });
  assert.match(o.failures.join(), /take it in/);
  assert.equal(hardFailures(o).length, 0);
  assert.equal(o.next.scenarios['page:x']['axe.serious'].value, 0);
});

test('a missing value fails', () => {
  assert.match(compare(base(0), {}).failures.join(), /missing/);
});

test('a baseline worse than main fails without a decision id and passes with one', () => {
  const cur = { 'page:x': { 'axe.serious': 1 } };
  assert.match(compare(base(1), cur, base(0)).failures.join(), /names no decision/);
  assert.deepEqual(compare(base(1, 'R-02-D1'), cur, base(0)).failures, []);
});
