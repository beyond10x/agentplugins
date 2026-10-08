---
format: aep.planning-md/3
id: task:follow-releases-2026-10-08
kind: task
status: active
title: 'Follow the releases of 2026-10-08: aep 0.70.0, worktree 0.14.0, connectors v0.37.0, metaharness 0.9.3'
relations:
- decomposes: epic:ahead-of-the-alternative
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T22:38:10Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-08T22:38:10Z", actor: "human:timo", revision: 3}
---
# Task: follow the releases of 2026-10-08

## Outcome

`verified.json` and the eval tool versions name the newest released aep, worktree, connectors and
metaharness, each re-verified, and the skills teach what those releases changed.

## Context

Released since `verified.json` (aep 0.69.1, ess 0.56.0, worktree 0.13.0, metaharness 0.9.1,
connectors v0.33.0), read from `gh release list` on 2026-10-09:

- aep 0.70.0: `--planning-scope` on `aep plan store migrate git` and `aep plan reverse init`;
  the scope written from a linked worktree is fixed.
- worktree 0.14.0: `worktree prune-archives` (dry run by default, `--apply --id`).
- connectors v0.34.0 to v0.37.0: `help describe|invoke|serve`; a read timing out after dispatch
  reports `stage = dispatch`; Runpod and Slack catalog entries; v1alpha2 invoke on the HTTP host.
- metaharness 0.9.2, 0.9.3: documentation build and ESS 0.56.0 specification; binary unchanged.
  0.9.3 still links AEP 0.68.0 (`crates/metaharness-aep/Cargo.toml` rev `6d7a44d`, the 0.68.0
  tag), so the eval pair is AEP 0.68.0 with Metaharness 0.9.3.
- ess: 0.56.0 is still the newest.

## Acceptance

- `agentplugins-check tools` passes against the new `verified.json`.
- `plugins/worktree/skills/managing-worktrees` names `worktree prune-archives` as 0.14.0's
  `worktree skill` does, merged by hand.
- `plugins/connectors/skills/integrating/SKILL.md` matches `connectors --help` of v0.37.0.
- The worktree-onboarding trial passes against the baseline.
- `.github/workflows/eval.yml` names Metaharness 0.9.3, AEP 0.68.0, connectors v0.37.0 and
  worktree 0.14.0.
- `task check` passes.

## Also moved (agentplugins-check upstream, 2026-10-09)

- `.engineering/project.yaml` protocols eb003e4 (aep 0.69.1) to 91a8251 (aep 0.70.0).
- `website/package.json` Docs System 86cd6c6 to 0.8.0 (6169592), with its lockfile and
  `task site-build`.
- `.github/workflows/shared-gates.yml` gates 0f59bdb to 1a5ede1, after reading the diff of the
  called workflow. The generated docs-system and website workflow pins are Atlas's.
- https://github.com/beyond10x/aep/issues/60 is cited only in the dated tutorial
  `website/docs/tutorials/first-governed-plan-2026-09-28.md:228`, which already says AEP 0.65.0
  fixed it; no change.
