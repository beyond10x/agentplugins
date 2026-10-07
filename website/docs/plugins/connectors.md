---
title: Connectors
---

# Connectors

The `connectors` plugin guides setup, connection diagnostics and governed integration calls in
Claude Code and Codex. It follows the current Connectors CLI, verified against
[v0.31.0](https://github.com/beyond10x/connectors/releases/tag/v0.31.0).

## Install in either host

`b10x` installs the plugin and its CLI together. Select your host and review the resulting plan:

```bash
b10x init connectors --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. Apply an approved plan with:

```bash
b10x setup apply --plan ~/.local/state/b10x/plan.json --yes
```

The current release has no prebuilt CLI assets. Setup builds the `connectors` Cargo package from
its exact release tag with locked dependencies; v0.31.0 requires Rust 1.91 or newer. Adapter
executables, configuration and credentials are separate prerequisites. The local runtime currently
targets Linux. [Setup](../install.md) preserves other installed plugins and snapshots changes.

A new session loads the installed skills. In Claude Code invoke `/connectors:integrating`; in
Codex select `integrating` or invoke `$connectors:integrating`. Both hosts load the same skill
files. `b10x skill connectors:integrating` prints the current instructions without a reload.

## First use

Ask: “Use connectors to inspect my local Connector readiness.” The skill checks CLI availability,
runs `connectors --output json setup check` and lists configured adapters. These checks do not
authenticate, start services or prove that cached metadata is current.

For an existing integration, it selects a configured adapter and connection, lists operations,
and describes one before invocation. The returned schema identity and revision bind the call;
its JSON input must match the described schema. Invocation rechecks admission and may start the
supervised local owner/adapter. Missing capability or access is reported explicitly.

For onboarding, the operator enters the credential document through protected terminal, file or
stdin acquisition. The skill reads safe status, not secret values. External writes need the
user's authorization and the adapter's admission, exact approval subject and protected proof.
The current CLI has a separate explicit service interface; MCP runtime integration remains deferred.

## Upgrade

`b10x upgrade connectors --host claude --out ~/.local/state/b10x/plan.json` compares both the
plugin and CLI with current releases. Use the corresponding host, inspect the plan and apply
within the user's authorization. An unresolved release check is reported as unchecked.

Moving from 0.7.x changes CLI and configuration contracts. Preserve the old state, inspect the
consumer operations, then establish current connections through protected acquisition. No v1
configuration or credential migration occurs automatically. After any upgrade, inspect help,
check prerequisites and validate the operations the application actually needs.
