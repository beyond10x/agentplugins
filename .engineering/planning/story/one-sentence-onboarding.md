---
format: aep.planning-md/1
id: story:one-sentence-onboarding
kind: story
status: draft
title: One-sentence onboarding through b10x
summary: An agent given one sentence and the repository link asks, installs b10x, plans plugins and CLIs from recorded state, and applies them only after confirmation.
revision: 1
---
# Story: one-sentence onboarding through `b10x`

## Goal
A person gives Claude Code or Codex one sentence and this repository's link. The agent follows
`SETUP.md`: it asks before installing the `b10x` binary, then runs `/b10x:init`, which asks what the
person wants to do (plan and deliver work, write specifications, isolated Git checkouts,
integrations) and how to install the CLIs (prebuilt archives by default, or `cargo`). `b10x` reads
both hosts, the binaries on `PATH` and project settings, and plans exact actions: plugins from the
`b10x` marketplace, CLIs at their newest release, earlier installs under retired names and
marketplaces replaced in place. It applies them only after confirmation, and `/<plugin>:upgrade`
and the SessionStart check keep them current.
Measured before (2026-09-24): on the operator's machine `ess` 0.26.0 ran under plugin 0.30.0 and
`worktree` 0.4.1 under 0.6.0; nothing reported it.

## Acceptance
In an isolated headless trial (`task trial:sandbox`, `task trial:run`, `agentplugins-check
trial-isolation`), a home seeded with a pinned agentplugins 0.12.0 and a retired `aep-plan` plugin
ends after `/b10x:upgrade` with `aep@b10x` and `b10x@b10x` at the current version and no retired id
left; and a fresh agent given only a sentence and the link reaches a validated ESS specification
without cloning a product repository.

## Scope
- `crates/b10x/`, `catalog.json`, `SETUP.md`, `.github/workflows/release.yml`
- `plugins/*/skills/{init,upgrade}`, `plugins/b10x/hooks/hooks.json`
- `crates/agentplugins-check/` (concept rules, `tools`, `trial-isolation`)
- README, AGENTS.md, website install and plugin pages

## Provenance
- Trials 1–3 and rounds 3–4 (2026-09-24/25); agentplugins 0.12.0–0.14.7.
- Rebuilt on 2026-09-25 from a 2026-09-24 draft that could not be applied to the current store.
