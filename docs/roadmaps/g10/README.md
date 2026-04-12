# g10 Jetstream Focus

Status: active
Updated: 2026-04-12

## Context

`g10` is now the active generation after `g09`.
Its original Jetstream feasibility framing is still useful, but the live queue
has clearly broadened into a component-overhaul and renderer-parity recovery
surface that needs to be made explicit again.

## Scope

- Jetstream renderer feasibility and constraints
- Jetstream component implementation depth
- Jetstream specimen and preview coverage
- Jetstream parity evidence and documented deltas
- Jetstream-specific closeout work
- any component-overhaul recovery work that is materially part of the current
  Poodle execution queue rather than historical side work

## Working Rule

- do not use `g10` to absorb unfinished `g09` work by pretending `g09` is still
  active
- keep the live queue explicit instead of relying on handoff-only momentum
- use `g10` for the active Jetstream and component-overhaul tranche

## Current State

- `g10.001` complete — Jetstream feasibility proven
- `g10.002` complete — recovery/control lane opened, queue frozen, seams
  classified, next milestone compiled
- `g10.003` complete — Svelte Component Overhaul Closeout
- `g10.004` complete — Unified Component Package
- `g10.005` complete — GPUI preview shell, navigation, and native state parity
- `g10.006` complete — GPUI component page usage docs and shape parity
- `g10.007` complete — GPUI long-tail component parity sweep and closeout
  checkpoint

Three active seams identified:
- Seam A (Svelte Component Overhaul): complete via `g10.003`
- Seam B (Jetstream Implementation): active next lane
- Seam C (Parity/Verification): closeout checkpoint reached via `g10.005` to
  `g10.007`

## Active Runway

- current active lane: Seam B Jetstream implementation
- parity checkpoint reached: `g10.005` to `g10.007` complete
- only reopen GPUI parity for fresh evidence or a new bounded milestone

## Next Task

Return to the Jetstream execution seam in `g10`: compile the next Jetstream
milestone or resume the current implementation card with the GPUI parity lane
treated as checkpointed.
