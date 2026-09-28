---
format: aep.planning-md/3
id: dependency-blocker:metaharness-links-aep-0-55
kind: dependency-blocker
status: open
title: metaharness links aep 0.55.0, so no eval case can be recorded beside aep 0.64.0
relations:
- blocks: story:plugin-eval-cases
revision: 2
---
# Blocker: metaharness links aep 0.55.0

aep 0.64.0 moved live evaluation to `metaharness aep drive eval run`. metaharness 0.8.0 and its
`main` (13a8378) link aep at 28abe09 (0.55.0) and refuse every case with EVAL-RUN-017 when the `aep`
on the child's PATH is 0.64.0 (observed 2026-09-28 on the four plan-critic cases). Filed as
beyond10x/metaharness#10. Cleared when a metaharness release links aep 0.64.0 or newer.

Ready for then: `fixtures/library-reservations-drafted` (the AEP tutorial's store before its
critics ran) is the working tree for the four plan-critic cases, whose tasks now name
`epic:book-reservations`.
