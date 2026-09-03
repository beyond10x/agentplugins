# `recorded/` — acceptance critic verdict

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
    --case evals/plan-critic-acceptance-verdict \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep-plan \
    --cwd <a checkout with a drafted `epic:commercial-clients` in its store> \
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

A store holding `epic:commercial-clients` and at least two draft stories decomposed from it — the
shape [the golden path](../../../website/docs/golden-path.md) § 3 produces. With fewer than two stories
the panel step is skipped and the case measures nothing.
