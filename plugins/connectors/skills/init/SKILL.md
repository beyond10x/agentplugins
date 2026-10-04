---
name: init
description: Start with Connectors in this project — make sure the `connectors` CLI is available and take the first step. Connectors is provider setup, connection diagnostics and governed invocation of integrations. Use when the user wants to connect external tools or providers, asks to set up connectors, or when `connectors:integrating` reports that `connectors` is missing. Installs CLIs only after the user confirms the plan.
---

# Start with Connectors

## 1. Have the CLI

Run `connectors --help`; derive release identity from the install receipt or source pin when
the CLI exposes no version flag. The plugin itself is set up with `b10x`: plan, reuse authorization or confirm once, then
`b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`. Plan for the host you run in
(`--host claude` in Claude Code, `--host codex` in Codex):

```bash
mkdir -p ~/.local/state/b10x
b10x init connectors --host claude --out ~/.local/state/b10x/plan.json
```

The catalog registers Cargo installation for the CLI. Review the planned source tag and package
identity, then apply the authorized plan; do not invent release binary assets.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first; its step 1 asks the user before it installs `b10x`.

## 2. First step here

Check the CLI and what it can reach:

```bash
connectors setup check
```

Use `connectors:integrating` for configured adapter discovery and schema-bound invocation.

## 3. Pick the work

| the task | skill |
|---|---|
| set up a provider, diagnose a connection, or invoke an integration | `connectors:integrating` |

A skill that is not loaded yet in this session prints with `b10x skill connectors:<skill>`.

## Next

- Connect or diagnose: `connectors:integrating`.
- Later, `connectors:upgrade` checks for a newer plugin and CLI.
