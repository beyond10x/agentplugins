---
format: aep.planning-md/3
id: task:refresh-resources-release
kind: task
status: active
title: Refresh current CLI resources and release Agentplugins 0.20.0
relations:
- informed_by: task:prepare-release-0-19-2
revision: 5
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

Implementation units and independent reviews are integrated on wave/current-resources, published as bot PR #59. Maintained release pins are current: AEP 0.68.0, ESS 0.53.0, Worktree 0.8.2, Metaharness 0.9.1, Connectors v0.28.0, Docs System 0.7.0. Generated documentation workflow pins remain Atlas-owned.

The offline gate passes 155 tests; current-tool verification passes with the exact compiled Connectors release, native ESS tutorial and new capability examples. Site typecheck/build pass. CI Gate, Tools, documentation build and security pass at 29767c7; the first Gate attempt received no runner and timed out, and its bot retry passed. CI source-check feedback was corrected: ambiguous console fences became bash in both tutorials and fixtures. The exact pinned source checker now passes locally.

All nine primary trial runs finished. Eight passed their full intended scope. The AEP tutorial completed reviewed planning and its first story, with 26 ESS scenarios Passed, 29 Unsupported explicitly deferred, and seven Cargo tests passing. Its missing Worktree setup caused a noncompliant host-worktree fallback. The fixture now installs Worktree, and a bounded rerun from the accepted plan is exercising managed coordinator/unit trees, leases, independent review, merge and recovery. That rerun is still active; do not claim the complete managed-wave trial has passed yet.

Trial feedback corrected frozen-marketplace upgrade planning, source-only Connectors installation, Rust test measurement, tutorial partial-wave coverage and recovery guidance. The old Go baseline is retained and higher Rust tutorial cost is documented. The managed trial additionally exposed runner directory scope: Taskfile now allows only its own sandbox root through --add-dir; focused verification remains pending. Two adversarial verifier false greens were reproduced and fixed. Latest documentation dependencies retain 32 npm audit findings without an available direct-package fix.

Release target is 0.20.0. Final report, bounded trial, final bot source publication, green main/tag checks and six published assets remain before completion. Documentation publication is asynchronous; no downstream release or deployment is included.
