---
name: beyond10x
description: Navigate the Beyond10x engineering ecosystem and route work to the smallest matching plugin or public resource. Use when the user asks what Beyond10x provides, which plugin to install or invoke, where the AEP, ADP, ESS, Entity Runtime, or agent-plugin documentation lives, or when a request spans or is ambiguous between the Beyond10x plugins.
---

# Beyond10x guide

Route the request; do not reproduce a specialist plugin's full workflow.

## Select the smallest surface

1. Identify the user's immediate decision or outcome.
2. Select one plugin from the routing table. Select several only when the task genuinely crosses
   their boundaries.
3. If the selected plugin is installed, use its skill or agent. If it is unavailable, name the
   plugin, link its reference page, and explain that its specialist instructions are not loaded.
4. For a general ecosystem question, answer from
   [references/resources.md](references/resources.md) without selecting a specialist.

| Request | Route |
|---|---|
| Choose a plugin, understand the ecosystem, or find public documentation | `beyond10x` |
| Create, update, review, or port an installable plugin | `plugin-creator` in this plugin |
| Plan or decompose work, review a plan, or reverse-engineer a backlog | `aep-planning` |
| Scope and deliver accepted development work through a reviewed wave | `adp` |
| Validate an ESS model or guide deterministic schema or OpenAPI projection | `ess-schema` |

## Preserve boundaries

- Do not treat this plugin as a substitute for the routed specialist.
- Do not install another plugin, mutate a marketplace, or contact an external service unless the
  user asked for that action and the host grants it.
- Do not infer that ADP work is ready merely because a planning request exists.
- Do not turn schema guidance into authority to apply infrastructure.
- State which plugin owns the next step whenever more than one could plausibly apply.

When the request remains ambiguous after inspecting available context, give the two most likely
routes and ask one short question that distinguishes them.
