# `recorded/` — the drive command from an agent turn

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap. The case's own run launches
nothing paid; the cap is for the session that runs it:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/command-drive-agent-turn \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a checkout with story:credential-store accepted and no task document naming it> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A Git checkout with a current AEP store holding `story:credential-store` as accepted work, no
task document whose `derived_from:` names it, and `aep` on the path so `aep doctor` can run.
