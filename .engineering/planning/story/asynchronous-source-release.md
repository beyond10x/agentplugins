---
format: aep.planning-md/1
id: story:asynchronous-source-release
kind: story
status: implemented
title: Publish source releases independently of website rendering
summary: Keep provenance and repository checks at publication; let the existing documentation pipeline converge asynchronously.
refs:
- provider: atlas
  reference: story:organization-release-boundaries
scope:
- confidence: cited
  path: .github/workflows/release.yml
- confidence: cited
  path: AGENTS.md
revision: 6
---
The release workflow made public-site rendering a prerequisite for publishing its source release.

The gate continues to verify the annotated tag, version agreement and the complete offline repository checks. Publication depends on that gate. The existing independent Pages workflow retains site validation, and Atlas publishes documentation asynchronously. No release tag, plugin version, artifact or deployment changes in this rollout.

Acceptance: source publication has no documentation-build prerequisite; provenance and source checks still gate publication; standalone agent guidance reports the source and documentation states separately.

Validation: task check passed; actionlint passed; the release DAG has gate as the publication prerequisite and no website dependency. Atlas owns the coordinated organization rollout.
