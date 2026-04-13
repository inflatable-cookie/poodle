# g10.008 GPUI Contract Coverage Gaps And List Composition

Status: complete
Owner: Poodle core
Depends on: g10.007
Updated: 2026-04-13

## Context

Post-audit: `packages/gpui/components` still misses several contracts that Svelte
or the spec crate already treat as real surface area. Separately, **list
layout helpers** are easy to misread as “missing from preview” when the
contract deliberately scopes specimens.

### ListGrid and ListCardCounter (Svelte preview)

- **ListCardCounter** — not a standalone preview route **by contract**.
  `docs/contracts/components/list-card-counter.md` states it has no standalone
  specimen; it is shown **inside** the List card specimen
  (`packages/svelte/preview/src/specimens/ListCardSpecimen.svelte`). It is a
  **composition helper** for `ListCard` footers, exported from `@poodle/svelte`.
- **ListGrid** — **public** layout primitive (exported, documented in
  `docs/guides/013-admin-feature-delivery-recipes.md`). It is **not** internal;
  the gap is **no dedicated Svelte preview specimen** (only parity/registry
  references). Apps compose it directly; GPUI has **no** implementation yet.

## Governing Refs

- `docs/contracts/components/list-grid.md`
- `docs/contracts/components/list-card-counter.md`
- `docs/contracts/components/list-card.md`
- `CLAUDE.md` (contract-first, token rules)

## Goals

- add GPUI **ListGrid** matching `list-grid.md`, with a real preview specimen
- add GPUI **ListCardCounter** matching `list-card-counter.md` (likely composed
  from tokens + `Icon`; may be demonstrated inside the GPUI ListCard specimen
  rather than a top-level nav card if that matches contract specimen rules)
- add a **Svelte preview specimen for ListGrid** so web parity review matches
  GPUI (optional sub-goal if web team wants catalogue symmetry)
- rename or alias GPUI **TimeField** to align with contract **TimeInput**
  (`time-input.md`) — module, type, and preview route copy — unless a single
  documented exception is preferred

## Non-Goals

- redesigning ListCard anatomy
- Jetstream implementation (stay in GPUI + Svelte + contracts lane)

## Execution Plan

### Batch 8.1 — List surface

- [x] implement `ListGrid` in GPUI from `list-grid.md` (anatomy, props, tokens)
- [x] implement `ListCardCounter` in GPUI from `list-card-counter.md`
- [x] wire GPUI preview: ListGrid specimen; extend ListCard specimen with
      counters where the contract expects them
- [x] Svelte preview specimen for `ListGrid` (`ListGridSpecimen.svelte`)

### Batch 8.2 — Naming

- [x] public alias: `TimeInput` = `TimeField` in `gpui/components` primitives
      re-exports (`time-input.md` contract name without renaming the module yet)

### Batch 8.3 — Other contract-only gaps (same milestone or spill)

Deferred to later milestones:

- [ ] `debug-dialog`, `error-boundary` — confirm web-only vs GPUI scope
- [ ] `format-display-date`, `format-file-size` — GPUI helpers or defer
- [ ] `inline-list-section`, `state-tile` — scope and priority vs apps

## Validation

- `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
- `git diff --check`
- contract checklist pass for touched components

## Outcome

- Added `ListGridSpec` / `ListGridVariant` in `poodle-specs`, GPUI `ListGrid`
  (flex-wrap grid approximation), and preview catalogue entry + specimen.
- Added GPUI `ListCardCounter`; list-card specimen footers use real counters and
  tokens (`space.inline.xs`, `typography.counter.size`, `size.list.grid.minItemWidth`)
  via new semantic token schema entries and GPUI + Jetstream theme resolution.
- Exposed `TimeInput` as a type alias for `TimeField` on the GPUI components crate.
- Known GPUI deltas: no `tabular-nums`; linked counter click propagation differs
  from DOM `stopPropagation`; tooltip on counter needs `on_tooltip_open_change`
  wiring to open.
