# g09.001 — Component Gap Audit: Svelte Surface vs GPUI Components

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g08.014
Primary repos: `pug`

## Goals

- [ ] produce an exhaustive gap register comparing Svelte components to GPUI
  component structs
- [ ] classify each gap as: new struct needed, covered by existing config,
  or not applicable to GPUI
- [ ] produce a prioritized implementation list with batch assignments for
  g09.002–g09.006

## Execution Checklist

- [ ] enumerate all 77 Svelte primitives from `@pug/svelte-primitives`
- [ ] enumerate all 42 Svelte composites from `@pug/svelte-composites`
- [ ] enumerate all workstation surfaces from `@pug/svelte-workstation`
- [ ] enumerate all 41 GPUI component structs from `pug_gpui_components`
- [ ] cross-reference and produce gap table with columns: Svelte component,
  tier, GPUI struct exists (Y/N), classification, batch assignment
- [ ] identify components where Svelte specimen exists but GPUI specimen is
  a hand-built mockup — flag for specimen upgrade
- [ ] identify Svelte components with no Rust spec in `pug_gpui_primitives` —
  these need spec creation before component implementation
- [ ] produce final gap register document in `docs/roadmaps/g09/`
- [ ] review and confirm batch assignments with component dependency order

## Acceptance Criteria

- [ ] gap register covers all 127 Svelte components without omission
- [ ] every gap is classified with clear rationale
- [ ] batch assignments in 002–006 are balanced and respect dependency order
- [ ] no ambiguous "maybe" classifications — each component has a clear
  disposition

## Next Task

Open `g09.002` and implement missing structural and informational primitives.
