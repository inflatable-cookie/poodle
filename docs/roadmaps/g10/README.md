# g10 Jetstream Focus

Status: active
Updated: 2026-04-09

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

`g10.001` is complete, but the active thread work has drifted beyond that
single feasibility milestone. The next honest step is to open a fresh
recovery/control lane that freezes the real current queue from evidence and
then compiles the next bounded milestone.

## Next Task

Execute `g10.002`: recover the real live queue across Jetstream feasibility,
component-overhaul work, and parity/specimen follow-through, then compile the
next bounded milestone from that evidence.
