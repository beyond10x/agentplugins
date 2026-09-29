---
name: routing
description: Navigate the Beyond10x engineering ecosystem and route work to the smallest matching plugin or public resource. Use when the user asks what Beyond10x provides, which plugin fits a task, where the AEP, ESS, Entity Runtime, or agent-plugin documentation lives, or when a request spans or is ambiguous between the Beyond10x plugins.
---

# Beyond10x guide

Route the request; do not reproduce a specialist plugin's full workflow.

## Select the smallest surface

1. Identify the user's immediate decision or outcome.
2. For non-trivial implementation, cross-repository work, or release/deployment changes in a
   Beyond10x repository, route through `aep:planning` first so the owning artifact, relations, and
   scope exist before implementation continues.
3. Select one plugin from the routing table. Select several only when the task genuinely crosses
   their boundaries.
4. If the selected plugin is installed, use its skill or agent. If it is unavailable, name the
   plugin, link its reference page, and explain that its specialist instructions are not loaded.
5. For a general ecosystem question, answer from
   [references/resources.md](references/resources.md) without selecting a specialist.

| Request | Route |
|---|---|
| Install or repair the Beyond10x plugins and their binaries | `b10x:init` |
| Check whether the Beyond10x plugins and CLIs are current, or upgrade them | `b10x:upgrade` |
| Set up one product and take its first step | `aep:init`, `ess:init`, `worktree:init`, `connectors:init` |
| Check or upgrade one product's plugin and CLI | `aep:upgrade`, `ess:upgrade`, `worktree:upgrade`, `connectors:upgrade` |
| Choose a plugin, understand the ecosystem, or find public documentation | `b10x:routing` (this skill) |
| Create, update, review, or port an installable plugin | `b10x:authoring-plugins` |
| Plan or decompose work, review a plan, or reverse-engineer a backlog | `aep:planning` |
| Move an existing backlog into the AEP store without losing its sources | `aep:migrating` |
| Scope and deliver accepted development work through a reviewed wave | `aep:implementing` |
| Diagnose a failing, flaky or slow behaviour through a red-capable loop | `aep:diagnosing` |
| Investigate a production incident, an outage, an onset or a ship state from cited evidence | `aep:investigating` |
| Specify a system or API | `ess:specifying` |
| Derive a specification for an existing system | `ess:retrofitting` |
| Run or raise a conformance suite | `ess:testing-conformance` |
| Harden a specification once its suite is green | `ess:hardening` |
| Create, inspect, finish, or safely clean Git worktrees | `worktree:managing-worktrees` |
| Set up providers, inspect Connector readiness, or invoke configured integrations through the CLI | `connectors:integrating` |

Five entry points are commands: only the operator starts them, and a model cannot invoke them.
When a request matches one, route to the activity it hands off to and name the command to the operator.

| Command | Hands off to |
|---|---|
| `/aep:wave [story-id…]` (`aep:wave`) | `aep:implementing`, wave mode |
| `/aep:drive <story-id>` (`aep:drive`) | `aep:implementing`, drive mode |
| `/aep:review-plan [artifact-id…]` (`aep:review-plan`) | `aep:planning`, `plan-reviewer` role |
| `/aep:decompose <epic-id>` (`aep:decompose`) | `aep:planning`, `decomposer` role and the critic panel |
| `/worktree:cleanup` (`worktree:cleanup`) | `worktree:managing-worktrees` |

## Preserve boundaries

- Do not treat this plugin as a substitute for the routed specialist.
- Do not install another plugin, mutate a marketplace, or contact an external service unless the
  user asked for that action and the host grants it.
- Do not infer that development work is ready merely because a planning request exists.
- Do not turn schema guidance into authority to apply infrastructure.
- State which plugin owns the next step whenever more than one could plausibly apply.

When the request remains ambiguous after inspecting available context, give the two most likely
routes and ask one short question that distinguishes them.
