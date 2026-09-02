---
sidebar_position: 3
title: Install
---

# Install from the `beyond10x` marketplace

The marketplace source is the GitHub repository `beyond10x/agentplugins` and the marketplace
identity is `beyond10x`. The installable names are `beyond10x`, `aep-planning`, `adp`,
`ess-schema`, and `workspace-hygiene`.

## Before you install: put `aep` on your `PATH`

`aep-planning` and `adp` are instruction surfaces for a program they do not ship. Both drive the
`aep` CLI, so neither does anything until an `aep` binary is on your `PATH` — the skills stop at
their first command. The binary is published as a GitHub release of the sibling repository
[`beyond10x/aep`](https://github.com/beyond10x/aep/releases); the current release is `0.41.0`.
Installing it is outside this page.

Confirm it before installing anything:

```bash
aep --version
```

A printed version means you are ready. `command not found` means the plugins will install and then
refuse the first thing you ask them to do. `ess-schema` and the `beyond10x` front door need no
binary.

## Claude Code

Copy the whole block into a Claude Code session:

```text
/plugin marketplace add beyond10x/agentplugins
/plugin install aep-planning@beyond10x
/plugin install adp@beyond10x
/plugin install ess-schema@beyond10x
```

The first line registers the marketplace under the identity `beyond10x`; each install names its
plugin in the `<plugin>@beyond10x` form. Add `/plugin install beyond10x@beyond10x` for the front
door and `/plugin install workspace-hygiene@beyond10x` for managed worktrees. Claude Code reads
`.claude-plugin/marketplace.json` and the selected plugin's `.claude-plugin/plugin.json`.

## Codex

Codex offers the same five plugins from the same repository under the same `beyond10x` identity,
but no slash-command equivalent of the block above is published, so this page does not print one.
Add the GitHub repository as a marketplace from the Plugins surface, then select the front door or
one of the four specialists. The authoritative description of what Codex will find is
[`.agents/plugins/marketplace.json`](https://github.com/beyond10x/agentplugins/blob/main/.agents/plugins/marketplace.json)
in this repository; Codex reads it together with the selected plugin's `.codex-plugin/plugin.json`.
The `aep` binary requirement above applies unchanged.

## Pinning

For a reproducible team installation, pin the repository to a bare release tag such as `0.4.0`.
The release gate validates both marketplace formats, every declared instruction file, the public
documentation, and the version recorded by each plugin manifest.

After installation, invoke the skill by its displayed name or ask the agent for the capability the
plugin describes. Start with `beyond10x` if you want the front door to select a specialist.
Installation does not grant filesystem, network, credential, or approval authority; the host and
repository rules still decide those boundaries.
