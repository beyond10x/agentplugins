---
sidebar_position: 5
title: How the plugins are organised
---

# How the plugins are organised

Eight rules decide where everything goes. `task check` enforces each one, and `agentplugins-check
tools` checks the skills against the newest CLI releases. A change that breaks a rule does not merge.

| rule | what it says |
|---|---|
| **R1 marketplace** | One marketplace, `b10x`, in both the Claude Code and Codex formats. Every plugin lives in this repository. |
| **R2 plugin** | One plugin per product. The plugin, the product and the CLI it drives share one name: `aep`, `ess`, `worktree`, `connectors`. The front door is `b10x`, with the `b10x` CLI. |
| **R3 skill** | Every plugin has two lifecycle skills: `init` (set it up and take the first step) and `upgrade` (check it and offer the upgrade). Every other skill is an activity, named in `-ing` form, one or two words: `aep:planning`. |
| **R4 agent** | An agent is a role: `implementor`, `author`. Exactly one skill of the same plugin owns it and lists it under `## Agents`. |
| **R5 content** | A skill describes its CLI's newest release and quotes no CLI version. `agentplugins-check tools` runs every spelled command against that release. |
| **R6 distribution** | `SETUP.md` and `b10x` install everything; CLIs come prebuilt or from `cargo`. Retired names live only in `catalog.json`, and setup migrates them. |
| **R7 references** | Every `<plugin>:<skill-or-agent>` written in this repository names a file that exists. |
| **R8 docs** | One README row, one page under `plugins/` and one sidebar entry per plugin. The README is one paragraph and that table. |

## The plugins

| plugin | lifecycle | activities | agents |
|---|---|---|---|
| `b10x` | `init` (guided onboarding), `upgrade` | `routing`, `authoring-plugins` | — |
| `aep` | `init`, `upgrade` | `planning`, `migrating`, `implementing` (wave or drive mode) | `planning`: decomposer, four plan critics, plan reviewer, reverse engineer · `implementing`: story scoper, implementor, adversary, security reviewer |
| `ess` | `init`, `upgrade` | `specifying`, `retrofitting`, `testing-conformance` | `specifying`: author · `retrofitting`: retrofitter · `testing-conformance`: conformance |
| `worktree` | `init`, `upgrade` | `managing-worktrees` | — |
| `connectors` | `init`, `upgrade` | `integrating` | — |

Every plugin carries this repository's version. The CLIs have their own versions; `b10x` installs
their newest release and `b10x check` says at session start when one is behind.
