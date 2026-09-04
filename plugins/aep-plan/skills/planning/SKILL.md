---
name: planning
description: Plan engineering work in a governed markdown artifact store — create, relate, move and validate epics, stories, tasks and initiatives through the `aep` CLI. Use when the user mentions planning, a backlog, an epic, a story, a task, decomposing or breaking down work, an artifact's status ("move this to active", "what is still in draft?", "why can't this be implemented?"), or when the project contains a `.engineering/planning/` directory. Use it at adoption too — the user asks to adopt AEP, to migrate from or replace the track plugin, to start a first backlog, or works in a repository with no `.engineering/` directory at all — because § 5 says how a first store is populated and it is worth nothing after one has been hand-written. Also use before editing any file under `.engineering/planning/`.
---

**Skill version 0.7.0** — the version in `.claude-plugin/plugin.json`.

# Planning in a governed artifact store

## 0. When the record is required

In a Beyond10x repository, use this skill before implementation when the work is non-trivial,
crosses repository boundaries, or changes a release or deployment. A transient chat plan is useful
while discovering facts, but it is not the governed record. Before the first implementation edit:

1. run `aep plan artifact list` and `aep plan artifact kinds` in the repository that owns the outcome;
2. create or select the artifact that owns the work and record its relation to the existing plan;
3. record machine-readable scope before scheduling concurrent work; and
4. keep evidence and lifecycle state current as the implementation and release progress.

Read-only discovery may happen first so the artifact is evidence-based. If implementation is
already in flight when the missing record is noticed, create or select it immediately and continue
under it; do not discard correct work merely to make the timeline look cleaner. When no store is
present, follow section 5 rather than inventing one.

## 1. The model

Artifacts are markdown files under `.engineering/planning/<kind>/<slug>.md`: YAML frontmatter the
CLI owns, and a body you and the operator own. Which kinds exist, which statuses each kind may hold
and which moves between them are legal come from validated lifecycle documents, not from convention
and not from this file. The `aep` CLI is the authority on both, so every question about
vocabulary has a command that answers it.

## 2. Discover, do not memorise

This skill inlines **rules only**. It deliberately carries no list of kinds, statuses, legal moves
or relations. Ask for them at the moment you need them:

| Question | Command |
|---|---|
| What kinds can I create? | `aep plan artifact kinds` |
| What edges exist between artifacts? | `aep plan artifact relations` |
| What statuses does this kind have, and what moves where? | `aep plan artifact lifecycle <kind>` |
| What is already in the store? | `aep plan artifact list [--kind k] [--status s] [--ref p:key] [--format json]` |
| What, here, is this ticket? | `aep plan artifact list --ref jira:DEV-630` |
| Which tracker keys does this repository mention that the plan does not? | `aep plan reverse tickets --provider jira` |
| What does it look like as a board? | `aep plan artifact board [--kind k]` |
| What is stopped, on what type of thing, and on which item? | `aep plan artifact blocked [--type t]` |
| How is it wired together? | `aep plan artifact graph` |
| What does this one artifact say, frontmatter and body? | `aep plan artifact show <id>` |
| What has happened to it, oldest first? | `aep plan artifact history <id>` |
| Why is it at this status — what did the store admit before each move? | `aep plan artifact explain <id>` |
| What writes has one side of a hybrid plan taken that the other has not? | `aep plan artifact divergences` |
| Is the whole store still consistent? | `aep plan artifact validate` |

That is every `aep plan artifact` verb that answers a question. The seven that are missing from it
write — `new`, `move`, `relate`, `unrelate`, `body`, `evidence`, `catch-up` — and guardrail 2 governs
those.
Run `aep plan artifact --help` when this table and the CLI disagree; the CLI is right.

The reason is the reason this project exists. Lifecycle and relation documents are validated and
versioned; a prose copy of them in a skill file is neither, and it goes stale the first time a kind
gains a status. An agent that recites `draft → proposed → active` from memory will confidently
propose an illegal move in a store that renamed one of them. Reading `aep plan artifact
lifecycle story` costs one command and cannot be wrong.

When a store is present but you have not looked at it yet in this session, start with `aep
artifact list` and `aep plan artifact kinds`. Two commands buy you the whole vocabulary.

## 3. Seven guardrails

These are inlined because they hold whatever the store's vocabulary is.

**1. A status changes only through `aep plan artifact move`.** Never edit the `status:` field in
frontmatter, and never write it into a file with `Edit` or a heredoc. The CLI validates the move
against the kind's lifecycle; a hand-edited status is an unvalidated one, indistinguishable in the
file from a legal one and wrong in exactly the cases that matter.

```console
$ aep plan artifact move story:credential-store --to proposed
story:credential-store moved draft -> proposed (revision 2)
```

Some rungs cost evidence — their kind's lifecycle declares a `requires:` entry for them — and the
way to pay is to record the observation, before the move rather than at it:

```console
$ aep plan artifact evidence story:credential-store --kind test_result \
    --source "task check" --ref https://ci.example/run/8412 --at 2026-08-30T14:02:00Z
```

`move` finds evidence recorded against the artifact without being told, so nothing is passed to it.
The kind list is **closed**: `aep plan artifact evidence --help` documents the flag, and a kind
outside the list is refused with the whole list printed — read it from that refusal rather than
inventing a plausible name for what you observed. `--ref` and `--at` are optional and are what make
the record checkable later. `move --evidence <kind>=<count>` is the asserted form, kept for a
record that lives outside the store; it names no run and no artifact, and the store marks a move
that rests on it as resting on an assertion.

**2. Every store mutation uses `aep plan artifact`; never edit a store file directly.** Creation,
relations, status, and prose use `new`, `relate`/`unrelate`, `move`, and `body` respectively. **A
wrong edge is taken back in the words that made it** — `unrelate <id> <relation> <target>`, or the
colon form `unrelate <id> <relation>:<target>` — so a backwards `blocks` is a command, not a
paragraph apologising for one. Before AEP 0.53.0 there was no such verb and sessions wrote the
correction into the artifact's body instead; if `unrelate` is refused as an unknown verb, the
installed CLI predates it and the honest report says so. Supply the complete
body from a file or standard input — `body --from` after creation, or `new … --from` at creation; the
CLI preserves frontmatter, validates the store, and bumps the revision once when bytes change. For a
kind whose records are immutable (`review-result` refuses `body`), `new --from` is the only way a
body arrives, and `move --to archived` is the only way one is retired.

Two corollaries, both from a recorded run (2026-09-03). **Never create an artifact to look at its
template.** `aep plan artifact new` writes the kind's template into the store, and a probe you then
archive is still an artifact: it stays in the store, and its `move --to archived` is a lifecycle
move a checker reads as work scheduled before the open question was filed. The complete example file
is in [references/store-conventions.md](references/store-conventions.md), and `aep plan artifact new
--help` lists every flag. **A body file for `--from` goes under `$TMPDIR`, never a hard-coded
`/tmp`.** The runner sets `TMPDIR` to a directory it owns and reads back; `/tmp` is outside every
record it keeps.

```console
$ aep plan artifact body story:credential-store --from story-body.md
story:credential-store body replaced (revision 2) at .engineering/planning/story/credential-store.md
```

**3. After a batch of edits, run `aep plan artifact validate`, and relay its output verbatim.** It
accumulates every problem rather than stopping at the first, and exits 1 if any remain. Do not
summarise it into "validation failed" — the output names each artifact and each defect, which is the
only part the operator can act on.

**4. A refusal is the answer, not an obstacle.** When the CLI refuses a move it exits 1 and names
every status legal from where the artifact stands. Relay that list. Do not retry with a different
spelling, do not route around it by editing the file, and do not pick an intermediate status to
"get there" without saying so.

```console
$ aep plan artifact move story:credential-store --to implemented
story:credential-store is proposed; a story may move to: draft, rejected, active
$ echo $?
1
```

The right response is to tell the operator that the story must be active first, and ask whether to
walk it there — not to perform two moves nobody sanctioned.

**5. A request that is already satisfied gets an artifact, not a question.** Finding that the asked-
for behaviour already exists — or should not be built — is a real result and often a better one than
the change. Record it: write the `specification` that states what you found, cite the code and the
command output that show it, and say plainly that you did not build the thing and why. Then file the
gap you actually found, if there is one.

What this rules out is ending the turn on *which of these two would you like?*. A session that stops
to ask has produced nothing, and there is frequently nobody there to answer — the same work run
non-interactively ends with an empty tree and a question into a log. `adp/default` has a terminal
`declined` state for precisely this outcome, and the distinction it draws is the one that matters
here: **a decline that is written down is a result; a decline that is only said is a run that did
nothing.** § 4 *When there is no operator* says what to write instead — here, and at every other
stop this file has.

This is not licence to argue with the request. Build what was asked for unless you have evidence it
is already there or actively wrong, put that evidence in the artifact with `aep plan artifact
body`, and leave the decision where § 4 leaves the ones that are the operator's — with the
operator, who can now read what you found instead of answering a question.

**6. Something parked is recorded, not described.** If `aep plan artifact kinds` lists no blocker
kind, that is not the answer — ask `aep plan artifact lifecycle <type>-blocker` as well. Where that
ladder answers, file the blocker as a `decision-blocker`, or as the `<type>-blocker` whose ladder
answered, and say in your report that `kinds` does not list it. Where neither answers, the store
cannot hold it as an artifact: record it in the blocked artifact's body through `aep plan artifact
body`, and say plainly that it is a paragraph rather than an artifact, so nobody expects
`aep plan artifact blocked` to find it.

A blocker artifact is typed by what would clear it and carries a `blocks` edge to the work it is
stopping. Never leave the fact in a status field: an item parked for nine days on a credential and
an item somebody is working on today are both `active`, and only the blocker tells them apart.
Unblocking is `aep plan artifact move <blocker> --to cleared`, a move like any other, which is why
the record survives it.

```console
$ aep plan artifact lifecycle decision-blocker
decision-blocker starts at open
  cleared -> nothing
  open -> cleared

$ aep plan artifact new decision-blocker api-token-scope \
    --title "Nobody has decided which account mints the CI token" \
    --withholds test_result --relate blocks:story:ci-evidence
created decision-blocker:api-token-scope (open) at .engineering/planning/decision-blocker/api-token-scope.md
```

`--withholds` is optional and names the evidence kind nobody can produce while this is open, so
`aep plan artifact explain <blocked-id>` answers *why is there no record for the next move*. It only
means something beside `blocks:`, and `validate` says so.

**7. An epic or story that introduces a new noun models it first.** Where the artifact's outcome
names an entity no `ess/1` document in the repository declares, do not decompose it and do not write
stories around it. Draft the domain first — `aep plan reverse openapi --domain <name> --out <domain-doc>
<openapi-doc>` where an OpenAPI document already describes it, otherwise the minimal document in the
`ess-specify:specify` skill — run `ess specify validate --path <specification>`, and cite the file by path in the
artifact body through `aep plan artifact body`. A noun with no typed home is the relation nobody can
check later.

**A relation between two nouns is modelled the same way the nouns are: as a `relations:` entry on
the entity, in the `ess/1` document.** Not as a sentence in a story body, and not as a field somebody
will recognise as a foreign key later. The entry names the far entity, whether this side owns it or
merely references it, and the cardinality; `ess specify validate` refuses an entry whose target does not
exist, whose linking field is missing or of the wrong type, and a second entity claiming to own the
same one. That refusal is the whole value of writing it there: a relation in prose is checked by
nobody.

The draft is a proposal, not a silent completion. Every relation you could not read from code, an
OpenAPI document or an existing artifact is written into the domain with an `UNMAPPED:` marker and
named again in your report; ESS refuses guessed semantics, and so does this. **A relation whose
cardinality you do not know is an `UNMAPPED:` marker too** — not an entry with a guessed
cardinality beside a caveat. One-to-many and one-to-one produce different schemas, different
lifecycles and different stories, so a guess there is not a smaller guess than inventing the
relation. Where the noun is already declared, cite the existing document instead of drafting a
second one.

## 4. Who decides

A move is a claim about the state of the world, and for most rungs the store already holds what
settles it. Read it there before asking anybody.

* New artifacts are created in the lifecycle's initial status — `aep plan artifact new` does this,
  and it is the correct starting point. Do not immediately move them.
* **Make the move when the store holds what the rung requires.** A rung whose lifecycle declares no
  `requires:` entry costs nothing to reach: on work you were asked to do, make the move and report
  it. A rung that costs evidence is settled by what has been recorded against the artifact — record
  the evidence (guardrail 1), run `aep plan artifact move`, relay what it printed. Do not ask
  permission for a move the store would allow: it buys one more question and no work, and whether
  the thing is implemented is a fact the store holds and the operator does not.
* **Ask only when the evidence is missing, and name what is missing.** The refusal writes that
  question for you — it names the rung, what the requirement said, and what nothing was presented
  at. Relay it, name the record that would close it, and say who or what could produce that record.
  A question carrying a named missing record is worth an operator's time; "shall I move this?" is
  not.
* **A decomposition is drafted and reported, not held for confirmation.** Draft the stories, write
  their bodies, and report the set together with what you deliberately did not cover. Every draft
  lands in the initial status and is reversible; an undrafted decomposition is not the safer one,
  it is the one nobody can read.
* Never perform a bulk move autonomously. "Archive everything still in draft" is an instruction;
  inferring it from a tidy-up request is not.

Two things stay with the operator: a move whose evidence does not exist, and a bulk move nobody
asked for. Everything else was already asked for when the work was.

### When there is no operator

A stop is only worth taking when somebody is there to take it. **Decide once, at the start of the
session, which kind of run this is, and say which in your report.** It is non-interactive when
either holds:

* **The task says so** — *run without stopping*, *no operator is present*, *record each stop and
  continue*.
* **The harness gives no operator turn** — a batch or print-mode session, an `aep drive eval run` case, a
  sub-agent dispatch. Nothing you emit reaches a person before the session ends, so a question is a
  question into a log.

In a non-interactive run **you do not end your turn at a stop.** You record the stop and carry on:

```console
$ aep plan artifact new approval-record wave-proposal-2026-09-03 \
    --title "Wave proposal accepted with no operator present" \
    --tag non-interactive --relate decides:story:credential-store \
    --from bypass-body.md
created approval-record:wave-proposal-2026-09-03 (draft) at .engineering/planning/approval-record/wave-proposal-2026-09-03.md
```

**The kind is `approval-record`**, and it is the store's own rather than a name invented here:
`aep plan artifact kinds` lists it, and `aep plan artifact lifecycle approval-record` answers that it declares
no lifecycle — so the record lands at the status `new` gave it and needs no move afterwards, which
is what makes one command the whole of a bypass. Ask both before you rely on this paragraph, the way
§ 2 says to ask about any other vocabulary. Where `kinds` does not list it, take the closest kind the
store admits, **say in the report which you took and why**, and where nothing answers, the bypass has
nowhere to live: stop at the stop and report that, rather than continuing unrecorded.

The record carries four things, and the body is where three of them go: **which stop** — the wave
proposal, a critic round, a blocker, a move the store gates on evidence; **the reason
`non-interactive`**, as the tag so `aep plan artifact list --kind approval-record` finds every one of
them, and in words in the body; **what was decided in the operator's absence**; and **what the
operator would have been asked**. One record per stop, written at the stop. A single record composed
at the end is a summary of a session, not a decision anybody can audit.

**A bypass record replaces the question, never the answer.** Where a stop exists because somebody is
being asked to *choose*, the record names the choice and the run continues. Where it exists because
something is *missing*, the operator's absence supplies nothing:

| The stop | Non-interactive |
|---|---|
| a move the store would allow, held only for confirmation | record it, make the move, continue |
| a move whose evidence does not exist | **still refused.** Put the refusal verbatim in the record's body, and move on to work that is not blocked |
| an open `decision-blocker` | **still blocking.** Never `move <blocker> --to cleared`: a blocker is cleared by the answer to its question, and nobody answered it |
| a bulk move nobody asked for | still nobody asked. It is not a stop, so a bypass does not cover it |

Never record evidence the run did not observe. An `aep plan artifact evidence --kind approval` written
to satisfy a gate is the silent auto-approval this whole section exists to refuse, and it is
indistinguishable later from something somebody watched happen. **Nothing is auto-approved silently:** every bypass above is an artifact
`aep plan artifact list --kind approval-record` returns, and a run that passed a stop without writing one
has done the thing this section prevents. Report the count and the ids.

## 5. Starting from a repository that has no store

An empty store in a repository with 40,000 lines of code is not a blank page — it is a plan somebody
has been carrying in their head. Do not open the editor and start typing epics.

**If that plan is already written down somewhere — a story tree, a `TODO.md`, plan or issue
documents — use the `story-migration` skill instead of this section.** Adopting beside a legacy
backlog rather than migrating it produces two plans and no record that one replaced the other, and
that has already happened once in this organisation.

```console
$ aep plan reverse init --protocols <source> --profile <profile>
$ aep plan reverse scan --format json
```

`reverse init` writes `.engineering/project.yaml` and refuses the two things that quietly break
later — an absolute path, and a `git+` source pinned to a branch rather than a commit. `reverse scan`
reads and interprets nothing: it emits located facts — README headings, marked lines, tests that say
they will not run, CI jobs and the variables on them, task targets, packages, published contracts —
each carrying the `path:line` it was read from. It writes nothing and it has no clock and no network,
so two runs over one tree give identical bytes.

Then, in a Git working tree:

```console
$ aep plan reverse history --format json
```

The axis the scan has none of. It joins to the scan on `path:line` and dates every marked line and
every disabled test from the commit that wrote it, alongside what the history says about itself:
reverts, commits that hedged (*for now*, *until we*), churn, dormancy, tracker keys. Dates are quoted
from commits and never compared against today, so this is byte-stable too.

**Use it, and lead with what it tells you.** A suite switched off is an observation; a suite switched
off since February 2024 is a finding. The first gets scrolled past.

**Then cite what you file.** An artifact drafted from a bundle entry carries that entry's
`path:line` in its body. This is the whole reason the scan is a separate program from the session
that reads it: a plan you invented from a plausible reading of the code is indistinguishable, later,
from a plan somebody agreed to, and it is the worse of the two because nobody can check it.

The `reverse-engineer` agent does this end to end and reports what it could **not** cite. Use it
rather than reproducing the loop by hand, and read its fourth report section first — the things that
look like real work and have no evidence in the tree are the questions worth the operator's time.

A scan reports what is *written down*. A convention that lives in review comments, or the reason a
module exists, is not in the bundle and must not be invented into one.

## 6. A worked decomposition

An epic, two stories derived from it, one move, one validation.

```console
$ aep plan artifact new epic passkey-login \
    --title "Passkey login" \
    --summary "Replace password sign-in with WebAuthn passkeys."
created epic:passkey-login (draft) at .engineering/planning/epic/passkey-login.md

$ aep plan artifact new story credential-store \
    --title "Store and retrieve passkey credentials" \
    --relate decomposes:epic:passkey-login
created story:credential-store (draft) at .engineering/planning/story/credential-store.md

$ aep plan artifact new story registration-ceremony \
    --title "Register a passkey during sign-up" \
    --relate decomposes:epic:passkey-login
created story:registration-ceremony (draft) at .engineering/planning/story/registration-ceremony.md
```

Then write each story's complete body through `aep plan artifact body <id> --from <path|->` — or
hand it to `new … --from <path|->` in the first place — one acceptance statement per story, because
guardrail 2 makes the CLI the store's sole writer.

Then the one move the operator asked for, and the check:

```console
$ aep plan artifact move story:credential-store --to proposed
story:credential-store moved draft -> proposed (revision 2)

$ aep plan artifact validate
3 file(s) in .engineering/planning: 3 artifact(s)
valid
```

Had `validate` found something, its output would have gone to the operator unedited.

## 7. Argue with the decomposition before the operator reads it

§ 4 says a decomposition is drafted and reported rather than held for confirmation. That leaves one
agent's reading of the work standing as the plan. **After you have reported it, put it in front of a
panel of critics and revise what they find** — the operator then reads a plan that has already been
argued with, and the findings nobody fixed, rather than a first draft with a hopeful tone.

**Skip the step when fewer than two artifacts sit under the one you decomposed, and say in the
report that you skipped it and why.** Three of the four perspectives compare items with each other,
so on a set of one they have nothing to do and produce agreeable noise. Count the edges rather than
guessing: `aep plan artifact list --format json` prints every artifact with its relations, and
`aep plan artifact graph` draws the same edges — count the ones pointing at the artifact you decomposed.

### Dispatch the four at once

| Agent | The one question it asks |
|---|---|
| `aep-plan:plan-critic-acceptance` | could anybody ever tell whether these are done? |
| `aep-plan:plan-critic-design` | is this set the right shape — coupling, cycles, a split abstraction? |
| `aep-plan:plan-critic-scope` | is everything the parent promised claimed, and nothing else? |
| `aep-plan:plan-critic-parallel-safety` | which two of these land on one file, and does the plan say so? |

Name the agent type in full, with its plugin prefix, in your report. A built-in agent used where one
of these exists is a deviation you report, not a substitution you make.

**They run at once, and none of them sees another's findings.** That is the mechanism, not a
scheduling convenience: four independent readings are worth more than four agents converging on the
first one's framing. It is also why they are read-only — the journal is append-only and
single-writer, so N critics writing it is a race. They return text; you write.

Each returns a first line that is exactly `approve` or exactly `needs-revision`, then one line per
finding in the form `artifact — reason — citation`. The rules they work to are in
[references/critic-rubric.md](references/critic-rubric.md); read it before you judge a verdict, and
before you decide a finding is not worth acting on.

### Record every verdict, including the approvals

One record per critic per round, holding that critic's returned text. An approval you did not record
is the round nobody can see later, and it is the half that makes the record evidence rather than a
complaint log.

The kind that holds a verdict is immutable — `aep plan artifact body` refuses it, because a record that
can be edited after the fact is not evidence — so the body arrives at creation or never:

```console
$ aep plan artifact lifecycle review-result
review-result starts at active
  active -> archived
  archived -> nothing

$ aep plan artifact new review-result acceptance-round-1 \
    --title "Acceptance critic, round 1" \
    --relate reviews:story:credential-store \
    --relate reviews:story:registration-ceremony \
    --from round-1-acceptance.md
created review-result:acceptance-round-1 (active) at .engineering/planning/review-result/acceptance-round-1.md
```

* `--from` takes the critic's text **as it returned it**, written to a file first. Summarise it and
  you have recorded your reading of the review rather than the review. That includes the fenced
  ` ```findings ` block the rubric has each critic close with: it is the half a program reads, and a
  record whose findings were flattened into prose is one `aep plan artifact findings` cannot compare
  against the next round.
* Repeat `--relate` once per artifact the critic judged. Read the edge name from
  `aep plan artifact relations` before you rely on it, the way you would any other vocabulary.
* Write them one at a time. Four critics return at once; the store takes one writer.
* A later round is a **new** record, not an edit of the first. Two records that disagree are the
  history of a plan changing its mind, which is the thing worth having.

### Revise, at most twice

On `needs-revision`, revise the drafts — never the record. Each finding names the one artifact a
revision would change; rewrite that artifact's body through `aep plan artifact body <id> --from <path|->`
with the complete body, or `--section <heading>` where one section changes, one artifact at a time.
Then dispatch the same four again over the revised set.

**Stop after the second round.** There is no third, and a panel that runs until it approves is a
panel that has been talked into approving. Whatever is still open at that point goes in your report:
one line per finding, verbatim from the critic with its citation, naming the record that holds it and
the round it survived. Three open findings with citations serve an operator better than a plan four
agents were argued into.

### Record what became of every finding

A `review-result` says what a critic thought. On its own it never says whether anybody acted, so a
store full of them cannot answer *is this panel worth what it costs* — which is the question the
model pin in [references/critic-rubric.md](references/critic-rubric.md) is waiting on. **After each
revision round, record one outcome per finding**, against the artifact the finding named:

```console
$ aep plan artifact evidence story:credential-store --kind review_outcome \
    --review review-result:acceptance-round-1 --outcome fixed
```

| Outcome | When you write it |
|---|---|
| `fixed` | the revision changed the artifact because of this finding. The change is in the body you rewrote and the revision number moved |
| `no-op` | the finding held and the artifact needed no change — it was already covered elsewhere, or a sibling's revision answered it |
| `escalated` | you did not act, and it goes to the operator open. Every finding still standing after the second round is this one |

**The outcome is what you did, not what you think of the finding.** A finding you disagree with and
did not act on is `escalated`, not `no-op`: `no-op` means the revision did not need to change
anything, and using it for *I decided this critic was wrong* buries a disagreement the operator is
entitled to read. Three outcomes, and none of them is a verdict on the critic.

Every finding gets exactly one, including on a round where nothing changed, and including the
findings of a critic that returned `approve` (there are none, so there is nothing to record). A
round that recorded outcomes for the findings it acted on and none for the rest has produced the
table's most misleading possible input.

The verb takes the id of the artifact that was reviewed and the id of the record that reviewed it,
so a finding is attributable to the round it came from without reading either body. Read
`aep plan artifact evidence --help` for the flag spellings before you rely on the ones above; the kind
list is closed and a refusal prints the whole of it (guardrail 1).

**On a binary that predates the kind, the refusal prints the list and the outcome has nowhere to
go.** Do not route around it by editing a file. Write the three counts — how many `fixed`, `no-op`
and `escalated` — into your report and say which `aep` version refused, so the round is still
readable and nobody records it as done. That is the one case where the outcome lives in prose.

A verdict is not a move. Whether an artifact advances is § 4's question and the store's; four agents
approving its prose is not evidence anybody asked for. Finish the batch the way guardrail 3 says —
`aep plan artifact validate`, output relayed verbatim.

## 8. Scoping what is already there

A store that does not record what its stories touch cannot be sequenced. Two items on one file
conflict whichever order they land in, so *which surfaces does this touch* is the property that
decides what can be worked at once — and in most stores nothing holds it.

The `story-scoper` agent answers it for one artifact: it reads the body, its edges, the symbols it
names and the tree, and returns a `## Scope` section marking every line **cited** or **inferred**.
It is read-only, so run one per story and run them at once; write what they return through
`aep plan artifact body`, one at a time, because the journal is append-only and N writers race.

Read the confidence line, not just the surface. A scope that mixes what was read with what was
guessed is worse than none — it gets trusted exactly where it is weakest — which is why the section
separates them and why a scoper reports what it could **not** establish beside what it could.

## Reference

The on-disk format — directory layout, filename and id rules, which frontmatter fields are
machine-owned, and a complete example file — is in
[references/store-conventions.md](references/store-conventions.md). Read it before changing a store
document.

The rules every critic in § 7 works to — the two verdicts and what binds them to the findings, the
`artifact — reason — citation` line, what is not a finding, why a critic writes nothing — are in
[references/critic-rubric.md](references/critic-rubric.md). Read it before dispatching a panel or
judging what one returned. It is rules only for the same reason this file is: it names no kind,
status or relation.

Everything else is a question for the CLI.
