---
format: aep.planning-md/3
id: task:worktree-finish-discard-cache
kind: task
status: implemented
title: 'Track Worktree 0.9.0: finish with discard-cache and archive'
summary: Skills end work with finish --discard-cache --archive; sub-agents finish their trees
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-06T23:42:59Z", actor: "human:timo", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-10-06T23:42:59Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-10-06T23:42:59Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1}}}
---
## Context

On 2026-10-06 one machine held 343 active managed trees (72 GB). 105 of them differed from HEAD
only by ignored build output, and in 14 days of transcripts 142 `worktree finish` calls were refused
as `worktree-dirty`. The managing-worktrees skill told agents to delete only build directories
"owned by this task"; agents that could not establish ownership left the trees. Sub-agents created
trees and returned without finishing them (105 of 153 sub-agent sessions). Worktree 0.9.0 adds
`worktree discard-cache` and `worktree finish --discard-cache --archive`, which delete only cache
recognised by structure and archive everything else.

## Acceptance

The worktree skill, the AEP wave teardown, the governed-plan tutorial and the Worktree plugin page
end work with `worktree finish --discard-cache --archive <tree>`. No instruction asks an agent to
prove ownership of a build directory or to delete one by name. A sub-agent that creates a tree
finishes it or returns its id; the parent finishes returned trees. `verified.json` tracks Worktree
0.9.0 after `agentplugins-check tools` and the worktree-onboarding trial pass. Release 0.20.1 has
aligned versions.

## Scope

Cited: plugins/worktree/skills/managing-worktrees/SKILL.md, plugins/aep/skills/implementing/references/wave.md,
website/docs/tutorials/first-governed-plan.md, trials/aep-tutorial/fixture/tutorial.md,
website/docs/plugins/worktree.md, verified.json, CHANGELOG.md, plugin manifests, Cargo.toml, Cargo.lock.

## Progress

Released in 0.20.1 (main `efa67af`, PR #61). The tag's Release workflow (run 37449630075) passed at
2026-10-06T10:47:38Z, rerunning the gate; the GitHub Release was published by `b10x-bot[bot]` at
2026-10-06T10:48:44Z with 6 assets. `verified.json` moved to worktree 0.9.0 in that release.
