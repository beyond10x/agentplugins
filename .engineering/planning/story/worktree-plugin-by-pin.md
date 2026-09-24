---
format: aep.planning-md/1
id: story:worktree-plugin-by-pin
kind: story
status: draft
title: The b10x catalog lists worktree by pin, not by copy
summary: Rename the marketplace to b10x and replace the workspace-hygiene copy with a git-subdir entry pinned to a Worktree release, checked offline for shape and online for tag, commit, version and recency.
revision: 1
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
