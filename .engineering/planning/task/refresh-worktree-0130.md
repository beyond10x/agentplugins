---
format: aep.planning-md/3
id: task:refresh-worktree-0130
kind: task
status: implemented
title: Track Worktree 0.13.0
summary: managing-worktrees takes the 0.13.0 gc scope and tree references; verified.json moves
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T09:40:09Z", actor: "human:timo", revision: 4}
- {from: "proposed", to: "active", at: "2026-10-08T09:40:09Z", actor: "human:timo", revision: 5}
- {from: "active", to: "implemented", at: "2026-10-08T09:40:09Z", actor: "human:timo", revision: 6}
---
## Context

Worktree 0.13.0 shipped on 2026-10-08 (verified.json pins 0.12.1). `gc` without `--id` now
assesses only the current repository (`--scope profile` keeps the old selection), one tree
reference (id, path or unique directory name) works in `finish`, `discard-cache`, `archive`,
`gc --id` and `reconcile --id`, and `finish` prints `finished <id> <path>`.

## Acceptance

`worktree:managing-worktrees` carries the 0.13.0 text of `worktree skill` for gc scope and tree
references; `agentplugins-check tools` exits 0 with `verified.json` at worktree 0.13.0; the pull
request's `Gate` and `Tools` checks pass.

## Scope

Cited: plugins/worktree/skills/managing-worktrees/SKILL.md, website/docs/plugins/worktree.md,
verified.json, .github/workflows/eval.yml, CHANGELOG.md.

## Classification

| Change | Resource | Decision |
|---|---|---|
| `gc` default scope is the repository; `--scope profile` | managing-worktrees step 3, sweep bullet; plugin page | generator text taken; older releases named |
| one tree reference in finish, discard-cache, archive, gc --id, reconcile --id | managing-worktrees; plugin page | generator paragraph taken |
| `finish` prints `finished <id> <path>` | managing-worktrees; plugin page | included in the paragraph |
| `create --id` refuses a dot | managing-worktrees | included in the paragraph |

## Note

The checked-in skill is a curated variant (name `managing-worktrees`, cleanup authorization, sub-agent
tree ownership, Next section), so `worktree skill --out --force` would drop that text; the 0.13.0
paragraphs were taken from `worktree skill` output verbatim and merged.

## Verification

- worktree 0.13.0 x86_64 Linux archive from the release, checksum OK; `worktree --version` prints `b10x-worktree-cli 0.13.0`.
- `agentplugins-check tools`: exit 0; worktree 0.13.0 45 spelled commands checked.
- The worktree-onboarding trial was not rerun: the change adds 0.13.0 reference text and moves no step of the onboarding path.
