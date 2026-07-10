# g11.005 Appearance Recipe Productization And Per-App Overrides

Status: planned
Owner: Poodle core
Depends on: `g11.003` (needs `data-part`/`data-state` attributes on at least
the pilot + overlay waves), `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md`
Updated: 2026-07-10

## Purpose

Turn spec `026`'s override strategy into a shipped, documented surface: slot
recipes per component that consuming apps can override per-app without forking
components or redefining tokens. This is the "custom design language without
rebuilding the suite" deliverable.

## Shape

- per behavioral component: a **slot recipe** — style map keyed by anatomy
  part (from the machine spec) and state/size/variant/density
- default recipes resolve entirely from semantic tokens and reproduce current
  visuals **exactly** (pixel parity is the migration gate)
- consuming apps override at three sanctioned depths, per spec `026`:
  1. token palette (exists today)
  2. treatment roles (`interactive`, `surface`, ... — exists as seed)
  3. per-component recipe slot overrides (new productized surface)
- delivery mechanism decision (record in spec `062`): CSS custom properties
  per part/state, style-object maps, or both. Constraint: the chosen shape
  must stay describable for GPUI/Jetstream so recipes remain cross-runtime
  where they fall in the cross-runtime lane.

## Deliverables

1. recipe format + registration/override API in `@poodle/svelte` (or core if
   the format is framework-free data — decide, record)
2. default recipes for all components migrated to core so far
3. override docs + a worked example: one consumer app (suggest `soundcheck` or
   `finch/app-electron` — small, direct consumers) restyles 2–3 components
   with app-owned recipes as the acceptance proof
4. contract updates: recipe surface + stability guarantees layered onto spec
   `026`; per-component recipe slot table generated from anatomy

## Compatibility

Additive only. No consumer changes required anywhere; defaults reproduce
current rendering. Proof: consumer typecheck matrix + before/after visual
check on the preview demo screens.

## Exit Criteria

- recipe layer shipped with exact-parity defaults
- one real consumer app running custom recipes as evidence
- override boundary documented (what is stable, what is internal)
- spec `026` promotion note updated

## Validation

- visual parity check across preview demo screens
- consumer typecheck matrix
- `effigy docs:lint`, `effigy svelte:surface-audit`

## Next Task

`g11.006` Rust mirror, or continue `g11.004` waves in parallel.
