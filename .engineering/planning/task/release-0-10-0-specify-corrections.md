---
format: aep.planning-md/1
id: task:release-0-10-0-specify-corrections
kind: task
status: draft
title: Release 0.10.0 shipping the coverage skill and the specify corrections
relations:
- informed_by: task:release-0-9-2-connectors-skill-v1-line
- implements: story:specify-corrected-by-a-real-adoption
revision: 1
---
## Intent

Ship what Unreleased holds — the new `ess-specify:coverage` skill, and the `specify` corrections a
real adoption paid for — so the install pins people copy point at instructions that ask what the
binary speaks instead of asserting a release number that has been wrong since ESS moved past it.

## Steps

- Workspace version, all twelve plugin manifests, the three `Skill version` lines, `README.md`,
  `website/docs/install.md` and `website/docs/plugins/connectors.md` move together to `0.10.0`;
  `Cargo.lock` follows. Minor rather than patch: a new skill is a new capability.
- `## [Unreleased]` becomes `## [0.10.0] — 2026-09-18`.
- `task check` green, then an annotated tag, then push, then the release.

## Evidence

`task check`: `valid: 9 eval case(s), 1 recorded transcript(s) replayed`,
`valid: marketplace beyond10x, 6 focused plugin(s)`. `release verify 0.10.0` refuses until the
annotated tag exists, which is the order it is meant to run in.
