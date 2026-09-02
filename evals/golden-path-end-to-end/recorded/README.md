# `recorded/` — the golden path, end to end

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail. `aep eval run
--stream` is what reads a file once one is here; drop it in this directory and the replay picks it
up with no change to the case.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 aep eval run \
    --case evals/golden-path-end-to-end \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep-planning \
    --cwd <a fresh copy of the accounts service the page is written against> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

`--redact` is not optional for anything committed here: an un-redacted record quotes the transcript,
and a report that quotes a transcript is not a thing to publish.

The run leaves `<out>/<name>.events.jsonl` beside its manifest and record. **The stream is what
belongs in this directory**, copied in with the manifest's `observed_at`, harness version and model
recorded beside it — a transcript with no provenance is a file, not evidence.

## What it needs in the working tree

The accounts service the page is written against: a README with a *Not yet decided* heading,
one module holding `create_account`, `read_account`, `update_account` and `delete_account`,
a `TODO` at the deletion site, and **no `.engineering/` directory**. § 1 of the page is an
adoption step, and it measures nothing against a tree that has already been adopted.

This is the one case that needs two plugins installed — `aep-planning` for steps 2 to 5 and `adp`
for step 6 — and **`aep eval run --plugin-dir` takes one** at 0.42.0
(`crates/protocol-cli/src/eval.rs`: `plugin_dir: Option<PathBuf>`). `metaharness run claude` does
take several. So a run launched through the command above installs `aep-planning` and reports
`the-wave-skill-was-offered` as a gap, which is why that row is advisory and says so. A run that
must satisfy it is launched through `metaharness` directly, and its stream is ingested here with
`aep eval run --stream`.

**It is also the expensive one.** Six steps with a fan-out in two of them; a sweep's budget
is mostly spent here. Record it last, when the five cheaper cases have already shown the
rows decide something.
