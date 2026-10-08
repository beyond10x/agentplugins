---
format: aep.planning-md/3
id: task:refresh-aep-069-ess-056-worktree-0121-connectors-032
kind: task
status: implemented
title: Track AEP 0.69.0, ESS 0.56.0, Worktree 0.12.1 and Connectors v0.32.0
summary: Review each release against the skills, regenerate managing-worktrees from worktree 0.12.1, and move verified.json
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T02:31:40Z", actor: "human:timo", revision: 5}
- {from: "proposed", to: "active", at: "2026-10-08T02:31:41Z", actor: "human:timo", revision: 6}
- {from: "active", to: "implemented", at: "2026-10-08T02:31:41Z", actor: "human:timo", revision: 7}
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

| release entry | resource | outcome |
|---|---|---|
| AEP 0.69.0 `evidence --suite` admits ESS 0.55.0 suites `/35`–`/45`, newer scenario keys, provenance members | `aep:planning` | no skill text said a fresh suite is refused or gave a workaround; no change |
| AEP 0.69.0 `govern … --task` loads the project's `protocols` source | none | no skill gave a `--root` workaround; no change |
| AEP 0.69.0 `plan store migrate git --verify` compares records field by field | `aep:upgrade` | says what `--verify` compares; the store-version tables `tools` checks unchanged |
| AEP 0.69.0 eval pin | `.github/workflows/eval.yml` | stays 0.68.0: Metaharness 0.9.1 embeds AEP 6d7a44d and `tools` checks the pair |
| AEP 0.69.0 protocols revision | `.engineering/project.yaml` | `protocols` at f4363b7; `aep plan artifact validate` valid |
| ESS 0.56.0 `ESS-COMMAND-004` branch order | `later-formats.md`, `ess:upgrade` | rule and reorder hint added; every example in `plugins/ess` validates |
| ESS 0.56.0 Entity Runtime `alphabet`/`.count` lowering | `later-formats.md` | per-target limits updated from the release notes; no CLI command lowers |
| ESS 0.56.0 Rust `exact-numbers` feature | `ess:specifying`, `ess:upgrade` | added; reproduced with `ess generate types --target rust` |
| ESS 0.56.0 `ess-output-state/3`; older `ess` refuses `/3` | `ess:specifying`, `ess:upgrade` | added; 0.55.0 `--check` on `/3` exits 1 `missing field 'root'` |
| ESS 0.56.0 owned file differs from `.ess-output` | `ess:specifying`, `ess:upgrade` | refusal and `ess generate output adopt` route added; route reproduced |
| ESS 0.56.0 `state.next` removal | `ess:specifying` | added |
| ESS 0.56.0 concurrent `ess generate` | none | no skill told readers to avoid it; no change |
| ESS 0.56.0 tutorials | ESS tutorial page, trial fixture, AEP tutorial fixture, drafted fixture | require ESS 0.56.0; 17 scenarios 0 refusals, conformance 2/2; no step changed |
| beyond10x/ess#112 closed | `hardening/references/reference-model.md` | text current (`ESS-COMMAND-018` reproduced); citation is a full URL |
| beyond10x/aep#60 closed | governed-plan tutorial (dated) | transcript kept; says AEP 0.65.0 fixed it |
| Connectors v0.32.0 JSON values in `--output json` | `connectors:integrating`, `writes-and-services.md`, `connectors:upgrade`, plugin page | "schema encoded as a string" replaced: read directly, never decode twice |
| Connectors v0.32.0 GitLab selection set moves the configuration revision | `setup.md`, `connectors:upgrade`, plugin page | bootstrap, `connections revalidate`, re-issued approval policies, `pending` until then |
| Connectors v0.32.0 feed profile capabilities and GitLab binding | `connectors:integrating` | declared capabilities explained with the GitLab profile as example |
| Connectors v0.32.0 catalog feed declaration fields and required `capabilities` | none | declaration authoring is not taught here |
| Connectors v0.32.0 ESS, Entity Runtime, Eventlog dependency moves | none | internal |
| Worktree 0.12.0 nested repository images, `worktree.archive/2` | `managing-worktrees`, plugin page | added, with the narrower `archive-unsupported-entry` list and `tar -xpf` restore |
| Worktree 0.12.0 Cargo `tmp/` discarded; 0.12.1 untagged `target/` recognised | `managing-worktrees`, plugin page | added |
| Worktree 0.12.0 plain `finish` refuses a dirty tree without a matching archive | `managing-worktrees` | added |
| Worktree 0.12.0 `gc --apply` makes read-only directories writable | `managing-worktrees` | added |
| Worktree `worktree skill` text of 0.12.1 | `managing-worktrees` | every 0.12.x behaviour taken in; this plugin's trial-settled passages kept (step 1 local-commit wording, detached HEAD, cleanup authorization, tree ownership, `--id` on gc, sweep detail, `## Next`) |
| shared Gates workflow pin 0f59bdb, Gates main d53a081 | `.github/workflows/shared-gates.yml` | not moved: `common.yml` is unchanged across the 12 commits |
| Docs System and Website workflow pins | generated files | Atlas reconciliation owns them |

## Verification

On 2026-10-08, against plugins 0.22.0, `agentplugins-check tools` exited 0: aep 0.69.0 218,
ess 0.56.0 88, worktree 0.12.1 42, metaharness 0.9.1 13 and connectors v0.32.0 21 spelled
commands (connectors as source contract only; the release has no binaries); syntax example 14
scenarios, 0 refusals; tutorial 17 scenarios, 0 refusals.

| trial | isolation | tool calls | measures | outcome |
|---|---|---|---|---|
| `worktree-onboarding-0220` | isolated, worktree 0.12.1 | 12 (baseline 9) | — | quoted the lifecycle-hooks sentence and the step 1 / step 5 pair, resolving both as the skill directs; accepted as baseline |
| `ess-tutorial-0220` | isolated, ESS 0.56.0 | 67 | valid, 17 scenarios 0 refusals, 5/5 outputs, cargo test 2/0 | three wording quotes appended to `task:ess-tutorial-wording-round-0211`; accepted as baseline |
| `ess-full-package-0220` | isolated, ESS 0.56.0 | 67 (baseline 103) | valid, 31 scenarios 0 refusals, 0 unmapped, 9/9 outputs, cargo test 3/0 (baseline 5/0) | the agent wrote 3 tests where the previous one wrote 5, none failing; it read `testing-conformance`'s `skipped` sentence as applying to the Rust runner, which has no skipped status in 0.56.0 (`ess-conformance/src/report.rs`); the sentence now names the Go runner and says what the Rust runner does; accepted as baseline |
