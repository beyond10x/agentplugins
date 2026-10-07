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

No `b10x`? Follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md first.

## Next

- Continue the work that prompted the check: `ess:specifying`, `ess:retrofitting` or
  `ess:testing-conformance`.
