---
format: aep.planning-md/3
id: approval-record:d397-waves-from-accepted-stories
kind: approval-record
status: draft
title: 'Operator decision: predefined waves of accepted stories, findings to the backlog, release 0.26.0'
relations:
- decides: story:waves-from-accepted-stories
revision: 1
---
## Decision

The operator decided the wave model for the `aep:implementing` skill on 2026-11-02, relayed by the
coordinating session under decision D397, and asked for a release of this repository in the same
change, with ESS, AEP and Worktree references moved to their newest releases.

Operator, verbatim: "You are the one defining the waves up front. They must be defined up front, and
stuff from findings isn't part of a wave yet, and it won't become part of a wave until all waves
which were predefined are solved."

Operator, verbatim: "So new findings should land basically in a backlog, yeah, and the wave, so to
say, is a sprint, yeah, a mini sprint. And only what's basically approved is even considered for a
wave. And new findings ... which aren't caused by new fixes and new implementations, they are
present before. So there is no point in them now taking precedence over what we have planned for
the waves."

Operator, verbatim: "Of course, the AEP thing is selecting the waves. Yeah, it's doing that by
measuring the dependencies, but it would only consider what's accepted, okay? And new findings, they
are not by default accepted. At least they must not. That's the whole point."

Operator, verbatim: "make sure the agents plugin is fixed, a new version is cut for it. While you're
doing that, also make sure it references the most recent versions for ESS, AEP, worktree, and has an
up-to-date documentation. Then cut the release for this plugin".

Decider: operator
