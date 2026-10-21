---
format: aep.planning-md/3
id: story:publish-0-24-0
kind: story
status: active
title: Publish Agentplugins 0.24.0
relations:
- decomposes: epic:release-0-24-0
- depends_on: story:changelog-keep-a-changelog
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 3}
---
## Context

The connectors spelling change is breaking for anyone following the old skill text, so the next
release is 0.24.0. AGENTS.md § Publishing and § Release completion set the procedure.

## Acceptance

`CHANGELOG.md`, the workspace version, every plugin manifest and every `**Skill version**` line say
0.24.0; `task check`, `task site-build`, `agentplugins-check tools` and `upstream` pass on the release
commit; the annotated tag 0.24.0 is on `main`; the tag run's `release-publication` artifact is
verified and the release with its six assets is published by the bot; the published author, tag and
assets are verified.

## Scope

`CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `plugins/*/.claude-plugin/plugin.json`,
`plugins/*/.codex-plugin/plugin.json`, `**Skill version**` lines, the tag and the GitHub release.
