---
title: g18.007 ordinary changelog maintenance scope
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-10
updated: 2026-09-10
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Operator selected 'Structural lane' to resolve g18.005 PR #238's deterministic ordinary-scope CI blocker; preserve the existing task, PR, worker, workspace, and review."
queue:
  capability: complex
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.007, changelog, certification, scope]
---

## What This Thread Was Doing

Execute [`g18.007`](../roadmaps/g18/007-ordinary-changelog-maintenance-scope.md)
from the exact pushed planning commit. Add a fail-closed ordinary-CI lane for
semantic-preserving changelog syntax maintenance.

## Why It Matters

g18.005 correctly normalized the changelog, but PR #238 cannot pass because
ordinary pack-install CI treats every changelog edit as release preparation.
Merging red or globally allowing changelog changes would weaken certification.
A content-aware maintenance rule lets the valid repair land while retaining
the guard against real release mutation.

## Current State

Retained Queue task `b02f77de-5ab0-4123-8229-0001a940b9f6`, PR #238, worker
`8aebadd1-85ab-4dbe-90fc-ed4080b91d41`, workspace, branch, exact reviewed head
`40f9e0a3d44f485fd24b4aa6ba55dff2bc759a51`, and accepted review all remain
intact. CI run `34531894294` passed 3819 tests and failed only on the explicit
ordinary `CHANGELOG.md` release-surface guard. Do not mutate that lane.

## Boundaries

Follow the task exactly. Own only the package-install scope classifier, focused
tests/falsification harness, narrowly required contract docs, and one log. Do
not edit `CHANGELOG.md`, PR #238, workflows, package versions, candidate
release content, product code, tags, registry state, or Desktop. Never encode
the blocked PR, task, branch, commit, or historical releases as an exception.

The rule must admit only docs-only syntax normalization with identical release
semantics and an empty Unreleased section. Ambiguity fails closed. Real release
entries or changes remain forbidden in ordinary mode.

## Important Context

Read the task, package-install README, `scope.ts`, its tests, web-preview
falsification laws, working rules, and release-channel spec before editing.
Preserve strict and historical candidate behavior. Existing ordinary guards
for workflows, versions, publication, registries, and mixed changes remain
contractual negative cases.

## Suggested Next Move

Plant the exact generic before/after normalization fixture and adversarial
mutations first. Define the semantic inventory and bounded docs-only range,
then implement the smallest content-derived classifier that makes only the
normalization case green.

## Completion Protocol

Run focused scope tests, web-preview falsification coverage, relevant headless
web CI selectors, docs QA, and `git diff --check`. Prove the accepted
normalization passes and every semantic/mixed/release mutation stays red.
Commit, push, open one non-draft PR, and report `ready_for_review` with its
number, exact head, classifier boundary, negative proofs, and explicit next
action: retry validation on retained g18.005 after this task merges. Leave the
tracked tree clean. Never merge, revise PR #238, tag, publish, or edit Desktop.
