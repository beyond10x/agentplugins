# ESS repository preflight

Run before every ESS activity, lifecycle skill and direct agent procedure. Coordinators pass this
record and the user's decision to workers. Reuse it for matching repository, selected inputs and
effective toolchain; changed selections need another capability inspection, not a repeated offer.

## Identify what actually runs and what it reads

1. Record the repository and requested input roots. Inspect `ess-inputs.yaml` selection and its
   `requires` pin, `b10x.toml`, CI/task wrappers and toolchain overrides. Run `ess --version` and
   `ess specify toolchain which` from each applicable project root. Installed and effective CLI
   releases can differ. Read the installed plugin version from its manifest.
2. Inventory **selected** source documents and their format families, product `version: vN`,
   generated formats and output ownership separately. An unselected example is not an upgrade
   target. A CLI release, toolchain pin, `ess/N`, `ess-inputs/N`, conformance/output formats and
   product version are different identities; never mechanically align their numbers.
3. Reuse `b10x upgrade ess --host <host> --out <plan-file>` release discovery. Read the stable
   target's format contract and per-family format history, using an immutable release reference.
   The [format contract](https://github.com/beyond10x/ess/blob/0.52.0/website/docs/reference/formats.md)
   and [history](https://github.com/beyond10x/ess/blob/0.52.0/website/docs/reference/spec-versions.md)
   are a verified reference snapshot, not a permanent latest-version claim. Compare each applicable
   source family with that target, including sources that still validate under their older format.
   Check effective compiler and generated-runner capabilities, not only a release number.
4. If remote discovery is unavailable, report freshness unknown and retain verified local
   capability evidence. Offline evidence may support compatible work; it cannot prove latest.
   Future/unknown formats stay refused until a matching reader is demonstrated. Do not downgrade
   a header to force validation. No specifications present means no source migration, but still
   inspect pins and CLI/plugin state before reporting current.

## Offer once, including still-valid sources

Keep a session record keyed by `(repository identity, ESS, session)` with selected inputs,
installed/effective releases, pins, source/generated formats, freshness provenance, proposed target,
affected files, verification and decision (`authorized`, `declined`, `pending`, `current` or
`freshness-unknown`). Pass it to workers; consult it on every entry point.

Offer a source upgrade whenever the target has a newer source format in an applicable family,
even if the old source validates and needs no new construct. Explain what the newer semantics
change and whether retaining the lowest sufficient format remains compatible. Present one combined
offer covering source edits, semantic decisions, regeneration, effective toolchain and pin changes.
Existing authorization for that exact scope is sufficient; otherwise ask once. Pending or declined
upgrades allow compatible original work with the selected tool and existing pins. Refused work
stays blocked. An unattended run with no authorization leaves the upgrade pending.

A no-action installation plan is not grounds to say the repository is current. Complete the source
and effective-toolchain checks first. Dirty files do not suppress an offer; prepare an isolated diff
and preserve unrelated edits. Record the decision so later skills neither prompt again nor treat
a decline as a required upgrade.

## Execute the accepted upgrade

ESS has no general source migration command. Derive explicit edits from the target release's
semantics; a format-header replacement alone is not proof of compatibility. List any newly required
meaning (identity, lifecycle, relations, error mappings or output ownership). Resolve decisions from
the repository and accepted intent; ask for genuinely missing required semantics before dependent
changes. Preserve the product `version: vN` unless a separate accepted contract change requires it.

1. Capture baseline validation, source selection and owned generated output. Prepare the source
   diff in a managed worktree; retain the old source and pin for review.
2. Install an accepted CLI/plugin plan using `b10x setup apply --plan <plan-file> --yes`. Where
   changing the effective repository toolchain was accepted, run
   `ess specify toolchain install <target-release> --pin` and confirm with
   `ess specify toolchain which`. Otherwise preserve the pin; a newer PATH binary does not prove
   a pinned repository now uses it. Check flags against the selected release's help.
3. Validate and compile the changed selected specification. Regenerate owned schema, OpenAPI,
   conformance suites, runners and other outputs through the repository's generation commands;
   read their actual help/ownership contract, and never repair generated bytes by hand.
4. Run projection/drift checks and conformance against the real target, recording effective
   producer, suite/report identities, counts and refusals. Missing execution evidence or all-skipped
   results are not a pass. Review semantic and generated diffs with their provenance.

Complete only when the resulting source selection, effective toolchain, source/generated formats,
validation, regeneration and conformance evidence match the accepted scope. Report unsupported
migration semantics or unavailable verification as explicit gaps. Update the session record and
resume the original task.
