---
sidebar_position: 3
title: Install
---

# Install from the `beyond10x` marketplace

Use the GitHub repository `beyond10x/agentplugins` as the marketplace source in a supported agent
client. The marketplace identity is `beyond10x`; the installable names are `beyond10x`,
`aep-planning`, `adp`, `ess-schema`, and `workspace-hygiene`.

## Codex

Open the Plugins surface, add the GitHub repository as a marketplace, then select the front door or
one of the four specialists. Codex reads `.agents/plugins/marketplace.json` and the selected plugin's
`.codex-plugin/plugin.json`.

## Claude Code

Add the repository as a plugin marketplace, then install the selected plugin under the
`beyond10x` marketplace identity. Claude Code reads `.claude-plugin/marketplace.json` and the
plugin's `.claude-plugin/plugin.json`.

## Pinning

For a reproducible team installation, pin the repository to a bare release tag such as `0.3.2`.
The release gate validates both marketplace formats, every declared instruction file, the public
documentation, and the version recorded by each plugin manifest.

After installation, invoke the skill by its displayed name or ask the agent for the capability the
plugin describes. Start with `beyond10x` if you want the front door to select a specialist.
Installation does not grant filesystem, network, credential, or approval authority; the host and
repository rules still decide those boundaries.
