# Set up Beyond10x agent plugins

For an agent in Claude Code or Codex. Follow the steps in order. Ask the user only where a step
says so. Change nothing the user did not confirm.

1. **Install `b10x`.**
   - If `b10x --version` already runs, go to step 2.
   - Otherwise pick the target from `uname -s` and `uname -m`:

     | `uname -s` | `uname -m` | target |
     |---|---|---|
     | `Linux` | `x86_64` | `x86_64-unknown-linux-gnu` |
     | `Linux` | `aarch64` or `arm64` | `aarch64-unknown-linux-gnu` |
     | `Darwin` | `x86_64` | `x86_64-apple-darwin` |
     | `Darwin` | `arm64` | `aarch64-apple-darwin` |

   - Download, check and install into a fresh directory under `~/.cache` (not `/tmp`):

     ```bash
     base=https://github.com/beyond10x/agentplugins/releases/latest/download
     work=$(mktemp -d "$HOME/.cache/b10x-setup.XXXXXX")
     curl -fsSL -o "$work/b10x-<target>.tar.gz" "$base/b10x-<target>.tar.gz"
     curl -fsSL -o "$work/SHA256SUMS" "$base/SHA256SUMS"
     (cd "$work" && sha256sum --check --ignore-missing SHA256SUMS)   # macOS: shasum -a 256 --check --ignore-missing SHA256SUMS
     tar -xzf "$work/b10x-<target>.tar.gz" -C "$work"
     mkdir -p "$HOME/.local/bin" && mv "$work/b10x-<target>/b10x" "$HOME/.local/bin/b10x"
     rm -r "$work"
     ```

   - Stop if the checksum does not match.
   - If `~/.local/bin` is not on `PATH`, tell the user and use `~/.local/bin/b10x` below.
2. **Follow the setup instructions.** Run `b10x setup guide` and follow what it prints from its
   step 2 on. It plans with `b10x setup plan`, shows the result, asks which products to have
   (`aep`, `ess`, `worktree`, optionally `connectors`), lists every change, and runs
   `b10x setup apply` only after the user confirms.
3. **Use what you installed.** A plugin loads in the next session. To use it now, run
   `b10x skill <plugin>` to list its skills and agents and `b10x skill <plugin>:<skill>` to print
   one, then follow the printed text.
