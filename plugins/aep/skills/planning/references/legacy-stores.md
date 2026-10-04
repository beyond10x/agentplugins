# Verified legacy-store compatibility

Compatibility evidence: AEP [0.68.0 planning stores](https://github.com/beyond10x/aep/blob/0.68.0/website/docs/reference/planning-stores.md)
and [migration contract](https://github.com/beyond10x/aep/blob/0.68.0/website/docs/guides/migrate-an-older-store.md).
These versions are a reproducible exception, not a preferred everyday installation.
Read the target release's contract again if these commands or schemas differ.

| Existing store | Supported path |
|---|---|
| `/1` Markdown journal, or manifest-free journal store | Current migration command below; ordinary planning commands refuse it |
| `/1` SQLite or PostgreSQL | Contract-directed manifest conversion to `/5` with `planning_scope` and the same database selector; no Git migration |
| `/2`–`/4` event-log | Isolated pinned bridge below |
| `/1` hybrid | No migration supplied; explicit capability gap, preserve the original store |
| Unknown/future | Inspect target capability evidence; never relabel its version to force acceptance |

For Markdown/journal stores first require a clean `.engineering`. Preview and then apply:

```console
aep plan store migrate git --dry-run
aep plan store migrate git --verify
aep plan artifact validate
```

For a manifest-free journal store append the same `--protocols <pinned-source>` and
`--profile <profile>` to **both** migration commands (values in `aep:planning` § 5). These options
are refused if the manifest already exists. Inspect changed artifact bodies, revisions,
transitions and evidence, then commit through the repository's authorized path.

For SQLite/PostgreSQL, snapshot through the backend's supported mechanism and record a safe
before/after inventory. Follow the release's manifest conversion, retaining database identity and
credentials without printing them; verify the same artifact inventory and validation afterward.
The Git migration is not a general backend converter. Without access to verify the database,
report the conversion unverified and keep it out of the completed work.

For event-log stores install the verified bridge in a unique temporary cache directory. Preserve
the active executable and PATH; execute the bridge by absolute path:

```bash
mkdir -p ~/.cache/aep-bridges
aep_bridge_root=$(mktemp -d ~/.cache/aep-bridges/bridge.XXXXXX)
cargo install --locked --root "$aep_bridge_root" \
  --git https://github.com/beyond10x/aep --rev 9c0f1da44429ff935fa0b2d743457945d51e1c51 aep-cli
"$aep_bridge_root/bin/aep" plan store migrate git --help
"$aep_bridge_root/bin/aep" plan store migrate git --dry-run
"$aep_bridge_root/bin/aep" plan store migrate git --verify
aep --version
aep plan artifact validate
```

The bridge imports moves and evidence; event-log audit/invocation history remains in the original
history. Review this preservation boundary in the accepted offer. A bridge build failure, missing
dry-run support or verification difference stops migration and is reported verbatim. Retain the
small verification evidence, then remove only this task's temporary bridge directory when it is
no longer in use. The active CLI is never replaced or restored as part of this procedure.
