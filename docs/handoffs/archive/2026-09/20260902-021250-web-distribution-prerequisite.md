---
title: Plan the compiled web distribution prerequisite
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: planning
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-021250-web-distribution-prerequisite.md
base_required: pushed-main
tags: [coordination, handoff, planning, packaging, release]
---

## Goal

Produce one evidence-backed decision packet for the mandatory compiled web
distribution prerequisite to Poodle `0.3.0`. The operator has decided that raw
TypeScript/Svelte source is not an acceptable release artifact. Do not reopen
that decision and do not ask the operator questions in this lane.

## Authority

- Read `AGENTS.md`, `docs/triage/20260901-233708-holistic-posture-assessment.md`,
  `docs/triage/20260901-230400-history-release-adoption-decision.md`, and
  `docs/roadmaps/g16/054-historycenter-v030-release-candidate.md`.
- Inspect current core, Svelte, and React package manifests, export maps,
  workspace build/test selectors, CSS side effects, `marked` use, and packed
  consumer proofs.
- The operator already requires compiled JavaScript plus declarations for
  `0.3.0`. React remains source-only and unpublished until it has a named
  consumer. The release package set remains core plus Svelte.
- This is planning only. Do not change package manifests, build scripts,
  exports, workflows, release files, source, generated output, or version
  numbers.

## Output

Create exactly one new triage packet under `docs/triage/`. It must settle or
recommend, with repo evidence:

- build tool and deterministic output layout;
- declaration and Svelte type emission;
- package export-map shape and consumer module resolution;
- CSS delivery and exact `sideEffects` policy;
- isolation of `marked` from consumers that do not use markdown;
- package contents, source maps, notices, provenance, and clean-tree rules;
- positive and expected-failure installed-tarball proofs with no workspace
  aliases;
- interaction with `g16.054`, immutable `0.3.0`, rollback, and stop conditions;
- the smallest serial implementation-card decomposition.

Treat recommendations as delegate proposals pending orchestrator/operator
review. Do not silently select a breaking public export shape if current
consumer resolution leaves a genuine choice; record the choice and preferred
answer in the packet without pausing for interactive input.

## Validation And Completion

Run `effigy docs:lint` and `git diff --check origin/main...HEAD`. Keep the PR to
the single packet. Push and open one PR against current `main`; report URL and
exact head. Do not merge or dispatch implementation.
