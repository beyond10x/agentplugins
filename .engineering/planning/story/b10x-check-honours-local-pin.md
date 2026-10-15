---
format: aep.planning-md/3
id: story:b10x-check-honours-local-pin
kind: story
status: active
title: b10x check honours a local pin
scope:
- confidence: cited
  path: SETUP.md
- confidence: cited
  path: crates/b10x/src
- confidence: cited
  path: plugins/b10x/skills/init
- confidence: cited
  path: plugins/b10x/skills/upgrade
- confidence: cited
  path: website/docs/install.md
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-14T22:46:40Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-14T22:46:40Z", actor: "human:timo", revision: 3}
---
## Context

The session-start hook (`plugins/b10x/hooks/hooks.json`) runs `b10x check`
(`crates/b10x/src/check.rs`). Whenever a newer release of a tracked CLI exists, its line invites
`/b10x:upgrade`. An operator who holds a CLI back on purpose on one machine (because a newer
release refused existing stores, for example) has no way to say so: every new session reads the
line as an instruction and upgrades, undoing the hold. The only pin today is the per-repository
`b10x.toml`, which is committed and shared and is found only inside that repository.

## Outcome

A user-level pin file holds a CLI back on this machine, in every directory:
`$B10X_PINS` when set, else `$XDG_CONFIG_HOME/b10x/pins.toml`, else `~/.config/b10x/pins.toml`.
Its `[pins]` table takes the same specs as `b10x.toml` (`x.y.z` or `x.y`), as a string or as
`{ version = "…", reason = "…" }`. For the same CLI, the local pin wins over a repository pin.

## Acceptance

- Pinned and equal: with a local pin `aep = { version = "0.71.3", reason = "…" }` and `aep`
  0.71.3 installed while 0.72.0 is the newest release, `b10x check` prints one line saying `aep`
  is held back by a local pin, with the reason, and the line does not contain `/b10x:upgrade`.
- Pinned and different: with the same pin and `aep` 0.72.0 installed, `b10x check` prints one
  line saying the installed version differs from the local pin.
- Unpinned: with no pin file, every line `b10x check` printed before this change is unchanged.
- Malformed: a local pin file that does not parse makes `b10x check` print exactly one warning line
  naming the file and exit 0; `b10x upgrade` refuses with that error instead of planning past it.
- Upgrade: `b10x upgrade` installs nothing for a locally pinned CLI and its finding names the local
  pin and its reason.
- The pin is documented in the `b10x:upgrade` and `b10x:init` skills and in `SETUP.md`.
- `task check` passes.

## Scope

crates/b10x/src/pins.rs, check.rs, resolve.rs, plan.rs, main.rs; plugins/b10x/skills/upgrade,
plugins/b10x/skills/init, SETUP.md, website/docs/install.md, CHANGELOG.md and the release's
version lines.

## Notes

The repository has no ESS specification of the `b10x` CLI (every `system.yaml` here is a fixture,
a trial or a tutorial), so the change is driven by unit tests in the crate.

## Verification

- `cargo test -p b10x` in the wave tree: 77 passed, 0 failed (2026-10-15), covering pinned and equal (held-back line with reason, no `/b10x:upgrade`), pinned and different, unpinned unchanged, malformed file (one warning line with line and column, exit 0; upgrade refuses), dangling symlink, relative `$B10X_PINS`, a name outside the catalog, multi-line reasons, and `b10x pin`/`unpin` naming a local hold.
- An adversary pass wrote 9 tests against the first implementation; 3 failed (multi-line TOML warning, dangling symlink read as no pins, newline in a reason) and 3 more findings followed (pin/unpin silent about a local hold, unknown names silent, relative `$B10X_PINS`); all six are fixed and the adversary tests are kept.
- `cargo clippy -p b10x --all-targets -- -D warnings` and `cargo fmt --all -- --check`: clean.
- Isolated trial `upgrade-seeded`: 11 tool calls, no measure worse than `trials/baseline.json`.
- Known limitation: `b10x pin` rewrites a table-form `b10x.toml` entry as a plain string, dropping its reason.
