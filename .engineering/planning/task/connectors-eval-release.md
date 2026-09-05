---
format: aep.planning-md/1
id: task:connectors-eval-release
kind: task
status: implemented
title: Fix portable readiness evaluation and release the connectors plugin
relations:
- derived_from: task:connectors-plugin
revision: 4
---
## Outcome
Fix both reviewed readiness-eval defects: Codex exec_command/cmd calls must satisfy doctor evidence, and CLI help must not be counted as a mutation. Cut an AgentPlugins release containing the portable connectors plugin and these fixes, update both user-global installations, and safely clean this session's managed worktrees.

## Scope
The connectors-readiness case and repository-owned Rust evaluation runner, regression tests, release versions and installation guidance. The command contract runs on replay and live eval output; AEP continues to own generic session trace predicates. No changes to Connector runtime, AEP's trace language, or credentials.

## Acceptance
Synthetic Claude and Codex traces exercising doctor plus help pass. Missing doctor, actual mutations, chained hidden mutations, and unreadable shell commands fail without executing transcript text. Both marketplace manifests and all versions align at release. task check, skill/plugin validators and task site-build pass. Verify the published tag/release and global installs before exact-id worktree cleanup.

## Authorization
Interactive user explicitly requested all review fixes, commit, release, and worktree cleanup. Existing global installation authorization covers upgrading the connectors plugin in both hosts.
