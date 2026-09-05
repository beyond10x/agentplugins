---
title: Connectors
---

# Connectors

The `connectors` plugin provides one shared `connectors` skill for Claude Code and Codex. It
guides provider setup, connection diagnostics, and the search → describe → invoke sequence for
admitted integrations. It ships no binary, credentials, daemon, hooks, or automatic MCP connection.

Install the standalone CLI from [Connectors releases](https://github.com/beyond10x/connectors/releases)
and verify `connectors --version`. The skill targets the grouped commands in `0.6.0` and reads the
installed binary's help before selecting options. Service setup and credentials are separate from
plugin installation.

## Install in either host

This plugin is new on `main`; the `0.7.0` marketplace release does not contain it. For a fresh
marketplace registration, use the current branch:

```bash
claude plugin marketplace add https://github.com/beyond10x/agentplugins.git#main
claude plugin install connectors@beyond10x
```

```bash
codex plugin marketplace add https://github.com/beyond10x/agentplugins.git --ref main
codex plugin add connectors@beyond10x
```

An existing `beyond10x` registration pinned to `0.7.0` must be repointed before it can offer this
plugin; refreshing an immutable tag does not add newer content. Preserve the other installed
plugins when changing that registration. For development, both marketplace-add commands also
accept the absolute path to a current local checkout containing both marketplace files. `main`
moves; use an exact commit ref when a reproducible pre-release installation is required.

Reload Claude Code's plugins with `/reload-plugins`, or start a new Codex thread. In Claude Code,
invoke `/connectors:connectors`; in Codex select the `connectors` skill or invoke `$connectors`.
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
