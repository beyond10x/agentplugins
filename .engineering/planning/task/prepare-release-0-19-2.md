---
format: aep.planning-md/3
id: task:prepare-release-0-19-2
kind: task
status: implemented
title: Prepare Agentplugins 0.19.2 and bot-owned publication
relations:
- delivers: task:cleanup-request-authorization
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T15:57:29Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T15:57:29Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-03T15:58:32Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Context

The operator requested a release carrying the completed cleanup authorization fix. Release versions must agree, the tag must name remote main, and publication must use the organization bot. The existing workflow publishes through the default Actions token, while this repository has no bot App credentials in Actions.

## Acceptance

The 0.19.2 candidate has aligned workspace, lockfile, plugin and skill versions, a dated changelog, passing gates, and a read-only workflow that prepares all release assets for bot-authenticated publication.

## Delivery

Merge the candidate through a green PR; tag exact remote main through the bot; verify the release workflow; download and verify its four archives, checksums and setup guide; create and publish the GitHub Release through the bot; verify the published assets and retire the managed worktree. Documentation delivery remains asynchronous.
