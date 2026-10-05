---
format: aep.planning-md/3
id: task:refresh-resources-release
kind: task
status: active
title: Refresh current CLI resources and release Agentplugins 0.20.0
relations:
- informed_by: task:prepare-release-0-19-2
revision: 4
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

Three implementation units are integrated: Connectors 92f498e (54 tests and exact v0.28.0 source installation with 20 runtime help checks); ESS 363ae4c plus a83926e (17 native scenarios, two Cargo tests, guard/freshness mutants rejected); verification 96fe417 plus e7f8860 (98 unit and 2 integration tests). Independent adversary found two verifier false-greens: equals globals hid commands and a crashed Cargo target followed a passing target. Both fixed and original adversarial probes now green. ESS independent probes rejected 17 unsupported, 17 errored and 14 failed scenarios; stale transport digest refused. No remaining blocking unit finding.

Coordinator refreshed AEP/Metaharness matching release pair, ESS and Connectors/Worktree eval prerequisites and child PATH, planning protocol, shared Gates workflow, Docs System dependency and lockfile, current and historical public tutorials, maintenance guidance and release metadata. Generated Atlas files remain owner-managed. A seeded upgrade trial found stale local marketplace refresh could not discover the replacement AEP plugin; source-switch planning fix reproduced red and passed 55 b10x tests, independent review and rerun pending.

The isolated nine-trial round is running. Worktree onboarding, ESS pipeline and ESS retrofit passed isolation and metrics; seeded upgrade's first run failed its actual task despite permissive metric summary and is being rerun. Its failure is preserved. Do not advance verification pins or declare release completion from trial metric exit codes alone. Release target remains 0.20.0.
