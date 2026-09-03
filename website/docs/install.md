---
sidebar_position: 3
title: Install
---

# Install from the `beyond10x` marketplace

The marketplace source is the GitHub repository `beyond10x/agentplugins` and the marketplace
identity is `beyond10x`. The installable names are `beyond10x`, `aep-planning`, `adp`,
`ess-schema`, and `workspace-hygiene`.

## Before you install: put `aep` and `ess` on your `PATH`

`aep-planning` and `adp` are instruction surfaces for a program they do not ship. Both drive the
`aep` CLI. `ess-schema` likewise drives the `ess` CLI. Install both before the plugins so the first
task does not stop at a missing command. Every AEP and ESS release publishes native archives for
x86-64 and ARM64 Linux and macOS, plus a `SHA256SUMS` file. Windows archives are not published.

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

Download AEP `0.44.0`, verify the selected archive against the release manifest, and install its
canonical `aep` command:

```bash
B10X_AEP_VERSION=0.44.0
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

Install ESS `0.5.1` the same way:

```bash
B10X_ESS_VERSION=0.5.1
B10X_ESS_ARCHIVE="ess-${B10X_ESS_VERSION}-${B10X_TARGET}.tar.gz"
B10X_ESS_RELEASE="https://github.com/beyond10x/ess/releases/download/${B10X_ESS_VERSION}"
curl --fail --location --remote-name "${B10X_ESS_RELEASE}/${B10X_ESS_ARCHIVE}"
curl --fail --location --output ESS-SHA256SUMS "${B10X_ESS_RELEASE}/SHA256SUMS"
if command -v sha256sum >/dev/null; then
  grep "  ${B10X_ESS_ARCHIVE}$" ESS-SHA256SUMS | sha256sum --check -
else
  grep "  ${B10X_ESS_ARCHIVE}$" ESS-SHA256SUMS | shasum --algorithm 256 --check
fi
tar -xzf "${B10X_ESS_ARCHIVE}"
mkdir -p "$HOME/.local/bin"
install -m 0755 "ess-${B10X_ESS_VERSION}-${B10X_TARGET}/ess" "$HOME/.local/bin/ess"
```

If `$HOME/.local/bin` is not already on your `PATH`, add it in your shell profile. Building from
source remains a fallback: use the exact release tag with Cargo, never a moving branch.

Confirm it before installing anything:

```bash
aep --version
ess --version
```

The expected lines are `protocol 0.44.0` and `ess 0.5.1`. (`aep` retains `protocol` as its version
label for compatibility.) `command not found` means the affected plugin will install and then stop
at its first CLI command. Only the `beyond10x` front door needs neither binary.

## Claude Code

Copy the whole block into a Claude Code session:

```text
/plugin marketplace add https://github.com/beyond10x/agentplugins.git#0.5.1
/plugin install aep-planning@beyond10x
/plugin install adp@beyond10x
/plugin install ess-schema@beyond10x
/reload-plugins
```

The first line registers the repository at the immutable `0.5.1` release; each install names its
plugin in the `<plugin>@beyond10x` form. `/reload-plugins` activates them immediately. Add
`/plugin install beyond10x@beyond10x` for the front door and
`/plugin install workspace-hygiene@beyond10x` for managed worktrees. Claude Code reads
`.claude-plugin/marketplace.json` and the selected plugin's `.claude-plugin/plugin.json`. See
[Claude Code's plugin documentation](https://code.claude.com/docs/en/discover-plugins) for the host
commands and supported marketplace sources.

## Codex

Codex offers the same five plugins from the same repository under the same `beyond10x` identity,
but no slash-command equivalent of the block above is published, so this page does not print one.
Add the release-pinned GitHub repository as a marketplace from the Plugins surface, then select the
front door or one of the four specialists. The authoritative description of what Codex will find is
[`.agents/plugins/marketplace.json`](https://github.com/beyond10x/agentplugins/blob/main/.agents/plugins/marketplace.json)
in this repository; Codex reads it together with the selected plugin's `.codex-plugin/plugin.json`.
The `aep` and `ess` binary requirements above apply unchanged.

## Pinning

The block above is already pinned to the bare `0.5.1` release tag. Upgrade by changing that tag
deliberately, re-registering the marketplace source, and running `/reload-plugins`. The release gate
validates both marketplace formats, every declared instruction file, the public documentation, and
the version recorded by each plugin manifest.

After installation, invoke the skill by its displayed name or ask the agent for the capability the
plugin describes. Start with `beyond10x` if you want the front door to select a specialist.
Installation does not grant filesystem, network, credential, or approval authority; the host and
repository rules still decide those boundaries.
