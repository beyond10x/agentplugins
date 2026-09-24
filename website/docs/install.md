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
command-line tools (`cargo` when you have it, or prebuilt archives), lists every change — including
earlier installs it replaces — and applies them only after you confirm. It snapshots every file it
changes; `b10x setup undo` restores them. Each product then starts with its own `/<plugin>:init`,
and `/b10x:upgrade` or `/<plugin>:upgrade` checks for newer versions. The rest of this page is the
manual route.

## The marketplace

The marketplace source is the GitHub repository `beyond10x/agentplugins` and the marketplace
identity is `b10x`. The installable names are `b10x`, `aep`, `ess`, `worktree` and `connectors`,
in both hosts; every one lives in this repository. The CLIs come from their own repositories'
releases, prebuilt or with `cargo install`. The [Connectors guide](plugins/connectors.md) covers its
separate CLI prerequisite.

## Before you install: put `aep` on your `PATH`

The `aep` plugin is an instruction surface for a program it does not ship. It drives the
`aep` CLI. Install it before the plugins so the first task does not stop at a missing command. The
AEP version pinned below publishes native archives for x86-64 and ARM64 Linux and macOS, plus a
`SHA256SUMS` file. Windows archives are not published.

Select the native target once:

```bash
case "$(uname -s):$(uname -m)" in
  Linux:x86_64)  B10X_TARGET=x86_64-unknown-linux-gnu ;;
  Linux:aarch64) B10X_TARGET=aarch64-unknown-linux-gnu ;;
  Darwin:x86_64) B10X_TARGET=x86_64-apple-darwin ;;
  Darwin:arm64)   B10X_TARGET=aarch64-apple-darwin ;;
  *) echo "unsupported platform: $(uname -s) $(uname -m)" >&2; exit 1 ;;
esac
```

Download AEP `0.55.0`, verify the selected archive against the release manifest, and install its
canonical `aep` command:

```bash
B10X_AEP_VERSION=0.55.0
B10X_AEP_ARCHIVE="aep-${B10X_AEP_VERSION}-${B10X_TARGET}.tar.gz"
B10X_AEP_RELEASE="https://github.com/beyond10x/aep/releases/download/${B10X_AEP_VERSION}"
curl --fail --location --remote-name "${B10X_AEP_RELEASE}/${B10X_AEP_ARCHIVE}"
curl --fail --location --output AEP-SHA256SUMS "${B10X_AEP_RELEASE}/SHA256SUMS"
if command -v sha256sum >/dev/null; then
  grep "  ${B10X_AEP_ARCHIVE}$" AEP-SHA256SUMS | sha256sum --check -
else
  grep "  ${B10X_AEP_ARCHIVE}$" AEP-SHA256SUMS | shasum --algorithm 256 --check
fi
tar -xzf "${B10X_AEP_ARCHIVE}"
mkdir -p "$HOME/.local/bin"
install -m 0755 "aep-${B10X_AEP_VERSION}-${B10X_TARGET}/aep" "$HOME/.local/bin/aep"
```

If `$HOME/.local/bin` is not already on your `PATH`, add it in your shell profile. Building from
source remains a fallback: use the exact release tag with Cargo, never a moving branch.

Confirm it before installing anything:

```bash
aep --version
```

The expected line is `protocol 0.55.0`. (`aep` retains `protocol` as its version label for
compatibility.) `command not found` means the affected plugin will install and then stop at its
first CLI command. The `b10x` front door does not need the `aep` binary.

### The `aep:implementing` skill also needs Metaharness

AEP 0.55.0 refuses to run a model-backed step map itself and names `metaharness aep drive run` as
the command that does. Metaharness `0.7.0` includes this host; tags through `0.6.5` predate it.
This release provides source. With Rust 1.98 or newer, install the binary from its exact tag:

```bash
cargo install --locked --git https://github.com/beyond10x/metaharness \
  --tag 0.7.0 metaharness-cli
metaharness aep drive run --help
```

Metaharness does not call the `aep` binary installed above — it links AEP as a library at whatever
`beyond10x/aep` git revision `crates/metaharness-aep/Cargo.toml` pins in the Metaharness tag you
install (at `0.7.0` that revision is `a23176ae`, which `git describe --tags` in the AEP repository
reports as `0.54.0-33-ga23176ae`, i.e. behind the `0.55.0` you put on `PATH`), so read that manifest
for the Metaharness↔AEP pair rather than assuming the two versions match.

Wave mode, `aep:planning` and every other plugin need no Metaharness.

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
The `aep` binary requirement above applies unchanged.

## Upgrading

Run setup again, or `b10x setup plan` and `b10x setup apply` yourself. The release gate validates both
marketplace formats, every declared instruction file, the public documentation, and the version
recorded by each plugin manifest this repository carries.

After installation, invoke the skill by its displayed name or ask the agent for the capability the
plugin describes. Start with `b10x:routing` if you want the front door to select a specialist.
Installation does not grant filesystem, network, credential, or approval authority; the host and
repository rules still decide those boundaries.
