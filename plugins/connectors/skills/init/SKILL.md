---
name: init
description: Start with Connectors in this project — make sure the `connectors` CLI is available and take the first step. Connectors is provider setup, connection diagnostics and governed invocation of integrations. Use when the user wants to connect external tools or providers, asks to set up connectors, or when `connectors:integrating` reports that `connectors` is missing.
---

# Start with Connectors

## 1. Have the CLI

Run `connectors --version`. The plugin itself is set up with `b10x init connectors` (plan, confirm,
`b10x setup apply`); the CLI is installed by hand as `connectors:integrating` describes.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## 2. First step here

Check the CLI and what it can reach:

```bash
connectors inspect doctor
```

`b10x` does not install the `connectors` CLI; `connectors:integrating` names the release to install.

## 3. Pick the work

| the task | skill |
|---|---|
| set up a provider, diagnose a connection, or invoke an integration | `connectors:integrating` |

A skill that is not loaded yet in this session prints with `b10x skill connectors:<skill>`.

## Next

- Connect or diagnose: `connectors:integrating`.
- Later, `connectors:upgrade` checks for a newer plugin and CLI.
