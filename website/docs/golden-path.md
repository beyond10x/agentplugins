---
sidebar_position: 4
title: Golden path
description: One worked run, from a feature idea to a critiqued plan, on a repository that already exists.
---

# From a feature idea to a critiqued plan

You type the prompts on this page; the agent runs the commands. It is one run end to end, on a
repository that already exists and has never been planned.

Every console block below is the output of actually running the command shown above it. None of it
is written by hand; the only edit is that the recording machine's absolute paths are shortened to
`…`. What the agents *say* is described in prose instead of quoted, because yours will not say it
the same way.

The worked feature is deliberately small: a **commercial client** record that belongs to exactly one
**account**, with create, read, update and delete. The interesting part is the one thing about it
nobody has decided, and what the plan does with that instead of guessing.

## Prerequisites

Install `aep-planning` and `adp` from the marketplace — see [Install](./install.md) — and have the
`aep` CLI on your PATH.

```console
$ aep --version
protocol 0.41.0
```

That build produced every output on this page. The binary prints `protocol` in `--version` and in
its `--help` usage lines; the command you install and type is `aep`.

## 1. Adopt the repository

The example service owns accounts and nothing else: a README, and one module holding
`create_account`, `read_account`, `update_account` and `delete_account`. It has no `.engineering/`
directory yet.

```text
Adopt this repository for AEP planning. Run `aep reverse init` with the protocol source
git+https://github.com/beyond10x/aep#b857bbebcb44f77275bc745659226f4826897e78 and the profile
development.standard, then run `aep reverse scan`. Report what the scan found, and file nothing yet.
```

Two commands and a report is the whole of this step. A scan is evidence, not a plan, and an agent
that starts drafting artifacts out of one has skipped the part where you get to disagree with it.

```console
$ aep reverse init --protocols 'git+https://github.com/beyond10x/aep#b857bbebcb44f77275bc745659226f4826897e78' --profile development.standard
…/.engineering/project.yaml written
  protocol source resolves to …/aep/protocol-sources/cd43e0b7f3341c9d6329bc502188182e1b0f38df9eda89fd7546517a078e2573/snapshots/b857bbebcb44f77275bc745659226f4826897e78
  profile development.standard
```

`reverse init` refuses the two things that break quietly later — an absolute path, and a `git+`
source pinned to a branch rather than a commit — which is why the source above carries a full commit
hash.

```console
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

```console
$ aep artifact new epic commercial-clients --title "Commercial clients on an account" --summary "A commercial client that belongs to exactly one account, with create, read, update and delete." --from epic-body.md
created epic:commercial-clients (draft) at …/.engineering/planning/epic/commercial-clients.md
```

The agent writes the body to a file and hands it to `--from`, because the CLI is the store's only
writer: it owns the frontmatter, and it placed the epic at its kind's initial status without anybody
typing one. Which statuses your store has is a question for the CLI, not for a page — `aep artifact
kinds` and `aep artifact lifecycle epic` answer it, and they answer for *your* store.

## 3. Decompose it

```text
Decompose epic:commercial-clients. Before you draft a single story, list every domain relation the
epic implies — entity to entity, cardinality, ownership, lifecycle coupling — and classify each one
as inferable, with the path:line or artifact that settles it, or as needing a stakeholder decision.
Draft no story that depends on a relation you could not settle.
```

The decomposer enumerates the relations before it drafts anything. For this epic it found three:

| Relation | Classification | Settled by |
|---|---|---|
| commercial client to account: many-to-one, mandatory | `inferable` | the epic's own outcome |
| the account is the owning side of the pair | `inferable` | the epic's own outcome |
| what deleting an account does to the clients it holds | `requires-stakeholder-input` | nothing — `README.md:13` and `src/accounts.py:30` both say it is open |

Two are settled, so three stories are drafted against them:

```console
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
refuse while clients remain, delete them with it, or leave them with no account; each answer
produces a different story, and none of the three can be read out of the tree. So it becomes an
artifact with an edge, rather than a sentence in a report nobody re-reads:

```console
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

```console
$ aep artifact blocked
decision-blocker:account-deletion-cascade  decision  open, withholding approval  Nobody has decided what happens to an account's commercial clients when the account is deleted
  blocks epic:commercial-clients  draft  Commercial clients on an account
```

The decomposer reports in four parts. The third lists what it deliberately did not cover, each with
the question that blocked it — here, that blocker. **The fourth is the one to read first:** the
complete output of `aep artifact validate`, verbatim, with its exit status. If it exited 1, nothing
else in the report is safe to act on. Here it did not:

```console
$ aep artifact validate
5 file(s) in …/.engineering/planning: 5 artifact(s)
valid
```

## 4. Scope the stories

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

```console
$ aep artifact body story:commercial-client-record --from record-body.md
story:commercial-client-record body replaced (revision 2) at …/.engineering/planning/story/commercial-client-record.md
```

Read the cited-or-inferred marking, not just the file list. A scope that mixes what was read with
what was guessed gets trusted exactly where it is weakest.

## 5. Run the critic panel

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

```console
$ aep artifact lifecycle review-result
review-result starts at active
  active -> archived
  archived -> nothing
```

There is no draft rung there and no way back: a verdict is written once and later archived, never
edited. That is what makes it evidence rather than an opinion somebody kept updating.

## 6. Implement one story through the wave

A draft is not implementable, and the store says so rather than letting you pretend otherwise:

```console
$ aep artifact move story:commercial-client-record --to implemented
story:commercial-client-record is draft; a story may move to: proposed, archived
$ echo $?
1
```

That refusal is the answer, not an obstacle: it names every status legal from where the artifact
stands. Walk it, deliberately, and say that you did:

```console
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

## What you should have

```console
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
