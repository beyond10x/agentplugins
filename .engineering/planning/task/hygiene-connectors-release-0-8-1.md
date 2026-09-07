---
format: aep.planning-md/1
id: task:hygiene-connectors-release-0-8-1
kind: task
status: implemented
title: Release Worktree hygiene and current Connectors guidance
owner: codex-hygiene-release
relations:
- informed_by: task:connectors-plugin
revision: 4
---
## Intent

Release Agentplugins 0.8.1 with the Worktree 0.4.0 generated skill and current Connectors 0.7.1 guidance. Refresh installation pins and preserve portability between Codex and Claude Code.

## Acceptance

- Worktree guidance is generated from the updated tool, including inspection, explicit leases and cleanup/handoff.
- Connectors guidance uses explicit deployment targets, current operation contracts, bounded collection and verified upgrade instructions from released CLI/source.
- Every plugin manifest, workspace version, changelog and current install pin agrees on 0.8.1.
- Skill, plugin, repository and website checks pass, followed by an annotated release and public documentation delivery.

## Authorization

The operator explicitly requested implementation and a new Agentplugins release, including the bundled Connectors skill, on 2026-09-07.
