---
title: Governed plan recording (2026-09-28)
sidebar_label: Historical governed plan recording
description: Continue the ESS tutorial's lending library with AEP. Adopt a planning store, plan a feature that adds a new state, have four critics review the plan, and implement the first story in a reviewed wave.
---

# Governed plan recording (2026-09-28)

This is the historical Go recording. For current Rust instructions, use [Your first governed plan](./first-governed-plan.md).

[Your first ESS specification](./first-ess-specification.md) ended with a lending library: a
specification in `spec/` and a Go implementation that passes its 17 conformance scenarios. This
tutorial continues from there with **AEP**, which keeps the plan for changing that library inside the
repository as files the `aep` command validates: an epic, the stories under it, the critics'
verdicts, the evidence that moved each story, and what is still open.

In this tutorial you ask for one feature, *members can reserve a book that is on loan*, and follow
it from a question to a merged, reviewed change. It takes about an hour, most of it the agent's.

**What you end with:**

- a planning store in `.engineering/`, which `aep plan artifact validate` accepts;
- the specification extended by the agent before any story was written, with the suite grown from
  17 to 55 scenarios;
- an epic and six stories, each naming the scenarios it turns from skipped to passing, and four
  critic verdicts recorded as immutable review records;
- the first story implemented in a wave, attacked by an adversary agent, merged, and moved to
  `implemented` on the evidence it recorded.

Every output block on this page is what the command printed when this page was recorded, on
2026-09-28, with `aep` 0.64.0, `ess` 0.39.0, the `aep` and `ess` plugins 0.17.0 and Claude Code.
Your run will differ in ids, counts and wording (another agent names a vision differently, or asks
six questions instead of five); the steps and what each one records are what repeat. Paths are shortened to `~`. The recorded session cost $5.48 for the plan and critics, $6.34 for
accepting the stories, and $13.19 for the wave.

## What you need

- The `library/` directory from [the ESS tutorial](./first-ess-specification.md), committed to Git.
- Claude Code or Codex, and Go 1.23 or newer.

## 1. Install `aep` and its plugin

If you did the ESS tutorial through `b10x`, add the `aep` product:

```shell-session
$ b10x init aep,ess --host claude --out plan.json
```

The actions it plans (the rest of the plan lists what is already installed):

```text
Actions (6):
   1. register marketplace `b10x`: claude plugin marketplace add beyond10x/agentplugins
   2. install `b10x@b10x`: claude plugin install b10x@b10x --scope user
   3. install `aep@b10x`: claude plugin install aep@b10x --scope user
   4. install `ess@b10x`: claude plugin install ess@b10x --scope user
   5. install `aep`: install aep 0.64.0 into ~/.local/bin (prebuilt archive)
   6. install `ess`: install ess 0.39.0 into ~/.local/bin (prebuilt archive)

```

```shell-session
$ b10x setup apply --plan plan.json --yes
```

Restart Claude Code so it loads the plugin. `aep@b10x` brings the `aep:planning` skill, which owns
the store, and `aep:implementing`, which runs waves.

## 2. Adopt a planning store

Ask:

```text
This repository has an ESS specification in spec/ and a Go implementation in impl/. Set up AEP
planning here so the work on it is planned in the repository. Tell me what you did.
```

The agent ran `aep plan reverse init`, which wrote `.engineering/project.yaml` in the current store
format (`aep.project/5`) and pinned the shared planning rules to the exact commit of the `aep`
release. Then its `aep:reverse-engineer` agent read the repository and its history and drafted the
first artifacts, each citing the line it came from:

```shell-session
$ aep plan artifact list
```

```text
executable-system-specification:library  executable-system-specification  draft  library v1 — ESS specification of the lending domain
story:borrow-member-check-tested         story                            draft  The registered-member check on Borrow is exercised by a test
vision:lending-library                   vision                           draft  A small lending library, specified in ESS and implemented in Go
```

It also found a real gap: the check that a borrower is a registered member is never exercised
(`impl/conformance_test.go:90` always passes `false`), and filed it as a story instead of fixing it
unasked. It ended with five questions the code could not answer, such as whether a network service
is owed (your run may ask a different number). Commit the store before you go on: it is a plan, and a plan belongs in version control.

## 3. Ask for a feature

```text
Members should be able to reserve a book that is on loan, so that when it comes back it is held
for them instead of going back on the shelf. Plan this change. Before you write anything, ask me the
decisions you need from me, then stop and wait for my answers.
```

A reservation changes the book's lifecycle, so `aep:planning` will not write a story around it
until the specification says what a reservation is. It asked first, each question with the answer
it would take:

<details>
<summary>The agent's questions (9)</summary>

I read `spec/domains/lending.yaml` and `impl/library.go`. Right now a Book can only be `OnShelf → OnLoan → OnShelf` (or `Withdrawn`). The borrower is stored on the Book itself, there's no loan record, and the only actor in the spec is the Librarian. Reservations change the Book's lifecycle, so I need these decided before I model it. I've put a suggested default on each, so you can answer "defaults, except 3b" if that's easier.

**The model**

1. **How many reservations can a book have?** Just one at a time, or a queue served first come, first served? This decides whether a reservation is a field on Book (`reserved_for_id`) or its own entity with a one-to-many relation to Book. I won't guess this one. *Default: one at a time; a second attempt is refused.*
2. **What does a returned, reserved book become?** I'd add a new Book state, `OnHold`, alongside `OnShelf`. The alternative is to keep it `OnShelf` with a hold marker on it. *Default: a new state `OnHold`.*

**The lifecycle**

3. **How does a held book stop being held?**
   a. The member who reserved it borrows it: `OnHold → OnLoan`. When anyone else tries to borrow it, should that be a `BookStateConflict` or a new error such as `BookHeldForAnotherMember`? *Default: a new error, since it tells the librarian why.*
   b. Is there a way to release the hold, putting the book back on the shelf (`OnHold → OnShelf`), for when the member never comes to collect it? If so, who does it? Expiring holds after N days would need a clock, and the spec has none. *Default: a manual `ReleaseHold` command for the Librarian; no time-based expiry.*
4. **Can a reservation be cancelled while the book is still on loan?** *Default: yes, with a `CancelReservation` command.*
5. **Can a held book be withdrawn?** *Default: no. Release the hold first; withdraw keeps acting only from `OnShelf`.*

**The rules**

6. **Who places a reservation?** The only actor in the spec is the Librarian. Do they reserve on a member's behalf, or should I add a Member actor who can reserve for themselves? *Default: the Librarian, as with borrowing.*
7. **Which reservations are refused?** The member already has the book on loan? The book is on the shelf? (Your wording suggests on-loan books only, so the book would just be borrowed instead.) *Default: refuse both.*
8. **Must the reserving member be registered?** For borrowing, that check was left to the implementation and is off in the conformance target, which is the untested gap the draft story covers. Should reservations do the same, or should the spec model the check this time? *Default: the same as borrowing, for consistency.*

**What people can see**

9. **Views and events.** Should the Catalogue show `reserved_for_id`? Should there be a new "held awaiting pickup" view? And when a reserved book comes back, should the return emit `BookReturned` plus `BookHeld`, or have its own outcome? *Default: add the field to the Catalogue, add a `BooksOnHold` view, and give the return a separate `held` outcome that emits `BookHeld`.*

Once you've answered, I'll update the ESS domain and validate it. Then I'll create an epic that cites the updated spec and break it down into stories. A panel of four review agents will check the stories for testability, overlap, scope and parallel work, and I'll write up anything they find that I didn't fix. I won't touch any file until you reply.

</details>

This page answered:

```text
Defaults for all nine. Go ahead: model it, plan it, and have the critics review the plan. Tell me
what you did and what is still open.
```

## 4. What it planned

**The specification first.** The agent extended `spec/domains/lending.yaml` before drafting a
single story: two new book states, a `reserved_for_id` field, four commands, two errors, a
`BookHeld` event and a `BooksOnHold` view. The book's lifecycle now reads:

```yaml
    lifecycle:
      initial: OnShelf
      states: [OnShelf, OnLoan, OnLoanReserved, OnHold, Withdrawn]
      terminal: [Withdrawn]
      transitions:
        - name: lend
          from: [OnShelf]
          to: OnLoan
        - name: return
          from: [OnLoan]
          to: OnShelf
        - name: reserve
          from: [OnLoan]
          to: OnLoanReserved
        - name: cancel_reservation
          from: [OnLoanReserved]
          to: OnLoan
        - name: return_to_hold
          from: [OnLoanReserved]
          to: OnHold
        - name: collect
          from: [OnHold]
          to: OnLoan
        - name: release_hold
          from: [OnHold]
          to: OnShelf
        - name: withdraw
          from: [OnShelf]
          to: Withdrawn
```

It reported one deviation from the answers it had proposed: taking a held book out is its own
command, `CollectHold`, because one ESS command cannot both depend on the book's state and check who
the book is held for. It wrote that into the epic instead of bending the model.

```shell-session
$ ess verify conform synthesize --path spec --target go --out impl
```

```text
warning: spec/ess-inputs.yaml requires ess 0.38.0 and this is ess 0.39.0, which is newer; continuing (--strict-requires refuses)
55 scenario(s) (0 authored), 0 refusal(s), 7 file(s) written to impl
```

**Then the plan.** An epic, `epic:book-reservations`, and six stories drafted by the
`aep:decomposer` agent. Each story's acceptance names the exact scenarios it turns from skipped to
passing; together they cover all 55 once.

**Then the critics.** Four agents read the drafted set at once, none seeing another's verdict:
acceptance (can each story be checked?), design (coupling, cycles, a split abstraction), scope
(everything the epic promised is claimed, nothing else), and parallel safety (which stories land on
one file). All four approved in the first round, and each verdict is stored word for word as a
`review-result` that cannot be edited afterwards.

```shell-session
$ aep plan artifact validate
```

This output was taken at the end of the recording, after the wave had added two adversary records
(14 artifacts at this point, 16 below):

```text
16 file(s) in ~/library/.engineering/planning: 16 artifact(s)
4 review(s) recorded no findings block:
  - review-result:acceptance-round-1 states its findings as prose only — nothing can enumerate what it found, so                  the next review starts from nowhere
  - review-result:design-round-1 states its findings as prose only — nothing can enumerate what it found, so                  the next review starts from nowhere
  - review-result:parallel-safety-round-1 states its findings as prose only — nothing can enumerate what it found, so                  the next review starts from nowhere
  - review-result:scope-round-1 states its findings as prose only — nothing can enumerate what it found, so                  the next review starts from nowhere
valid
```

`valid` is the verdict. The four warnings above it come from `aep` itself: it counts a review that
ended with an empty findings block as having none (beyond10x/aep#60). AEP 0.65.0 fixed this: an
`approve` review with `findings: []` no longer draws the warning, which is kept for a review with
no findings block at all.

## 5. Accept the stories and propose a wave

```text
Leave the open points as they are for now. Commit what you did. Then accept the six reservation
stories and propose the first wave with aep:implementing: tell me which stories are in it and why,
and stop there until I approve.
```

Accepting a story is a lifecycle move through `aep plan artifact move`, which the store validates.
In this recording, moving the first story to `active` was refused,
`story:reservations-suite-baseline is proposed and serves no objective`, so the agent linked every
story to the vision it serves and moved them again. The decomposer now draws that link when it drafts
a story, so your store will not refuse the move. The agent committed the plan on a branch of its
own, `book-reservations`, which becomes the wave's base.

A **wave** is the set of stories that can be built at the same time without touching the same file.
The store derives it from each story's declared scope: the files a story lands on, which the
`aep:story-scoper` agent works out and records. If `waves` answers `nothing selected declares a scope`,
ask the agent to scope the stories first.

```shell-session
$ aep plan artifact waves --kind story --status active
```

```text
wave 1
  story:reserve-book-on-loan
wave 2
  story:return-reserved-book-to-hold
wave 3
  story:cancel-reservation
wave 4
  story:collect-held-book
wave 5
  story:release-hold
collision: story:cancel-reservation story:collect-held-book impl/conformance_test.go
collision: story:cancel-reservation story:collect-held-book impl/library.go
collision: story:cancel-reservation story:release-hold impl/conformance_test.go
collision: story:cancel-reservation story:release-hold impl/library.go
collision: story:cancel-reservation story:reserve-book-on-loan impl/conformance_test.go
collision: story:cancel-reservation story:reserve-book-on-loan impl/library.go
collision: story:cancel-reservation story:return-reserved-book-to-hold impl/conformance_test.go
collision: story:cancel-reservation story:return-reserved-book-to-hold impl/library.go
collision: story:collect-held-book story:release-hold impl/conformance_test.go
collision: story:collect-held-book story:release-hold impl/library.go
collision: story:collect-held-book story:reserve-book-on-loan impl/conformance_test.go
collision: story:collect-held-book story:reserve-book-on-loan impl/library.go
collision: story:collect-held-book story:return-reserved-book-to-hold impl/conformance_test.go
collision: story:collect-held-book story:return-reserved-book-to-hold impl/library.go
collision: story:release-hold story:reserve-book-on-loan impl/conformance_test.go
collision: story:release-hold story:reserve-book-on-loan impl/library.go
collision: story:release-hold story:return-reserved-book-to-hold impl/conformance_test.go
collision: story:release-hold story:return-reserved-book-to-hold impl/library.go
collision: story:reserve-book-on-loan story:return-reserved-book-to-hold impl/conformance_test.go
collision: story:reserve-book-on-loan story:return-reserved-book-to-hold impl/library.go
5 wave(s), 20 collision(s), 0 unassessed
```

Every pair of stories lands on the same two Go files, so each wave holds one story. (This listing
was taken after wave 1, which held the sixth story, `reservations-suite-baseline`.) The agent
proposed that wave, named every commit your approval would authorise and nothing more (no push, no
tag, no second wave), and asked three things: which branch to use as the base, whether plain
`git worktree` is acceptable (the `worktree` plugin, `b10x init worktree`, gives it managed
worktrees instead), and how much model budget is left.

## 6. Run the wave

```text
Approved. 1: use book-reservations as the base. 2: plain `git worktree add` is fine here. 3: the
budget is not a limit for this wave. Implement the first wave, with the adversary review, and stop
when it is merged. Tell me what happened, including anything the adversary found.
```

What the wave did, in order:

1. The `aep:implementor` agent built the story in its own worktree: the smallest change that turns
   its 17 scenarios from skipped to passing.
2. The claim was checked against the base: the same test command before and after. 10 of 17 passed
   before, 17 of 17 after, none failed. That comparison is recorded as `verification` evidence.
3. The `aep:adversary` agent tried to break the change, for up to two passes. It found no failing
   behaviour, but three gaps in the tests: the new view code was never exercised, a doc comment
   misdescribed `Return`, and nothing tested that `Return` refuses the two new states. Each was
   fixed by a test, and each finding's outcome was recorded.
4. The unit merged into the wave's integration branch, the gate ran there, and the integration
   branch merged into the base.

```shell-session
$ git log --oneline --graph
```

```text
*   9dce7df Merge wave-1/integration into book-reservations
|\  
| * 950900a Wave 1 closed: story:reservations-suite-baseline implemented
| * aee0d43 Merge impl/reservations-suite-baseline into wave-1/integration
|/| 
| * c3bebc2 Pin Return's refusal of the two new states
| * e7658b7 Pin the reservation views with tests that force the new states
| * f0cc3e2 Add reservation states and answer the reservation views
|/  
* 3163f1b Wave 1: accept reservation stories, open wave page
* 7d2a962 Plan book reservations: ESS model, epic, six stories, critic round 1
* e89b41c Adopt AEP planning
* 30e5ba2 Lending library: ESS specification and Go implementation
```

Ask the store why the story is where it is:

```shell-session
$ aep plan artifact explain story:reservations-suite-baseline
```

```text
story:reservations-suite-baseline in ~/library/.engineering/planning: implemented, revision 7
  draft -> proposed  2026-09-28T13:54:57Z  (revision 3)
    no record: nothing was recorded about how this was decided
  proposed -> active  2026-09-28T13:54:57Z  (revision 4)
    no record: nothing was recorded about how this was decided
  active -> implemented  2026-09-28T14:09:35Z  (revision 7)
    review_outcome from review-result:adversary-wave-1-u1-pass-1, observed 2026-09-28T14:04:39Z, admitted at revision 4
    review_outcome from review-result:adversary-wave-1-u1-pass-1, observed 2026-09-28T14:04:40Z, admitted at revision 4
    verification from go test -count=1 -v ./... in impl/, impl/essconform regenerated from spec/ (55 scenarios): acceptance scenarios PASS 10/17 at baseline 3163f1b, 17/17 at c3bebc2; 0 FAIL both (.engineering/drafts/wave-1/integration/scratch/baseline.txt vs .engineering/drafts/wave-1/u1/scratch/treatment.txt), observed 2026-09-28T14:09:02Z, admitted at revision 4
    review_outcome from review-result:adversary-wave-1-u1-pass-2, observed 2026-09-28T14:09:02Z, admitted at revision 4
    test_result from gate on wave-1/integration at merge aee0d43: aep plan artifact validate (valid); ess specify validate (valid); ess verify conform synthesize --target go --out impl (55 scenarios, 0 refusals); go vet ./... (exit 0); go test -count=1 -v ./... (ok; TestConformance 17 PASS / 38 SKIP / 0 FAIL, 3 unit tests PASS); go run cmd/gofmt -l impl (empty, exit 0) (aee0d43), observed 2026-09-28T14:09:35Z, admitted at revision 6
  next: archived needs no record
```

Every move is there with what the store admitted before it: the adversary's two recorded outcomes,
the before/after verification, and the gate that ran on the merge.

```shell-session
$ aep plan artifact board --kind story
```

```text
draft (1)
  story:borrow-member-check-tested  The registered-member check on Borrow is exercised by a test

active (5)
  story:cancel-reservation  A librarian cancels a reservation while the book is still on loan
  story:collect-held-book  The member a book is held for collects it
  story:release-hold  A librarian releases a hold and the book goes back on the shelf
  story:reserve-book-on-loan  A librarian reserves a book on loan for a member
  story:return-reserved-book-to-hold  A returned reserved book is held for the member who reserved it

implemented (1)
  story:reservations-suite-baseline  Existing lending behaviour passes the regenerated reservations suite
```

## 7. Keep going

Five stories are `active`, and `aep plan artifact waves` already says which comes next. Ask for the
next wave the same way. Each wave ends with the suite a little greener:

```shell-session
$ cd impl && ESS_REPORT_FORMAT=2 go test ./...
```

```text
ok  	example.com/library	0.043s
?   	example.com/library/essconform	[no test files]
```

The open points from steps 2 and 4 stay in the store until somebody answers them: a store keeps an
unanswered question as a record instead of a guess.

## Next

- **The longer walk.** The [golden path](../golden-path.md) goes through the same steps on a larger
  repository, and ends by handing one story to `metaharness aep drive`, where an engine rather than
  the agent decides every step.
- **Test the suite itself.** `ess:hardening` runs `ess verify conform mutate` and the concurrent
  explorer against your implementation.
- **Go deeper into AEP.** The [AEP documentation](https://beyond10x.github.io/docs/aep/) covers
  lifecycles, evidence and the planning commands.
