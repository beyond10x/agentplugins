---
title: Slot
---

# `slot`

Use this plugin when several agent sessions build on one machine. Each `cargo test` sizes itself to
the whole machine, so a dozen of them oversubscribe it. With the plugin, agents prefix builds with
`slot`: every build goes through one daemon that owns a GNU-make jobserver, so cargo, rustc and
make in every session draw from one pool of slots, a go job holds a fixed share, and a build that
has to wait is queued. Output and exit code stay the same.

| skill | for |
|---|---|
| `slot:init` | install the `slot` CLI, run the daemon as a systemd user service, take the first build through it |
| `slot:building` | which commands get the `slot` prefix, checking the daemon, reading the queue, `--local`, remote mode, what never to do |
| `slot:upgrade` | check the plugin and CLI, offer the upgrade, restart the daemon when no build is active |

```text
/plugin install slot@b10x
```

Codex: `codex plugin add slot@b10x`. Then `/slot:init` installs the CLI — the prebuilt,
checksummed archive from the [slot releases](https://github.com/beyond10x/slot/releases) for
x86-64 or ARM64 Linux, or the `b10x-slot-cli` package with `cargo` on request — and starts the
daemon. slot runs on Linux only.

## Building

```bash
slot cargo test -p foo     # queued on the shared pool; output and exit code as usual
slot go test ./...         # holds 4 slots, runs with -p 4
slot task check            # the wrapper passes the pool down to cargo inside
slot --local cargo build   # bypass the daemon
```

If the daemon is not running, `slot` runs the command locally and says so in one line, so the
prefix is always safe. The live view at `http://127.0.0.1:7411/` shows every queued and running
job, slot usage and each job's output; `curl -fsS http://127.0.0.1:7411/api/state` returns the same
state as JSON.

A job that has to wait says so on stderr (`slot: job N queued for K slot(s): …`) before it starts.
A `slot` call inside a job that already runs through slot uses the outer job's slots and queues
nothing, so the skill has agents prefix the outermost command. It tells them never to set
`CARGO_BUILD_JOBS`, `-j` or `MAKEFLAGS` themselves and never to wrap builds in `flock` slot files.

## Remote mode

With `SLOT_HOST` set, `slot` syncs the repository to that build server with `rsync` and runs the
build there over `ssh`; target directories stay on the server, and only allowlisted variables cross.
Tests that need local services run with `slot --local`.

The [slot documentation](https://beyond10x.github.io/slot/) covers the daemon, the toolchain model
and the CLI reference. `agentplugins-check tools` checks every `slot` subcommand and option the
plugin spells against the newest `slot` release; a wrapped build command such as `slot cargo test`
is not run.
