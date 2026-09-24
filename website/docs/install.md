---
sidebar_position: 3
title: Install
---

# Install from the `b10x` marketplace

The marketplace source is the GitHub repository `beyond10x/agentplugins` and the marketplace
identity is `b10x`. The installable names are `beyond10x`, `aep-plan`, `aep-drive`, `connectors`
and, in Claude Code, `worktree`. `worktree` ships from the
[worktree repository](https://github.com/beyond10x/worktree) at the version of the `worktree`
binary; this marketplace pins it by release tag and commit. The [Connectors guide](plugins/connectors.md) covers its
separate CLI prerequisite.

The ESS plugin is not in this marketplace. It ships from the
[ESS repository](https://github.com/beyond10x/ess#point-your-agent-here) at the version of the `ess`
binary it describes, and `ess skill` prints the same skills from the binary.

## Before you install: put `aep` on your `PATH`

`aep-plan` and `aep-drive` are instruction surfaces for a program they do not ship. Both drive the
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
first CLI command. The `beyond10x` front door does not need the binary.

### `aep-drive`'s `drive` skill also needs Metaharness

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

The wave skill, the planning skill and every other plugin need no Metaharness.

## Claude Code

Copy the whole block into a Claude Code session:

```text
/plugin marketplace add https://github.com/beyond10x/agentplugins.git#0.11.0
/plugin install aep-plan@b10x
/plugin install aep-drive@b10x
/reload-plugins
```

The first line registers the repository at the immutable `0.11.0` release; each install names its
plugin in the `<plugin>@b10x` form. `/reload-plugins` activates them immediately. Add
`/plugin install beyond10x@b10x` for the front door and
`/plugin install worktree@b10x` for managed worktrees, or
`/plugin install connectors@b10x` for integrations. Claude Code reads
`.claude-plugin/marketplace.json` and the selected plugin's `.claude-plugin/plugin.json`. See
[Claude Code's plugin documentation](https://code.claude.com/docs/en/discover-plugins) for the host
commands and supported marketplace sources.

## Codex

Codex offers the four plugins this repository carries under the same `b10x` identity. It reads
`worktree` from the worktree repository instead:
`codex plugin marketplace add https://github.com/beyond10x/worktree.git --ref <version>`, then
`codex plugin add worktree@worktree`. For a fresh installation, run this release-pinned block:

```bash
codex plugin marketplace add https://github.com/beyond10x/agentplugins.git --ref 0.11.0
codex plugin add beyond10x@b10x
codex plugin add aep-plan@b10x
codex plugin add aep-drive@b10x
codex plugin add connectors@b10x
```

An immutable marketplace pin does not advance when `codex plugin marketplace upgrade` runs. To
replace an older pin, remove the installed plugins and marketplace registration, then run the fresh
block above:

```bash
codex plugin remove beyond10x@b10x
codex plugin remove aep-plan@b10x
codex plugin remove aep-drive@b10x
codex plugin remove connectors@b10x
codex plugin marketplace remove b10x
```

The commands leave every focused plugin installed and enabled. Start a new Codex thread afterwards;
plugin instructions are injected when a thread starts, not retroactively into a running thread.
The same plugins remain available from the Plugins surface. The authoritative description of what
Codex will find is
[`.agents/plugins/marketplace.json`](https://github.com/beyond10x/agentplugins/blob/main/.agents/plugins/marketplace.json)
in this repository; Codex reads it together with the selected plugin's `.codex-plugin/plugin.json`.
The `aep` binary requirement above applies unchanged.

## Pinning

The blocks above are already pinned to the bare `0.11.0` release tag. Upgrade by changing that tag
deliberately, re-registering the marketplace source, and running `/reload-plugins`. The release gate
validates both marketplace formats, every declared instruction file, the public documentation, and
the version recorded by each plugin manifest.

After installation, invoke the skill by its displayed name or ask the agent for the capability the
plugin describes. Start with `beyond10x` if you want the front door to select a specialist.
Installation does not grant filesystem, network, credential, or approval authority; the host and
repository rules still decide those boundaries.
