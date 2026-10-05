---
sidebar_position: 3
title: Install
---

# Install from the `b10x` marketplace

## Set up with one sentence

Tell Claude Code or Codex:

> Set up Beyond10x: follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md

The agent installs the `b10x` binary and runs `/b10x:init`: it asks what you want to do (plan and
deliver work, write specifications, isolated Git checkouts, integrations) and how to install the
command-line tools (prebuilt archives by default, or `cargo`), lists every change — including
earlier installs it replaces — and applies them only after you confirm. It snapshots every file it
changes; `b10x setup undo` restores them. Each product then starts with its own `/<plugin>:init`,
and `/b10x:upgrade` or `/<plugin>:upgrade` checks for newer versions. The rest of this page is the
manual route.

## The marketplace

The marketplace source is the GitHub repository `beyond10x/agentplugins` and the marketplace
identity is `b10x`. The installable names are `b10x`, `aep`, `ess`, `worktree` and `connectors`,
in both hosts; every one lives in this repository. The CLIs come from their own repositories'
releases, prebuilt or with `cargo install`. The [Connectors guide](plugins/connectors.md) covers its
source-build requirements and separate provider configuration.

## The command-line tools

The `aep`, `ess`, `worktree` and `connectors` plugins drive command-line tools they do not ship.
`b10x` installs each at its newest release, from a published archive checked against its
`SHA256SUMS`, or with `cargo`. Connectors currently publishes source only and needs Rust 1.91
or newer:

```shell-session
$ b10x init ess --host claude --out plan.json  # use --host codex in Codex
$ b10x setup apply --plan plan.json --yes
```

`b10x init` prints the plan and changes nothing; `b10x setup apply` carries it out after you have
read it, and snapshots every file it changes first. Archives exist for x86-64 and ARM64 Linux and
macOS; elsewhere `--method cargo` builds from the release tag. The
[tutorial](tutorials/first-ess-specification.md) shows the output of both commands. The `b10x`
front door itself needs none of these product CLIs.

### The `aep:implementing` skill's drive mode also needs Metaharness

Drive mode hands one story to `metaharness aep drive run`. Metaharness is optional in the `aep`
product, so `b10x init aep` does not install it; install it on its own:

```shell-session
$ b10x install metaharness
$ metaharness aep drive run --help
```

Metaharness links AEP as a library at the revision its own release pins. Its planning executable
must match that release: check the runner's dependency metadata before launching a drive or eval,
and supply that executable with `--aep-binary` for a driven run. The eval preflight also checks the
child's `aep` on `PATH`. Wave mode and `aep:planning` need no Metaharness.

`b10x-harness`, the agent loop Metaharness's `b10x` adapter runs, is optional as well:
`b10x install b10x-harness` installs its newest release. It runs on Linux only, from a prebuilt
archive where the release has one for the machine, otherwise built with `cargo`.

## Claude Code

```text
/plugin marketplace add beyond10x/agentplugins
/plugin install b10x@b10x
/plugin install aep@b10x
/reload-plugins
```

Add `/plugin install ess@b10x`, `/plugin install worktree@b10x` or
`/plugin install connectors@b10x` as needed. The marketplace follows the default branch, whose
gate keeps every entry installable; `/reload-plugins` activates new plugins. Claude Code reads
`.claude-plugin/marketplace.json` and each plugin's `.claude-plugin/plugin.json`. See
[Claude Code's plugin documentation](https://code.claude.com/docs/en/discover-plugins) for the host
commands and supported marketplace sources.

## Codex

```bash
codex plugin marketplace add beyond10x/agentplugins
codex plugin add b10x@b10x
codex plugin add aep@b10x
```

Add `ess@b10x`, `worktree@b10x` or `connectors@b10x` the same way. `codex plugin marketplace upgrade`
refreshes the marketplace. Start a new Codex thread afterwards;
plugin instructions are injected when a thread starts, not retroactively into a running thread.
The same plugins remain available from the Plugins surface. The authoritative description of what
Codex will find is
[`.agents/plugins/marketplace.json`](https://github.com/beyond10x/agentplugins/blob/main/.agents/plugins/marketplace.json)
in this repository; Codex reads it together with the selected plugin's `.codex-plugin/plugin.json`.
The command-line tools above apply unchanged.

## Upgrading

Run setup again, or `b10x setup plan` and `b10x setup apply` yourself. The release gate validates both
marketplace formats, every declared instruction file, the public documentation, and the version
recorded by each plugin manifest this repository carries.

If the registered marketplace points at a different source, the plan names that source change
before upgrading plugins. This includes a frozen local checkout: refreshing it alone cannot
discover plugins that only exist in the selected current marketplace. The snapshot preserves the
previous registration for undo.

After installation, invoke the skill by its displayed name or ask the agent for the capability the
plugin describes. Start with `b10x:routing` if you want the front door to select a specialist.
Installation does not grant filesystem, network, credential, or approval authority; the host and
repository rules still decide those boundaries.

## Pin a version

A repository can hold a CLI at one release instead of the newest:

```bash
b10x pin aep 0.68.0   # exactly this release
b10x pin aep 0.68     # the newest 0.68.x
b10x unpin aep
```

The pins go into `b10x.toml` in the current directory (or the nearest one above); commit it.
`b10x init`, `b10x upgrade`, `b10x setup plan` and `b10x install` then use the pinned release,
`upgrade` names a newer one without installing it, and `b10x check` says when the CLI on `PATH`
does not match the pin.

Beyond10x product repositories keep ESS on its newest release; they do not hold it back with a
`b10x.toml` pin. Update the specification's toolchain requirement and validate its generated outputs
when moving to a new release.
