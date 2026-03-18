# g12 GPUI Workstation Parity

Status: planned
Updated: 2026-03-17

## Context

`g11` produces documented workstation contracts and a complete Svelte
implementation covering window hosts, region grammar, strip rails,
resize/collapse affordances, docks, tabs, panel variants, and hosted external
surfaces. `g12` implements the same contracts in GPUI, bringing the GPUI
workstation surface to parity with Svelte.

Loophole Spark (the GPUI-based UI) is the primary downstream consumer driving
this work. The contracts are already stable from `g11`; this generation is
purely implementation and parity verification.

## Starting State

- workstation contracts exist in `docs/contracts/workstation/` from `g11`
- Svelte workstation implementation is complete and proven
- GPUI workstation surface is limited to the pre-`g11` substrate
  (`WorkspaceShell`, `SplitView`, `DockRegion`, `PanelSurface`, etc.)
- GPUI preview app and specimen framework are in place from `g09`

## Exit State

- GPUI workstation implementation covers all `g11` contracts
- GPUI preview app demonstrates the new workstation surface
- Svelte/GPUI parity evidence is updated for the new surface
- any intentional GPUI-specific deltas are documented

## Non-Goals

- no new contract work — contracts come from `g11`
- no Jetstream implementation (deferred to `g13`)
- no DAW-specific or Loophole-specific shell semantics

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | GPUI workstation gap audit against g11 contracts | g11.013 | Foundation | Planned |
| 002 | GPUI workstation implementation batch 1 — windows, regions, strips | 001 | Implementation | Planned |
| 003 | GPUI workstation implementation batch 2 — docks, tabs, panels, hosted surfaces | 002 | Implementation | Planned |
| 004 | GPUI preview specimens and demo coverage | 003 | Implementation | Planned |
| 005 | Svelte/GPUI workstation parity evidence and delta register | 004 | Hardening | Planned |
| 006 | Downstream reference adoption proof (Loophole Spark) | 005 | Hardening | Planned |
| 007 | Generation closeout and next-program cutover | 006 | Closure | Planned |

## Dependency Shape

```text
g11.013 Contracts and Svelte Complete
  -> 001 GPUI Gap Audit
      -> 002 GPUI Batch 1
          -> 003 GPUI Batch 2
              -> 004 Specimens + Demo
                  -> 005 Parity Evidence
                      -> 006 Downstream Proof
                          -> 007 Closeout
```

## Execution Lanes

### Lane A: Implementation

`001 -> 002 -> 003 -> 004`

### Lane B: Evidence and Closeout

`005 -> 006 -> 007`

## Milestone Details

### 001 — GPUI Workstation Gap Audit Against g11 Contracts

Review the workstation contracts from `g11` against the current GPUI
workstation surface. Identify:
- what maps directly to existing GPUI patterns
- what requires new GPUI components or spec structs
- any GPUI-specific adaptation needed (intentional deltas)

### 002 — GPUI Workstation Implementation Batch 1

Implement the first half of the workstation substrate in GPUI:
- workspace window host primitives
- expanded region snapshot support
- strip rail components and variants
- resize handles, split dividers, and collapse affordances

### 003 — GPUI Workstation Implementation Batch 2

Complete the GPUI workstation implementation:
- deeper dock behavior
- window-aware surface tabs and panel tabs
- panel variants
- hosted external-surface containers

### 004 — GPUI Preview Specimens and Demo Coverage

Add or update GPUI preview app specimens for all new workstation components.
Extend demo scenes to exercise the new substrate.

### 005 — Svelte/GPUI Workstation Parity Evidence and Delta Register

Perform systematic Svelte/GPUI comparison for the new workstation surface.
Update the parity evidence and document any intentional deltas.

### 006 — Downstream Reference Adoption Proof (Loophole Spark)

Demonstrate that the GPUI workstation substrate meaningfully reduces local
shell glue in at least one downstream consumer (Loophole Spark).

### 007 — Generation Closeout and Next-Program Cutover

Close the generation with:
- summary of GPUI workstation parity achieved
- documented intentional deltas
- clean handoff to `g13` for Jetstream parity

## Cross-Project Dependencies

| Dependency | Direction | Description |
|-----------|-----------|-------------|
| Pug g11 | Pug -> Pug g12 | Workstation contracts and Svelte reference implementation |
| Loophole Spark | Downstream | GPUI workstation adoption proof target |
| GPUI workstation package | Internal | Primary implementation target |

## Next Task

Wait for `g11` completion, then execute `g12.001` to audit g11 contracts
against the current GPUI workstation surface.
