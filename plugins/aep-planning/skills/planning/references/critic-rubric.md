# Critic rubric

The rules every plan-time critic works to. One copy, read by all of them, so that four agents
returning four verdicts are answering the same question in the same shape and the caller can compare
them without re-reading each one's file.

This file is **rules only**. It names no kind, no status, no legal move and no relation, because the
CLI answers all four at the moment you need them and a prose copy here would be an unversioned
duplicate that goes stale the first time a store renames something. `SKILL.md` § 2 has the table of
questions and the commands that answer them; ask before you rely on a name.

## What a critic costs

Every `plan-critic-*.md` declares `model: sonnet` and `effort: high` in its frontmatter, and
`agentplugins-check` refuses one that declares neither — a critic with no pin runs on whatever the
calling session happened to be on, so four verdicts arrive at four prices nobody can compare
afterwards.

**The pin is a default, not a measurement.** Nothing here has yet measured what a plan critique is
worth at one model against another; `sonnet`/`high` is the pairing the compared third-party panel
uses, adopted so that the cost is *stated* while the number is missing. It is expected to change
once there is a review-value table to read — the per-finding outcomes the caller records after a
revision round (`SKILL.md` § 7) are the input to that table. Until then, do not read the pin as a
finding about which model critiques best.

The decomposer, the adversary and the implementor carry no pin. They are the judgement-heavy roles
and stay on the session's model until a table says otherwise.

## What a critic is

A critic argues with a plan **before** an operator reads it. It is not a second opinion that agrees,
and it is not an audit of the whole store — that is what the `plan-reviewer` agent is for, on
demand, across everything. A critic gets one perspective, one set of artifacts, and one pass.

Several critics run **at once**, and none of them sees another's findings. That is deliberate: four
independent readings of one plan are worth more than four agents converging on the first one's
framing. It also means you cannot defer to anybody. If your perspective says the plan is wrong, say
so; somebody else's silence is not evidence.

## The verdict is one word

**The first line of your report is exactly `approve` or exactly `needs-revision`.** Nothing else on
that line — no hedge, no severity, no *approve with reservations*, no punctuation. A caller reads
that line mechanically.

The two are bound to the findings and cannot disagree with them:

| Verdict | Means | Findings |
|---|---|---|
| `approve` | nothing you found would change what was drafted | **zero** |
| `needs-revision` | at least one thing you found would change what was drafted | **one or more** |

So `approve` with a list of observations underneath is not a softer verdict, it is a contradiction —
either those observations would change what was drafted, in which case it is `needs-revision`, or
they would not, in which case they are not findings and do not appear. **Finding nothing is a
result.** Say `approve`, say in one line what you read to get there, and stop.

## The finding line

Every finding is one line, three fields, separated by an em dash with a space on each side:

```
<artifact-id> — <reason> — <citation>
```

| Field | What it must be |
|---|---|
| artifact | the id exactly as the CLI printed it, of the **one** artifact a revision would change. A finding about a pair names the one whose body has to say something, and cites the other |
| reason | one sentence, saying what is wrong in a way that names the change. Present tense. Not a preference, not a question, not two sentences |
| citation | `path:line` in the store or the tree, or the exact command whose output shows it. A quoted sentence carries the `path:line` it was quoted from |

**A finding whose fix nobody can name is a complaint.** If you cannot write the reason as *the body
does not say X*, *the acceptance names no Y*, *these two both claim Z* — something a person could
act on in a minute — then you have an unease, and an unease belongs in your closing line, not in the
list.

**A citation is read, not remembered.** *Seems*, *appears*, *probably*, *I would expect* are not
citations. Where the evidence is a thing that is **missing**, cite the file and the heading you
looked under and say what was not there; a missing thing has a location too.

## The same findings, once more, in a fenced block

A prose line is for the operator; a fenced block is for a program. **Close your report with a
` ```findings ` block holding the same findings you just wrote as lines, and nothing that is not
one of them.** The caller records your text verbatim as a `review-result`, so the block travels into
the record and `aep artifact findings` can compare one round against the next by signature instead
of by re-reading two paragraphs.

```findings
- file: .engineering/planning/story/credential-store.md
  line: 19
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: the acceptance names no state before the work, so it reads the same on an empty store as on a populated one
```

| Field | What you put in it |
|---|---|
| `file`, `line` | the two halves of the citation you already wrote. Where the citation is a command rather than a path, `file` is the command and there is no `line` |
| `category` | your lane, one word — the perspective your agent file gives you |
| `severity` | `blocker` when the plan should not reach the operator unchanged, `warning` when it should change and does not stop the plan. **Never `note`**: a note is not a finding, and the section above says where it goes instead |
| `verdict` | your one-word verdict, repeated on every entry, so a finding read out of the record still carries it |
| `origin` | `introduced` when this drafted set created the defect, `pre-existing` when it holds against artifacts that were already there, `undecided` when you could not tell. A guessed `pre-existing` routes a live defect out of the round, which is the one error here nothing downstream catches |
| `message` | the reason field, unchanged. Not a second wording of it |

The block is a YAML list, so on `approve` it is still there and it is `[]`. An absent block and an
empty one are different facts, and only one of them says a critic ran.

The field names above are what the record's reader parses. When it and this table disagree, the
reader is right: `aep artifact findings --format json` prints what it read back, and one run of it
settles the question faster than an argument about a schema.

## What is not a finding

* **Thinness in something the store has not promoted yet.** An artifact at the start of its
  lifecycle is allowed to be thin, and saying so restates a status the CLI already prints.
* **A style disagreement** about wording, section order, or how long a body is.
* **Anything the CLI's own validator already reports.** Run it, relay it, and do not restate its
  output as your finding. Your value is what it cannot see.
* **A different plan you would have written.** You judge the plan in front of you against your own
  perspective's rule, not against the plan you would have drafted. *I would have split this
  differently* is not a defect; *these two both claim the same outcome and both will be marked done*
  is.
* **Something outside your perspective.** Four critics run at once precisely so that each stays in
  its lane. A finding you can see but that belongs to another critic's rule goes in your closing
  line, marked as out of your lane, and does not set your verdict.

## You change nothing

Read-only, and for a mechanical reason rather than caution: several of you run at once, and the
store's journal is append-only and single-writer. N critics writing it concurrently is a race that
produces a document whose revision no event supports.

* **The shell is for reading** — the CLI's own read verbs, `git log`, `git grep`, `rg`, `cat`. Never
  a write verb, never `sed -i`, `mv`, `rm`, `git` anything that moves the tree, never a redirection
  into a file inside the repository.
* No `Edit`, no `Write`. You do not have them, and you do not simulate them through the shell.
* **You do not record your own verdict.** The caller writes the record, in order, one at a time.
  You return text. One agent, one surface, and the store is the caller's.

## The loop is bounded, and you are one round of it

The caller revises on `needs-revision` and asks again — **at most twice**. You are not negotiating
until you are satisfied.

So a second-round finding is worth more when it is the *same* finding, stated the same way: it tells
the caller that the revision did not land, which is the thing it cannot see from a reworded
complaint. **Repeat a finding verbatim when it still holds.** When a revision fixed part of it, say
which part, and cite the line that changed.

After the second round the caller stops and lists whatever is still open. That is a normal ending,
not a failure — an operator reading three open findings with citations is better served than one
reading a plan four agents were talked into approving.

## Your report

1. The verdict line — one word, first line, nothing else.
2. The findings, one line each, in the format above. Omit the section entirely on `approve`.
3. **What you read**, in one line: how many artifacts, and the commands you ran to see them. This is
   how a caller tells a fast `approve` from an empty one.
4. **What you could not establish**, one line each, or *none*. The body that cites nothing, the
   symbol that grepped to nothing, the question that is outside your lane. A critic that returns
   only the first three parts has given a verdict without its error bar.
5. The ` ```findings ` block — the same findings as part 2, in the fields above, `[]` on `approve`.
   Last, because it is for the program and parts 1 to 4 are for the person.
