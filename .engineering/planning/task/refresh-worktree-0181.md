---
format: aep.planning-md/3
id: task:refresh-worktree-0181
kind: task
status: active
title: Track Worktree 0.18.1
summary: the skills are verified against Worktree 0.18.1, and verified.json moves
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-13T14:46:12Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-13T14:46:12Z", actor: "human:timo", revision: 3}
---
## Context

Worktree 0.18.1 was published after release 0.22.9, which verified the skills against Worktree
0.18.0. Its changelog moves every dependency to its newest release, raises `rust-version` from 1.85
to 1.89, and states that behaviour is unchanged. `b10x upgrade` offers a release only once
`verified.json` names it. AEP 0.71.2, ESS 0.59.0, Metaharness 0.9.3 and Connectors v0.48.0 are
still the newest releases.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at worktree 0.18.1. `worktree skill` and
every subcommand's `--help` of the released 0.18.1 binary are compared with 0.18.0's, and any
difference is reflected in `worktree:managing-worktrees` or recorded under Classification. The
isolated `worktree-onboarding` trial measures no worse than `trials/baseline.json`. The pull
request's `Gate` and `Tools` checks pass.

## Scope

Cited: verified.json, .github/workflows/eval.yml, CHANGELOG.md, Cargo.toml, Cargo.lock, every
plugin manifest and `**Skill version**` line.

## Classification

| Change | Resource | Decision |
|---|---|---|
| Dependencies to newest; `rust-version` 1.89 | none | internal; the archive route installs a prebuilt binary, and `worktree skill` and every `--help` equal 0.18.0's |
| Workflow pins of the worktree repository | none | internal to that repository |
