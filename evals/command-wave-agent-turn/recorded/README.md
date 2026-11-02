# `recorded/` — the wave command from an agent turn

**Empty on purpose, and this file says what would fill it.** No transcript of this case has been
recorded, and none was synthesized: a hand-written transcript here would be a fixture the case's own
rows were fitted to, which measures the document and not the plugin.

`task check` skips an empty `recorded/` with a printed notice and does not fail.

## The run that would produce it

Live, paid, and refused without both `METAHARNESS_LIVE=1` and a cap:

```console
$ METAHARNESS_LIVE=1 metaharness aep drive eval run \
    --case evals/command-wave-agent-turn \
    --arm plugin \
    --harness claude \
    --plugin-dir plugins/aep \
    --cwd <a checkout with a wave plan naming two ready stories and no recorded scope> \
    --budget-usd 5 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

Copy `<out>/<name>.events.jsonl` in with the manifest's `observed_at`, harness version and model.

## What it needs in the working tree

A Git checkout on its base branch with a wave plan page whose first wave names two accepted stories
the wave reference treats as ready (nothing blocking, no unfinished `depends_on`), each serving a
declared objective, neither with a recorded scope, and a small source tree each would touch. The
store also holds one draft tagged `from-finding` that is in no wave, so a run that proposes it has
drawn from the backlog.
