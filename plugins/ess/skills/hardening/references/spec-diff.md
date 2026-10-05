# Compatibility in the gate

ESS 0.53.0 classifies semantic changes for **callers**, **readers** and **history**. Use the native
classification instead of treating every added field or enum variant as automatically compatible:
closed readers and required inputs make that assumption unsafe.

Materialise the specification at the previous release into a scratch directory, then compare:

```console
ess verify diff --from <release-spec-dir> --to <spec-dir> --compatibility --format json
ess verify diff --from <release-spec-dir> --to <spec-dir> --fail-on breaking-or-unknown --format json
```

The first writes `ess-diff/14`, including each change's dimensions and compatibility. The second
fails at exit 4 for an unacknowledged breaking or unknown change. Exit 1 is an input or
acknowledgement refusal; it is not a compatible result. Repeat `--dimension callers`,
`--dimension readers` or `--dimension history` only where the gate deliberately narrows its claim;
the default checks all three.

A reviewed exception uses `--acknowledgements <file>` with an
`ess-diff-acknowledgements/1` document naming exact change IDs and both endpoint digests. Read the
current CLI's format before writing that document. Keep it committed with the review rationale.
Do not carry it to another comparison: stale endpoint digests must refuse.

Before trusting the gate, compare a specification with itself and require exit 0. Then remove a
command grant or an event field in a copy, compare against that copy, and require exit 4 naming
the change. Restore the copy and verify exit 0. Retain the actual diff and exit statuses beside the
release evidence. The [current-features example](../../specifying/references/current-features.md)
provides a small validated control model.
