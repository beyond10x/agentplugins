# `recorded/` — `ess-schema` on a new entity

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
    --case evals/ess-schema-new-entity \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/ess-schema \
    --cwd <a checkout with no ESS specification and a story naming an unmodelled noun> \
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

No `system.yaml` anywhere, a story introducing the shipment, and the `ess` binary on `PATH`
— the skill's own § *Starting a domain from nothing* is written for a repository with no
specification at all, and a tree that already has one measures a different behaviour.
`ess` is published by the sibling `beyond10x/ess` repository; `the-specification-was-validated`
gaps without it, and the gap is about the machine rather than about the run.
