# Beyond10x Agent Plugins

Agent plugins for Claude Code and Codex, from one marketplace: `b10x`.

Add the marketplace once — Claude Code: `/plugin marketplace add beyond10x/agentplugins` ·
Codex: `codex plugin marketplace add beyond10x/agentplugins` — then install what you need:

| plugin | for | Claude Code | Codex |
|---|---|---|---|
| [`ess`](website/docs/plugins/ess.md) | Executable System Specifications | `/plugin install ess@b10x` | `codex plugin add ess@b10x` |
| [`aep`](website/docs/plugins/aep.md) | governed planning and delivery | `/plugin install aep@b10x` | `codex plugin add aep@b10x` |
| [`worktree`](website/docs/plugins/worktree.md) | isolated Git worktrees | `/plugin install worktree@b10x` | `codex plugin add worktree@b10x` |
| [`connectors`](website/docs/plugins/connectors.md) | integrations | `/plugin install connectors@b10x` | `codex plugin add connectors@b10x` |
| [`b10x`](website/docs/plugins/b10x.md) | setup, upgrades, routing | `/plugin install b10x@b10x` | `codex plugin add b10x@b10x` |

Most plugins drive a CLI of the same name. To have an agent install plugins and matching CLIs, and
migrate older installs, tell it: *"Set up Beyond10x: follow
https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md"*.

More: [install guide](website/docs/install.md) · [evals](evals/README.md) ·
[changelog](CHANGELOG.md) · [contributing](AGENTS.md)

<!-- b10x-docs:start -->
## Documentation

[Agent Plugins documentation](https://beyond10x.github.io/docs/agentplugins/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
