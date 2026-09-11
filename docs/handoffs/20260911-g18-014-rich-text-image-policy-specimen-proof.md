---
title: g18.014 rich-text image-policy specimen proof
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-11
updated: 2026-09-11
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom reproduced the inert Image Policy specimen on 2026-09-11 and said Go for it after the exact dead-URL diagnosis and bounded repair proposal."
queue:
  dependsOn:
    - e3a0e8cb-287c-447d-90c3-c226bca3d763
  capability: general
  skipPRReview: false
tags: [coordination, handoff, worker, g18, g18.014, rich-text, image, specimen]
---

## What This Thread Was Doing

Execute [`g18.014`](../roadmaps/g18/014-rich-text-image-policy-specimen-proof.md)
after g18.013 closes. Make the paired Image Policy specimens visibly prove
seeded and host-requested image behavior without external networking.

## Why It Matters

The current specimens are wired to the image feature but use a non-resolving
`x.test` source for both the seeded and inserted image. The DOM can change while
the human-visible result looks completely inert. That cannot serve as release
evidence for an optional feature.

## Current State

Both specimens already pass `features` with `images`, provide `requestImage`,
and control the returned document. Both use the shared dead URL. g18.013 is
actively repairing the same toolbar/specimen surfaces, so this task is serial
behind it. g18.011 is Queue-held; g18.006 and g18.009 remain held.

## Boundaries

Follow g18.014 exactly. Use a deterministic local or self-contained safe raster
fixture, add visible host feedback, and prove one click yields one additional
visible image and one exact controlled callback. Repair the bounded engine path
only if that proof exposes a real defect. Do not add media management, embeds,
public schema changes, external networking, release work, or Desktop edits.

## Important Context

The image source remains host-owned; the specimen callback represents a
consumer media picker. Prove images off/on, seeded load, insertion at retained
selection, remount, cancellation, unsafe-source refusal, Svelte/React parity,
and zero external image requests. A present `<img>` with a failed load is not a
passing visual specimen.

## Suggested Next Move

Rebase after g18.013, plant paired browser assertions against the dead source,
then replace only the fixture and add the host-visible count/document evidence
before considering any engine change.

## Completion Protocol

Open one non-draft PR from the queue-owned branch, prove the exact head with
focused preview/component/browser checks, both preview builds, accessibility
checks, docs QA, and `git diff --check`, then report `ready_for_review`. Never
merge, release g18.011, resume g18.006/g18.009, publish, or mutate Desktop.
