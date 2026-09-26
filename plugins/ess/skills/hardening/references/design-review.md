# Design review: a brief for an agent

Technique 8. Dispatch one agent with the brief below, filled in. It found the most of any technique
on the one adopter where all eight were run — 26 findings — so run it early.

The agent needs read access to three things and write access to none of them.

---

## Brief

You are reviewing a specification against the design it claims to implement. You do not edit
either. You produce a list of findings.

**Inputs**

- the design document: `<path>`
- the mapping, if one exists — the document or table that says which design statement became which
  spec declaration: `<path>`, or `none`
- the specification: `<directory>`; validate it first with `ess specify validate --path <directory>`
  and stop if it refuses

**Procedure**

1. Read the design end to end. List every statement that promises behaviour: a rule, a limit, an
   ordering, a uniqueness, a "never", an "always", a "only when".
2. For each statement, find where the spec declares it — an entity field or invariant, a lifecycle
   transition, a command outcome and its `when:` guard, an actor's grant, a view.
3. Then go the other way: for every spec declaration, find the design statement that justifies it.
4. Classify each mismatch with exactly one of the classifications below.

**Classifications**

| classification | meaning |
|---|---|
| `missing` | the design states it; the spec does not declare it |
| `contradicts` | both state it, and they disagree |
| `stale mapping` | the mapping points a design statement at a declaration that no longer says it, or at one that does not exist |
| `spec-only` | the spec declares it; no design statement justifies it |
| `unclear` | the design statement is too vague to decide which of the above applies |

**Every finding cites both sides.** The design by its line (`design.md:42`, quoted), the spec by
`file:line` (quoted). A `missing` finding cites the spec file and line where the declaration would
belong; a `spec-only` finding cites the nearest design section and says it has nothing. A finding
without both citations is not a finding; drop it.

**Quote a vague statement and mark it `unclear`; never invent a meaning for it.** "Players should
not be able to cheat" is `unclear` with that sentence quoted. Supplying a plausible rule and then
reporting the spec as missing it is the one failure this review exists to avoid: downstream, the
invented rule reads as the designer's.

**Output**, one row per finding:

```text
| # | classification | design (file:line, quoted) | spec (file:line, quoted) | note |
```

Then the counts per classification, and the design sections you did not reach, if any.

---

## After the review

- Each `missing` and `contradicts` finding goes to the design's owner or becomes a spec change
  (`ess:specifying`); the review does not decide which.
- `spec-only` findings are candidates for deletion — "a declared refusal nobody executes is a
  fiction" (`ess:testing-conformance`).
- `unclear` findings go back to the design's author as questions, with the quote.

**Plant a defect before trusting a clean review:** on a copy of the spec, delete one declaration the
design plainly states, and run the brief on the copy. It must come back `missing`, citing both sides.
