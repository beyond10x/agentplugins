---
name: upgrade
description: Check whether the Connectors plugin and the `connectors` CLI are current, and upgrade them within the user's authorization. Use when the user asks whether Connectors is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `connectors`, or when a `connectors` command behaves differently from what a Connectors skill describes.
---

# Upgrade Connectors

```bash
b10x upgrade connectors --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. The plan compares the installed plugin with the marketplace and
the CLI on `PATH` with the newest Connectors release. It changes nothing yet. Current releases
have no prebuilt assets, so installation uses Cargo package `connectors` at the exact release tag
with locked dependencies; `v0.45.0` requires Rust 1.91 or newer.

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
   A new plugin version loads in a new session; until then
   `b10x skill connectors:<skill>` prints its text.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.
Then continue the work that prompted the check; `connectors:init` starts the workflow.
