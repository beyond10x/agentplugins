# `/drive <story-id>`

Before acting, complete the [repository preflight](../../planning/references/repository-preflight.md)
or reuse the coordinator’s matching completed record and user decision.

An interactive session **instructs**; a driven run **decides**. This skill is the entry to the second
one, and it does exactly four things: check the checkout, launch the driver over one story, print how
to follow the run, and stop.

## Establish the runner contract

Read the installed Metaharness command help and its stable release notes before launch. Record
the runner version, supported harness adapters and source-matched AEP executable. Historical
dogfood stops are evidence about those runs, not a claim that current runs cannot complete.
Report the actual selected map, budget and stopping conditions to the operator.

## 1. `aep doctor` first, and read it

```console
$ aep doctor
```

One line per check — the binary's version, the project file, the protocol source it names, the
planning store, each plugin directory given, and the newest release tag — each `ok`, `warn` or
`fail`, exit 1 on any `fail`. It fixes nothing, which is what makes it safe to run first.

**A `fail` stops the skill.** Relay the failing lines verbatim and do not launch: every one of them
is a condition the driver would hit later, after money has been spent, and the whole reason this
check exists is that it costs nothing.

Pass `--plugin-dir` for each plugin directory the run will load, because `doctor` checks the ones it
is given and guesses none.

## 2. Point the driver at the story

**`metaharness aep drive run --help` has to answer before anything else.** AEP hands model-backed maps to Metaharness. Verify that this runner provides the command
and the flags the selected map requires.
`unrecognized subcommand 'aep'` means the installed
Metaharness is older than the AEP that sent you here: say so, point at the install page's
Metaharness block, and stop.

`metaharness aep drive run` walks a **task document**, not a story id — the story is the contract and the task
document is what a run needs to resolve a plan against it. It names the story in `derived_from:`,
along with the protocol, the profile, and the facts nothing can observe about the change.

So `/drive story:credential-store` resolves to one of two situations, and you say which:

* **The project already has a task document naming that story.** Use it. Read it first and quote its
  `derived_from:` line, so the operator can see the run is pointed at the story they named.
* **It does not.** Writing one is a decision about scope, profile and surface that the operator owns
  — draft it, show it, and stop. A task document you wrote and launched in one step is a run whose
  constraints nobody read.

**The step map is the project's, and you do not choose it.** Run without `--map` and let the driver
select the one that fits; where two fit, it **refuses and names both**, and that refusal goes to the
operator to answer. Do not pick one to get the run started.

```console
$ METAHARNESS_LIVE=1 metaharness aep drive run --project . --map <the map the project declares> \
    --aep-binary <the AEP executable built from the runner's pinned source> \
    --plugin-dir <the plugin directory the run loads> \
    --pause-on-approval --budget-usd <what the operator said> --assume-usd-per-run <what the operator said>
```

**`--budget-usd` and `--assume-usd-per-run` are not optional on a map with an `llm` step, and they
are not yours to invent.** The cap is checked before every session spawn, because one applied
afterwards is a receipt rather than a bound. Ask the operator for both numbers and pass what they
said; a run launched on a guessed budget is a run whose ceiling nobody agreed to.

**`METAHARNESS_LIVE=1` is the opt-in, and it goes on the command.** Without it a map with an `llm`
step is refused before a run id is allocated — *a model session can cost money; opt in explicitly* —
and that refusal is the answer, not a flag to find a way around.

`--pause-on-approval` runs to the first thing a person owes and exits 0 having persisted. It is the
right default here: the stop is the point of the exercise.

**Dry-run it first, free.** `--max-iterations 0` resolves the plan, allocates a run id and runs
nothing — the whole pre-flight at no cost. Read what it prints, then retain that preflight run’s identity and output before the real launch.

## 3. Where the nested launch happens, and what to do when it will not

Each `llm` step of the map is a harness session that the **driver** spawns using the map’s
declared harness adapter. Discover supported adapters from the installed runner; do not assume
that the current host or every step uses Claude Code. The hermetic scratch home is metaharness's own: the child gets a
constructed environment and a scratch config home rather than this session's, so it does not inherit
the identity, the credential handling or the tool surface of the session you are sitting in. That is
imposed by the adapter, not assembled here, and it is the reason a driven run's writes are
attributable to the run instead of to whoever was logged in.

Check the selected host’s nesting capabilities before launch. Use the authorized adapter and
retain its actual refusal if the environment cannot run it. One fallback:

| | |
|---|---|
| **the nested launch is accepted** | report the run id and follow it. Say in the report that the launch was nested, so a later comparison knows which path produced the numbers |
| **it is refused or unsupported** | do not work around it, do not switch harness, and do not retry with different flags. **Print the exact command for the operator to paste into a terminal** — the full `metaharness aep drive run` line above, with the real paths, the real budget and the working directory to run it from — and stop |

Both paths end in a report; neither ends in a second attempt. A refusal here is a fact about this
machine's harness nesting, and the operator can act on it in one paste.

## 4. Print the run id and how to follow it

```console
$ metaharness aep drive status
```

`status` reports what the store's last run is doing and who holds the lock. `metaharness aep drive resume <run>` continues a compatible paused run, retaining its budget and plugin inputs. A version or integrity refusal is binding.

The AEP planning executable and the Metaharness runner are distinct tools. Pass the source-matched AEP executable with `--aep-binary`; do not substitute the Metaharness binary for planning commands. AEP still provides planning, command-only driving and offline evidence ingestion.

Use `metaharness aep drive status --help` to select the supported status interface. Preserve run
state and transcripts for follow-up; do not prescribe a historical repository script or invent a
watch subcommand.

## Hard rules

1. **You move no artifact.** No `aep plan artifact move`, for any artifact, for any reason. The run's
   moves are the driver's, made through the engine, which is the entire property being tested; a
   move you make beside it is the one thing that makes the record unreadable afterwards.
2. **A refusal is relayed verbatim and ends the turn.** A held lock, missing evidence, two maps that
   both fit, a budget the run would exceed — each names what it wants. Paste the sentence, do not
   summarise it, do not route around it, and do not re-run with a flag that suppresses it.
   `--take-lock` and `--allow-evidence-gap` exist and are the operator's to ask for, never yours to
   reach for.
3. **You start one run.** Not a loop, not a retry, not the next story. A run that wedges is a
   recorded result and reporting it is the work.
4. **Nothing here is a wave.** Where the operator wants several stories implemented at once in an
   interactive session, that is wave mode ([wave.md](wave.md)). Drive mode takes exactly one story under the
   engine.

## Report

1. The story id, and the task document the run was pointed at with its `derived_from:` line.
2. `aep doctor`'s output, or the failing lines and nothing else if it exited 1.
3. The run id, the map that was selected, the budget passed, and which launch path was taken —
   nested, or printed for a terminal.
4. Where the run is now: `metaharness aep drive status`, verbatim, with the supported follow-up invocation.
5. Any refusal, verbatim.
