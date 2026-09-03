---
format: aep.planning-md/1
id: review-result:adversary-rename-round-1
kind: review-result
status: active
title: 'Adversary, round 1: rename plugins to product and verb'
relations:
- reviews: story:rename-plugins-to-product-and-verb
revision: 1
---
# Adversary, round 1 — story:rename-plugins-to-product-and-verb

Verdict: CONFIRMED (2 blockers). Cases executed 20 → 24, red 4. Origin: introduced 7 / pre-existing 0.
Agent: `adp:adversary` (opus). Cases added (red when written, kept): `src/evals.rs:1055`, `:1103`,
`src/main.rs:686`, `tests/the_rename_acceptance_statement.rs:126`.

Attacked, did not break: all 5 plugin manifests agree with directories and both marketplaces at
version 0.6.2; every `SKILL.md` `name:` matches its directory; no agent charter names a retired
sibling id; the sweep catches a retired name inside a fenced block and in a new website page and
skips `evals/*/recorded/` and `changes/`; the committed golden-path replay is `conformant` 27/27 and
the gate refuses the 2 gating gaps that appear if the 5 kept rows are renamed; website sidebars,
config, index and `b10x.docs.yaml` name only existing doc ids; the `beyond10x` routing table targets
exist; install blocks install the three new names.

```findings
- file: crates/agentplugins-check/src/evals.rs
  line: 591
  category: boundary
  severity: blocker
  verdict: CONFIRMED
  origin: introduced
  message: >-
    contradicted() counts only `gap` rows, so a replay aep prints as "undecided … (exit 3)" and
    exits 0 on passes the gate as a replayed transcript, and the doc comment's claim that on_unknown
    has already resolved the row is falsified by the row aep writes.
- file: evals/ess-specify-new-entity/case.yaml
  line: 15
  category: contract-drift
  severity: blocker
  verdict: CONFIRMED
  origin: introduced
  message: >-
    renaming subject.skills to `ess-specify:specify` silently disables aep's EVAL-RUN-018 preflight,
    which is keyed on `starts_with("ess-schema:")`, so the next labelled live run spawns and pays
    instead of refusing on a runner that never installs ess.
- file: .engineering/planning/story/rename-plugins-to-product-and-verb.md
  line: 63
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: >-
    the acceptance statement's rg predicate returns 51 lines rather than nothing, because the
    shipped retired_names sweep exempts wire ids, RECORDED_SPELLING lines and main.rs and the
    statement exempts none of the three.
- file: crates/agentplugins-check/src/main.rs
  line: 309
  category: mutant
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: >-
    the sweep reads file contents and never file paths, so a leftover plugins/<retired>/ directory
    or docs page whose body does not spell the name passes the gate, and no other check in the
    crate reads a sixth plugin directory.
- file: crates/agentplugins-check/src/evals.rs
  line: 598
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: >-
    contradicted() is the only document this crate reads without checking its format claim, so a
    report declaring a format it was not written against reads as nothing contradicted rather than
    as a refusal.
- file: CHANGELOG.md
  line: 34
  category: contract-drift
  severity: note
  verdict: NEEDS-CHANGE
  origin: introduced
  message: >-
    the changelog lists four exemptions for the retired-name sweep and the code has six, omitting
    the whole of main.rs and the RECORDED_SPELLING line marker.
- file: crates/agentplugins-check/src/main.rs
  line: 269
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: >-
    RECORDED_SPELLING exempts an entire line for all three retired names in any file, so it is a
    general escape hatch rather than the single-case marker its doc comment describes.
```
