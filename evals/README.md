# The eval corpus

One directory per case, and a directory holding a `case.yaml` **is** a case — nothing registers one
anywhere. `task check` enumerates this tree; `aep eval run --corpus evals` runs it.

A case is four things and no others:

| Part | File | What it is |
|---|---|---|
| the task statement | `task:` in `case.yaml` | what an agent is asked to do, in the words it would be asked in |
| what it is about | `subject:` in `case.yaml` | the agents, skills and paths this case judges — the diff scope a live CI run uses |
| the expectations | `expectations.trace.yaml` | a `trace-spec/1` document: what the run must have looked like |
| the transcript | `recorded/` | a recorded run, replayed through the checker for nothing |

## The eight cases

| Case | Subject | The claim it holds the subject to |
|---|---|---|
| `plan-critic-acceptance-verdict` | `aep-planning:plan-critic-acceptance` | the verdict became an immutable `review-result` through `new --from`, and nothing moved |
| `plan-critic-design-verdict` | `aep-planning:plan-critic-design` | the same, for the shape lane |
| `plan-critic-scope-verdict` | `aep-planning:plan-critic-scope` | the same, for the coverage lane |
| `plan-critic-parallel-safety-verdict` | `aep-planning:plan-critic-parallel-safety` | the same, for the concurrency lane |
| `decomposer-relation-census` | `aep-planning:decomposer` | an undecided relation became a `decision-blocker` with a `blocks` edge, filed before the first story |
| `ess-schema-new-entity` | `ess-schema:ess-schema` | the noun got a validated typed home before a story rested on it, and the unread relation stayed unread |
| `golden-path-end-to-end` | `website/docs/golden-path.md` | the six published steps in the published order, with the CLI as the store's only writer |
| `adversary-tests-only` | `adp:adversary` | tests were written, `src/` was not touched, and no `aep artifact` command ran |

## `recorded/` is empty, and that is stated rather than implied

**No transcript in this corpus was recorded, and none was synthesized.** Each `recorded/README.md`
carries the exact live command that would produce its case's stream, the budget it runs under, and
what the working tree has to hold for the case to measure anything.

Nothing here is hand-written, and the difference from `aep/conformance/eval/` is deliberate. That
corpus commits transcripts written by hand against the event stream, says so at length, and uses them
to hold its *checker* to its documents. This corpus exists to hold **plugins** to their charters, and
a transcript written by the same hand that wrote the rows measures the rows. So the replay step skips
an empty `recorded/` with a printed notice and never fails: an unrecorded case is a case nobody has
run, which is a true thing to report and not a broken gate.

Two transcript sets were checked for something reusable and neither fits.
`aep/conformance/eval/*/transcript.jsonl` is stated by its own README to be *structurally faithful
and not observed*, and `aep eval run --stream` refuses all five besides — their `session.started`
states no `hermetic.installed_plugins`, which the run manifest is read out of (`EVAL-STREAM-004`).
`metaharness/evals/aep/checks/transcripts/` is stated by its README to be hand-written inputs to a
discrimination check, *"nothing here came from a model"*.

## What the rows are allowed to assert

Three rules run through every document here, all of them the `aep` corpus's, and each is argued in
the case that applies it rather than only here.

* **No row reads the subject's markdown.** A check that greps an agent file asserts that a sentence
  is still written, which is the failure mode a behavioural case exists to remove.
* **Every absence has a positive control over the same tool.** `tool.absent` is green against a
  transcript carrying none of the agent's calls at all, so an uncontrolled absence reports a dead run
  as a clean one.
* **A vacuous row is worse than a missing one, because it reads like coverage.** Where an agent's
  frontmatter grants no write verb, a `tool.absent` over `Write` is true of every possible run and is
  left out with a note saying so. Only two cases here can make that claim honestly — the adversary,
  whose charter grants `Edit` and `Write` and then forbids most of what they do, and the golden path,
  whose parent session holds them and uses them.

## The CLI has two spellings and the rows carry both

This repository's instructions spell the command `aep` (`AGENTS.md` § *Invariants*); the binary's own
`--version` and usage lines print `protocol`, and an adopter may have either name on `PATH`. Every
command matcher here is a regex over both. A row naming one spelling reports `never_occurred` against
a run that did the work under the other — the checker shrugging at work that visibly happened, which
is the argument `aep/conformance/eval/README.md` makes for scoping a write to a set of verbs. It is a
widening of what can witness a claim, never of the claim.

## Running them

Free, and what `task check` does:

```console
$ cargo run --quiet --locked --bin agentplugins-check
```

Live, which costs money and is refused without both `METAHARNESS_LIVE=1` and a cap — see
[`README.md` § Evals](../README.md#evals) for the arithmetic of a full sweep:

```console
$ METAHARNESS_LIVE=1 aep eval run --corpus evals --workflow adp/default \
    --arm plugin --harness claude --plugin-dir plugins/aep-planning \
    --cwd <a working tree> --budget-usd 20 --assume-usd-per-run 5 \
    --observed-at <date> --redact --out <a directory outside this repository>
```

## Adding a case

```console
$ mkdir -p evals/<slug>/recorded
$ $EDITOR evals/<slug>/case.yaml            # format, id (= the directory name), workflow, states,
                                            # arm, subject, task, expectations
$ $EDITOR evals/<slug>/expectations.trace.yaml
$ $EDITOR evals/<slug>/recorded/README.md   # the run that would fill it, with its budget
$ cargo run --quiet --locked --bin agentplugins-check
```

`subject:` must name at least one agent or skill that exists in this repository, written as the
harness qualifies it — `<plugin>:<the name the file's frontmatter declares>`. The check resolves each
one to a file and refuses a case that names something that is not there, so a case cannot outlive the
agent it judges.
