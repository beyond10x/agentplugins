---
format: aep.planning-md/1
id: story:codex-aep-bootstrap
kind: story
status: implemented
title: Make governed planning available before Beyond10x work starts
summary: Publish exact Codex plugin bootstrap commands and require AEP records for substantial cross-repository and release work.
relations:
- decomposes: epic:ahead-of-the-alternative
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: README.md
- confidence: cited
  path: plugins
- confidence: cited
  path: plugins/aep-planning/skills/planning/SKILL.md
- confidence: cited
  path: plugins/beyond10x/skills/beyond10x/SKILL.md
- confidence: cited
  path: website/docs/install.md
revision: 5
---
## Context

A Beyond10x workspace can require AEP in prose while the host still has an old immutable Agentplugins
marketplace pin or no `aep-planning` plugin installed. In that state, an agent can begin a substantial
cross-repository release from a transient chat plan and only discover the planning surface later.
Codex now exposes non-interactive marketplace and plugin commands, so the adopter instructions no
longer need to send users only to the Plugins UI.

## Acceptance

A fresh or stale Codex installation can follow exact release-pinned CLI commands to leave all five Beyond10x plugins installed and enabled, while the planning and repository instructions require substantial cross-repository or release/deployment work to be recorded in the repository's AEP store before implementation continues.

## Scope

- `AGENTS.md`
- `README.md`
- `website/docs/install.md`
- `plugins/aep-planning/skills/planning/SKILL.md`
- `plugins/beyond10x/skills/beyond10x/SKILL.md`
- release version and changelog
