# Compatibility in the gate

Current ESS classifies semantic changes for **callers**, **readers** and **history**. Use the native
classification instead of treating every added field or enum variant as automatically compatible:
closed readers and required inputs make that assumption unsafe.

Materialise the specification at the previous release into a scratch directory, then compare:

```console
ess verify diff --from <release-spec-dir> --to <spec-dir> --compatibility --format json
ess verify diff --from <release-spec-dir> --to <spec-dir> --fail-on breaking-or-unknown --format json
```

The first writes `ess-diff/14`, including each change's dimensions and compatibility, or the
newer format a change needs, which earlier readers refuse:

| format | what it adds |
|---|---|
| `ess-diff/15` | a domain added to or removed from `system.yaml` is `domain/<name>/added` (compatible) or `domain/<name>/removed` (breaking for callers and readers); a purely additive revision passes `--fail-on breaking-or-unknown` |
| `ess-diff/16` | a command input whose wire form moves between a value, an object and an array (`String` to a record, text to `List` or `Map`, a newtype redefined as a struct) is breaking for callers, with `shapes: {before, after}` |
| `ess-diff/17` | a change that refuses an existing caller is breaking for callers, with `narrows`: `required-input` (an added input that is not `Optional`), `refusal-added`, `refusal-widened` (a `when:` that now refuses an input the earlier revision accepted) |
| `ess-diff/18` | a move of `undeclared_fields` (`ess/24`) is `type/<name>/undeclared-fields-changed` or `command/<name>/response-undeclared-fields-changed`, never `unclassified-changed`. Opening is `expanded`: an opened struct is breaking for readers of its output use, an opened response breaking for readers. Closing is `narrowed`: breaking for callers of a struct's input use and for its stored use, compatible for a response |

The two refusal ratings are decided only where every outcome of the earlier revision, and each up
to the refusal in the later one, is a plain `when:` or `otherwise` branch; a refusal added before
an unguarded `creates:` branch stays `unknown`.

The second command fails at exit 4 for an unacknowledged breaking or unknown change. Exit 1 is an
input or acknowledgement refusal; it is not a compatible result. Repeat `--dimension callers`,
`--dimension readers` or `--dimension history` only where the gate deliberately narrows its claim;
the default checks all three. A change to an enum variant's attributes (`ess/23`) is reported as
`unclassified-changed`, so the gate fails it at exit 4 until it is reviewed and acknowledged.

A reviewed exception uses `--acknowledgements <file>` with an
`ess-diff-acknowledgements/1` document naming exact change IDs and both endpoint digests. Read the
current CLI's format before writing that document. Keep it committed with the review rationale.
Do not carry it to another comparison: stale endpoint digests must refuse.

Before trusting the gate, compare a specification with itself and require exit 0. Then remove a
command grant or an event field, or add a command input that is not `Optional`, in a copy;
compare against that copy, and require exit 4 naming the change. Restore the copy and verify
exit 0. Retain the actual diff and exit statuses beside the release evidence. The [current-features example](../../specifying/references/current-features.md)
provides a small validated control model.
