# g11.005 Appearance Recipe Productization And Per-App Overrides

Status: complete (2026-07-10)
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

## Completion Notes (2026-07-10)

- **Decision:** recipes are CSS custom-property contracts in a dedicated
  read-only namespace — `--poodle-recipe-<component>[-<variant>]-<slot>
  [-<state>]` — no JS API. Resolution chain per component variable:
  recipe hook → treatment role → semantic token. Recorded in
  `docs/architecture/007-appearance-recipe-contract.md` and spec 062.
- **Design correction caught live:** component-local
  `--poodle-<component>-*` variables cannot be the public surface —
  components define them, so app-scoped overrides lose the cascade.
  Verified in the preview; the g03.005 seed's `--poodle-recipe-*` pattern
  (already adopted by Card/PageHeader/ListCard/BulkActionBar) was correct
  and is now the documented contract. Button (secondary + primary-variant
  hooks) and Pill adopted in this milestone.
- **Inventory tooling:**
  `packages/svelte/preview/scripts/build-recipe-inventory.ts` generates
  `artifacts/recipe-inventory.json` — 34 recipe hooks, 46 hook candidates
  (appearance vars pending hooks, added on demonstrated need), 290 metric
  vars (explicitly out of contract, size/density owns them).
- **Worked example:** `soundcheck` restyles Button (violet fill + hard
  shadow), Pill (amber tint), and text-input focus chrome via a scoped
  recipe block in its `app.css`; soundcheck `bun run build`
  (svelte-check + vite) passes.
- **Runtime-verified:** app-scoped `--poodle-recipe-button-fill` and
  `--poodle-recipe-button-primary-fill` override secondary and primary
  buttons independently and restore exactly on removal. Verification
  gotcha recorded: buttons transition `background-color`, so computed
  reads must wait out the transition.
- Bonus fix: soundcheck's stricter svelte-check exposed a `stickyTones`
  typing defect in ToastHost (optional tone entries vs core `ToastTone[]`)
  — fixed.

## Next Task

`g11.006` Rust mirror.
