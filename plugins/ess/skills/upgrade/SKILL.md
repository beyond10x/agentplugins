---
name: upgrade
description: Check whether the ESS plugin and the `ess` CLI are current, and upgrade them with the user's confirmation. Use when the user asks whether ESS is up to date or to upgrade or update it, when a session-start line starting with `b10x:` names `ess`, or when an `ess` command behaves differently from what an ESS skill describes.
---

# Upgrade ESS

Before acting, complete the [repository preflight](../specifying/references/repository-preflight.md)
or reuse the coordinator’s matching completed record and user decision.

Read the [repository preflight](../specifying/references/repository-preflight.md) in full. It
compares installed and effective releases, project pins, source families and generated formats
before declaring anything current, and combines applicable upgrades in one session offer.

Apply only the accepted installation, source semantics, pin and regeneration changes. A source
that still validates may still have an applicable format upgrade; a no-action installation plan
is not a completed repository check. ESS has no general migration command.

A new plugin version loads in a new session; `b10x skill ess:<skill>` prints the installed text.
When `b10x` is missing, follow https://github.com/beyond10x/agentplugins/releases/latest/download/SETUP.md.

Finish with the preflight’s effective-toolchain, validation, regeneration and conformance evidence,
then continue the original task.
