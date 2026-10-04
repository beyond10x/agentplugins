# AEP repository preflight

Run this before any AEP activity, including init, upgrade, a command handoff or a directly invoked
role. A coordinator passes the completed record and user decision to workers; a worker reuses it
when the repository, selected store and effective binary match. A changed selection needs a fresh
capability check, not another offer for the same product in this session.

## Inspect before offering

1. Identify the repository, selected project/store (including explicit CLI overrides), installed
   binary path and `aep --version`. Read `b10x.toml`, CI/task wrappers and other toolchain pins to
   establish which binary the requested operation actually uses. Read the plugin version from its
   installed manifest. Installed release, effective release, project format and backend are separate
   facts; record each, with evidence and unknowns.
2. Inspect `.engineering/project.yaml` without printing database credentials. Read its `version`,
   backend selector and `planning_scope`. Inspect the selected planning directory even if the
   manifest is absent: existing artifacts or a journal mean an existing manifest-free store, not
   a new project. An absent manifest alone never authorizes `aep plan reverse init`.
3. Reuse release discovery from `b10x upgrade aep --host <host> --out <plan-file>` and read the
   **stable target release's** project/store contract and migration help. Compare installed and
   effective capabilities separately. A plugin/CLI plan with no actions says nothing about the
   repository format. Preserve any `b10x.toml` pin. If discovery is offline or unavailable, report
   freshness as unknown, retain known pins and use verified local contracts for compatible work;
   never call unknown freshness current.
4. Classify the store using that contract. Current project formats can select Git, SQLite or
   PostgreSQL; preserve the selected backend and database identity. Ordinary planning commands in
   the verified released contract refuse legacy Markdown-journal and event-log stores; migration
   support is not permission to read or write them normally. Unknown/future formats or selectors
   remain unsupported until the selected binary demonstrably supports them. Relay refusals.

## One decision for this repository and product

Keep a session record keyed by `(repository identity, AEP, session)` containing the observations,
release provenance, selected store, effective binary, offered target, affected files, verification
commands and decision (`authorized`, `declined`, `pending`, `current` or `freshness-unknown`).
Reuse the same record across skills and workers. Never put credentials in it.

When a newer applicable CLI, plugin or repository format exists, show one combined offer: current
and target identities, backend retained, artifact/manifest/pin changes, dry-run and verification.
Check prior user authorization first; an accepted plan covering these exact changes is sufficient.
Otherwise request the missing decision once. A pending or declined offer permits operations the
selected tool supports, but cannot unlock a refused store. Do not repeatedly prompt or install an
older CLI just to continue refused planning operations. With no operator and no authorization,
leave upgrades pending and continue compatible work only.

No offer is needed when all applicable identities are verified current; still record the completed
repository inspection. Dirty source is an inspection finding, not a reason to omit the offer.
Prepare changes in the authorized managed worktree; preserve unrelated edits. A migration needing
clean `.engineering` waits until those changes are safely committed or otherwise handled by their
owner. Never silently stash, discard or overwrite them.

## Apply only the accepted changes

- A new project with no store follows `aep:planning` § 5.
- For an existing legacy store read [legacy-stores.md](legacy-stores.md) and select by backend,
  not only by format number. Its isolated bridge is the exception to ordinary release selection.
- Apply an accepted installation plan through `b10x setup apply --plan <plan-file> --yes`.
  A plugin reload is separate from the effective repository CLI. Preserve project pins unless
  their change was included in the accepted offer.
- Run the supported dry-run, review the actual diff, apply and verify. Finish with the effective
  `aep --version`, project format/backend inspection and `aep plan artifact validate`; capture
  exact output. A failed equivalence check, unavailable database or absent migration support is
  an explicit unresolved gap, never a completed upgrade.

Update the session record with the resulting state and continue the original task. This preflight
is complete only when repository state, capability evidence, freshness and the user's decision are
recorded; a version comparison alone does not complete it.
