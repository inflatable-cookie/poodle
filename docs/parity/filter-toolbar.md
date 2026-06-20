<!-- parity consv=fixed gpui=7 jetstream=6 specimen=gap -->
# Parity: FilterToolbar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/filter-toolbar.md`
- Svelte (authoritative): `packages/svelte/components/src/FilterToolbar.svelte`
- GPUI: `packages/gpui/components/src/composites/filter_toolbar.rs`
- Jetstream: `packages/jetstream/components/src/filter_toolbar.rs`
- Spec: `packages/contracts/components/src/filter_toolbar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FilterToolbarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/filter_toolbar_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/filter_toolbar.rs`

## Contract ↔ Svelte

Divergences in the density token table — Svelte authoritative, contract is wrong. Reconciled.

- [x] FIXED **Density `default` root gap**: contract §8 density table said `var(--poodle-space-stack-sm)`; Svelte `[data-density="default"]` override (`FilterToolbar.svelte:273`) uses `var(--poodle-space-inline-sm)`. Contract density table now `space-inline-sm`. (Base un-scoped Root rule keeps `space-stack-sm`, matching Svelte base line 168.)
- [x] FIXED **Density `compact` root gap**: contract said `0.375rem`; Svelte uses `0.25rem` (`FilterToolbar.svelte:264`). Contract compact gap → `0.25rem`.
- [x] FIXED **Density padding-block**: default density padding now `0.75rem 1rem` (block inline) matching Svelte literals (lines 274-275); compact `0.5rem 0.75rem`, comfortable `1rem 1.25rem` already matched. Noted in §8 as the documented panel-internal-padding compositional exception.
- **`data-collapsed` value**: contract §8 lists `data-collapsed`; Svelte sets it to `collapsible && collapsed` (line 62). Consistent, no fix, noted for GPUI/Jetstream.
- Anatomy: expanded-collapsible header renders as `<button aria-expanded>` (Svelte lines 101-127). Contract §2/§6 already document this. OK.
- `summary` flex: contract §8 summary `flex: 1`; Svelte matches (line 206). OK.

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded toggle button dims — `.w(px(20.0)).h(px(20.0))` and `.rounded(px(4.0))` at `filter_toolbar.rs:217-223`. Resolve toggle hit-area + radius from tokens (icon-size + radius), not raw px.
- [ ] Hardcoded actions gap `.gap(px(4.0))` at `filter_toolbar.rs:258`; Svelte actions gap = `0.25rem`. Resolve from `space.inline.xs` token, not literal.
- [ ] Sticky shadow uses raw color + raw offsets — `hsla(0.0, 0.0, 0.0, 0.06)`, `point(px(0.0), px(2.0))`, `blur px(8.0)` at `filter_toolbar.rs:189-194`. Contract §8 sticky `box-shadow: var(--poodle-elevation-surface)`; resolve the elevation token, not a hand-built shadow.
- [ ] Collapsed header is not a focusable button — Svelte renders collapsed/expanded header as `<button aria-expanded>` with focus ring (`filter_toolbar.rs` builds a plain `div` header row, no button role, no focus outline). Add button affordance + focus ring (`accent.focusRing`, offset `0.125rem` per contract §6).
- [ ] Density not applied to root gap/padding — uses `panel_space_x/y_rem(density)` and `gap_token()` (`filter_toolbar.rs:147-179`) which do not match the contract density table (compact `0.25rem` gap / `0.5rem` block, comfortable `space-inline-md` / `1rem` block). Drive root gap + padding from a density-aware token resolution.
- [ ] Controls grid does not reduce gap by density — fixed `inline_sm` at `filter_toolbar.rs:277`; contract compact controls gap = `0.25rem`, comfortable = `space-inline-md`.
- [ ] No collapsed-header summary-in-label semantics — when collapsed, Svelte still shows summary + actions inside the toggle button; GPUI header path renders these but toggle interactivity is render-only (`on_toggle` must be wired by caller; specimen never wires it — collapse cannot be toggled in preview).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`aria-expanded` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No collapse toggle rendered — `js_filter_toolbar` header (`filter_toolbar.rs:56-73`) emits only summary + actions; never draws a chevron toggle even when `spec.collapsible`. Contract §2 CollapseToggle part + §4 collapsed state are missing. Add chevron (chevron-down/right by `is_grid_visible()`).
- [ ] Summary font-size hardcoded match arms — `filter_toolbar.rs:27-33` hardcodes `0.6875/0.71875/0.8125/0.875` rem per size; these should resolve from a size-scaled label-size token, not literals (mirrors GPUI's same smell but flagged per contract token rule).
- [ ] Density not applied — root uses single `panel_space_x_rem(spec.density)` for all padding (`filter_toolbar.rs:34`, `.p(pad)`) and `gap_token()` for gap; contract density table needs distinct compact/default/comfortable gap + block padding. `.p(pad)` also applies x-padding to y (no separate panel_y).
- [ ] Controls grid gap fixed at `inline_sm` (`filter_toolbar.rs:77`); contract varies controls gap by density.
- [ ] No focus ring / button semantics on header (interaction lives in preview `main.rs` event loop — note if collapse toggle wiring is absent there too).
- [ ] Summary uses `label` with no `flex_1`/`margin-left:auto` actions anchoring — actions will not right-align (Svelte summary `flex:1`, actions `margin-left:auto`). Add flex-grow on summary so actions anchor right.
- accepted: ARIA channel absent; interaction (collapse click) belongs in preview event loop, not the component.

## Specimen parity

- Svelte covers: Responsive grid, Collapsible with actions (expanded, bindable), Explicit collapsed state, With secondary slot, Sizes, Densities (`FilterToolbarSpecimen.svelte`).
- GPUI covers: Responsive grid, Sizes, Collapsible+actions (expanded), Explicit collapsed, With secondary — missing: **Densities** group; collapse toggle is not interactive (no `on_toggle` wired, so collapsed↔expanded cannot be exercised).
- Jetstream covers: Expanded with summary, Collapsed with actions, With secondary, Empty — missing: **Sizes** group, **Densities** group; uses stand-in `chip` labels not real TextInput/Select (acceptable: primitives noted as dependency), hardcoded `text_size(11.0)` group labels + `text_size(11.0)` Refresh action (specimen literal).

## Notes

- Spec default `collapsed = true` (`filter_toolbar.rs:48`) intentionally differs from Svelte prop default `collapsed = false` (Svelte line 30) — the Rust spec comment says it defaults collapsed to match Svelte, but Svelte's default is `false`. Minor: spec default should be `false` to truly match. Flag low-priority.
- GPUI/Jetstream both lack the responsive `@media (max-width:640px)` 1-column collapse — accepted Tier-3 implementation freedom (grid mechanics internal), not a todo.
- `columns` prop is legacy/unused for the grid in all three (auto-fit), per contract §3 — no action.
