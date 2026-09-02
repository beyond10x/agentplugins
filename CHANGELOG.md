# Changelog

## [Unreleased]

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
