---
format: aep.planning-md/3
id: task:recover-2026-10-04-refinement
kind: task
status: draft
title: Recover or retire the unmerged 2026-10-04 refinement work
summary: Four branches and four worktree archives hold four stories, two new skill references and a tools.rs rewrite that never reached main
revision: 1
---
## Context

On 2026-10-07 four managed trees from 2026-10-04 were retired. Their commits were never merged and
are not patch-equivalent to anything on `main`; they sit on base `4f529f1`, before the 0.20.0
refresh. The commits were pushed unchanged and the uncommitted state was archived:

| branch on origin | head | holds |
|---|---|---|
| `refine/current-capabilities-independent-docs` | `d05580d` | the integration merge of the three below |
| `impl/refinement-tools` | `759088d` | `tools.rs` and `upstream.rs` verify every catalogued CLI and upgrade behavior; `catalog.json`, `eval.yml`, `verified.json`, `crates/b10x/src/plan.rs` |
| `impl/refinement-skills` | `39ebe42` | skills check repository capabilities before offering upgrades; new `plugins/ess/skills/specifying/references/repository-preflight.md` and `plugins/ess/skills/testing-conformance/references/runner-capabilities.md` |
| `impl/refinement-docs` | `c39535d` | the planning commit: stories `capability-preflight`, `catalog-compatibility`, `independent-docs`, `upgrade-trials` |

Uncommitted work is in the worktree archives `wt-072468ad82ab` (59 tracked files changed),
`wt-a6538da4a5b9` (28 tracked files, a new `.github/workflows/b10x-docs-site.yml`),
`wt-7b93694fb7bf` (13 tracked files, review evidence) and `wt-4d31d6db3ac0` (a brief only), under
the worktree state directory's `archives/agentplugins/`. Restore one as the managing-worktrees
skill's *Audit and recovery* section says.

## Acceptance

Each of the four stories is either re-planned against current `main` (0.21.0) and delivered in a
wave, or archived with the reason it is no longer wanted. When that is settled, the four origin
branches are deleted and no worktree archive above is the only copy of wanted work.
