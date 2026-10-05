---
title: Your first governed plan
sidebar_label: Your first governed plan
description: Continue the Rust lending library with AEP. Specify a reservation feature, review its plan, and implement the first story in a managed wave.
---

# Your first governed plan

Continue [Your first ESS specification](./first-ess-specification.md) with **AEP**. The starting
repository has a specification in `spec/` and a Rust implementation in `impl/`. Its native ESS
runner executes 17 conformance scenarios inside one Cargo integration test. AEP records the plan,
reviews, evidence and remaining work in the repository.

You will plan one feature: members can reserve a book that is on loan. The agent specifies the
change before drafting stories, sends those stories to four critics, and implements the first
approved wave with an independent adversary. Counts and identifiers depend on the resulting model;
do not copy scenario counts from a different run.

These instructions target AEP 0.68.0 and ESS 0.53.0. The complete
[2026-09-28 recording](./first-governed-plan-2026-09-28.md) preserves the earlier Go session and its
costs as historical evidence.

## What you need

- The completed ESS tutorial committed to Git.
- Claude Code or Codex and the Rust toolchain used by the ESS tutorial.
- Time and model budget for planning, four critics, implementation and an adversary review.

## 1. Install the plugins and tools

Use your host in place of `claude` if necessary:

```console
b10x init aep,ess,worktree --host claude --out plan.json
b10x setup apply --plan plan.json --yes
```

Review the plan before applying it, then restart the host so it loads the installed plugins.
Confirm the starting point:

```console
ess specify validate --path spec
ess verify conform synthesize --path spec --target ir --out impl/suite.json
cargo test --locked --manifest-path impl/Cargo.toml -- --nocapture
```

The Cargo summary counts test functions. The native ESS report counts executed scenarios;
retain both. A green function that skips required scenarios is not complete conformance.

## 2. Adopt the planning store

Ask:

```text
This repository has an ESS specification in spec/ and a Rust implementation in impl/. Set up AEP
planning here so work is planned in the repository. Read the implementation and its conformance
coverage, record gaps as draft work, and tell me what you did. Keep executable code in Rust.
```

The `aep:planning` skill owns the store. The agent adopts the current project format and exact
protocol source, then derives artifacts from the repository. Ask it to cite evidence for inferred
behaviour. In this example the declared borrowing conformance covers book lifecycle and views;
registered-member enforcement is a separate gap, not an assertion that the starting suite proves.

```console
aep plan artifact list
aep plan artifact validate
```

Commit the validated store before continuing.

## 3. Specify the feature before planning it

Ask the agent to clarify the model before drafting it:

```text
Members should be able to reserve an on-loan book so it is held for them when returned. Plan this
change, with its ESS specification first. Ask me for the decisions you need before writing it.
```

For this walkthrough, use these decisions:

- One reservation per book; a second reservation is refused.
- Reservations apply only to on-loan books and cannot name the current borrower.
- Returning a reserved book puts it on hold. Only its reserving member may collect it.
- A librarian may cancel an on-loan reservation or release a hold; no timed expiry.
- A held book cannot be withdrawn until the hold is released.
- The librarian acts on a member's behalf. Keep registered-member enforcement as explicit
  separate work; do not claim the existing suite verifies it.
- Expose the reservation in the catalogue and add a view of books on hold.

Then ask:

```text
Use those decisions. Model and validate the feature, then create an epic and stories whose
acceptance names generated conformance scenarios. Have all four planning critics review it.
Record unsupported semantics and unresolved questions in the store instead of guessing.
```

The agent may use separate commands for collecting a hold and borrowing a shelf book. The exact
command and guard structure must be supported by the released ESS validator and synthesis path.
In ESS 0.53, an existence-only input-related guard cannot accompany `wrong_state`, and related
guards cannot accompany `unknown_instance`. Some present-row predicate refusals can accompany
`wrong_state` in `ess/22`; see the ESS tutorial's qualified examples. Keep the intended rule
visible; do not remove lifecycle assertions or invent predicates to obtain a green synthesis.

## 4. Review the plan and its evidence

Regenerate the canonical suite:

```console
ess specify validate --path spec
ess verify conform synthesize --path spec --target ir --out impl/suite.json
aep plan artifact validate
```

Ask the agent to show which story owns each new scenario and which existing scenarios must stay
green. The four critics cover acceptance, design, scope and parallel safety. Their findings and
resolutions belong in review records; an approval word alone does not describe what was checked.
The unimplemented feature should produce a visible coverage gap or failure in the baseline target.
Preserve that evidence before changing the implementation.

## 5. Accept the stories and propose a wave

```text
Commit the specification and reviewed plan. Accept the ready stories, record their file scopes,
and propose the first wave with aep:implementing. Name the stories, evidence, managed worktrees
and model budget required. Stop for my approval of that concrete wave.
```

```console
aep plan artifact waves --kind story --status active
```

Stories touching the same Rust files usually run in different waves. If the command reports no
scope, have the agent scope the stories before selecting a wave. Each story must serve the
appropriate objective and satisfy the store's lifecycle requirements.

## 6. Implement the approved wave

After reviewing its scope and cost, approve it explicitly:

```text
Approved: implement the proposed first wave with managed worktrees and an independent adversary.
Keep all executable changes in Rust. Merge the green wave into this repository's working branch,
record the evidence and remaining work, and stop before a second wave or publication.
```

The implementor works in an isolated tree. The adversary checks the result independently, including
whether the suite would catch a broken implementation. Compare before and after at the same seam:

```console
cargo test --locked --manifest-path impl/Cargo.toml -- --nocapture
aep plan artifact validate
aep plan artifact board --kind story
```

A completed wave records executed scenario counts, remaining skips or refusals, the adversary's
findings and fixes, and the merge gate. Worktree cleanup needs published recovery proof or an
archive; local merge alone does not make a managed checkout disposable.

## Keep going

Ask for the next proposed wave when ready. Keep unanswered questions and remaining feature
scenarios in the store. See the [golden path](../golden-path.md) for a longer recorded example and
[the AEP plugin](../plugins/aep.md) for governed engine delivery.
