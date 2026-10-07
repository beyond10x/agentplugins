---
format: aep.planning-md/3
id: task:refresh-worktree-0110
kind: task
status: implemented
title: Track Worktree 0.11.0
summary: Re-verify the worktree skills against Worktree 0.11.0 and move verified.json and the eval pin
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-07T02:39:53Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-07T02:39:53Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-07T02:59:40Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":2}}}
---
## Context

Worktree 0.11.0 was released on 2026-10-07 (tag `0.11.0`, four archives and `SHA256SUMS`).
`verified.json` pins worktree 0.10.0 and `.github/workflows/eval.yml` installs 0.10.0, so the daily
`agentplugins-check tools` run reports the release as unverified. With no `--out`,
`worktree skill` now prints the skill and writes no file; 0.10.0 wrote `.agents/skills/worktree/`
into whatever checkout it ran in.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at worktree 0.11.0, and the eval workflow
installs worktree 0.11.0. Every consumer-visible entry of the Worktree 0.11.0 changelog is either
reflected in the owning skill or recorded below as needing no change. The `worktree-onboarding`
trial runs isolated against worktree 0.11.0 and does not regress against `trials/baseline.json`.
`task check` passes on the candidate.

## Scope

Cited: verified.json, .github/workflows/eval.yml, plugins/worktree/**, trials/baseline.json,
CHANGELOG.md, Cargo.toml, Cargo.lock, every plugin manifest and `**Skill version**` line.

## Classification

| release entry | resource | outcome |
|---|---|---|
| `worktree skill` without `--out` prints `SKILL.md` and writes no file; `--check` and `--force` need `--out` | none | no skill, page or setup step here runs `worktree skill`; the plugin ships `worktree:managing-worktrees` |
| `worktree --json skill` without `--out` refused as `operation-failed` | none | not taught here |
| `activate --install-agent-guidance` block says `worktree skill` prints to standard output | `worktree:init` | the sentence there (it writes a block pointing at `worktree:managing-worktrees` between its own markers) still holds; no change |

## Verification

On 2026-10-07 against this tree (plugins 0.21.1), `agentplugins-check tools` exited 0:
`worktree` 0.11.0, 42 spelled commands checked; the other CLIs unchanged.

| run | isolation | tool calls | outcome |
|---|---|---|---|
| `worktree-onboarding-0110` | refused: plugins 0.21.1, version under test read as 0.21.0 (the bump landed after the task read `Cargo.toml`) | not measured | discarded |
| `worktree-onboarding-0110b` | isolated, worktree CLI 0.11.0 | 9 (baseline 11) | quoted `managing-worktrees` step 1, "Commit and publish every wanted change", as conflicting with a commit-only request |
| `worktree-onboarding-0110c`, after the step 1 rewording | isolated | 9 | step 1 not quoted; recorded as the new baseline |

The `0110c` agent quoted two more passages and resolved both as the skills direct, so neither
changed: `worktree:init`'s workspace root (settled by the headless fallback), and
`managing-worktrees` "Never leave a tree silently active" beside "A tree belongs to the session
that created it" (ended with the step 5 handoff).

## Release

Released in 0.21.1: PR #67, main `0a0877e`, annotated tag `0.21.1` by `b10x-bot[bot]`. On that
commit every main workflow passed, Tools included (run 37563908395, 2026-10-07T02:51:52Z). The
tag's Release workflow (run 37564216949) passed at 2026-10-07T02:58:03Z; its `release-publication`
artifact's four archives matched `SHA256SUMS`, each held `b10x` and `LICENSE`, `SETUP.md` matched
the tag, and the Linux x86_64 binary printed `b10x 0.21.1`. The GitHub Release was published by
`b10x-bot[bot]` as latest, with six assets uploaded by the bot and digests equal to the verified
files.
