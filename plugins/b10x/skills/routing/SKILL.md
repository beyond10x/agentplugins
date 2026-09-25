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
| Install, upgrade or repair the Beyond10x plugins and their binaries | `b10x:init` |
| Choose a plugin, understand the ecosystem, or find public documentation | `b10x:routing` (this skill) |
| Create, update, review, or port an installable plugin | `b10x:authoring-plugins` |
| Plan or decompose work, review a plan, or reverse-engineer a backlog | `aep:planning` |
| Scope and deliver accepted development work through a reviewed wave | `aep:implementing` |
| Specify a system or API | `ess:specifying` |
| Derive a specification for an existing system | `ess:retrofitting` |
| Run or raise a conformance suite | `ess:testing-conformance` |
| Create, inspect, finish, or safely clean Git worktrees | `worktree:managing-worktrees` |
| Set up providers, inspect Connector readiness, or invoke configured integrations through the CLI | `connectors:integrating` |

## Preserve boundaries

- Do not treat this plugin as a substitute for the routed specialist.
- Do not install another plugin, mutate a marketplace, or contact an external service unless the
  user asked for that action and the host grants it.
- Do not infer that development work is ready merely because a planning request exists.
- Do not turn schema guidance into authority to apply infrastructure.
- State which plugin owns the next step whenever more than one could plausibly apply.

When the request remains ambiguous after inspecting available context, give the two most likely
routes and ask one short question that distinguishes them.
