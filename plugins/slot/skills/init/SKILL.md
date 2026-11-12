---
name: init
description: Start with slot on this machine — make sure the `slot` CLI is installed and its daemon runs, then take the first build through it. slot is one shared pool of build slots for every build on a machine (one GNU-make jobserver, a queue and a live web view), optionally on a build server. Use when the user wants to stop parallel agent builds from oversubscribing the machine, asks to set up or install slot, or when `slot:building` reports that `slot` or its daemon is missing. Installs the CLI and the daemon only after the user confirms.
---

# Start with slot

slot runs on Linux only (x86-64 and ARM64); it reads `/proc`. On another system say so and stop.

## 1. Have the CLI

Run `slot --help`. If it prints the usage, the CLI is installed (by `b10x:init` or earlier); go to
step 2, and use `slot:upgrade` later for newer releases. If it is missing, install it with `b10x`.
Plan for the host you run in (`--host claude` in Claude Code, `--host codex` in Codex):

```bash
mkdir -p ~/.local/state/b10x
b10x init slot --host claude --out ~/.local/state/b10x/plan.json
```

It prints what it would do, including the install method: the prebuilt, checksummed release archive
by default, or `cargo` (a source build of the `b10x-slot-cli` package) when the user asks for it
and a Rust toolchain is on `PATH`; the plan names the directory. Say which is planned and
offer the other; re-run with `--method cargo` if they choose it, show the plan, and after they
confirm run `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first; its step 1 asks the user before it installs `b10x`.

## 2. Have the daemon

One daemon serves every session of the user. Check whether it already runs:

```bash
curl -fsS http://127.0.0.1:7411/api/state
```

JSON with `tokens` and `free` means it runs; go to step 3. Otherwise ask the user once, then run
it as a systemd user service, so it outlives this session and no session owns it:

```bash
mkdir -p ~/.config/systemd/user
printf '[Unit]\nDescription=slot build daemon\n\n[Service]\nExecStart=%s serve\nRestart=on-failure\nCPUWeight=20\nEnvironment=PATH=%%h/.cargo/bin:%%h/.local/bin:%%h/go/bin:/usr/local/bin:/usr/bin:/bin\n\n[Install]\nWantedBy=default.target\n' "$(command -v slot)" > ~/.config/systemd/user/slot.service
systemctl --user daemon-reload
systemctl --user enable --now slot.service
```

This is the slot repository's `contrib/slot.service` with `ExecStart` set to wherever the plan
installed `slot`. The `PATH` line matters for builds a remote client sends: they start from the daemon's own
environment, so the toolchains must be on it. The defaults are a pool of 20 slots, go jobs holding 4 and the web view on `127.0.0.1:7411`. When
the user wants other figures, add them to `ExecStart` (`slot serve --tokens 16 --go-tokens 4`;
`slot serve --help` lists every option) and restart the service. Without systemd, the user starts
`slot serve` in a terminal of their own; never start it as a foreground command of this session,
it does not exit.

Check again with the `curl` line above, and give the user the live view:
<http://127.0.0.1:7411/>.

## 3. First build here

Run one build of this repository through the pool, for example `slot cargo build` or
`slot task check`, and confirm it prints the usual output and exits with the usual code. The job
appears in the live view while it runs. From now on follow `slot:building` for every build.

When the user wants a repository to build through slot by default, add one line to its
`AGENTS.md`, and only on request: *Build through slot: prefix builds with `slot` (`slot cargo test
…`); `slot --local` bypasses the daemon.*

When the user wants every agent session on the machine to build through slot without a prefix,
and only on request, install the `cargo` and `go` shim from slot's agent-integration guide
(<https://beyond10x.github.io/slot/docs/agent-integration>): it sends build, check, test, clippy and
install subcommands through slot, runs everything else directly, and goes first on `PATH` for agent
shells only, never for the editor. `SLOT_SHIM_OFF=1` bypasses it for one command.

### A build server (optional)

To run builds on another Linux machine: install the CLI and the daemon there with steps 1 and 2,
make sure `ssh <server>` works without a prompt and `rsync` is on both machines, then on this
machine set `SLOT_HOST=<server>` in the user's shell profile. `slot:building` covers what changes.

## 4. Pick the work

| the task | skill |
|---|---|
| run, queue, watch or debug a build through slot | `slot:building` |

A skill that is not loaded yet in this session prints with `b10x skill slot:<skill>`.

## Next

- Build through the shared pool: `slot:building`.
- Later, `slot:upgrade` checks for a newer plugin and CLI.
