---
format: aep.planning-md/3
id: story:settle-pr-87
kind: story
status: active
title: Close PR 87 as superseded and its tree
relations:
- decomposes: epic:release-0-24-0
- depends_on: story:scrub-other-org-sample-keys
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-21T08:39:48Z", actor: "human:timo", revision: 3}
---
## Context

PR #87 (`fix/leak-scrub-20261019b`, 2026-10-19) is open and red on "Skills match the newest CLI
releases"; `main` does not have its effect except the epic edit. `story:scrub-other-org-sample-keys`
carries its remaining change. Its tree `agentplugins-leak-scrub-2` is open.

## Acceptance

PR #87 is closed with a one-line comment naming the merge that superseded it; its branch is left;
the tree `agentplugins-leak-scrub-2` is finished with `--discard-cache --archive` and collected.

## Scope

GitHub PR #87; the managed tree `agentplugins-leak-scrub-2`. No repository file.
