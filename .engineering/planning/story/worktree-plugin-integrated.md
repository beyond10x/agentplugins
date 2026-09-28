---
format: aep.planning-md/3
id: story:worktree-plugin-integrated
kind: story
status: implemented
title: The b10x catalog carries the worktree plugin and holds it to the newest worktree release
summary: worktree@b10x ships from ./plugins/worktree, owned here since 3c7d402; verified.json and the Tools check fail when a newer worktree release is out.
relations:
- supersedes: story:worktree-plugin-by-pin
scope:
- confidence: cited
  path: .agents/plugins/marketplace.json
- confidence: cited
  path: .claude-plugin/marketplace.json
- confidence: cited
  path: plugins/worktree
- confidence: cited
  path: verified.json
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-09-28T08:39:13Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":2}}}
- {from: "proposed", to: "active", at: "2026-09-28T08:39:14Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":2}}}
- {from: "active", to: "implemented", at: "2026-09-28T08:39:14Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":2}}}
---
# Story: the b10x catalog carries the worktree plugin and holds it to the newest worktree release

## Goal
The outcome `story:worktree-plugin-by-pin` asked for: the `b10x` marketplace installs the worktree
skills as `worktree@b10x`, and they cannot lag the `worktree` CLI unnoticed. Integrated by owning the
plugin here instead of pinning it from the worktree repository.

## What delivers it
- `3c7d402` (2026-09-24, released in 0.14.0): `plugins/worktree` is owned in this repository; both
  marketplace files list it from `./plugins/worktree`, and structure rule R1 refuses any other source.
  The worktree repository removed its own copy the same day (`beyond10x/worktree` `bec104b`).
- `verified.json` pins the worktree release the skills were last checked against (0.8.2), and
  `agentplugins-check tools` fails when a newer one is out. `.github/workflows/tools.yml` runs it on
  every pull request, `main` push and daily.
- The marketplace is named `b10x` (`.claude-plugin/marketplace.json`, `.agents/plugins/marketplace.json`).

## Acceptance
- The 0.16.0 release run published a `b10x` build whose catalog lists `worktree` from
  `./plugins/worktree`: https://github.com/beyond10x/agentplugins/actions/runs/36374131183
- The Tools run on the same commit checked 19 worktree commands against worktree 0.8.2 and passed:
  https://github.com/beyond10x/agentplugins/actions/runs/36374062848

## Scope
- `plugins/worktree/`, `.claude-plugin/marketplace.json`, `.agents/plugins/marketplace.json`
- `verified.json`, `crates/agentplugins-check/`, `.github/workflows/tools.yml`
