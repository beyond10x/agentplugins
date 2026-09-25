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

What each plugin ships:

<!-- plugin-tree:start -->
- [`ess`](plugins/ess/) · [docs](website/docs/plugins/ess.md)
  - skills: [`init`](plugins/ess/skills/init/SKILL.md) · [`upgrade`](plugins/ess/skills/upgrade/SKILL.md) · [`retrofitting`](plugins/ess/skills/retrofitting/SKILL.md) · [`specifying`](plugins/ess/skills/specifying/SKILL.md) · [`testing-conformance`](plugins/ess/skills/testing-conformance/SKILL.md)
  - agents: [`author`](plugins/ess/agents/author.md) · [`conformance`](plugins/ess/agents/conformance.md) · [`retrofitter`](plugins/ess/agents/retrofitter.md)
- [`aep`](plugins/aep/) · [docs](website/docs/plugins/aep.md)
  - skills: [`init`](plugins/aep/skills/init/SKILL.md) · [`upgrade`](plugins/aep/skills/upgrade/SKILL.md) · [`implementing`](plugins/aep/skills/implementing/SKILL.md) · [`migrating`](plugins/aep/skills/migrating/SKILL.md) · [`planning`](plugins/aep/skills/planning/SKILL.md)
  - agents: [`adversary`](plugins/aep/agents/adversary.md) · [`decomposer`](plugins/aep/agents/decomposer.md) · [`implementor`](plugins/aep/agents/implementor.md) · [`plan-critic-acceptance`](plugins/aep/agents/plan-critic-acceptance.md) · [`plan-critic-design`](plugins/aep/agents/plan-critic-design.md) · [`plan-critic-parallel-safety`](plugins/aep/agents/plan-critic-parallel-safety.md) · [`plan-critic-scope`](plugins/aep/agents/plan-critic-scope.md) · [`plan-reviewer`](plugins/aep/agents/plan-reviewer.md) · [`reverse-engineer`](plugins/aep/agents/reverse-engineer.md) · [`security-reviewer`](plugins/aep/agents/security-reviewer.md) · [`story-scoper`](plugins/aep/agents/story-scoper.md)
- [`worktree`](plugins/worktree/) · [docs](website/docs/plugins/worktree.md)
  - skills: [`init`](plugins/worktree/skills/init/SKILL.md) · [`upgrade`](plugins/worktree/skills/upgrade/SKILL.md) · [`managing-worktrees`](plugins/worktree/skills/managing-worktrees/SKILL.md)
- [`connectors`](plugins/connectors/) · [docs](website/docs/plugins/connectors.md)
  - skills: [`init`](plugins/connectors/skills/init/SKILL.md) · [`upgrade`](plugins/connectors/skills/upgrade/SKILL.md) · [`integrating`](plugins/connectors/skills/integrating/SKILL.md)
- [`b10x`](plugins/b10x/) · [docs](website/docs/plugins/b10x.md)
  - skills: [`init`](plugins/b10x/skills/init/SKILL.md) · [`upgrade`](plugins/b10x/skills/upgrade/SKILL.md) · [`authoring-plugins`](plugins/b10x/skills/authoring-plugins/SKILL.md) · [`routing`](plugins/b10x/skills/routing/SKILL.md)
<!-- plugin-tree:end -->

Most plugins drive a CLI of the same name. To have an agent install plugins and matching CLIs, and
migrate older installs, tell it: *"Set up Beyond10x: follow
https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md"*.

More: [install guide](website/docs/install.md) · [evals](evals/README.md) ·
[changelog](CHANGELOG.md) · [contributing](AGENTS.md)

<!-- b10x-docs:start -->
## Documentation

[Agent Plugins documentation](https://beyond10x.github.io/docs/agentplugins/) · [Start](https://beyond10x.github.io/) · [Ecosystem](https://beyond10x.github.io/ecosystem/) · [Impact](https://beyond10x.github.io/changes/) · [Releases](https://beyond10x.github.io/releases/)
<!-- b10x-docs:end -->
