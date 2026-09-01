# Codex and Claude Code plugin compatibility

This reference records the conservative authoring contract. Recheck the current product
documentation before changing it.

| Capability | Shared source | Codex | Claude Code | Authoring rule |
|---|---|---|---|---|
| Skill | `skills/<name>/SKILL.md` | Native | Native | Use as the canonical workflow |
| Skill references and assets | Below the skill directory | Native | Native | Keep paths relative and self-contained |
| Command-like workflow | Shared skill | Invoke/select the skill | Skill receives a slash shortcut | Prefer a skill; do not create a flat command for new work |
| Specialist-agent workflow | Shared skill | Execute directly or delegate when supported | Execute directly or add an `agents/` wrapper | Put the complete behavior in the skill |
| Claude command wrapper | `commands/<name>.md` | Not a plugin component | Native, legacy-compatible | Add only to preserve a required Claude invocation |
| Claude agent wrapper | `agents/<name>.md` | Not a plugin component | Native | Keep it thin and backed by a shared skill |
| Plugin identity | Two manifests | `.codex-plugin/plugin.json` | `.claude-plugin/plugin.json` | Keep names, versions, and descriptions aligned |
| MCP or hooks | Host-specific configuration | Support varies by surface | Support varies by component | Verify separately; never make the shared skill depend on an unsupported hook |

Primary documentation:

- [OpenAI: package a plugin](https://developers.openai.com/plugins/build/plugins)
- [OpenAI: build plugin skills](https://developers.openai.com/plugins/build/skills)
- [OpenAI: adapt a Claude Code plugin](https://developers.openai.com/plugins/guides/submit-claude-plugin)
- [Claude Code: create plugins](https://code.claude.com/docs/en/plugins)
- [Claude Code: plugin reference](https://code.claude.com/docs/en/plugins-reference)

The critical asymmetry is deliberate: current OpenAI guidance says to convert reusable Claude
`commands/` and `agents/` behavior into skills. Therefore portability means one shared capability
implemented as a skill, not identical native wrappers in both hosts.
