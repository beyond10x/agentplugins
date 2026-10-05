---
format: aep.planning-md/3
id: task:refresh-resources-release
kind: task
status: implemented
title: Refresh current CLI resources and release Agentplugins 0.20.0
relations:
- informed_by: task:prepare-release-0-19-2
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-05T20:23:43Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-05T20:23:43Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-05T22:09:00Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":2}}, executor: "agent:agentplugins-release-020", correlation: "release-agentplugins-020"}
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

Completed the resource refresh and source release. PR #59 merged as 2420704264dbcbcffe4bf5338d54bd722af23f50 through b10x-bot[bot]; its tree exactly equals the tested bot candidate fe26c80e058cd34b416ee4b70143a7a6e73f1edc. All required main checks, latest-tool verification, site build and documentation source checks passed.

Bot-owned annotated tag 0.20.0 is 6c7ac2ab75a52d62cffea166fac2390308c20efd and resolves to that exact main commit. Release workflow 37379526813 passed its gate, all four platform builds and publication preparation. The tag receipt was also published as signed local evidence.

GitHub Release 404127516 was published by b10x-bot[bot] at 2026-10-05T22:06:58Z: https://github.com/beyond10x/agentplugins/releases/tag/0.20.0. Its six assets are the Linux x86_64/aarch64 and macOS x86_64/aarch64 b10x archives, SHA256SUMS and SETUP.md. Local checks verified all archive checksums, exact contents, architecture and license; the Linux binary reports b10x 0.20.0 and its embedded setup guide is present. All six public assets were downloaded again and compared byte-for-byte with the exact workflow artifact. The anonymous latest-release SETUP.md URL also matches. This is a verified source release, not merely a queued tag.

Verification details and limits are retained in changes/0.20.0-verification.md: 155 tests, current released CLI contracts, nine primary trials plus the corrected managed-wave follow-up, independent adversarial evidence, changed Rust baseline costs and a recorded review-serialization instruction deviation. Trial recovery bundles and logs are retained privately, and their managed trees were retired through the Worktree CLI. Source worktree cleanup follows publication of this record. The primary checkout was not changed.

Documentation publication remains asynchronous and was not verified. Two generated documentation workflow pins remain owned by Atlas reconciliation. The newest documentation packages still report 32 npm audit findings without an available direct-package fix. No downstream release, deployment or consumer pin promotion was performed.
