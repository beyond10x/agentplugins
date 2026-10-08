---
name: init
description: Start with Connectors in this project — make sure the `connectors` CLI is available and take the first step. Connectors is provider setup, connection diagnostics and governed invocation of integrations. Use when the user wants to connect external tools or providers, asks to set up connectors, or when `connectors:integrating` reports that `connectors` is missing. Applies installation plans within the user's authorization.
---

# Start with Connectors

## 1. Have the CLI

Run `connectors --version`. To install the plugin and CLI, plan for the host you run in
(`--host claude` in Claude Code, `--host codex` in Codex):

```bash
mkdir -p ~/.local/state/b10x
b10x init connectors --host claude --out ~/.local/state/b10x/plan.json
```

Review the actions and resolve warnings before applying within the user's authorization:
`b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`. Ask once if installation is not
already authorized. Current Connectors releases are source-only: `b10x` builds the `connectors`
Cargo package at the selected exact tag, with locked dependencies. Release `v0.33.0` needs Rust
1.91 or newer. A missing toolchain or failed build is a prerequisite failure, not a completed install.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.
For an existing 0.7.x deployment, read the migration guidance in `connectors:integrating` before
replacing the binary; old state and credential custody are not migrated automatically.

## 2. Inspect this deployment

```bash
connectors --version
connectors --help
connectors --output json setup check
connectors --output json adapters list
```

Report failed prerequisites even when the command exits successfully. `b10x` does not configure
adapters, acquire credentials or start a service. Initialization and connection acquisition belong
to the requested setup workflow in `connectors:integrating`; a diagnostic request ends with the
observed state and next action.

## 3. Continue

Use `connectors:integrating` for provider setup, connection diagnosis and operation invocation.
A skill not loaded in this session prints with `b10x skill connectors:<skill>`.
Later, `connectors:upgrade` compares both the plugin and CLI with current releases.
