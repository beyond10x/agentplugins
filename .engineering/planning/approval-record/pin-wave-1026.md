---
format: aep.planning-md/3
id: approval-record:pin-wave-1026
kind: approval-record
status: draft
title: Pin wave approved by the directing session
relations:
- decides: story:track-releases-1026
revision: 2
---
## Who approved

Session `b10x-chef-orchestrator`, in its messages to this session on 2026-10-26, under the grant in the
operator's workspace rules (a message from that session naming a class-C item counts as the operator's
approval; dispatch-queue row 17, class C dependency upgrade). The item: "move the aep pin from 0.76.0 to
0.78.0 ... through agentplugins' own process as its AGENTS.md defines it (`agentplugins-check tools`, the
isolated trials, `verified.json`), then release the plugins per AGENTS.md if a pin move is a release
there ... if the ess CLI or connectors CLI pins the plugins carry are behind the newest releases ... move
those too in the same wave, same process."

Asked whether Metaharness 0.19.0 joins, it answered: "A: metaharness verified.json 0.14.0 -> 0.19.0 joins
this wave, with eval.yml metaharness and the eval AEP pair moved together. Class-C dependency upgrade under
the grant ... Record this in your store (approval-record), then continue to the PR."

## What was decided

One pull request through `b10x-gates` carrying the story, merged by the bot when its checks are green,
then a release through `AGENTS.md` Publishing.
