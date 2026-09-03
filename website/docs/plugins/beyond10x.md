---
title: Beyond10x
---

# `beyond10x`

Use this plugin as the marketplace front door. It selects the smallest specialist for a request,
provides a map of public Beyond10x resources, and helps create plugins that keep their core
capabilities portable across Codex and Claude Code.

It provides:

- a `beyond10x` routing skill for selecting `aep-plan`, `aep-drive`, `ess-specify`, or
  `workspace-hygiene`;
- direct links to public product guides, command references, plugin references, and source;
- a `plugin-creator` skill for creating or porting dual-harness plugins.

## Portable plugin creation

The creator uses `skills/<name>/SKILL.md` as the shared capability layer. Both hosts load that
layout and its references, assets, and optional scripts. New command-like behavior is authored as a
skill, which Claude Code exposes as a slash shortcut and Codex exposes through skill invocation.

Specialist-agent behavior also lives in a shared skill. A thin Claude Code `agents/` wrapper may be
added when native delegation is useful, while Codex uses the shared skill directly or delegates
through its supported orchestration. The creator never presents a Claude-only command or agent file
as a portable component.

The full compatibility rules live with the plugin source and link to the current
[OpenAI plugin packaging guide](https://developers.openai.com/plugins/build/plugins) and
[Claude Code plugin reference](https://code.claude.com/docs/en/plugins-reference).

## Routing boundaries

The front door does not copy specialist instructions. Install the plugin it selects when you need
the governed workflow:

- [`aep-plan`](./aep-plan.md) for plans and artifact stores;
- [`aep-drive`](./aep-drive.md) for accepted development work;
- [`ess-specify`](./ess-specify.md) for ESS specification, validation and projections.
- [`workspace-hygiene`](./workspace-hygiene.md) for managed Git worktrees and safe cleanup.

For the broader organization map, start at [Beyond10x getting started](https://beyond10x.github.io/getting-started/).
