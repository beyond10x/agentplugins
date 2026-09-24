---
title: Connectors
---

# Connectors

The `connectors` plugin provides one shared `connectors` skill for Claude Code and Codex. It
guides provider setup, connection diagnostics, and the search → describe → invoke sequence for
admitted integrations. It ships no binary, credentials, daemon, hooks, or automatic MCP connection.

Install the standalone CLI from the
[Connectors `v0.7.2` release](https://github.com/beyond10x/connectors/releases/tag/v0.7.2), the
last release of the v1 line, and verify `connectors --version`. The skill targets the grouped
commands in `0.7.x` and reads the installed binary's help before selecting options.
`beyond10x/connectors`' default branch and its *Latest* release are the connectors_v2 lineage
(`v0.8.0` and up), which is a different CLI — `setup`, `adapters`, `connections`, `operations`,
`describe`, `invoke`, `serve` — per Atlas ADR 0051 on the `beyond10x/connectors` lineage
(accepted 2026-09-15); this skill drives the v1 CLI, `0.7.x`, so the generic releases page and
its *Latest* entry are not the CLI these instructions teach. Service setup and credentials are separate
from plugin installation.

Operation, connection and event commands default to local even when a hosted login is saved.
Choose `--target hosted` explicitly for a hosted workflow and keep that target throughout
search, describe and invoke. The operation contract defaults to v3 without automatic fallback.
Bounded reads can run locally without a daemon; ongoing sessions and events still need one.
When upgrading, verify the operations your application uses and replace the CLI and local daemon
together. The skill also covers pagination, restriction metadata and explicit handling of
rate-limit responses.

## Install in either host

Setup offers this plugin as optional. To add it by hand:

```bash
claude plugin marketplace add beyond10x/agentplugins
claude plugin install connectors@b10x
```

```bash
codex plugin marketplace add beyond10x/agentplugins
codex plugin add connectors@b10x
```

[Setup](../install.md) replaces an older or pinned registration and keeps the other installed
plugins. For development, both marketplace-add commands also accept the absolute path
to a current local checkout containing both marketplace files.

Reload Claude Code's plugins with `/reload-plugins`, or start a new Codex thread. In Claude Code,
invoke `/connectors:connectors`; in Codex select the `connectors` skill or invoke `$connectors:connectors`.
Both manifests load the same `skills/connectors/SKILL.md` bytes. These layouts follow the
[OpenAI plugin packaging contract](https://developers.openai.com/plugins/build/plugins) and
[Claude Code plugin reference](https://code.claude.com/docs/en/plugins-reference).

## First use

Ask: “Use connectors to inspect my local Connector readiness.” The skill checks CLI availability
and runs `connectors --output json inspect doctor`. For an existing configured integration, ask
it to find a particular operation; it searches admitted operations and reads a fresh description
before using the returned Connection and description reference.

For onboarding, the operator enters credentials directly into the CLI's hidden prompt. The skill
never reads secret values into the conversation. External writes require the user's authorization
and any Connector approval evidence; installing the plugin supplies neither. Hosted MCP setup is
an explicit workflow using `connectors serve mcp`, not an automatic install side effect.
