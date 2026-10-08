---
format: aep.planning-md/3
id: task:refresh-aep-069-ess-056-worktree-0121-connectors-032
kind: task
status: draft
title: Track AEP 0.69.0, ESS 0.56.0, Worktree 0.12.1 and Connectors v0.32.0
summary: Review each release against the skills, regenerate managing-worktrees from worktree 0.12.1, and move verified.json
revision: 2
---
## Context

`agentplugins-check upstream` on 2026-10-08 reported AEP 0.69.0 (pinned 0.68.0), ESS 0.56.0
(0.55.0), Worktree 0.12.1 (0.11.0) and Connectors v0.32.0 (v0.31.0) as newer than
`verified.json`, so the `Tools` check "Skills match the newest CLI releases" fails on every pull
request, among them the one carrying `story:ess-specifying-055-constructs`. Worktree changed the
text `worktree skill` prints in 0.12.0 and 0.12.1. The planning store's `protocols` source pins
AEP 6d7a44d; AEP 0.69.0 is f4363b7. Cited issues beyond10x/aep#60 and beyond10x/ess#112 are
closed.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at aep 0.69.0, ess 0.56.0, worktree
0.12.1 and connectors v0.32.0. `plugins/worktree/skills/managing-worktrees/SKILL.md` carries the
text `worktree skill` of 0.12.1 prints. Every consumer-visible entry of the four changelogs is
either reflected in its owning skill or recorded under Classification as needing no change. The
ESS trial round runs isolated against ESS 0.56.0 and does not regress against
`trials/baseline.json`. The pull request's `Gate` and `Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, .engineering/project.yaml, plugins/aep/**,
plugins/ess/**, plugins/connectors/**, plugins/worktree/**, website/docs/**, trials/baseline.json,
CHANGELOG.md, Cargo.toml, Cargo.lock, every plugin manifest and `**Skill version**` line.

## Classification

Filled in as each release is reviewed.
