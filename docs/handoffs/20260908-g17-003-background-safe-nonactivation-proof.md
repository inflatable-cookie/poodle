---
title: g17.003 background-safe non-activation proof worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Operator approved the durable background-safe capture correction on 2026-09-08 after confirming Poodle Lab must permit normal foreground app use during background runs."
tags: [coordination, handoff, worker, g17, g17.003, capture, foreground]
---

## What This Thread Was Doing

Poodle's native `poodle-window-capture` transport supplies the GPUI leg for
Poodle Lab's cohort comparison. The Lab run is blocked after its current
foreground proof rejected ordinary operator app switching.

## Why It Matters

The transport is meant to be non-activating and usable in the background. Its
current proof instead requires the whole desktop foreground to remain frozen,
so normal operator work invalidates long capture batches despite the capture
window remaining unfocused, non-key, and inactive.

## Current State

Execute `docs/roadmaps/g17/003-background-safe-nonactivation-proof.md` from the
exact pushed planning commit on current `origin/main`. The ready frontier is
revision 25 of `docs/roadmaps/dispatch.md`. Poodle Lab `g01.006` remains in its
existing queue-managed workspace; do not touch or duplicate that task.

The shared native monitor is
`packages/gpui/preview/src/bin/window_capture/transport.rs`. It records a
baseline identity and the set of observed frontmost identities, then marks any
identity other than the baseline as `Changed`. The capture window already has
independent focus/key/application-active evidence. No windowed proof is needed
for this implementation batch.

## Boundaries

Scope is g17.003 only. Sample frontmost process identity plus PID and compare
against the capture process's own PID. Permit unrelated operator transitions
while retaining them as evidence. Fail closed if the capture process is the
baseline or appears later, if required readings fail, if sampling is too short,
or if the independent window proof is focused/key/active.

Keep every capture mode on the shared transport. Update the smallest complete
receipt, parser, diagnostic, test, and documentation surface needed to make the
new claim auditable. If the receipt shape changes, migrate it directly before
v1; do not add aliases, compatibility shims, or fallbacks. Do not change
components, scenarios, fixtures, pixels, capture ordering, scale, permission,
repeat, process-bounding, or publication laws. Do not edit workflows or run
release mutations.

## Important Context

Read the card, the full shared transport, its focused Rust tests, every receipt
mode that serializes `ForegroundEvidence`, the window-capture diagnostic and
smoke scripts, and the directly affected roadmap claims. Use Effigy task
inventory and test planning to select narrow checks. Produce a named Lab
adoption request that identifies the exact Poodle commit and any receipt
contract changes; Lab must later update its own external watcher and pin.

## Suggested Next Move

Plant identity/PID observations around the pure verdict function first. Prove
editor → browser → editor is publishable when the capture PID is never
frontmost, while one capture-PID sample fails. Then thread the evidence through
the live monitor and all receipt modes without opening a window.

## Completion Protocol

Run the focused window-capture tests, relevant headless Rust checks, docs checks,
`git diff --check`, and one windowless capture-binary build. Record an execution
log and Lab adoption request. Commit, push, open one PR, and report
`ready_for_review` with its number and exact head. Leave the tracked tree clean.
Do not merge or invoke any windowed selector.
