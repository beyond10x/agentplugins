---
name: upgrade
description: Check whether the ESS plugin and the `ess` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether ESS is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `ess`, or when an `ess` command behaves differently from what an ESS skill describes.
---

# Upgrade ESS

```bash
b10x upgrade ess --host claude --out ~/.local/state/b10x/plan.json
```

Use `--host codex` in Codex. It compares the installed `ess` plugin with what the marketplace serves and the `ess` on `PATH` with
the newest ESS release, and prints each difference with the action that fixes it. Nothing is
changed yet.

- Nothing to change: say "ESS is current" with both versions, and stop.
- Otherwise show the actions in one list and ask once. After a clear yes:
  `b10x setup apply --plan ~/.local/state/b10x/plan.json --yes`.
- A new plugin version loads in a new session; until then `b10x skill ess:<skill>` prints the new text.
- A project whose `ess-inputs.yaml` pins `requires: ess X.Y.Z` keeps running that release after the
  upgrade. Moving the project is a separate change to its repository:
  `ess specify toolchain install <new-version> --pin` rewrites the pin, and
  `ess specify toolchain which` confirms the release that now runs there.
- A project that commits generated output checks it against the new release before claiming it
  current: `ess generate --path <specification> --kind <kind> --out <directory> --check` and
  `ess generate cli … --check` name every file whose bytes the release changed. Regenerate those,
  regenerate the conformance suite, and rebuild generated implementation code: a release can add a
  context port method (such as `generate_optional_<t>`) the implementation must then supply.
  Regenerated Rust type libraries (`ess generate types`, which has no `--check`) can change
  `Cargo.toml` and `types-report.json`.
- Run `ess specify validate --path <specification>` with the new release first: a release can
  refuse a specification an older one accepted, such as an accepting `when:` or `external:`
  branch declared before a held-state branch (`ESS-COMMAND-004`); reorder as its hint says. A
  refusal whose `when:` always holds (`when: true` beside `error:`) is also `ESS-COMMAND-004`:
  give it the condition it refuses on, or drop `when:` to make it the default refusal. A domain,
  command or view wire name containing `/`, or spelled `.` or `..`, is `path_segment_wire_name`
  (`ESS-DOMAIN-012`, `ESS-COMMAND-012`, `ESS-VIEW-012`).
- Documents a release writes in a newer format are refused by older readers: a spec diff naming a
  domain change, an input wire-form move or a narrowing (`ess-diff/15`–`/17`), a mutation report or
  manifest naming an `identical_answer` (`/5`), a suite with a constrained `String` response
  (`ess-conformance/46`, `/47`), and a specification on `ess/24` with what it writes: a spec diff
  naming an `undeclared_fields` move (`ess-diff/18`) and a suite with an opened response or struct
  (`ess-conformance/48`, `/49`), a spec diff naming a view's `absent:` row (`ess-diff/19`), a
  mutation report or manifest naming `overlap_taken_by` (`/6`), a composition with `roles:`
  (`ess-composition/4`) and an `ess-http/1` binding; from 0.62.0 a suite with a `hold_now` step
  (`ess-conformance/50`, `/51`), a mutation report naming an `unchanged_suite` (`/7`) and an
  `ess-runtime/2` document; from 0.63.0 a suite carrying a one-time trace, a wrapped value
  invariant or a system-issued command (`ess-conformance/52`, `/53`) and a spec diff naming a
  struct `shorthand:`, a system actor, an `await_within:` bound or a domain's display name or
  summary (`ess-diff/20`); from 0.64.0 a suite with a view read through a reference
  (`ess-conformance/54`, `/55`) and a spec diff naming `field-source-changed` (`ess-diff/21`);
  from 0.67.0 a suite with a one-time read cell that names its reader (`ess-conformance/56`,
  `/57`); from 0.68.0 a specification on `ess/30`, which 0.67.0 refuses as `ESS-SPEC-009`
  ("this build implements specification format(s) [1, …, 29], not `ess/30`"). Move every reader (a CI job's `ess`, the runner crates or packages) in the same change that
  commits them.
- From 0.63.0 `validate` refuses a branch whose literal writes break an entity invariant
  (`sets: {capacity: 0}` beside `invariants: [capacity > 0]`) as `conflicting_declaration`; a
  `now` in a view filter stays refused, its hint naming `from`/`to` view parameters the caller
  sends. Under `ess/27` one precedence rule orders a command's branches (request, existence, held
  state, related rows, then accepting), so a missing related row now answers after the addressed
  record's existence and held state: raising a header to `ess/27` can change which branch a
  request takes, and its suite. Below `ess/27` every command keeps its order. A JSON Schema
  projection of a `String` newtype with `alphabet:` now asserts it as a `pattern`, so
  `ess generate --check` reports drift once per such newtype.
- From 0.66.0 a regenerated Go or TypeScript package shows an unsupported required scenario as a
  skipped subtest that still fails the run once (`ess:testing-conformance`).
- From 0.67.0, raising a header to `ess/29` refuses a guarded branch declared below one a later
  precedence step answers (`conflicting_declaration` naming both branches and the order to declare
  them in): move each named branch up ([later-formats.md](../specifying/references/later-formats.md#ess29)).
  Below `ess/29` declaration order is checked as before. Regenerate committed suites with no
  specification change: a guard's `coalesce(x, d)` (the structured form from `ess/27`) now reads
  `d` where `x` is absent instead of unknown; a ranked head scoped by another head's value ranks
  above the rows that scope placed, and one over owned rows is synthesized under the other head's
  owner; a connective child ordering two `Integer` inputs (`a <= b`) is witnessed instead of
  refused; an authored scenario that isolates a connective child withdraws its `ESS-SYNTH-022`;
  and a model with one-time responses, actors and a view gains a read cell per reader, written at
  `ess-conformance/56` (`ess:testing-conformance`). Read the change with
  `ess verify conform diff`.
- From 0.68.0 regenerate every committed Go package: it passes `go vet` from Go language 1.26
  only once regenerated (beyond10x/ess#629). Regenerate committed suites with no specification
  change: synthesis witnesses an `existing_instance:` outcome whose creator sits behind
  `when_related:` guards, or refuses it as `ESS-SYNTH-004` naming the guards; a ranked head on a
  field a later command sets is arranged through that command (beyond10x/ess#626); the interpreter
  answers a response field typed by a constrained string newtype. A view whose `source:` is a list
  is now `unsupported_construct` at `view.<name>.source` at every format, not a parse error. No
  suite or diff format moves. Raising a header to `ess/30` lets a kept reference drop its
  `exists: false` branch, which adds re-reads to the suite
  ([later-formats.md](../specifying/references/later-formats.md#ess30)). Read the change with
  `ess verify conform diff`.
- From 0.60.0 `validate` refuses a literal outside its newtype's `alphabet:` or `invariants:` as
  `type_mismatch` (in `sets:`, `payload:`, a binding's `mapping:`, enum `attributes:`), which 0.59.0
  accepted: write a value of the type. An `ess-composition/1` with `conformances:` null is refused
  as `unsupported_format`. No ESS crate depends on `serde_yaml` any more (YAML goes through
  `ess-yaml`), so a `cargo deny` advisory RUSTSEC-2024-0449 inherited from ESS clears with the
  lockfile update. A suite whose model reads its caller beside a `one_time_response:` outcome
  changes bytes, its disclosure scenarios now sending the caller. A regenerated Go module whose
  bounded contexts reference each other in a loop puts them in one package under `types/`; an
  explorer seed can name a different sequence; and the explorers now draw `preserves:` commands, so a green explorer run can turn red on a real
  defect (`ess:hardening` technique 2).
- Re-run authored scenarios (`ess verify conform author`) and regenerate committed suites: the
  current release judges an authored act by the command's precedence plan, so an act that claims a
  branch another branch answers first is refused as `ESS-AUTHOR-041` naming that branch (an
  `exists: false` claim over a stored reference where an input refusal's guard holds, or a
  `when_related:` or row-set claim where an earlier accepting `when:` holds); claim the branch it
  names. A suite for a command whose external branch comes before a `when_related:` refusal among
  its accepting branches can change bytes, because the synthesized external witness differs.
- From 0.59.0 a regenerated suite can gain scenarios: a command guarded by `when_subject:` on a
  field no immediate view covers now gets its success, refusal, transition and wrong-state
  scenarios instead of `ESS-SYNTH-001`, and one whose only covering view is `eventual` gains an
  absent-subject send. A `when_related:` guard reading a member of a struct input sent as a captured
  instance now picks its related row under the subject's real key, so that scenario's bytes change
  (from the release notes). A model with an immediate view over the guarded fields keeps its suite
  byte for byte. Authored scenarios that state `caller:` need `type: ess-scenario/5`.
- Regeneration that rewrites `.ess-output/state.json` to a newer record format (such as
  `ess-output-state/3`) breaks every older `ess` that reads it: move a CI job's `ess` pin in the
  same change that commits the regenerated record. A refusal that an owned file differs from its
  `.ess-output` record names the re-enroll route; `ess:specifying` walks it.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check: `ess:specifying`, `ess:retrofitting` or
  `ess:testing-conformance`.
