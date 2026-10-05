---
format: aep.planning-md/3
id: task:refresh-resources-release
kind: task
status: active
title: Refresh current CLI resources and release Agentplugins 0.20.0
relations:
- informed_by: task:prepare-release-0-19-2
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-05T20:23:43Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-05T20:23:43Z", actor: "human:timo", revision: 3}
---
## Context

The operator approved every finding in the Agentplugins resource review and requested a new release on 2026-10-05. The baseline is remote main 30acac4. Existing local drafts are outside this work.

## Acceptance

The marketplace references current releases and teaches their actual contracts. The ESS tutorial passes its generated suite under the latest ESS; obsolete specification and runner limitations are corrected and new capabilities have validated examples. Connectors uses the current CLI and its upgrade claims match installer behavior. Eval runners use source-matched AEP. Upstream detection covers every maintained pin, and generated website output does not pollute source checks. Offline checks, latest-tool checks, documentation build and isolated trials pass before verification pins advance. The new release has aligned versions, a bot-owned exact tag, successful release checks, verified archives, checksums and published assets. Documentation delivery is reported separately.

## Scope

Cited: plugins/ess, website/docs/tutorials/first-ess-specification, fixtures/library-reservations-drafted, trials: ESS resources and runnable tutorial.
Cited: plugins/connectors, website/docs/plugins/connectors.md, catalog.json, crates/b10x: current integration commands and installer discovery.
Cited: crates/agentplugins-check, .github/workflows/tools.yml: command and pin coverage, source traversal and regression checks.
Cited: .github/workflows/eval.yml, .github/workflows/shared-gates.yml, .engineering/project.yaml, .agents/skills/following-upstream, website/package.json, website/package-lock.json, README.md, AGENTS.md, verified.json, CHANGELOG.md, plugin manifests: integration, maintenance and release.

## Execution

AEP implementing skill 0.19.2. Operator's "do all of it, then cut a new release" authorizes the complete reviewed scope, implementation commits, integration merges, bot PR and publication, version bump, release tag and assets. One accepted work item; no decomposition critic panel is needed.

Three bounded implementation units use separate managed trees: ESS resources, Connectors migration, verification tooling. The coordinator alone owns the planning store, shared workflow/package pins, verification evidence, trials and release. All executable repository additions are Rust with clap derive; website builds remain Node. Existing Go tutorial implementation must be migrated to Rust rather than expanded.

Base: 30acac4. Integration branch: wave/current-resources. Integration tree id: wt-a631f55d9426. Scratch: agentplugins-release-020 under the operator cache (not public source). Builds stay per tree and bounded to four jobs. Disk at preflight: 27 GiB free. Workers return scoped commits and checks for adversarial review; coordinator integrates serially and runs the complete gate. Generated Atlas workflow pins are reported, not manually rewritten. No downstream releases or site migration.

## Progress

The integrated candidate updates AEP 0.68.0, ESS 0.53.0, Worktree 0.8.2, Metaharness 0.9.1, Connectors v0.28.0 and Docs System 0.7.0. The final upstream report confirms maintained release pins are current; two generated workflow pins remain owned by Atlas reconciliation. All executable additions are Rust. The current tutorials, runnable fixtures, CLI resources, installer source migration, verifier and eval prerequisites agree with those releases.

Independent reviews reproduced and fixed command-parser and Cargo-result false greens. Native tutorial guard/freshness mutants and unsupported/error targets are rejected. The seeded marketplace upgrade first failed, then passed after source-switch planning was corrected; native Claude and Codex probes preserve plugin state. Exact Connectors source installation and 20 runtime help checks passed.

Local final gate: 155 tests (98 checker unit, 2 integration, 55 installer); all 17 eval definitions validate and one recorded transcript replays. Tools verifies 213 AEP, 78 ESS, 36 Worktree, 13 Metaharness and 20 Connectors command spellings and runs the current ESS capability examples. Documentation typecheck/build pass. The exact CI documentation source checker passes 15 documents, five change records and 117 fences after correcting ambiguous tutorial language labels. Paid live CI evals were not run.

Nine primary trials and the bounded managed-wave rerun completed; actual results, retained old Go baseline and changed Rust baseline are recorded in changes/0.20.0-verification.md. The governed-plan trial implements only its first story, with 26 ESS scenarios passed and 29 explicitly pending for future stories. The corrected managed run passed 16 Cargo tests, nine deliberate mutants and isolation; both managed trees had their own leases, verified archive recovery and reviewed exact-ID GC. A separate permission probe passed. Setup refuses existing sandboxes. No claim of complete reservation-feature conformance or perfect model adherence is made; the report records a malformed-review serialization deviation and its instruction correction.

PR #59 publishes the source as the organization bot. Release 0.20.0 remains pending until the exact main tag, release checks, four platform archives, checksums, setup guide and GitHub Release are verified. Documentation publication is asynchronous. Latest documentation packages retain 32 npm audit findings (30 high, 2 low); no available direct-package fix is claimed.
