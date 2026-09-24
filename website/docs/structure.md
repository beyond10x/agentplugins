---
sidebar_position: 5
title: How the plugins are organised
---

# How the plugins are organised

Eight rules decide where everything goes. `task check` enforces each one; a change that breaks a rule
does not merge.

| rule | what it says |
|---|---|
| **R1 marketplace** | One marketplace, `b10x`, in both the Claude Code and Codex formats. It names no version of anything it points at. |
| **R2 plugin** | One plugin per product. The plugin, the product and the CLI it drives share one name: `aep`, `ess`, `worktree`, `connectors`. The front door is `b10x`, with the `b10x` CLI. |
| **R3 skill** | A skill is an activity, named in `-ing` form, one or two words: `aep:planning`. No skill is named after its plugin. |
| **R4 agent** | An agent is a role: `implementor`, `decomposer`. Exactly one skill of the same plugin owns it and lists it under `## Agents`. |
| **R5 content** | A skill describes the CLI of its plugin. A plugin that ships from its product's repository has that CLI's version. |
| **R6 distribution** | `SETUP.md` and `b10x setup` install everything. Retired names live only in `catalog.json`, and setup migrates them. |
| **R7 references** | Every `<plugin>:<skill-or-agent>` written in this repository names a file that exists. |
| **R8 docs** | One README row, one page under `plugins/` and one sidebar entry per plugin. The README is one paragraph and that table. |

## The plugins today

| plugin | skills | agents | lives in |
|---|---|---|---|
| `b10x` | `installing`, `routing`, `authoring-plugins` | — | this repository |
| `aep` | `planning`, `migrating`, `implementing` (wave or drive mode) | `planning`: decomposer, four plan critics, plan reviewer, reverse engineer · `implementing`: story scoper, implementor, adversary, security reviewer | this repository |
| `ess` | `specify`, `retrofit`, `coverage`, `ess` | author, retrofitter, conformance | [ESS repository](https://github.com/beyond10x/ess) |
| `worktree` | `worktree` | — | [worktree repository](https://github.com/beyond10x/worktree) |
| `connectors` | `integrating` | — | this repository |

`ess` and `worktree` do not follow R3 yet. They are renamed in their own repositories
(`ess:specifying`, `ess:retrofitting`, `ess:testing-conformance`, `worktree:managing-worktrees`).
Until then, `agentplugins-check remote` lists them instead of failing.
