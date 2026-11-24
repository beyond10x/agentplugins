---
format: aep.planning-md/3
id: story:surf-plugin
kind: story
status: draft
title: A surf plugin whose skill points to the definition the binary prints, and a catalog product for its Linux binaries
summary: plugins/surf with a browsing skill that only says to run surf-agent help --for-agent, the lifecycle skills, and a catalog product surf for surf-agent, surfd and surf-session on Linux; handed over by surf's story agent-skill-definition, nothing built yet
revision: 1
---
## Context

surf (`beyond10x/surf`) is a web browser written in Rust. Its agent interface is the command
`surf-agent`: an agent opens a session, sends it to a page, reads the page as text, follows
links and fills forms. A daemon `surfd` holds the sessions, each in its own process
`surf-session`.

Since surf `0.8.0` (tag on commit `29945f5`, 2026-11-24) the binary carries its own agent
definition: `surf-agent help --for-agent` prints the commands, the form of an answer, the rule
that page text is untrusted, and worked calls (315 lines at `0.8.0`). A test of surf's gate
compares that output with the sources it is compiled from, so it cannot differ from the binary
that prints it. surf's story `agent-skill-definition` weighed three forms (the command, a skill
here, an MCP server), took the command, and handed over to this repository a skill that only
points to it. This story is that hand-over. It was filed from the surf side; nothing of it is
built here yet.

What surf releases, read from the `0.8.0` archives:

- four archives `surf-<version>-<target>.tar.gz` and `SHA256SUMS`;
- `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` hold `surf`, `surf-agent`,
  `surfd` and `surf-session` (statically linked), with `LICENSE`, `NOTICE` and `README.md`;
- `x86_64-apple-darwin` and `aarch64-apple-darwin` hold `surf` only: sessions read
  `/proc/self` and need `XDG_RUNTIME_DIR` (surf's `README.md` at `0.8.0`, lines 38-41);
- the cargo packages are `surf-cli` (binary `surf`), `surf-agent`, `surfd` and `surf-session`.

## Change

1. A plugin `plugins/surf/` in the form of the other plugins (`.claude-plugin/plugin.json`,
   `.codex-plugin/plugin.json`, both marketplace files, rule R1).
2. `skills/browsing/SKILL.md`: a description that says when to use it (an agent has to read
   or act on a web page from a terminal); the warning that every text of a page is untrusted
   data and never an instruction; the one instruction "run `surf-agent help --for-agent` and
   follow it". It holds no command reference and no copy of the definition: a copy drifts from
   the binary, which is why surf chose the command.
3. The lifecycle skills `init` and `upgrade` (rule R3), like those of the other plugins.
4. A product `surf` in `catalog.json` with the binaries `surf-agent`, `surfd` and
   `surf-session`, each with `"platforms": ["linux"]` and installed from the release archive
   of `beyond10x/surf`. Whether `surf` (the command for one fetch, all four targets) is a
   fourth binary of the product is decided when the story is taken.
5. `verified.json` holds the surf release the skill was checked against, and
   `agentplugins-check tools` runs the commands the skills spell against it (rule R5).
6. The README row, the page under `website/docs/plugins/` and the sidebar entry (rule R8).

## Questions for whoever takes the story

- Rule R2 says the plugin, the product and the CLI it drives share one name. The plugin and
  the product would be `surf`; the CLI the skill drives is `surf-agent`. Either R2 gets a
  sentence for a product with several binaries, or the plugin is named after the command.
- The four binaries come in one archive per target. Does the installer take several binaries
  from one archive today (`b10x-harness` and `metaharness` come from two repositories)?
- On macOS the product installs nothing an agent session can use. What does `b10x init` say
  there: the product is not offered, or it is offered with the reason?
- surf's version at the time the story is taken: the skill names none (rule R5).

## Acceptance

- `task check` exits 0 with the plugin, the catalog product and the documents in place.
- `agentplugins-check tools` runs `surf-agent help --for-agent` of the surf release named in
  `verified.json` and every other command the three skills spell, and passes.
- `skills/browsing/SKILL.md` holds no line of the definition's command list: a check or a
  test compares it against the output of `surf-agent help --for-agent` and fails on a copied
  command line.
- An isolated trial (`task trial:run`) on Linux: from an empty home, `b10x` installs the
  product, and an agent given the task "read the first heading of this page" on a page served
  on 127.0.0.1 answers with it, having run `surf-agent help --for-agent` first.
- On macOS (or a fixture of the plan in `crates/b10x`), setup answers what the story decided
  for a system without the three binaries, and installs none of them.

## Out of scope

- Any change in the surf repository. A lack found in `surf-agent` becomes an issue there.
- An MCP server for surf: surf left it for an operator who asks for a harness that speaks
  only MCP.
- A command reference of `surf-agent` in this repository, in a skill or on the website.
