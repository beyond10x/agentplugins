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
  (`ess-conformance/46`, `/47`). Move every reader (a CI job's `ess`, the runner crates or
  packages) in the same change that commits them.
- Regeneration that rewrites `.ess-output/state.json` to a newer record format (such as
  `ess-output-state/3`) breaks every older `ess` that reads it: move a CI job's `ess` pin in the
  same change that commits the regenerated record. A refusal that an owned file differs from its
  `.ess-output` record names the re-enroll route; `ess:specifying` walks it.

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check: `ess:specifying`, `ess:retrofitting` or
  `ess:testing-conformance`.
