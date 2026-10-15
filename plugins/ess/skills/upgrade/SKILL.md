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
  (`ess-composition/4`) and an `ess-http/1` binding. Move every reader (a CI job's `ess`, the
  runner crates or packages) in the same change that commits them.
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
