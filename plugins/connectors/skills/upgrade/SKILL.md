---
name: upgrade
description: Check whether the Connectors plugin and the `connectors` CLI are current, and upgrade them within the user's authorization. Use when the user asks whether Connectors is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `connectors`, or when a `connectors` command behaves differently from what a Connectors skill describes.
---

# Upgrade Connectors

```bash
b10x upgrade connectors --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. The plan compares the installed plugin with the marketplace and
the CLI on `PATH` with the newest Connectors release. It changes nothing yet. From `v0.48.0`
releases carry checksummed prebuilt archives for Linux and macOS (x86-64 and ARM64), which the
plan uses by default; an older source-only release or `--method cargo` builds Cargo package
`connectors` at the exact tag with locked dependencies and needs Rust 1.91 or newer. A copy
`cargo` installed into `~/.cargo/bin` is replaced where it runs.

1. Inspect every finding. Report unresolved release lookups, pins and toolchain failures as such;
   an empty action list alone does not establish that Connectors is current. If every version was
   checked and is current, report those versions and finish.
2. For a 0.7.x installation, read the setup/migration reference in `connectors:integrating`,
   preserve its configuration and credential state, and identify consumer contract differences.
   Current Connectors does not migrate that state automatically.
3. Show the concrete actions. Apply within existing authorization, or ask once if changes were
   not requested: `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
4. Verify `connectors --version`, help and `connectors --output json setup check`. Validate the
   consumer operations authorized by the task; version/help checks alone do not prove their
   compatibility. From a release before `v0.31.0`, a command within 60 s of the last one before
   the upgrade may answer `metadata_unavailable` once; repeat it after 60 s before treating that
   as a failure. From a release before `v0.32.0`, `--output json` carries `result.result`,
   `input_schema` and `output_schema` as JSON values, not text: a caller that decoded them a
   second time reads them directly. When the release notes say a configuration revision moved
   (`v0.32.0`: instances using the shipped GitLab selection set), the adapter does not start and
   its connections report `pending` until the setup reference's recovery is done.
   From `v0.33.0`, a store `setup init` creates cannot be opened by `v0.32.0` or earlier: upgrade
   every installed `connectors` binary that opens the store first. An existing store is not
   changed by the upgrade; `setup checkpoints-enable --confirm one-way` converts it, one-way, and
   only when the operator asks (the setup reference).
   From `v0.50.0` the store records its newest writer. A binary older than that writer whose
   record model differs refuses the store with `metadata_schema_newer` (`next_action = upgrade`,
   naming `store_version` and `binary_version`) before it reads a record; upgrade that binary,
   or give a pinned consumer its own `--state-dir`. Binaries share one state directory only
   while the oldest is at least the newest that wrote it. A release up to `v0.49.0` answers such
   a store with `metadata_unavailable`, which no retry clears.
   From `v0.52.0` a single-source `roughtime-clock/1` approval clock is refused: an install that
   issues approvals rewrites its `[approval_clock]` table as `roughtime-quorum/1` (the integrating
   skill's writes reference) and runs `connectors approvals clock-check --adapter <alias>` before
   its next approval.
   A new plugin version loads in a new session; until then
   `b10x skill connectors:<skill>` prints its text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.
Then continue the work that prompted the check; `connectors:init` starts the workflow.
