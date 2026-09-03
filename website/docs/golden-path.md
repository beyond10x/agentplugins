---
sidebar_position: 4
title: Golden path
description: One worked run, from a feature idea to a critiqued plan, on a repository that already exists.
---

# From a feature idea to a critiqued plan

You type the prompts on this page; the agent runs the commands. It is one run end to end, on a
repository that already exists and has never been planned. If nobody is going to be there to type
them — a batch run, an eval case, anything with no operator turn — read [Running it without an
operator](#running-it-without-an-operator) at the foot of the page first: it is one extra sentence in
front of step 1, and without it the run ends at the first stop.

Every console block below is the output of actually running the command shown above it. None of it
is written by hand; the only edit is that the recording machine's absolute paths are shortened to
`…`. What the agents *say* is described in prose instead of quoted, because yours will not say it
the same way. The commands were refreshed against AEP `0.44.0` and ESS `0.5.1`.

The worked feature is deliberately small: a **commercial client** record that belongs to exactly one
**account**, with create, read, update and delete. The interesting part is the one thing about it
nobody has decided, and what the plan does with that instead of guessing.

## Prerequisites

Install `aep-planning`, `adp` and `ess-schema` from the marketplace — see [Install](./install.md) —
and have the `aep` CLI on your PATH. Step 3 also uses the `ess` CLI.

```shell-session
$ aep --version
protocol 0.44.0
$ ess --version
ess 0.5.1
```

That build produced every output on this page. The binary prints `protocol` in `--version` and in
its `--help` usage lines; the command you install and type is `aep`.

## 1. Adopt the repository

The example service owns accounts and nothing else: a README, and one module holding
`create_account`, `read_account`, `update_account` and `delete_account`. It has no `.engineering/`
directory yet.

```text
Adopt this repository for AEP planning. Run `aep reverse init` with the protocol source
git+https://github.com/beyond10x/aep#8b4342a41fdd914341d9a479627ed76240f88d45 and the profile
development.standard, then run `aep reverse scan`. Report what the scan found, and file nothing yet.
```

Two commands and a report is the whole of this step. A scan is evidence, not a plan, and an agent
that starts drafting artifacts out of one has skipped the part where you get to disagree with it.

```shell-session
$ aep reverse init --protocols 'git+https://github.com/beyond10x/aep#8b4342a41fdd914341d9a479627ed76240f88d45' --profile development.standard
…/.engineering/project.yaml written
  protocol source resolves to …/aep/protocol-sources/cd43e0b7f3341c9d6329bc502188182e1b0f38df9eda89fd7546517a078e2573/snapshots/8b4342a41fdd914341d9a479627ed76240f88d45
  profile development.standard
```

`reverse init` refuses the two things that break quietly later — an absolute path, and a `git+`
source pinned to a branch rather than a commit — which is why the source above carries a full commit
hash.

```shell-session
$ aep reverse scan
aep.reverse-scan/1

readme headings: 3
  README.md:1  Accounts service
  README.md:5    What it does
  README.md:11    Not yet decided
unfinished work: 1
  src/accounts.py:30  TODO # TODO: deleting an account must decide what happens to whatever belongs to it
disabled tests: 0
ci jobs: 0
task targets: 0
packages: 1
  src  1 file(s), 38 line(s)  [Python 38]
api surfaces: 0
root documents: 0
```

`scan` reads and interprets nothing. It writes nothing, has no clock and no network, so two runs
over one tree print the same bytes — which is what makes it evidence you can cite. Read what came
back as exactly that: three README headings, one unfinished-work marker at `src/accounts.py:30`, no
CI jobs, no disabled tests. That marker matters in a minute.

## 2. File the feature as an epic

```text
File this as an epic in the planning store, and stop there — do not decompose it yet:

  A commercial client is a record of its own. It belongs to exactly one account. A caller can
  create, read, update and delete a commercial client.

Cite what in this repository the epic rests on.
```

```shell-session
$ aep artifact new epic commercial-clients --title "Commercial clients on an account" --summary "A commercial client that belongs to exactly one account, with create, read, update and delete." --from epic-body.md
created epic:commercial-clients (draft) at …/.engineering/planning/epic/commercial-clients.md
```

The agent writes the body to a file and hands it to `--from`, because the CLI is the store's only
writer: it owns the frontmatter, and it placed the epic at its kind's initial status without anybody
typing one. Which statuses your store has is a question for the CLI, not for a page — `aep artifact
kinds` and `aep artifact lifecycle epic` answer it, and they answer for *your* store.

## 3. Model the new noun before decomposing it

*Commercial client* is a noun this repository has never typed. The planning skill's domain-first
guardrail stops here rather than letting three stories be written around a shape nobody agreed on:
a noun with no typed home is the relation nobody can check later.

```text
epic:commercial-clients introduces a noun with no typed home. Draft the smallest ess/1 domain that
declares it and its relation to the account, validate it, and mark anything you cannot read from
this repository as UNMAPPED rather than guessing it.
```

The relation is the point of the step. *Belongs to exactly one account* is a sentence in the epic;
in the domain it is an entry a compiler checks:

```yaml
# system.yaml
format: ess/1
system: accounts
version: v1

domains:
  - accounts.account
```

```yaml
# domains/account.yaml
domain: accounts.account

entities:
  - name: accounts.account.Account
    identity:
      name: account_id
      type: Uuid
    fields:
      - name: name
        type: String
    relations:
      - name: commercial_clients
        kind: owns
        target: accounts.account.CommercialClient
        cardinality: many
        via: account_id
        # UNMAPPED: `owns` is read out of the epic — a commercial client belongs to exactly one
        # account and has no meaning without it. What deleting the account *does* to them —
        # refuse, or take them with it — is a command outcome and nothing here decides it.
        # README.md:13 and src/accounts.py:30 both say it is open.
    lifecycle:
      initial: Active
      states: [Active]
      terminal: [Active]

  - name: accounts.account.CommercialClient
    identity:
      name: client_id
      type: Uuid
    fields:
      - name: account_id
        type: Uuid
      - name: name
        type: String
    lifecycle:
      initial: Active
      states: [Active]
      terminal: [Active]
```

`via: account_id` sits on the **target** because the relation is `owns`: the child carries the field
typed by the owner's identity. `ess validate` refuses a target that is not declared, a `via` that is
missing or of the wrong type, and a second entity claiming to own the same one — which is what makes
this worth writing down rather than asserting in a story body.

```shell-session
$ ess validate --path <specification>
accounts v1 — 2 file(s), valid
```

ESS `0.5.1` validated both files and the ownership relation end to end. That validation is what lets
the next step cite the domain as evidence rather than treating the YAML as illustrative prose.

The `UNMAPPED:` marker is doing the same job here that the blocker does two steps down, one level
lower: it is the question written where the answer would go, so the next reader finds it instead of
finding a guess. Note what the relation entry *did* settle — with `owns` in the document, *leave the
clients with no account* is no longer one of the answers, and the open question is down from three
options to two.

## 4. Decompose it

```text
Decompose epic:commercial-clients. Before you draft a single story, list every domain relation the
epic implies — entity to entity, cardinality, ownership, lifecycle coupling — and classify each one
as inferable, citing the ess/1 document that settles it, or as needing a stakeholder decision.
Draft no story that depends on a relation you could not settle.
```

The decomposer enumerates the relations before it drafts anything. For this epic it found three:

| Relation | Classification | Settled by |
|---|---|---|
| commercial client to account: many-to-one, mandatory | `inferable` | `domains/account.yaml`, `Account.commercial_clients` — `cardinality: many` |
| the account is the owning side of the pair | `inferable` | the same entry — `kind: owns` |
| what deleting an account does to the clients it holds | `requires-stakeholder-input` | nothing — the `UNMAPPED:` marker beside that entry, and `README.md:13` and `src/accounts.py:30` |

The first two citations point at a document `ess validate` accepted, not at a sentence in a story and
not at a foreign key in `src/`. A `path:line` into code is admissible for a relation only when the
classification carries the word `inferred` — because a constraint in code says what this
implementation currently does, and says nothing about whether anybody decided it.

Two are settled, so three stories are drafted against them:

```shell-session
$ aep artifact new story commercial-client-record --title "Create and read a commercial client on one account" --relate decomposes:epic:commercial-clients --from record-body.md
created story:commercial-client-record (draft) at …/.engineering/planning/story/commercial-client-record.md
$ aep artifact new story commercial-client-amendment --title "Update and delete a commercial client" --relate decomposes:epic:commercial-clients --from amendment-body.md
created story:commercial-client-amendment (draft) at …/.engineering/planning/story/commercial-client-amendment.md
$ aep artifact new story account-client-listing --title "List the commercial clients one account holds" --relate decomposes:epic:commercial-clients --from listing-body.md
created story:account-client-listing (draft) at …/.engineering/planning/story/account-client-listing.md
```

Each `inferable` relation is written into the body of the story that rests on it, with its citation,
so the next reader can check it instead of re-deriving it.

The third relation is not settled, and the decomposer does not settle it. Deleting an account could
refuse while clients remain, or delete them with it; each answer produces a different story, and
neither can be read out of the tree. So it becomes an artifact with an edge, rather than a sentence
in a report nobody re-reads — or an `UNMAPPED:` comment that only somebody opening the domain file
would find:

```shell-session
$ aep artifact lifecycle decision-blocker
decision-blocker starts at open
  cleared -> nothing
  open -> cleared
$ aep artifact new decision-blocker account-deletion-cascade --title "Nobody has decided what happens to an account's commercial clients when the account is deleted" --withholds approval --relate blocks:epic:commercial-clients --from blocker-body.md
created decision-blocker:account-deletion-cascade (open) at …/.engineering/planning/decision-blocker/account-deletion-cascade.md
```

The `blocks` edge is the point of the whole exercise. A question in a report is read once; a blocker
with an edge is found by a command, and `--withholds` names the evidence nobody can produce while it
stands:

```shell-session
$ aep artifact blocked
decision-blocker:account-deletion-cascade  decision  open, withholding approval  Nobody has decided what happens to an account's commercial clients when the account is deleted
  blocks epic:commercial-clients  draft  Commercial clients on an account
```

The decomposer reports in four parts. The third lists what it deliberately did not cover, each with
the question that blocked it — here, that blocker. **The fourth is the one to read first:** the
complete output of `aep artifact validate`, verbatim, with its exit status. If it exited 1, nothing
else in the report is safe to act on. Here it did not:

```shell-session
$ aep artifact validate
5 file(s) in …/.engineering/planning: 5 artifact(s)
valid
```

## 5. Scope the stories

Nothing so far records which files each story touches, and that is the property that decides what
can be worked at the same time: two stories on one file conflict whichever order they land in.

```text
Scope each draft story under epic:commercial-clients: which files and symbols does it touch? Mark
every line as cited or inferred, say what you could not establish, and write each scope back into
its story's body.
```

The scopers are read-only, so run one per story and run them at once. The write-back is serial — the
store's journal is append-only and parallel writers race — and it goes through the CLI like every
other change to a body:

```shell-session
$ aep artifact body story:commercial-client-record --from record-body.md
story:commercial-client-record body replaced (revision 2) at …/.engineering/planning/story/commercial-client-record.md
```

Read the cited-or-inferred marking, not just the file list. A scope that mixes what was read with
what was guessed gets trusted exactly where it is weakest.

## 6. Run the critic panel

```text
Run the plan critics over epic:commercial-clients and its stories. Record every verdict, revise the
drafts that come back needing revision, stop after two rounds, and list what is still open.
```

Two to four read-only critics read the drafted plan at once, each with one job: acceptance (is every
story's acceptance observable, and does it cover the state transitions), design (coupling, cycles,
stories sharing a surface), scope (is the epic's outcome covered, and is anything drafted that sits
outside it), parallel safety (name the pairs that touch one file). Each returns `approve` or
`needs-revision` plus a list of *artifact — reason — citation* lines; a verdict with no citation is
not a verdict. On `needs-revision` the drafts are revised through `aep artifact body` and the panel
runs again, at most twice, and whatever is still open after that is listed rather than argued away.
With fewer than two stories under the epic the step is skipped, and says so.

Every verdict is recorded as an artifact carrying a `reviews` edge to what it judged, so the plan
you end up with also carries the argument that produced it:

```shell-session
$ aep artifact lifecycle review-result
review-result starts at active
  active -> archived
  archived -> nothing
```

There is no draft rung there and no way back: a verdict is written once and later archived, never
edited. That is what makes it evidence rather than an opinion somebody kept updating.

## 7. Implement one story through the wave

A draft is not implementable, and the store says so rather than letting you pretend otherwise:

```shell-session
$ aep artifact move story:commercial-client-record --to implemented
story:commercial-client-record is draft; a story may move to: proposed, archived
$ echo $?
1
```

That refusal is the answer, not an obstacle: it names every status legal from where the artifact
stands. Walk it, deliberately, and say that you did:

```shell-session
$ aep artifact move story:commercial-client-record --to proposed
story:commercial-client-record moved draft -> proposed (revision 3)
$ aep artifact move story:commercial-client-record --to active
story:commercial-client-record moved proposed -> active (revision 4)
```

```text
Take story:commercial-client-record through the adp wave: scope it into units, implement the units,
and have the adversary review the result against the story's acceptance and this repository's gate.
```

The wave splits the work three ways: a scoper turns the accepted story into bounded units, an
implementor owns exactly one unit, and an adversary checks the result against that scope, the
evidence recorded, and the repository's own invariants. None of it replaces your repository gate,
and none of it gives an implementor authority beyond its unit.

When the run produces an observation that a later move needs — a test run, a review — record it with
`aep artifact evidence` before the move, naming the source and where to look. `aep artifact move`
finds evidence recorded against the artifact without being told. If it still refuses, the refusal
names what is missing, and that sentence is what to relay.

## 8. Drive the first story

Everything in step 7 is an **instruction** an agent follows. The same story handed to the reference
driver is walked by an engine that decides each transition instead — every state entered through a
transition the engine returned, every gate it refuses printed with one reason per unmet requirement,
and every status move made by the CLI and by nothing else.

```text
Drive story:commercial-client-record. Run aep doctor first and stop if anything fails, tell me what
the run will cost before you launch it, and print the run id and how to follow it.
```

The `drive` skill checks the checkout, points `aep drive run` at the task document that names the
story, launches it against the project's step map, and prints the run id. It moves no artifact
itself — the moves are the driver's, which is the whole property being tested — and it relays a
refusal (a held lock, missing evidence, two step maps that both fit) verbatim and stops.

:::caution The walk has not reached `complete` yet

This step is real and it does not finish. `aep`'s own `story:governed-dogfood-run` records two
driven runs against real stories of its backlog: one stopped in `establish_verifiers` at $15.42, the
other in `adversarial_verify` at $31.46, neither reaching the review step. **A run that wedges is a
recorded result**, which is why those two are written down rather than retried until they worked —
and it is what to expect from this step today. The skill says so before it launches anything, and
the model spend is real, so it asks you for a budget rather than choosing one.

:::

## What you should have

```shell-session
$ aep artifact list
decision-blocker:account-deletion-cascade  decision-blocker  open    Nobody has decided what happens to an account's commercial clients when the account is deleted
epic:commercial-clients                    epic              draft   Commercial clients on an account                                                                blocked: decision
story:account-client-listing               story             draft   List the commercial clients one account holds
story:commercial-client-amendment          story             draft   Update and delete a commercial client
story:commercial-client-record             story             active  Create and read a commercial client on one account
```

Five artifacts: an epic the store knows is blocked, three stories, one open decision. One story is
active and on its way through a wave; the account-deletion question is on somebody's desk rather
than guessed at in a story nobody would have re-read. That last part is the difference between this
plan and one written straight through.

:::note In Codex

The prompts are identical. Claude Code dispatches the decomposer, the scopers and the critics as
sub-agents; in Codex the same behaviour runs from the shared skill directly, because current OpenAI
guidance is to express a reusable role as a skill rather than as an agent wrapper. What you type,
and what lands in the store, is the same.

:::

## Running it without an operator

Every step above hands the run back to you: it stops, you read what came back, you type the next
prompt. A run with nobody there has nobody to hand it to, so it stops once and is finished. One did,
on 2026-09-03: a single headless session over these eight prompts adopted the repository, scanned it,
wrote *step 1 — adopted, scanned* and ended — no wave, no fan-out, no blocker with an edge, no
verdicts recorded, and the store never validated. Ten of the checks this page is held to came back as
gaps and seven held, and nothing the run did was wrong; there was just nobody to continue it.

**One sentence in front of step 1 fixes it.** This is the instruction to give:

```text
Run this without stopping: no operator is present, so at every point you would pause for one — the
wave proposal, a critic round, a blocker, a move the store gates on evidence — record one
`approval-record` through `aep artifact new`, tagged `non-interactive`, naming the stop and what you
decided in the operator's absence, and continue. Do not clear a blocker, do not publish and do not
release.
```

The eight prompts then follow unchanged.

### What gets recorded at each stop

One record per stop, written **at** the stop and through the CLI like every other change to the
store — `aep artifact kinds` lists `approval-record`, and `aep artifact lifecycle approval-record`
answers that it declares no lifecycle, so creating it is the whole of the bypass. Afterwards,
`aep artifact list --kind approval-record` is the list of every place the run decided something you
would have decided.

| Where the page waits | What the run records | What it does next |
|---|---|---|
| **step 4** — the decomposer files `decision-blocker:account-deletion-cascade` and drafts no story behind it | the blocker id, and the story it did not draft | drafts the two stories that do not depend on the answer |
| **step 6** — a critic round comes back `needs-revision` | one record per round, holding the verdict nobody was there to read | revises through `aep artifact body`, runs the second round, stops at two, lists what is still open |
| **step 7** — the wave proposal | the units, the commits an approval would have authorised, the pre-flight numbers | writes the wave page anyway, then dispatches |
| **step 7** — `aep artifact move` refuses for want of evidence | the refusal, verbatim | records the evidence if the run actually observed it; otherwise leaves the story where it stands and goes on with what is not blocked |
| **step 8** — the driven run's budget | the budget the instruction named | launches. Where the instruction named none, it records that and prints the command rather than choosing a number to spend |

### What a headless run cannot do

These are not stops it may record its way past. They are the four things the absence of an operator
does not supply:

| | Why |
|---|---|
| **clear a blocker** | `aep artifact move decision-blocker:account-deletion-cascade --to cleared` *is* the answer to the question, and the question is the thing nobody was there to answer. The blocker stands and the epic stays blocked — which is the correct end state, not a gap |
| **publish** | a push, a tag and a version bump are the repository's release procedure, and none of them is a step of this plan |
| **release** | the wave's second stop, and the one that is not bypassable. Nothing mechanical enforces a release procedure, so a run nobody is watching is the case that stop exists for. The run closes at the merge and leaves the release |
| **turn a refusal into a pass** | a move whose evidence does not exist is still refused and a red gate is still red. A bypass record is not an exit status, and evidence is never recorded for an observation the run did not make |

So a headless run ends roughly where step 8 ends anyway — the driven walk has not reached `complete`
yet either. The difference is everything steps 2 through 7 produce: an epic, a validated domain,
three stories, a blocker with an edge, four verdicts, one story through a wave — in the store, with
every stop it passed sitting beside them as an artifact somebody can read.
