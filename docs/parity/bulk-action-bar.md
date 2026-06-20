<!-- parity consv=gap gpui=8 jetstream=9 specimen=gap -->
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

- **Total text differs.** Contract §2/§4/§8 say total renders as `"of {totalCount} visible rows"`. Svelte renders only `of {totalCount}` (line 70) — no "visible rows" suffix. **Fix: contract — change to `of {totalCount}`** (Svelte authoritative).
- **Select-all is an IconButton, not a text button.** Contract §2 + §8 + §6 describe a `.bulk-action-bar__button` text `<button>` with min-height/padding/border tokens and a `selectAllLabel`. Svelte renders an `IconButton icon="check-check"` inside the summary (lines 72-82), labelled via `ariaLabel`/`tooltip`, no visible text, no `.bulk-action-bar__button` element. **Fix: contract — replace the select-all text-button section (§2, §8 select-all table, §6) with the IconButton form.**
- **`selectAllLabel` prop dropped from Svelte's `Props`.** Contract §3 lists `selectAllLabel` (default `"Select all"`). Svelte still accepts it via `selectAllLabel` (line 75) using it for aria/tooltip — present. `ok`, but it is now an a11y label only, not a visible label; note in contract.
- **Select-all lives in Summary, not Actions.** Contract §2 anatomy places select-all inside `[Actions]` ahead of action icons. Svelte places it inside `__summary` (line 72). **Fix: contract anatomy — move select-all under Summary.**
- **Root is `position: fixed` floating bar.** Svelte root is a fixed bottom-docked bar with safe-area insets, z-index, and a heavy shadow (lines 134-155). Contract §7 says "fills parent width" / "above or below list" with no fixed positioning, and §8 root padding is `panel-y panel-x` whereas Svelte hardcodes `0.5rem var(--poodle-space-panel-x)` (line 146). **Fix: contract — document fixed positioning + the `0.5rem` vertical padding (it is intentionally not `panel-y`).**
- **Background border token differs.** Contract §8 root `border` uses `--poodle-color-border-subtle` ✓ (line 147 matches). Shadow (`box-shadow`, lines 152-154) is undocumented. **Fix: add root shadow to contract §8.**
- Danger fallback icon: Svelte uses `trash-2` for danger, `circle` otherwise (line 88); contract §8 says fallback `"circle"` only. **Fix: document the danger `trash-2` fallback.**
- Warning tone via `:global()` color override (lines 183-190) including hover/focus mix — matches contract intent. `ok`.

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
