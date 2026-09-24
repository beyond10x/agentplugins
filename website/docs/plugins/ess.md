---
sidebar_position: 4
title: ESS
---

# `ess`

Use this plugin to write, retrofit, validate and project an Executable System Specification, and
to raise or audit a conformance suite against a real implementation. It carries the skills `ess`,
`specify`, `retrofit` and `coverage`, and the agents `author`, `retrofitter` and `conformance`.

The plugin ships from the [ESS repository](https://github.com/beyond10x/ess), `plugins/ess/`, at
the same version as the `ess` binary; `ess skill` prints the same skills from the binary. This
marketplace carries no copy and names no version: its entry points at the ESS repository's default
branch, and `agentplugins-check remote` refuses a default branch that serves a version ESS never
released.

```text
/plugin install ess@b10x
```

Codex: `codex plugin add ess@b10x`.

The skills describe one exact `ess` binary. [Setup](../install.md) installs the checksummed release
archive whose version equals the installed plugin's, and the `b10x` session-start check says when the
`ess` on `PATH` differs. By hand, pick the archive for your platform from the matching
[ESS release](https://github.com/beyond10x/ess/releases) and verify it against its `SHA256SUMS`.
