---
name: drive
description: Start one governed `aep drive` run over a single story from an ordinary session — check the checkout with `aep doctor`, launch the driver against the project's step map, print the run id and how to follow it. Use when the operator says to drive a story, asks for a driven or governed run, asks to run a story under the engine rather than implement it interactively, or asks why a wave's rules are instructions here and enforced there. It starts a run and reports; it moves no artifact itself, and the driver's refusals are relayed unedited.
---

# `/drive <story-id>`

An interactive session **instructs**; a driven run **decides**. This skill is the entry to the second
one, and it does exactly four things: check the checkout, launch the driver over one story, print how
to follow the run, and stop.

## Read this before you start one

**The walk has never reached `complete`.** `aep`'s own `story:governed-dogfood-run` records two
attempts against real stories of its own backlog: `W4-1/1` on 2026-08-21 stopped in
`establish_verifiers` at $15.42, and `W4-2/1` stopped in `adversarial_verify` at $31.46. Neither
reached the review step, and the story's own acceptance line — *a run that wedges is a recorded
result* — is why they are written down rather than retried until they worked.

What that means for the operator, stated before anything is launched rather than discovered at the
stop:

| | |
|---|---|
| what you get | a run that walks the map, records every state, and refuses every transition the engine will not permit — the enforcement an interactive session cannot have |
| what you should expect | a **stop**, somewhere before `complete`, with a reason. That is the normal outcome today |
| what it costs | real model spend per `llm` step. Both recorded runs cost more than $15 |
| what closes the gap | `aep` `story:governed-dogfood-run`. Until it lands, a driven run is an experiment with a bounded cost, and saying otherwise would be selling it |

Say this to the operator, in one line, before the launch — not after the stop.

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

`aep drive run` walks a **task document**, not a story id — the story is the contract and the task
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
$ aep drive run --project . --map <the map the project declares> \
    --plugin-dir <the plugin directory the run loads> \
    --pause-on-approval --budget-usd <what the operator said> --assume-usd-per-run <what the operator said>
```

**`--budget-usd` and `--assume-usd-per-run` are not optional on a map with an `llm` step, and they
are not yours to invent.** The cap is checked before every session spawn, because one applied
afterwards is a receipt rather than a bound. Ask the operator for both numbers and pass what they
said; a run launched on a guessed budget is a run whose ceiling nobody agreed to.

`--pause-on-approval` runs to the first thing a person owes and exits 0 having persisted. It is the
right default here: the stop is the point of the exercise.

**Dry-run it first, free.** `--max-iterations 0` resolves the plan, allocates a run id and runs
nothing — the whole pre-flight at no cost. Read what it prints, then remove the run directory it
allocated before the real launch.

## 3. Where the nested launch happens, and what to do when it will not

Each `llm` step of the map is a harness session that the **driver** spawns through
`metaharness run claude`. The hermetic scratch home is metaharness's own: the child gets a
constructed environment and a scratch config home rather than this session's, so it does not inherit
the identity, the credential handling or the tool surface of the session you are sitting in. That is
imposed by the adapter, not assembled here, and it is the reason a driven run's writes are
attributable to the run instead of to whoever was logged in.

**Whether that spawn works from inside a Claude Code session is not settled.** So try it, and have
one fallback:

| | |
|---|---|
| **the nested launch is accepted** | report the run id and follow it. Say in the report that the launch was nested, so a later comparison knows which path produced the numbers |
| **it is refused or unsupported** | do not work around it, do not switch harness, and do not retry with different flags. **Print the exact command for the operator to paste into a terminal** — the full `aep drive run` line above, with the real paths, the real budget and the working directory to run it from — and stop |

Both paths end in a report; neither ends in a second attempt. A refusal here is a fact about this
machine's harness nesting, and the operator can act on it in one paste.

## 4. Print the run id and how to follow it

```console
$ aep drive status
```

`status` reports what the store's last run is doing and who holds the lock. `resume` continues one
that stopped.

**There is no `aep drive watch` yet.** It is a proposed verb — `aep` `story:drive-watch-is-a-verb`,
draft — so until it exists, print the script the `aep` repository documents instead:
`scripts/drive-watch` in `beyond10x/aep`, which follows a run's states as they happen and switches
to each new state's transcript by itself. Print the path, say it is a script in that repository and
not a verb of the installed binary, and let the operator run it. Do not print a `watch` command that
does not exist.

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
   interactive session, that is the `wave` skill. This skill drives exactly one story under the
   engine.

## Report

1. The story id, and the task document the run was pointed at with its `derived_from:` line.
2. `aep doctor`'s output, or the failing lines and nothing else if it exited 1.
3. The run id, the map that was selected, the budget passed, and which launch path was taken —
   nested, or printed for a terminal.
4. Where the run is now: `aep drive status`, verbatim, and the `scripts/drive-watch` line for
   following it.
5. Any refusal, verbatim.
