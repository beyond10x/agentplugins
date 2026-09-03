---
title: ESS Schema
---

# `ess-schema`

Use this plugin when an agent works with an Executable System Specification or a supported
projection.

Its `ess-schema` skill guides the agent to:

- validate before compiling or projecting;
- preserve ordered, deterministic output;
- use typed ESS structures instead of arbitrary property bags;
- report unresolved references and unsupported constructs explicitly;
- distinguish semantic round trips from source-text reproduction.

The plugin does not import credentials, contact a live system, or claim universal reversibility.
Concrete adapters remain responsible for declaring their supported kinds and directions.

For the model and command reference, use the [ESS documentation](https://beyond10x.github.io/ess/).
