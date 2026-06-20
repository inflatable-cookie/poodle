<!-- parity consv=fixed gpui=2 jetstream=9 specimen=gap -->
<!-- pass 21: GPUI rebuilt — actions now composed ghost IconButtons (danger tone for
     destructive), select-all + the missing clear-"×" control added (ghost IconButtons),
     total text "of {N}". Additive spec fields: BulkAction.icon (+resolved_icon fallback
     trash-2/circle), BulkActionBarSpec.disabled (+is_unavailable/actions_disabled).
     Build clean (gpui components+preview, specs 53 tests). Warning tone renders Default
     (no ButtonTone::Warning in GPUI) — noted. -->
# Parity: BulkActionBar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/bulk-action-bar.md`
- Svelte (authoritative): `packages/svelte/components/src/BulkActionBar.svelte`
- GPUI: `packages/gpui/components/src/primitives/bulk_action_bar.rs`
- Jetstream: `packages/jetstream/components/src/bulk_action_bar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/BulkActionBarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/bulk_action_bar_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/bulk_action_bar.rs`

## Contract ↔ Svelte

Several documented behaviors diverge from the Svelte implementation. Svelte is authoritative — update the contract — except where noted.

- [x] FIXED Total text: contract §2/§4/§8/specimen changed from `"of {totalCount} visible rows"` to `of {totalCount}` (Svelte line 70).
- [x] FIXED Select-all is an IconButton, not a text button: replaced the §2 text-button anatomy part + §8 select-all-button CSS table + §6 a11y line with the ghost `IconButton icon="check-check"` (`sizeRole="chrome"`) form (Svelte lines 72-82). §1 in-scope line and §9 Svelte notes updated.
- [x] FIXED `selectAllLabel` note: §3 prop note now says it is the IconButton's accessible label/tooltip (not a visible text label), suffixed with `(totalCount)`.
- [x] FIXED Select-all in Summary, not Actions: §2 anatomy moves select-all under `[Summary]` after count/total (Svelte line 72).
- [x] FIXED Root `position: fixed` floating bar: §7 sizing + §8 root table document fixed positioning, safe-area insets, sticky z-index, max-width, and the flat `0.5rem` vertical padding (`var(--poodle-space-panel-x)` inline). Density table corrected to change padding-inline + gap only (vertical pad stays `0.5rem`), fixing the prior orthogonality violation.
- [x] FIXED Root shadow: added the two-layer `box-shadow` (drop + hairline ring, Svelte lines 152-154) to §8 root table.
- [x] FIXED Danger fallback icon: §8 action-rendering now documents `trash-2` (danger) / `circle` (otherwise) fallback (Svelte line 88).
- Warning tone via `:global()` color override (lines 183-190) including hover/focus mix — matches contract intent; §8 note expanded to mention the 82% text-primary hover blend. `ok`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Vertical padding literal `px(rem_to_px(0.5))` at `bulk_action_bar.rs:124` — matches Svelte's `0.5rem` but is a raw float; resolve from a token once contract documents the `0.5rem` vertical pad.
- [ ] Actions-gap density literals `rem_to_px(0.125)` / `rem_to_px(0.5)` at `bulk_action_bar.rs:129,131` — resolve compact/comfortable action gaps from tokens, not raw rem floats.
- [ ] **Actions render as bordered text buttons, not ghost IconButtons.** Loop at `bulk_action_bar.rs:243-257` builds `div().h().px().border_1().child(action.label)` showing the text label. Contract §2/§9 require ghost `IconButton`s showing the action icon with label as tooltip/ariaLabel only. No icon is drawn; no ghost variant.
- [ ] No clear (`x`) IconButton — contract §2 requires a clear button; the GPUI actions row (lines 228-278) omits it entirely. `onClear` callback is absent from the builder.
- [ ] Select-all is a link-style text button (lines 202-221), not the `check-check` IconButton Svelte uses; also placed in summary which happens to match Svelte but contradicts contract anatomy.
- [ ] No `disabled` prop on the bar (only `loading`): `disabled` field/builder absent; contract §3 lists both. `actions_disabled` (Svelte gates on `selectionCount === 0` too) is not replicated.
- [ ] No warning `:global()`-equivalent hover mix — warning uses a static `warning_text` (line 234) with no hover-toward-text-primary blend (Svelte lines 187-189). Minor visual delta.
- [ ] Total text reads `"of {} selected"` (line 187) — wrong word order vs Svelte `of {totalCount}`; drop "selected".
- accepted: no ARIA (gpui has no accessibility API) — `role="region"` / `aria-label="Bulk actions"` / per-action ariaLabel not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Gap literal `rem_to_px(0.5)` at `bulk_action_bar.rs:17` — resolve summary↔actions gap from `space.inline.md` and action gap from `space.inline.sm` tokens, not a single raw float.
- [ ] Vertical pad heuristic `panel_space_y_rem(density) - 0.375` at `bulk_action_bar.rs:16` — ad-hoc `-0.375` offset; Svelte uses a flat `0.5rem`. Resolve from a token.
- [ ] **Actions render as plain text buttons** (lines 33-40) showing `action.label`, no icon, no ghost variant, no tone styling. Contract requires ghost IconButtons with icon + tooltip. `action.tone` is entirely ignored — no danger/warning treatment.
- [ ] No clear (`x`) IconButton — contract-required clear control absent.
- [ ] No select-all control — `show_select_all`/`all_selected`/`select_all_label` spec fields unused; nothing renders.
- [ ] No total-count display — `spec.total_count` never read; summary shows only `"{count} selected"` (line 29). Contract "with total" state unmet.
- [ ] No `loading`/`disabled` gating — neither field consulted; action buttons are always enabled-looking, no opacity reduction via a disabled-opacity token, no per-action `disabled`.
- [ ] No accent-tinted background — fill is flat `color.background.elevated` (line 20); contract §8 requires `color-mix(panel 93%, text-primary)`. No border, no radius, no shadow on the root (root has none of `.border`/`.rounded`).
- [ ] Per-action click wiring absent (`.focusable()` only, line 38) — interaction must live in preview event loop; confirm none exists.
- accepted: no ARIA channel (`role="region"` / aria-label not emitted).
- accepted: interaction (click handler) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: selection-count + total + select-all + 4 actions (incl. danger + warning) with last-action readout, single-item subset (2 actions), loading + disabled + per-action-disabled, plus Sizes and Densities variant snippets (`BulkActionBarSpecimen.svelte`).
- GPUI covers: selection-count + total + select-all, single-item subset, loading + disabled + per-action-disabled (`bulk_action_bar_specimen.rs`). — missing: **Sizes** and **Densities** variant groups; danger/warning tones only show as border/text color on text buttons (no icons), so warning/danger visual parity is not actually demonstrated.
- Jetstream covers: "With actions" (2 plain actions, no tones), "Empty selection" (`bulk_action_bar.rs`). — missing: **total count**, **select-all**, **danger/warning actions**, **loading/disabled**, **clear button**, **Sizes**, **Densities**. Far under Svelte coverage.

## Notes

- The dominant structural gap on both Rust targets: actions are rendered as labeled text buttons instead of ghost IconButtons, so neither the icon, the tooltip, nor the clear `x` button from contract §2 exists. This also means tone styling (danger/warning) has no icon to color in Jetstream and is purely border/text in GPUI.
- Jetstream is the weakest implementation here — no total, no select-all, no clear, no tones, no loading/disabled, no accent background. Treat it as roughly half-implemented against the contract.
- `consv=gap` driver: contract describes a non-floating text-button select-all and "of N visible rows" total, but Svelte ships a fixed-position floating bar with an IconButton select-all and a bare "of N" total. The contract trails the authoritative Svelte source on positioning, select-all form, and total text.
