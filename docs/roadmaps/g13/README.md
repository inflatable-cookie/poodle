# g13 Jetstream Workstation Parity

Status: planned
Updated: 2026-03-17

## Context

`g11` produces documented workstation contracts and a Svelte implementation.
`g12` implements the same contracts in GPUI. `g13` completes the workstation
parity cycle by implementing the contracts in Jetstream, the game-engine
rendering target.

Jetstream's constraints (retained-mode `UiTree`, flexbox-only layout, solid
colors, no gradients, no transforms, limited text rendering) mean some
workstation components may require native adaptation or intentional exclusion.
These should be documented explicitly in the delta register, following the
pattern established in `g08` and `g10`.

## Starting State

- workstation contracts exist in `docs/contracts/workstation/` from `g11`
- Svelte workstation implementation is complete from `g11`
- GPUI workstation implementation is complete from `g12`
- Jetstream adapter crate and preview app are in place from `g08`/`g10`
- Jetstream workstation surface is limited to pre-`g11` substrate

## Exit State

- Jetstream workstation implementation covers all feasible `g11` contracts
- Jetstream preview app demonstrates the new workstation surface
- cross-runtime parity evidence is updated for all three renderers
- any Jetstream-specific exclusions or adaptations are documented in the delta
  register

## Non-Goals

- no new contract work — contracts come from `g11`
- no DAW-specific or app-specific shell semantics
- no renegotiating Jetstream rendering constraints — adapt within them

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Jetstream workstation feasibility audit against g11 contracts | g12.007 | Foundation | Planned |
| 002 | Jetstream workstation implementation batch 1 — windows, regions, strips | 001 | Implementation | Planned |
| 003 | Jetstream workstation implementation batch 2 — docks, tabs, panels, hosted surfaces | 002 | Implementation | Planned |
| 004 | Jetstream preview specimens and demo coverage | 003 | Implementation | Planned |
| 005 | Cross-runtime workstation parity evidence (Svelte/GPUI/Jetstream) | 004 | Hardening | Planned |
| 006 | Delta register update and native adaptation documentation | 005 | Hardening | Planned |
| 007 | Generation closeout | 006 | Closure | Planned |

## Dependency Shape

```text
g12.007 GPUI Parity Complete
  -> 001 Jetstream Feasibility Audit
      -> 002 Jetstream Batch 1
          -> 003 Jetstream Batch 2
              -> 004 Specimens + Demo
                  -> 005 Cross-Runtime Parity
                      -> 006 Delta Register
                          -> 007 Closeout
```

## Execution Lanes

### Lane A: Implementation

`001 -> 002 -> 003 -> 004`

### Lane B: Evidence and Closeout

`005 -> 006 -> 007`

## Milestone Details

### 001 — Jetstream Workstation Feasibility Audit Against g11 Contracts

Review the workstation contracts from `g11` against Jetstream rendering
constraints. Classify each component as:
- fully supported
- supported with documented native adaptation
- intentionally excluded (with rationale)

### 002 — Jetstream Workstation Implementation Batch 1

Implement feasible workstation substrate in Jetstream:
- workspace window host primitives (or adapted equivalent)
- expanded region snapshot support
- strip rail components and variants
- resize handles, split dividers, and collapse affordances

### 003 — Jetstream Workstation Implementation Batch 2

Complete the Jetstream workstation implementation:
- deeper dock behavior
- window-aware surface tabs and panel tabs
- panel variants
- hosted external-surface containers (where feasible)

### 004 — Jetstream Preview Specimens and Demo Coverage

Add or update Jetstream preview app specimens for all new workstation
components. Extend demo scenes to exercise the new substrate.

### 005 — Cross-Runtime Workstation Parity Evidence

Perform systematic three-way comparison (Svelte/GPUI/Jetstream) for the new
workstation surface. Update the cross-runtime parity report.

### 006 — Delta Register Update and Native Adaptation Documentation

Update the Jetstream delta register with any workstation-specific exclusions or
adaptations. Document rationale and constraints.

### 007 — Generation Closeout

Close the generation with:
- summary of Jetstream workstation parity achieved
- final cross-runtime parity status for the full workstation surface
- any remaining backlog items for future generations

## Cross-Project Dependencies

| Dependency | Direction | Description |
|-----------|-----------|-------------|
| Pug g11 | Pug -> Pug g13 | Workstation contracts |
| Pug g12 | Pug -> Pug g13 | GPUI reference for three-way parity comparison |
| Jetstream workstation package | Internal | Primary implementation target |

## Next Task

Wait for `g12` completion, then execute `g13.001` to audit g11 contracts
against Jetstream rendering constraints.
