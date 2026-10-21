---
format: aep.planning-md/3
id: story:scrub-other-org-sample-keys
kind: story
status: implemented
title: Replace another organization's sample ticket keys and paths
relations:
- decomposes: epic:release-0-24-0
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-21T20:47:58Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

Thirteen sample tracker keys of another organization remain in four files on `main` d4926a4
(`CHANGELOG.md`, `plugins/aep/skills/planning/SKILL.md`, `plugins/aep/skills/migrating/SKILL.md`, the
golden-path recorded transcript). Open PR #87 (`fix/leak-scrub-20261019b`, based on 0.23.2) replaces
them with `EX-n`, replaces a downstream sample path in `crates/b10x/src/plan.rs` with `/work/demo` and
local worktree paths in the recorded transcript with `<tree>`; its epic edit is already on `main`.

## Acceptance

`git grep -E 'DEV-[0-9]+'` finds nothing; the remaining changes of PR #87 are on the unit branch;
the recorded transcript's manifest digest matches; `task check` exits 0.

## Scope

`CHANGELOG.md`, `plugins/aep/skills/planning/SKILL.md`, `plugins/aep/skills/migrating/SKILL.md`,
`crates/b10x/src/plan.rs`, `evals/golden-path-end-to-end/recorded/*`.
