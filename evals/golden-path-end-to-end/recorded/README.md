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
    --plugin beyond10x/agentplugins@adp@<this release> \
    --plugin beyond10x/agentplugins@ess-schema@<this release> \
    --cwd <a fresh copy of the accounts service the page is written against> \
    --budget-usd 25 \
    --observed-at <the date it was observed> \
    --redact \
    --out <a directory outside this repository>
```

`--budget-usd 25` and not the corpus's 5. The second headless run walked all eight steps in 118
turns for $10.96 with only `aep-planning` loaded. The third was the first with `adp` actually
loaded: it took the wave skill, spawned 12 sub-agents, and was still inside step 7 when the cap
stopped it at $15.0014. Record it at 25 — the case costs more with its plugins than without them.
The two `--plugin` pins must be installed at user scope at exactly that version
(`claude plugin list`); metaharness resolves them against the operator's own marketplace checkout
and refuses a pin it cannot find.

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
`ess-schema` for step 3, and `adp` for steps 7 and 8. `aep eval run --plugin-dir` takes one; the
other two go as `--plugin` pins, above. Two things the second run (2026-09-03) showed the working
tree also needs, both about the **child's** `PATH`, which metaharness constructs as
`$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin` and does not inherit:

- the `aep` at `~/.local/bin` is the one the run uses. A stale copy there (0.40.1 beside a 0.44.0 in
  `~/.cargo/bin`) made step 8 stop at `aep doctor: unrecognized subcommand`. aep 0.45.0's
  `eval run` refuses to spawn when that binary is not its own version.
- `ess` has to be on that `PATH` too, or step 3 is drafted by hand and never validated.

**It is also the expensive one.** Eight steps with a fan-out in two of them; a sweep's budget
is mostly spent here. Record it last, when the seven cheaper cases have already shown the
rows decide something.
