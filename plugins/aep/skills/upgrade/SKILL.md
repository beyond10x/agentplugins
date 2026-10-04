---
name: upgrade
description: Check whether the AEP plugin and the `aep` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether AEP is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `aep`, or when a `aep` command behaves differently from what a AEP skill describes or names the planning store's `aep.project` version.
---

# Upgrade AEP

Before acting, complete the [repository preflight](../planning/references/repository-preflight.md)
or reuse the coordinator’s matching completed record and user decision.

Read the [repository preflight](../planning/references/repository-preflight.md) in full. It
combines release discovery with selected-store inspection before deciding whether an upgrade is
needed, and reuses existing authorization instead of asking again.

Apply only the accepted installation and migration changes. Existing stores use the preflight’s
backend-specific compatibility reference; the active CLI remains installed during a legacy bridge.
A no-action installation plan is not a completed repository check.

A new plugin version loads in a new session; `b10x skill aep:<skill>` prints the installed text.
When `b10x` is missing, follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md.

Finish with the preflight’s verification evidence, then continue the original task.
