---
format: aep.planning-md/3
id: story:changelog-keep-a-changelog
kind: story
status: implemented
title: Convert CHANGELOG.md to Keep a Changelog 1.1.0
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:58Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

`CHANGELOG.md` (1,540 lines) uses em-dash version headings, no change-type groups and no link
references; 85% of its entries exceed 120 characters. `crates/agentplugins-check/src/main.rs:1133`
requires the em-dash heading (`release verify`, `release.yml:30`).

## Acceptance

`release verify` and its tests expect `## [x.y.z] - YYYY-MM-DD`; `CHANGELOG.md` has `## [Unreleased]`,
version headings in that form, only the groups Added, Changed, Deprecated, Removed, Fixed and Security,
one one-line bullet per change of at most about 120 characters, breaking ones starting `**Breaking:**`,
and link references to the tags at the bottom; the detail dropped from long entries is listed in a
scratch file outside the repository; the release workflow's notes extraction reads one version's
section; `task check` exits 0.

## Scope

`crates/agentplugins-check/src/main.rs` and its tests, `CHANGELOG.md`, `.github/workflows/release.yml` if
its extraction needs it.
