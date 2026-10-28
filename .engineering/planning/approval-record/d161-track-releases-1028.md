---
format: aep.planning-md/3
id: approval-record:d161-track-releases-1028
kind: approval-record
status: draft
title: Release tracking approved by the directing session
relations:
- decides: story:track-releases-1028
revision: 1
---
## Who approved

Session `b10x-chef-orchestrator`, in its message to this session on 2026-10-28, under the grant in the
operator's workspace rules (a message from that session naming a class-C item counts as the operator's
approval). The item, D161: "agentplugins main has been red since scheduled run 40678898507, job
"Skills match the newest CLI releases" ... The job names the fix: agentplugins-check tools plus an
ESS trial round, then bump verified.json. Follow the repository's own documented procedure for this
... In the same PR, close the AEP store's task:guard-bash-hook. It was merged in PR 98 and released
in 0.25.0 ... Merge on green through the bot route. Then, if the repository's release process calls
for a release, cut it."

## What was decided

One pull request through `b10x-gates` carrying the story and the close of `task:guard-bash-hook`,
merged by the bot when its checks are green, then a release through `AGENTS.md` Publishing. ESS
0.75.0, published after the failing run reported 0.74.0, joins as the newest release.
