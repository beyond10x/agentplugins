# `recorded/` — the golden path, end to end

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

## What the first run of it found, before anything is recorded here

One live session against this case on **2026-09-03** — `aep eval run`, $0.80, 173 events — adopted
the repository, scanned it, reported *step 1 — adopted, scanned* and ended. Ten rows gapped and seven
held: no wave, ESS or drive skill offered, no fan-out, no blocker with an edge, no `review-result`,
and the store never validated. Its stream is not in this directory, so this paragraph is what the
run said and not a document anybody can replay; it is written down because it is the observation the
case's task changed for (`.engineering/planning/story/non-interactive-golden-path.md`).

Nothing in that run was wrong. The page is eight prompts an operator types across eight turns, and a
case is **one task with no operator turn in it**, so the run reached the first stop and stopped. The
task now opens with the page's § *Running it without an operator* instruction, and the expectations
gained five rows about what that produces: an `approval-record` per stop, created through the CLI
with a body and the `non-interactive` tag, and the two prohibitions — no blocker cleared, nothing
tagged or pushed. **A recorded stream of this case is now expected to contain `aep artifact new
approval-record` calls**; one that walks all eight steps and carries none of them satisfies every
ordering row here and is the failure the instruction exists to catch.

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

This is the one case that needs three plugins installed — `aep-planning` for the planning steps,
`ess-schema` for step 3, and `adp` for steps 7 and 8 — and **`aep eval run --plugin-dir` takes
one** at 0.44.0
(`crates/protocol-cli/src/eval.rs`: `plugin_dir: Option<PathBuf>`). `metaharness run claude` does
take several. So a run launched through the command above installs `aep-planning` and reports
`the-wave-skill-was-offered` and `the-ess-skill-was-offered` as gaps, which is why those rows are
advisory and say so. A run that must satisfy them is launched through `metaharness` directly, and
its stream is ingested here with `aep eval run --stream`.

**It is also the expensive one.** Eight steps with a fan-out in two of them; a sweep's budget
is mostly spent here. Record it last, when the seven cheaper cases have already shown the
rows decide something.
