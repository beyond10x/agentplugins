---
format: aep.planning-md/3
id: task:guard-bash-hook
kind: task
status: implemented
title: 'b10x guard bash: a PreToolUse hook that refuses commands the operator rules forbid'
summary: b10x guard bash reads one PreToolUse event and refuses six configured command classes with the replacement named
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-28T06:16:00Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-28T06:16:00Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-28T19:02:13Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1}}}
---
## Context

Operators keep written rules for agent sessions that a model sometimes breaks: GitHub writes made
with a personal `gh` login instead of the organization bot, scratch files under `/tmp` past a
per-user quota, `read -t` on `/dev/zero` (zsh ignores the timeout and dumps core), `pgrep -f` wait
loops that match the waiting shell, heredocs into stdin readers that hung a session for 22 hours,
and `git commit -m` messages whose backticks run as commands. A text rule is advice; a PreToolUse
hook is enforcement.

## Acceptance

`b10x guard bash` reads one Claude Code PreToolUse event on stdin, exits 0 to allow and 2 to refuse
with one stderr line naming the replacement command, and fails open (exit 0) on unreadable input, a
non-Bash tool, an unreadable config or an internal error. The pure `guard::verdict` is
fixture-tested with refuse and allow cases for each of the six groups (`gh_writes`, `tmp_writes`,
`read_dev_zero`, `pgrep_loops`, `heredoc_stdin`, `commit_message_flag`). Groups are switched in
`~/.config/b10x/guard.toml`; with no file only the three hang and crash guards are on. The `b10x`
plugin registers it as a Claude Code `PreToolUse` hook on `Bash`. `task check` passes and the pull
request's checks are green.

## Scope

crates/b10x/src/guard.rs, crates/b10x/src/main.rs, crates/b10x/tests/fixtures/guard.toml,
crates/b10x/Cargo.toml, Cargo.lock, plugins/b10x/hooks/hooks.json, website/docs/plugins/b10x.md,
CHANGELOG.md.
