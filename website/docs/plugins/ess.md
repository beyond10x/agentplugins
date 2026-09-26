---
sidebar_position: 4
title: ESS
---

# `ess`

Use this plugin to write, retrofit, validate and project an Executable System Specification, and
to raise or audit a conformance suite against a real implementation.

| skill | for | agent |
|---|---|---|
| `ess:init` | install the `ess` CLI, learn what ESS is, take the first step | — |
| `ess:specifying` | write or extend a specification; `references/syntax.md` shows every section in one that validates | `author` |
| `ess:retrofitting` | derive a specification for a system that has none | `retrofitter` |
| `ess:testing-conformance` | raise or audit what a conformance suite tests | `conformance` |
| `ess:hardening` | after a green suite, the eight techniques that ask what it cannot; `references/` holds each procedure, the reference-model pattern, a design-review brief and spec-diff classification | — |
| `ess:upgrade` | check the plugin and CLI, offer the upgrade | — |

```text
/plugin install ess@b10x
```

Codex: `codex plugin add ess@b10x`. Then `/ess:init` installs the CLI — the prebuilt,
checksummed archive from the [ESS release](https://github.com/beyond10x/ess/releases), or with `cargo` on request.

In a headless run (`claude -p`), allow the CLI or nothing can be validated:
`--allowedTools "Bash(ess:*)"`.
