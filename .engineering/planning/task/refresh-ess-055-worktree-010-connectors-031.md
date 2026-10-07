---
format: aep.planning-md/3
id: task:refresh-ess-055-worktree-010-connectors-031
kind: task
status: implemented
title: Track ESS 0.55.0, Worktree 0.10.0 and Connectors v0.31.0
summary: Re-verify the skills against three new CLI releases and move verified.json and the eval pins
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-06T23:23:27Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-06T23:23:27Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-07T00:51:41Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":2}}}
---
## Context

On 2026-10-07 `agentplugins-check tools` failed with three problems against the newest releases:
ESS 0.55.0 (verified.json 0.53.0), Worktree 0.10.0 (0.9.0) and Connectors v0.31.0 (v0.28.0).
Every spelled command still matched (worktree 38, metaharness 13, connectors 20 against the
released source contract); only the versions were unverified. `agentplugins-check upstream`
also reported the eval pins moved (ESS 0.53.0, Worktree 0.8.2, Connectors v0.28.0) and three
cited issues closed: beyond10x/aep#60, beyond10x/ess#112, beyond10x/ess#460.

## Acceptance

`agentplugins-check tools` exits 0 with `verified.json` at ess 0.55.0, worktree 0.10.0 and
connectors v0.31.0, and the eval workflow installs the same three releases. Every
consumer-visible changelog entry of ESS 0.54.0 and 0.55.0, Worktree 0.10.0 and Connectors
0.29.0 to 0.31.0 is either reflected in the owning skill or recorded below as needing no change.
The `worktree-onboarding`, `ess-full-package` and `ess-tutorial` trials run isolated and do not
regress against `trials/baseline.json`. `task check` and `task site-build` pass on the candidate.

## Scope

Cited: verified.json, .github/workflows/eval.yml, plugins/ess/skills/**,
plugins/worktree/skills/managing-worktrees/SKILL.md, website/docs/plugins/worktree.md,
plugins/connectors/skills/integrating/**, website/docs/tutorials/first-ess-specification.md and
its trial fixture, trials/baseline.json, CHANGELOG.md.

## Classification

| release entry | resource | outcome |
|---|---|---|
| ESS `ess/23` constructs (re-key, `{subject: state}`, `deletes: instances:`, enum attributes, `affects: each:`, row-set selectors, whole-record `when_subject`) | specifying `later-formats.md`, `current-features.md`, `syntax.md`, retrofitting | documented with per-target limits; new `examples/shelves.yaml` checked by `tools` |
| ESS `ess specify formats` (ess#460) | specifying SKILL | workaround replaced by the command |
| ESS `ess generate --check` | specifying SKILL, upgrade, tutorial | documented |
| ESS validate `completeness`, `ESS-SPEC-012`/`017`, `ESS-ENTITY-019`/`ESS-COMMAND-019`, membership `type_mismatch`, `affects:` needs `where:`/`each:` | specifying SKILL, `syntax.md`, `later-formats.md` | documented |
| ESS suites `/44`, `/45`, decoys both sides, `ESS-SYNTH-003`/`004` | testing-conformance, `later-formats.md` | documented |
| ESS `generate_optional_<t>` port method | specifying SKILL, upgrade | documented with Rust and Go names |
| ESS CLI result `Json`, `ess-cli/1` trailing argument | upgrade | regenerate and `--check` drift noted; no skill teaches `ess generate cli` |
| ESS attribute change unclassified in diff | hardening `spec-diff.md` | documented |
| ESS mutation report history fix | hardening SKILL, `techniques.md` | `/1` corrected to `/4` |
| ESS interpreter, synthesis and wording fixes, guides, binding constants | none | internal or not taught here |
| ess#112 closed | hardening `reference-model.md` | requirement kept; `ESS-COMMAND-018` noted |
| aep#60 closed | none | cited only in the dated 2026-09-28 tutorial, kept as history |
| Worktree `sweep` | managing-worktrees, Worktree plugin page | documented |
| Worktree `retained_bytes`, `GitPort`/`WorktreeManager` API | none | library API, not taught |
| Connectors `oauth2_client_credentials` | integrating setup reference | documented |
| Connectors configuration-upgrade revalidation, `create_connection`, `insufficient_scope` | integrating setup reference | documented |
| Connectors `service_reason`, `429` retry and `retry_after_seconds`, `revalidate_connection` | integrating SKILL | documented |
| Connectors `operations list --family`, `connections launch` | integrating SKILL, setup reference | documented; no local deployment to run a launch, so no observed example |
| Connectors registry clock floor and `metadata_unavailable` once | connectors upgrade | documented |
| Connectors cost and memory fixes, Zendesk `per_page`, GitLab `issue.create`, feed engine | none | provider data and internals; operation schemas govern inputs |

## Verification

Isolated trials on 2026-10-07 against this tree (plugins 0.21.0), each checked by
`agentplugins-check trial-isolation` and recorded with `--write-baseline`:

| trial | measures | against the previous baseline |
|---|---|---|
| `worktree-onboarding` (worktree 0.10.0) | 11 tool calls | 18 → 11; the three passages the previous round quoted did not recur |
| `ess-full-package` (ess 0.55.0) | 103 tool calls; valid; 21 scenarios, 0 refusals; 0 unmapped; 9/9 outputs; cargo test 5/0/0 | tool calls +32%, inside the 50% limit |
| `ess-tutorial` (ess 0.55.0) | 71 tool calls; valid; 17 scenarios, 0 refusals; 5/5 outputs; cargo test 2/0/0, conformance 17/17 | cargo test 4 → 2: the page's implementation holds 2 tests, the earlier run's agent added 2 of its own |

`ess-full-package` quoted `ess:testing-conformance` calling `ErrUnsupported` a skip; the ESS 0.55.0
source records `unsupported` for observations and skips only entity setup and clock readings
(`ess-conformance/src/go/runtime.go:1196`, `:2736`, `reading.go:98`, `target.rs:1118-1123`), and
the skill now says so. It also found "release-resolution evidence" in `ess:specifying` unclear; the
sentence now names the plan file. The `ess-tutorial` run's five wording findings are
`task:ess-tutorial-wording-round-0211`.

## Release

Released in 0.21.0: PR #64, main `19acc43`, annotated tag `0.21.0` by `b10x-bot[bot]`. On that
commit every main workflow passed, Tools included (run 37551953157, 2026-10-07T00:28:21Z). The tag's
Release workflow (run 37552117166) passed at 2026-10-07T00:50:10Z; its `release-publication`
artifact's four archives matched `SHA256SUMS`, each held `b10x` and `LICENSE`, `SETUP.md` matched the
tag, and the Linux x86_64 binary printed `b10x 0.21.0`. The GitHub Release was published by
`b10x-bot[bot]` at 2026-10-07T00:50:50Z as latest, with six assets uploaded by the bot and digests
equal to the verified files.
