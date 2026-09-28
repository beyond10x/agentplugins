---
format: aep.planning-md/3
id: story:worktree-plugin-by-pin
kind: story
status: archived
title: The b10x catalog lists worktree by pin, not by copy
summary: Rename the marketplace to b10x and replace the workspace-hygiene copy with a git-subdir entry pinned to a Worktree release, checked offline for shape and online for tag, commit, version and recency.
revision: 4
transitions:
- {from: "draft", to: "archived", at: "2026-09-28T07:49:41Z", actor: "human:timo", revision: 3}
---
# Story: the b10x catalog lists worktree by pin, not by copy

## Goal
The marketplace is renamed `b10x` and lists the `worktree` plugin as a `git-subdir` entry pinned
to a Worktree release tag and its full commit, replacing the `workspace-hygiene` copy. The copy
here shipped on this repository's cadence: `workspace-hygiene` 0.10.0 lacked the
`patch-equivalent` recovery-proof guidance that Worktree 0.5.0 generates.

## Acceptance
`task check` refuses a `worktree` entry that is not a `git-subdir` at
`https://github.com/beyond10x/worktree.git` path `plugins/worktree` with a bare release tag and a
40-hex commit; `agentplugins-check remote` refuses a commit that is not the tag's, manifests at
that commit that declare another version, and a tag that is not Worktree's newest release, and
runs on every pull request, `main` push and daily.

## Scope
- `.claude-plugin/marketplace.json`, `.agents/plugins/marketplace.json`
- `crates/agentplugins-check/src/{main,remote,evals}.rs`, `.github/workflows/remote-plugins.yml`
- `plugins/workspace-hygiene/` (removed), routing skill, wave skill, README, AGENTS.md, website

## Resolution

Not built; archived on 2026-09-28. The approach was reversed the day the story was drafted, and its
goal is met by another mechanism.

- Reversed: `beyond10x/worktree` commit `bec104b` (2026-09-24, "move the agent plugin to
  agentplugins") removed `plugins/worktree`, its marketplace and its manifest test. Only tag 0.6.0
  ever carried `plugins/`; 0.8.2, the newest, carries none, so there is no subdirectory to pin.
  That commit makes this repository the owner of the worktree skill text.
- Goal met: the skill can no longer lag the CLI unnoticed. `verified.json` pins `worktree` 0.8.2, and
  `agentplugins-check tools` fails when a newer worktree release is out. It runs on every pull
  request, `main` push and daily (`.github/workflows/tools.yml`, cron `17 5 * * *`).
- The rename half is done: both marketplace files are named `b10x`.

## Decision

2026-09-28, the operator: keep this story archived; do not build the pin again.

This repository built the pin once: `cefcd53` (2026-09-24) listed `worktree` as a `git-subdir` entry
at a Worktree tag and retired `workspace-hygiene`. `3c7d402` reversed it the same day ("every plugin
here"): `plugins/worktree` is owned here, and structure rule R1 has the checker refuse any plugin
source outside `./plugins`. Building the pin again would mean undoing R1 for `worktree` and
re-adding `plugins/worktree` to the worktree repository.
