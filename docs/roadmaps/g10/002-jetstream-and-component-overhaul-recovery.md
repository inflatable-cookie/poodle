# g10.002 Jetstream And Component Overhaul Recovery

Status: complete
Owner: Poodle core
Depends on: g10.001
Updated: 2026-04-09

## Context

`g10.001` proved the Jetstream target was viable and sequenced a plausible
implementation plan, but the live execution surface has drifted away from the
roadmap entry points.

The front doors still implied `g09` was the active generation, while recent
handoff material and active implementation changes show a broader reality:

- Jetstream feasibility and renderer constraints are still live
- component-overhaul work is happening in Svelte and docs surfaces
- parity/specimen follow-through is still materially active
- the current queue is no longer represented by one bounded milestone

This roadmap is a recovery/control lane. Its job is to freeze the real active
queue from evidence and turn it back into an explicit bounded sequence.

## Goals

- recover the real active `g10` queue from current evidence
- classify active work across:
  - Jetstream renderer/runtime feasibility
  - shared component-overhaul work
  - specimen/parity/verification follow-through
- identify what is actively executing, what needs planning, and what is blocked
- compile the next bounded milestone so the thread can resume from roadmap
  authority instead of handoff-only momentum

## Non-Goals

- reopening `g09`
- treating every dirty implementation file as part of one undifferentiated lane
- silently promoting handoff notes into roadmap authority without review
- claiming repo closeout while `effigy doctor` still reports health-task failure

## Execution Plan

### Batch 2.1 - Planning Surface Recovery

- [x] close `g09` as the active generation
- [x] mark `g10` as active in the front doors
- [x] open one explicit recovery/control lane for the live queue

### Batch 2.2 - Queue Freeze

- [x] audit the live active work across Jetstream feasibility, component
      overhaul, and parity/specimen follow-through
- [x] classify which seams are actively executing, which need planning, and
      which are blocked on sibling/runtime work
- [x] record where the recent unified-select and related Svelte work belongs in
      the real queue

Classification:
- Seam A (Svelte Component Overhaul): actively executing, unbounded — needs
  closeout milestone
- Seam B (Jetstream Implementation): planned not started — needs own milestone
  after A stabilizes
- Seam C (Parity/Verification): ad-hoc incomplete — systematic pass after A

### Batch 2.3 - Next Milestone Compilation

- [x] compile the next bounded milestone from the recovered queue
- [x] keep any parallel or blocked follow-on work explicit but pending
- [x] leave one unambiguous next task in the active authority surface

Compiled milestone: `g10.003` Svelte Component Overhaul Closeout

## Exit Criteria

- Poodle’s front doors point at the real active generation
- the live `g10` queue is explicit again
- the next bounded milestone is named and active
- future threads can resume from roadmap authority instead of reconstructing
  intent from handoff docs and scattered dirty files

## Next Task

`g10.002` is complete. The next active milestone is `g10.003` Svelte Component
Overhaul Closeout — execute Batch 3.1 (remaining composite specimen review).
