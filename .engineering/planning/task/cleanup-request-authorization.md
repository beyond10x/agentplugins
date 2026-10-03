---
format: aep.planning-md/3
id: task:cleanup-request-authorization
kind: task
status: implemented
title: Execute authorized cleanup and restore the merge checks
refs:
- provider: github
  reference: https://github.com/beyond10x/agentplugins/pull/55
relations:
- informed_by: story:worktree-plugin-integrated
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T15:27:21Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T15:27:21Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T15:39:26Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":2}}}
---
## Context

An explicit cleanup request was followed by a redundant approval prompt. PR 55 corrects that guidance and the Rust 1.99 and CLI compatibility failures that prevented merging it.

## Acceptance

Explicit cleanup authorizes exact eligible removals without a second prompt, inspection requests remain read-only, and the repository and current CLI gates pass with verification pins backed by an isolated ESS full-package trial.

## Scope

Worktree cleanup and activity skills; obsolete AEP command inventory entries; existing Rust test assertion diagnostics; verified CLI pins; ESS synthesis language and Go suite placement guidance. Lifecycle safety checks and test assertions retain their semantics.

## Verification

Rust 1.99 task check passes all 145 tests and the offline plugin/eval gate. Command verification passes against AEP 0.68.0, ESS 0.52.0 and Worktree 0.8.2, including the ESS syntax example and executable tutorial. Plugin manifest validation passes.

Two isolated ESS full-package trials used ESS 0.52.0 and the candidate plugin. The first produced all nine outputs and passed 24 scenarios, with zero failures, skips or synthesis refusals and 37 tool calls. Its agent hand-transcribed its Go model after overlooking Go implementation synthesis. A direct released-CLI probe confirmed generated Go model and behavior/query contracts; guidance now names both implementation targets, ports and obligations, and the conformance package location.

The fresh second trial produced all nine outputs and passed 15 scenarios, with zero failures, skips, synthesis refusals or unmapped markers and 62 tool calls. Isolation and the existing baseline comparison passed for both trials. The second trial imported generated Go types and correctly wired the separately generated conformance module; it still wrote its storage adapter by hand, so these results establish command/output compatibility and conformance, not universal adherence to implementation-synthesis guidance. No generated suite was edited and no baseline was loosened.

Verification pins advanced only after the trials passed. Private transcripts and fixtures are retained in a credential-free archive with SHA-256 f2feae2d9b8e8242b141aa9f85d0ba331e831b27ea3b218f49d941771c81ca68. PR 55 carries publication and CI evidence.
