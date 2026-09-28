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
| **R3 skill** | Every plugin has two lifecycle skills: `init` (set it up and take the first step) and `upgrade` (check it and offer the upgrade). Every other skill is an activity or a command. An activity is named in `-ing` form, one or two words: `aep:planning`. A command is an entry point only the operator starts: named with a verb, one or two words (`worktree:cleanup`); it sets `disable-model-invocation: true`, has at most 20 lines of body, and names the one activity skill of its plugin it hands off to; its `agents/openai.yaml` sets `policy.allow_implicit_invocation: false`, the Codex form of the same flag. No skill is named after its plugin. |
| **R4 agent** | An agent is a role: `implementor`, `author`. Exactly one skill of the same plugin owns it and lists it under `## Agents`. The agent file is a thin Claude Code adapter: at most 20 lines of body, naming its owning skill as `<plugin>:<skill>`, and every file it links exists. The role's procedure lives in that skill or in its `references/<role>.md`, because Codex loads skills and not `agents/`; the skill says to run the role directly where the host has no subagents. |
| **R5 content** | A skill describes its CLI's newest release and quotes no CLI version. `agentplugins-check tools` runs every spelled command against that release. A `**Skill version X**` line names the version in its plugin's `.claude-plugin/plugin.json`. |
| **R6 distribution** | `SETUP.md` and `b10x` install everything; CLIs come prebuilt or from `cargo`. Retired names live only in `catalog.json`, and setup migrates them. |
| **R7 references** | Every `<plugin>:<skill-or-agent>` written in this repository names a file that exists. |
| **R8 docs** | One README row, one page under `plugins/` and one sidebar entry per plugin. The README is one paragraph, that table, and the generated tree of every skill and agent, each linked to its file. |

## The plugins

| plugin | lifecycle | activities | commands | agents |
|---|---|---|---|---|
| `b10x` | `init` (guided onboarding), `upgrade` | `routing`, `authoring-plugins` | — | — |
| `aep` | `init`, `upgrade` | `planning`, `migrating`, `implementing` (wave or drive mode), `diagnosing` | `wave`, `drive` (both hand off to `implementing`) · `review-plan`, `decompose` (both hand off to `planning`) | `planning`: decomposer, four plan critics, plan reviewer, reverse engineer · `implementing`: story scoper, implementor, adversary, security reviewer (each procedure in its skill's `references/`) |
| `ess` | `init`, `upgrade` | `specifying`, `retrofitting`, `testing-conformance`, `hardening` | — | `specifying`: author · `retrofitting`: retrofitter · `testing-conformance`: conformance |
| `worktree` | `init`, `upgrade` | `managing-worktrees` | `cleanup` (hands off to `managing-worktrees`) | — |
| `connectors` | `init`, `upgrade` | `integrating` | — | — |

Every plugin carries this repository's version. The CLIs have their own versions; `b10x` installs
their newest release and `b10x check` says at session start when one is behind.
