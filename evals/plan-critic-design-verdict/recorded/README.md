# `recorded/` — design critic verdict

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail. `aep drive eval run
--stream` is what reads a file once one is here; drop it in this directory and the replay picks it
up with no change to the case.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/plan-critic-design-verdict \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a Git checkout of fixtures/library-reservations-drafted> \
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

`fixtures/library-reservations-drafted`, copied out and committed as a Git repository: a store
holding `epic:book-reservations` and the six draft stories decomposed from it, recorded from [Your
first governed plan](../../../website/docs/tutorials/first-governed-plan.md) before its critics ran —
the shape [the golden path](../../../website/docs/golden-path.md) § 3 produces. With fewer than two stories
the panel step is skipped and the case measures nothing.
