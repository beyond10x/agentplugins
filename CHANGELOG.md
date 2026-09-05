# Changelog

## [Unreleased]

## [0.8.0] — 2026-09-05

- Fix Connector readiness evaluation for Claude `Bash`/`command` and Codex
  `exec_command`/`cmd` traces. A shared Rust command checker runs alongside AEP's session checks
  in replay and live CI; parsed help commands no longer count as mutations. Regression cases
  reject real mutations, missing doctor evidence, and mutations chained after help.
- Publish the connectors plugin in the release-pinned installation guides for both hosts.
- Repair YAML quoting in the ESS skill description so every released plugin validates.

- Add the focused `connectors` plugin to both marketplaces, with one shared skill for the
  Connectors 0.6.0 CLI, secret-safe onboarding, readiness diagnostics, and governed operation
  invocation. Include host installation guidance and a readiness eval case.

- **The planning skill says how to take an edge back.** AEP 0.53.0 adds
  `aep plan artifact unrelate <id> <relation> <target>`, and the skill now names it in the write-verb
  list, in guardrail 2, in the mutation-ownership table and in the renaming paragraph. Until this
  landed the skill described a store where a relation could be added and never removed, so a session
  that made a backwards `blocks` edge had nowhere to put the correction but the artifact's body —
  which is what a recorded run did on 2026-09-04. A refusal naming `unrelate` as an unknown verb now
  means the installed CLI predates 0.53.0, and the skill says so.

- Every authored document teaches the **grouped** CLI spelling. AEP 0.52.0 replaced a flat first
  level of twenty-three verbs with four areas plus `doctor`, and ESS 0.12.0 replaced twenty with
  four;
  both keep every flat spelling as a hidden alias with identical stdout, stderr and exit status, so
  **nothing here was broken and nothing is fixed** — what changes is what a skill, a charter, a
  reference, a page and an eval case teach a reader to type.

  | was | is |
  |---|---|
  | `aep validate resolve inspect evaluate explain describe schema workflow` | `aep govern <verb>` |
  | `aep artifact serve entity audit workspace conformance reverse` | `aep plan <verb>` |
  | `aep eval {matrix,run}` | `aep drive eval {matrix,run}` |
  | `aep trace contract property specification evidence` | `aep observe <verb>` |
  | `ess validate compile compose inspect graph realization runtime` | `ess specify <verb>` |
  | `ess synthesize project schema build release stack deployment` | `ess generate <verb>` |
  | `ess conform diff impact` | `ess verify <verb>` |
  | `ess import` | `ess infra import` |

  `aep drive run|status|resume|transition`, `aep doctor` and `ess generate --path …` are what they
  were: those verbs did not move. The `protocol` spelling follows `aep`'s, since they are two names
  for one binary.

- The eval corpus's command matchers **accept either spelling** rather than moving to the grouped
  one. `'(aep|protocol) artifact +move'` is now `'(aep|protocol) +(plan +)?artifact +move'`: the
  committed transcript of `golden-path-end-to-end` was recorded under the flat spelling and a row
  that stopped matching it would be a specification describing a run other than the one recorded,
  while a row that only matched the flat spelling would report `never_occurred` against the next
  recording made by an agent reading the migrated skills. Widening witnesses a claim; it does not
  widen one. The same holds for the `tool.absent` row in `adversary-tests-only`, where accepting
  both spellings is what keeps *"no `artifact` command ran"* from being satisfiable by typing the
  other one.

- `agentplugins-check` refuses a flat spelling in any `.md`, `.yaml` or `.yml` this repository
  authors, rather than leaving `AGENTS.md` § *Invariants* to a person remembering to grep. Sweeping
  the tree found 360 of them across 47 files, five of which were in a file no hand list had — the
  paid eval workflow.

  The sweep exempts four prefixes, each visible in a diff:

  | exempt | why |
  |---|---|
  | `CHANGELOG.md` | what a command *was* called |
  | `changes/` | a dated record, written on the day |
  | `.engineering/` | the planning store, whose only writer is the `aep` CLI |
  | `.github/workflows/` | a program, not a lesson: `eval.yml` pins `AEP_VERSION: '0.44.0'`, so its spelling has to be the surface that version has |

  Rust sources are not swept for the same reason as the last row: `evals.rs` spawns
  `aep eval run --stream` against whichever binary is on `PATH`, and that call is a call.

- The install page pins **AEP 0.52.0** and **ESS 0.13.1**, the latest published release of each, and
  the versions whose `--version` lines it quotes. The grouped spelling the pages now teach exists in
  AEP from 0.52.0 and in ESS from 0.12.0; the previous pins (0.51.0 and 0.11.1) have neither.

## [0.7.0] — 2026-09-04

- Every product plugin now carries its product's name and the verb it performs. `aep-planning`
  becomes `aep-plan`, `adp` becomes `aep-drive`, and `ess-schema` becomes `ess-specify` with its
  skill directory and frontmatter name both `specify`. The other two plugins, `beyond10x` and
  `workspace-hygiene`, keep their names, and so do the skill directories `planning`,
  `story-migration`, `drive`, `wave`, `beyond10x`, `plugin-creator` and `worktree`.

  | old | new |
  |---|---|
  | `aep-planning:planning` | `aep-plan:planning` |
  | `aep-planning:story-migration` | `aep-plan:story-migration` |
  | `aep-planning:decomposer` | `aep-plan:decomposer` |
  | `aep-planning:plan-reviewer` | `aep-plan:plan-reviewer` |
  | `aep-planning:reverse-engineer` | `aep-plan:reverse-engineer` |
  | `aep-planning:plan-critic-acceptance` | `aep-plan:plan-critic-acceptance` |
  | `aep-planning:plan-critic-design` | `aep-plan:plan-critic-design` |
  | `aep-planning:plan-critic-scope` | `aep-plan:plan-critic-scope` |
  | `aep-planning:plan-critic-parallel-safety` | `aep-plan:plan-critic-parallel-safety` |
  | `adp:drive` | `aep-drive:drive` |
  | `adp:wave` | `aep-drive:wave` |
  | `adp:story-scoper` | `aep-drive:story-scoper` |
  | `adp:implementor` | `aep-drive:implementor` |
  | `adp:adversary` | `aep-drive:adversary` |
  | `ess-schema:ess-schema` | `ess-specify:specify` |

  Reinstall under the new names: the marketplace no longer offers the old ones. The workflow id
  `adp/default` and the protocol id `adp/1` are unchanged — they name the AEP development profile,
  which this rename does not reach.
- `agentplugins-check` refuses a retired plugin name anywhere this repository authors — in a file's
  text, and in where the file sits — rather than leaving `AGENTS.md` § *Invariants* to a person
  remembering to grep. The path half is what an incomplete `git mv` leaves: a segment equal to a
  retired name under `plugins/`, `website/docs/plugins/` or `evals/` is refused even when the file's
  own text spells nothing.

  The sweep exempts exactly seven things, each visible in a diff:

  | exempt | why |
  |---|---|
  | `CHANGELOG.md` | records what the names were |
  | `changes/` | dated change records, written on the day |
  | `.engineering/` | the planning store, whose only writer is the `aep` CLI |
  | `evals/*/recorded/` | a transcript, its manifest and the README that produced them are evidence of a run under the old names |
  | `crates/agentplugins-check/src/main.rs` | has to spell what it forbids: the `RETIRED` table, the matcher's doc and its rule tests. Every plugin name in it is one `plugin`, `marketplace` or `critic_pins` resolves against the tree |
  | `adp/…` | the wire ids `adp/1` and `adp/default`, unless a `/` in front makes it a path into the plugin directory |
  | a line marked `# recorded-under-this-name` | **only** in `evals/*/expectations.trace.yaml`, where a transcript sits beside the row. The marker excuses nothing in a README, an install block or a manifest |

  `.git/`, `target/` and `node_modules/` are not exemptions; they are not authored files and are
  never walked.
- A replay is judged by the report it writes and no longer by its exit status. Measured on
  aep 0.44.0: `aep eval run --stream` prints *"not conformant: the run contradicted 2
  expectation(s) … (exit 1)"* and exits **0**, so the gate reported *"1 recorded transcript(s)
  replayed"* over a corpus that contradicted its own recording. On `protocol 0.50.0` the same
  fail-open has a second word: a gating row nothing in the transcript can decide prints
  *"undecided … (exit 3)"*, exits **0**, and is left at `verdict: "unknown"` — so reading only `gap`
  would have closed one half. The gate now refuses any report whose document `verdict` is not `ok`,
  which is `aep`'s own arithmetic over severity and `on_unknown:`, and refuses any report that does
  not claim `trace-report/1` or states no verdict at all.
- Five `env.*_available` rows in `golden-path-end-to-end` keep the plugin names of 2026-09-03,
  because they state what the recorded harness listed and that recording predates the rename.
  Renaming them was measured on that stream — 27 of 27 rows `ok` before, 22 of 27 after, two of them
  gating — so each carries `recorded-under-this-name` and moves when the case is re-recorded.
- The eval case `ess-schema-new-entity` is now `ess-specify-new-entity`, and the website's plugin
  reference pages are `plugins/aep-plan`, `plugins/aep-drive` and `plugins/ess-specify`.

## [0.6.2] — 2026-09-03

- Require non-trivial cross-repository and release/deployment work to enter the owning AEP store
  before implementation continues, and route such work through the planning specialist from the
  Beyond10x front door.
- Publish exact Codex CLI commands for a fresh release-pinned install and for replacing a stale
  immutable marketplace pin, installing and enabling all five focused plugins.
- Refresh the adopter binary guidance to ESS 0.9.2 and align every plugin manifest and shared skill
  version with this release.

## [0.6.1] — 2026-09-03

- The golden-path case's step 1 names the protocol source the page names,
  `git+https://github.com/beyond10x/aep#8b4342a41fdd914341d9a479627ed76240f88d45`, instead of "the
  source the project pins". The second headless recording found no pin — the fixture has no
  `.engineering/` by design — walked the filesystem for one and adopted with a relative path into
  another checkout.
- Two golden-path rows stop contradicting a correct run. `nothing-was-published` matches the write
  forms of `git tag` only (a name, `-a`, `-s`, `-f`, `-m`); the bare listing `git tag` in a step-1
  repository inspection took the row to `gap`. `nothing-was-promoted-before-the-blocker-was-filed`
  spells out the rungs that schedule work — `--to proposed|active|implemented` — so archiving two
  probe artifacts no longer reads as a promotion.
- The golden-path cost row and the documented invocation say `15`, not the corpus's `5`: the second
  recording walked all eight steps in 118 turns and stated $10.96 on the default model. The
  invocation also carries the two `--plugin` pins the case needs, and `recorded/README.md` names the
  two things the second recording showed the *child's* `PATH` must hold: the `aep` at `~/.local/bin`
  is the one the run uses, and `ess` has to be there too.
- `.github/workflows/eval.yml` installs the pinned `aep` into `$HOME/.local/bin` as well: the
  session metaharness spawns runs on a constructed `PATH` that never held the `~/.cargo/bin` copy, so
  every case task that ran `aep` in CI was running nothing.
- The planning skill's guardrail 2 gains two corollaries from that recording: never create an
  artifact to look at its template (a probe you archive is still a lifecycle move), and a body file
  for `--from` goes under `$TMPDIR`, never a hard-coded `/tmp`.

## [0.6.0] — 2026-09-03

- Rename the ESS skill's directory from `skills/schema-validation` to `skills/ess-schema`, its frontmatter name: the harness lists a skill by its directory and the eval rows and `agentplugins-check` resolve by the frontmatter name, so the first recording showed the skill offered and called on turn 1 while the row said it was not. The invocation is now `/ess-schema:ess-schema`.
- Give the planning and wave skills a non-interactive rule. Where the task says to run without
  stopping or no operator is present, or the harness gives no operator turn at all, the skill no
  longer ends its turn at a stop: it records one `approval-record` through `aep artifact new` —
  tagged `non-interactive`, naming the stop, what was decided in the operator's absence and what the
  operator would have been asked — and continues. The kind is the store's own and both commands that
  settle it are named, so a store that spells it differently is answered by the CLI rather than by
  the skill. A bypass covers the **asking** and nothing else: a move whose evidence does not exist is
  still refused, an open `decision-blocker` is never moved to `cleared`, a red unit still leaves the
  wave, a gate that is not green is still not green, and the release stop is not bypassable. Nothing
  is auto-approved silently — every bypass is an artifact `aep artifact list --kind approval-record`
  returns.
- Add § *Running it without an operator* to the golden path: the exact instruction to put in front of
  step 1, a table of what is recorded at each of the page's five stops, and the four things a
  headless run cannot do — clear a blocker, publish, release, or turn a refusal into a pass.
- Point the `golden-path-end-to-end` case's task at that instruction, verbatim, and give it five
  rows about the records it produces: an `approval-record` per stop created through the CLI with a
  body, the `non-interactive` tag that makes them findable, a per-stop count rather than one closing
  summary, and the two prohibitions. Its first headless run — 2026-09-03, $0.80, 173 events — adopted
  and scanned the repository and ended after step 1 with ten rows gapped and seven held; nothing it
  did was wrong, and `recorded/README.md` now says so beside what a recorded stream should contain.
- Have the decomposer return its unanswerable questions rather than hold a turn open for them: an
  unresolved epic id and a `requires-stakeholder-input` relation are both reported and returned, with
  the blocker filed and everything not behind it drafted.
- Fix `the-scope-was-actually-tested`, which gapped in all eight eval cases on their first live
  recording (2026-09-03) without one of the eight runs doing anything wrong. It asserted
  `permission.denied: {at_least: 1}` against a seam that observes and never adjudicates — every
  `session.started` records `permission_mode: default` and every `tool.decided` reads
  `decision: allow`, `decided_by: observe` — and it contradicted `nothing-was-refused` directly below
  it, which asserts `{at_most: 0}` over the same quantity. The row now selects **what the run
  touched**, one surface per case, in `trace-spec/1`'s neutral `operations:`/`subject:` selector.
  Re-checked offline against the eight recordings the gap counts go 2→1, 1→0, 2→1, 10→9 and 1→0 four
  times, with exactly one row's verdict moving in each. `evals/README.md` § *A control has to be able
  to pass* carries the argument, including why a subject glob is anchored with a leading `*` rather
  than written relative to the case's `--cwd`.
- State the adversary's authoring order where its charter lists its steps, and again in its report
  format: the failing case is written and its red output captured **before** the suite is run. The
  first recording of `adversary-tests-only` ran `task check` and wrote the case afterwards, which the
  charter's own `cases: executed <before>→<after>` line invited; hard rule 3 now names the two honest
  sources of `<before>` and refuses a pre-emptive suite run.
- Sharpen the `ess-schema` trigger: an entity introduction names the noun *and* something typed about
  it — an identifier, a field, or a relation to another noun — and a repository with no `system.yaml`
  anywhere is called out as in scope rather than left to be read into "whether or not a specification
  exists yet". The 0.4.0 widening is kept verbatim; the added sentences exclude the planning prose
  that merely mentions a noun.
- Record what the first recording actually settled about `the-skill-was-offered`, which is not what
  it looked like: the harness lists the ESS skill by its **directory**,
  `ess-schema:schema-validation`, while the case asserts the frontmatter name, `ess-schema:ess-schema`.
  The skill was offered and the run invoked it as its first tool call, so what gaps is the spelling —
  not the trigger and not the run. Closing it means either renaming the skill directory or re-spelling
  the row against what `agentplugins-check` resolves, and neither is taken here.

## [0.5.1] — 2026-09-03

- Correct the adopter path to install and verify the current AEP and ESS release binaries, pin the
  Claude Code marketplace source to this immutable release, reload installed plugins, and state
  that `ess-schema` requires the `ess` CLI.
- Re-record the eight-step golden path against AEP 0.44.0 and ESS 0.5.1 now that ESS validates
  entity relations, and keep its behavioural eval aligned with all eight published prompts.

## [0.5.0] — 2026-09-03

- Add `evals/`: one `eval-case/1` per agent and per user-facing skill — the four plan critics, the
  decomposer's relation census, `ess-schema` on a new entity, the published golden path end to end,
  and the wave's adversary — each judged by its own `trace-spec/1` document and run by
  `aep eval run --corpus evals`. Every row is about what a run did; none greps the agent's markdown,
  because a check that asserts a sentence is still written is the thing a behavioural case replaces.
- Record no transcript and synthesize none. Each case's `recorded/` carries a README naming the live
  command that would fill it, its budget, and what the working tree has to hold for the case to
  measure anything. The two existing transcript sets were checked and neither fits: `aep`'s
  conformance corpus states that its files are structurally faithful and not observed, and
  `aep eval run --stream` refuses them besides (`EVAL-STREAM-004`); `metaharness`'s check inputs are
  stated to be hand-written.
- Teach `agentplugins-check` the corpus, in a module of its own: every case's `id` is its directory
  name, its expectations document parses and declares one kind per expectation, and its `subject:`
  resolves to an agent or a skill this repository actually ships — read from the file's own
  frontmatter, not from its directory, because `ess-schema`'s skill directory and declared name
  differ. `task check` gains it for free; `task evals` runs that half alone.
- Replay whatever transcripts exist with `aep eval run --stream`, which spends nothing. An empty
  `recorded/`, and a machine with no `aep` on `PATH`, are printed notices and never a red gate — the
  same position the runner itself takes on a missing `metaharness`. A recorded stream without the run
  manifest beside it *is* refused: a transcript with no observation date cannot be replayed into a
  document that reproduces.
- Add `.github/workflows/eval.yml`: the live arm runs only with the `run-eval` label or a manual
  dispatch, only for the cases whose subject the diff touched, under `EVAL_BUDGET_USD` (default 20).
  A run needing more than the budget is refused **before a tool is installed**, with the four numbers
  that decided it in the check summary, and the `aep eval matrix` table is posted as the job summary.
  The credential is the organization bot's and is refused by name when absent; a personal key is
  never a substitute.
- Add `README.md` § Evals with the arithmetic: 8 cases at a $5 per-case cap is $40 for one full live
  run of one arm on one harness, against a $20 default budget — so a full sweep does not fit its own
  default and is refused rather than silently truncated. What fits is the diff-scoped run a pull
  request touching one agent actually selects.
- Pin the four plan critics: `model: sonnet` and `effort: high` in the frontmatter of every
  `plan-critic-*.md`, refused by `agentplugins-check` when either is missing. The rubric states the
  pin and states that it is a **default** — the pairing the compared third-party panel uses, adopted
  so the cost is written down while no local measurement exists — expected to change once there is a
  review-value table to read. The decomposer, adversary and implementor stay unpinned.
- Have the wave select on `aep artifact waves` rather than on its own reading of the pairs: the
  selection step runs the verb first and pastes its waves, collisions and unassessed lists into the
  proposal verbatim, dispatches a scoper for every unassessed story before proposing, and falls back
  to the pairwise prose reading only on `unrecognized subcommand` — saying so, with the version. A
  new *Failure modes* section decides the case the two paths will hit most: **the verb wins, and the
  disagreement is reported**, because the verb reads a record anybody can re-read and the prose
  reading is one agent's inference that will not survive the session.
- Write a story's scope twice — the `## Scope` section for a person, and `aep artifact scope --add`
  entries for the store, carrying the same `cited`/`inferred` mark. A prose section alone leaves the
  next wave re-deriving what this one established.
- Record what became of every review finding. The critic step writes one `review_outcome` per
  finding after a revision round (`fixed`, `no-op`, `escalated`), and the wave writes one as it takes
  each row of the adversary route table — the outcome is the row it took, not an opinion of the
  finding. Both say what to do on a binary that refuses the kind: put the counts in the report,
  never hand-edit a store file.
- Have the critic rubric and the adversary close their reports with a fenced ` ```findings ` block —
  the same findings as the prose, with `file`, `line`, `category`, `severity`, `verdict`, `origin`
  and `message` — so a later pass is compared by signature instead of by re-reading two reports. The
  wave records each adversary pass as a `review-result` holding that block, and before deciding on a
  third attack it runs `aep artifact findings` and pastes the carried, new and resolved lists.
- Require a domain relation to be a `relations:` entry in an `ess/1` document. `ess-schema`'s minimal
  document carries one `owns` relation and its refusal list gains the three that come with it — an
  unknown target, a missing or mistyped `via`, a second owner — with `via` on the target for `owns`
  and on the source for `references`. Guardrail 7 says the same, and adds that a relation whose
  cardinality is unknown is an `UNMAPPED:` marker rather than an entry with a guessed value.
- Require the decomposer's `inferable` citation for a relation to point at an `ess/1` document. A
  `path:line` into code is admissible only when the classification carries the word `inferred`,
  because a foreign key says what one implementation currently does and nothing about whether anybody
  decided it; the relation census now reports the split.
- Add step 3 to the golden path — model the new noun before decomposing it — carrying the `owns`
  relation between account and commercial client, with the delete behaviour left `UNMAPPED:`. That
  narrows the page's open question from three answers to two before the blocker is filed. Its
  document block is marked as **recorded before ESS shipped relations**, to be re-recorded when the
  construct lands, rather than showing output nobody produced.
- Add a `drive` skill to `adp`: `/drive <story-id>` runs `aep doctor` and stops on a `fail`, points
  `aep drive run` at the task document naming the story, launches it against the project's step map
  with a budget the operator states, and prints the run id. Each `llm` step's session is spawned by
  the driver through `metaharness run claude` into metaharness's own hermetic scratch home; where
  that nested launch is refused or unsupported the skill prints the exact terminal command instead
  and stops, rather than working around it. It moves no artifact, relays refusals verbatim, and says
  plainly that the walk has not reached `complete` on the `aep` side — two recorded runs stopped at
  `establish_verifiers` and `adversarial_verify` — so a driven run today is an experiment with a
  known cost. `aep drive watch` does not exist yet, so it prints the `scripts/drive-watch` path the
  `aep` repository documents. The golden path gains it as a final step.

## [0.4.0] — 2026-09-02

- Give `website/docs/install.md` and `README.md` one copy-paste install block — `/plugin marketplace
  add beyond10x/agentplugins` plus `aep-planning`, `adp` and `ess-schema` — and say before it that
  the planning and development plugins do nothing until the `aep` binary from the `beyond10x/aep`
  releases is on `PATH`, checked with `aep --version`.
- Add a golden-path page to the site: one recorded run from a feature idea to a critiqued plan on a
  repository that already exists, with the prompts to paste and every CLI block produced by running
  it under `aep` 0.41.0. The worked CRUD example shows the decomposer filing a `decision-blocker`
  with a `blocks` edge for the one entity relation nobody had decided, rather than drafting stories
  around it; the front door's resource list names the page.
- Teach the `decomposer` agent to enumerate the epic's domain relations before drafting any story:
  each is classified `inferable` — with the `path:line` or artifact that settles it, written into the
  story body — or `requires-stakeholder-input`, which becomes a `decision-blocker` with a `blocks`
  edge and no story, so an undecided relation reaches the operator as a question instead of a plan
  that improvised the answer.
- Add a plan-time critic panel to `aep-planning`: four read-only critics (acceptance, design, scope,
  parallel safety) argue with a decomposition before an operator reads it, each verdict recorded as
  an immutable `review-result` through `new --from`, revisions bounded at two rounds with open
  findings reported. The plugin validator now requires the four critics and their rubric.
- Add a domain-first guardrail to the planning skill: an epic or story that introduces a new noun
  models it as an `ess/1` document (or imports one with `aep reverse openapi`) before stories are
  written around it, with every unread relation marked `UNMAPPED:` rather than guessed.
- Widen `ess-schema` to fire when a story or epic introduces an entity, and say how to start a
  domain from nothing: the minimal `ess/1` document that `ess validate` accepts, the three refusals
  that shape it, and the `UNMAPPED:` marker.
- Adopt a planning store of this repository's own under `.engineering/`, pinned to the `aep`
  protocol tree at its 0.41.0 commit; the epic and stories behind this release are its first entries.
- Author the planning skill's first-store instruction as `aep artifact list`; `protocol` was the
  one remaining authored spelling.

## [0.3.7] — 2026-09-02

- Correct `story-migration`: a re-run does not replay, it is **refused**. 0.3.6 said the create
  command's idempotency key made an identical migration replay; running one against a real store
  showed the create rejected with `already exists at <path>`. Equally safe, different mechanism —
  and the sentence as written told a reader to expect a write that never happens. The refusal is now
  quoted, with the exit code to expect and the two things to check on the second run.
- Teach both skills AEP's `refs:` field, released in AEP 0.41.0. A ticket id found in a legacy file
  becomes `--ref jira:DEV-630` in frontmatter rather than only a line of prose, so
  `aep artifact list --ref jira:DEV-630` answers *what, here, is this ticket*; the URL is configured
  once in `.engineering/project.yaml` instead of copied into every artifact.
- Correct the version line in `planning` and `adp/wave`: both claimed 0.3.3 while their manifests
  had moved on.

## [0.3.6] — 2026-09-02

- Add the `story-migration` skill to `aep-planning`: migrate a repository's existing story tree,
  `TODO.md`, plan and issue documents into the planning store without deleting or rewriting the
  sources, and backlink both directions so no repository ends up with two plans.
- Retain each source's dates in the migrated artifact's body, read from git rather than from
  filesystem mtime, because AEP frontmatter carries no timestamp field and a fresh checkout would
  date every artifact to the day the migration ran.
- Point `planning` § 5 at it, so adoption in a repository that already tracks work does not start
  by hand-writing a second backlog.

## [0.3.5] — 2026-09-02

- Refresh the generated `worktree` skill so it records that `gc --repo` selects the activated
  workspace profile rather than the repository, that its dry-run therefore lists trees belonging
  to other repositories, and that `status` accepts no filter at all.
- Pin the public installation guidance to the matching Worktree 0.3.2 release.

## [0.3.4] — 2026-09-02

- Pin the documentation redirect façade to the 21-source Website verifier runtime.

## [0.3.3] — 2026-09-02

- Refresh the generated `worktree` skill for the guarded recovery of finished external legacy
  trees stranded by stale pre-0.3 relocation intent.
- Pin the public installation guidance to the matching Worktree 0.3.1 safety release.

## [0.3.2] — 2026-09-02

- Refresh the generated `worktree` skill for review-bound cleanup, durable removal recovery,
  interrupted-provisioning reconciliation, and explicit legacy-tree handling.
- Pin the public installation guidance to the matching Worktree 0.3.0 safety release.

## [0.3.1] — 2026-09-01

- Make GitHub release publication idempotent so rerunning a successful tag publication preserves
  the existing release instead of leaving a false-red workflow.

## [0.3.0] — 2026-09-01

- Add the `workspace-hygiene` plugin with the `worktree` CLI-generated skill for consistent,
  recoverable Git worktree lifecycle across agent hosts.
- Document installation and the separation between lifecycle guidance and the standalone public
  Rust toolchain.

## [0.2.0] — 2026-09-01

- Add the `beyond10x` front-door plugin for marketplace routing, public resource discovery, and
  portable Codex and Claude Code plugin creation.

## [0.1.1] — 2026-09-01

- Correct the marketplace documentation relationship to the public AEP Service documentation
  surface.

## [0.1.0] — 2026-09-01

- Publish the curated `beyond10x` marketplace with focused AEP planning, ADP, and ESS schema plugins.
- Publish adopter documentation for discovering, selecting, and installing each plugin.
