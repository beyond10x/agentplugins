# `recorded/` — the wave's adversary

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
    --case evals/adversary-tests-only \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/adp \
    --cwd <the implementor's worktree, suite green, with the unit brief in it> \
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

A worktree in which one unit has been implemented and its suite is green, laid out with an
implementation under `src/` and its tests under `tests/` — the two globs the rows scope to.
The unit brief must name an assigned scratch directory (`plugins/adp/skills/wave/references/unit-brief.md`),
because `nothing-was-written-to-tmp` measures the rule that scratch is assigned rather than chosen.

**The unit must have a defect to find.** Hard rule 4 says finding nothing is a result, so a
genuinely clean unit produces no test file and turns `a-test-file-was-written` red for a
reason that is not about the plugin. Record this case against a unit whose defect is known.
