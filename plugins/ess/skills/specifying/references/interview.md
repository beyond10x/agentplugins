# Interviewing before a domain is drafted

A domain drafted from a one-line request is mostly `UNMAPPED:` markers, and each marker is a
question somebody could have answered before the draft was written. The interview asks them first,
so the draft records decisions instead of gaps.

Run it when a noun has no typed home yet and the request does not already say what the noun is,
what it relates to, and how it changes state. Skip it when an OpenAPI document or existing code
answers those — read the source instead. When the request asks for a finished specification, or
nobody is there to answer, go straight to *Without an operator* below.

## The design tree

Every decision about the domain has prerequisites. Whether a `ShipmentLine` is owned by its
`Shipment` cannot be asked before anyone has said there are lines at all. Treat the open decisions
as a tree: each answer settles one node and makes the questions below it askable.

The **frontier** is every open decision whose prerequisites are already settled. Ask the whole
frontier at once, as one round. A question that depends on another question in the same round
belongs to the next round.

## One round

Number each question, state it with the options you can see, and give the answer you would take:

```
Q1 — Does a shipment own its lines?
If a shipment is deleted, do its lines go with it (owns) or stay behind (references)?
Recommended: owns. No line in the request stands without a shipment.

Q2 — Can a shipment have no lines?
Recommended: yes, while it is Draft. Nothing in the request forbids an empty draft.
```

Then wait for the answers. Each answer is written into the specification at once — the entity, the
`relations:` entry, the lifecycle state — and `ess specify validate --path <specification>` is run
before the next round, so a refused answer is found while the person who gave it is still there.

## Facts are yours, decisions are theirs

A question whose answer is in the repository — an existing type, a column, a route, an ADR — is not
a question for the person. Look it up and state what you found. Only a choice goes to them.

When a term they use conflicts with an entity the specification already declares, say so with the
file: "`domains/shipment.yaml` declares `Shipment` with one destination; you said a shipment can
split across two. Which is right?" Resolve it before the round ends. The resolved term lives in the
ESS document, not in a glossary beside it.

## When it is finished

The interview is finished when the frontier is empty: every decision the draft needs has an answer
or has been ruled out of scope, and nothing is left as a silent assumption. Only then draft the rest
of the domain. Any decision still open is an `UNMAPPED:` marker, as the skill says.

## Without an operator

In a non-interactive run, or when the request asks for a finished specification, do not hold the
interview. For each question you would have asked, take the recommended answer and write it into the
specification. Then record the question and the answer taken:

- where the repository has an AEP store and `aep` is on `PATH`, as an `approval-record`, as the
  `aep:planning` skill's § 4 *When there is no operator* describes;
- otherwise, as a numbered list in your report — the question, the answer taken, and the file and
  entity it changed — so the user can overrule each one.

A decision nobody could make — an ownership with no evidence either way — stays an `UNMAPPED:`
marker rather than a recommended guess.

---

The round format and the frontier rule are adapted from `skills/productivity/grilling` and
`skills/engineering/domain-modeling` in `github.com/mattpocock/skills` (MIT).
