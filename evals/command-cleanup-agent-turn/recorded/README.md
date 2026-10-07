# `recorded/` — the cleanup command from an agent turn

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/command-cleanup-agent-turn \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/worktree \
    --cwd <the primary checkout of a repository with the two managed trees below> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

An activated `worktree` workspace profile and a primary checkout with a remote, and two managed
trees of it: `wt-merged`, finished, its branch pushed and merged, so `worktree gc --dry-run` reports
it eligible; and `wt-leased`, holding a live lease taken by another session just before the run, so
the dry-run retains it.
