<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass 42: specimens backfilled to full contract coverage on both Rust targets — empty/partial/complete, numbers-only vs alphanumeric, masked, invalid, disabled, plus size + density ladders. All groups use real CodeInput/js_code_input from CodeInputSpec (no hand-rolled slots). Both previews build clean. -->
<!-- pass 41: both targets — square slot ladder (code_input_slot_size_rem), slot font ladder (code_input_slot_font_rem), token-resolved density gap (xs→sm→md inline), fixed split-after (space.inline.md) on Jetstream, active slot uses accent.border, numbers_only spec flag drives alphanumeric on both. Remaining: real input / paste / autofill / slot-click caret + monospace font + caret are runtime/preview-loop gaps (accepted). -->
# Parity: CodeInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/code-input.md`
- Svelte (authoritative): `packages/svelte/components/src/CodeInput.svelte`
- GPUI: `packages/gpui/components/src/primitives/code_input.rs`
- Jetstream: `packages/jetstream/components/src/code_input.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CodeInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/code_input.rs` · jetstream `packages/jetstream/preview/src/specimens/code_input.rs`

## Contract ↔ Svelte

Svelte slot geometry and the default label disagree with the contract. Svelte is authoritative — update the contract.

- [x] FIXED Default `label`: contract §3 default reconciled to `"Authenticator code"`, matching Svelte (line 37).
- [x] FIXED Slot height: contract §7 Slot height set to `2.25rem` (square) and the Slot table now notes slots are square; matches Svelte (lines 315-316).
- [x] FIXED Size table: contract §7 size table replaced with Svelte's square per-size values (xs `1.5`, sm `1.75`, md `2.25`, lg `2.75`, xl `3.25rem`, single width/height column) plus a density-gap note.
- [x] FIXED Root gap: contract §7 Root gap now reads `var(--poodle-space-inline-sm)` (and `width: max-content` added); Tier 2 checklist line updated.
- [x] FIXED Slot `font-weight`: added a `font-weight: 600` row to the §7 Slot table, matching Svelte (line 324).
- [x] FIXED Split-after gap: contract §2 anatomy now notes the index-2 `--split-after` marker, and §7 adds a "Slot — split-after" table (`margin-right: var(--poodle-space-inline-md)`) documenting the 3+3 grouping for 6-digit codes.
- [x] FIXED Validation focus colors: §7 now has a "Slot — active" table (`--code-slot-focus` border + `--code-slot-focus-ring` box-shadow) and a "Slot — validation state" table mapping default vs `invalid` to the border/focus/ring custom properties, matching Svelte (lines 69-83).
- [x] FIXED `validationState` valid/pending: §5 Behavior now notes only the `invalid` case (or a non-null `error`) changes slot visuals; other states render default colors.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Font + gap resolve from ladders/tokens — `code_input_slot_font_rem` for slot font; density gap now `space.inline.xs`/`sm`/`md` (compact/default/comfortable) and split-after = `space.inline.md` (both token-resolved), no raw rem-float density literals.
- [x] FIXED Slot sizing uses the explicit square ladder `code_input_slot_size_rem` (xs 1.5 → xl 3.25rem; md 2.25rem matches Svelte) instead of `control_height_rem`, so width == height == contract value at every size.
- [x] FIXED `numbers_only` spec flag added — `sanitized_chars()` filters digits when true, all chars when false; the key handler accepts alphanumeric in non-numbers-only mode. Contract `numbersOnly={false}` now supported.
- accepted: No real input / paste / autofill / one-time-code autocomplete (contract §5/§6) — focusable group with `on_key_down` approximates auto-advance + backspace-retreat; paste/autofill/slot-click caret are runtime concerns (preview-loop).
- accepted: No slot-click-to-focus / in-place replacement — `active_index` is preview-driven; caret placement is a runtime concern.
- accepted: no ARIA (gpui has no accessibility API) — `role="group"`, `aria-label`, `aria-invalid`, `aria-disabled` not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Magic multipliers dropped — font from `code_input_slot_font_rem`, gap from the density token ladder (`space.inline.xs/sm/md`), error font from `typography.label.size`, error gap from `space.inline.sm`. `border_width = rem_to_px(0.0625)` is the contract-exact 1px border (kept).
- [x] FIXED Slot width now uses the square ladder `code_input_slot_size_rem` (md 2.25rem), matching Svelte and the contract.
- [x] FIXED Slot height uses the same square ladder (width == height), removing the `control_height + offset` non-square drift.
- [x] FIXED Font size uses `code_input_slot_font_rem` (md 1rem) — the `×1.5` heuristic is gone.
- [x] FIXED 3+3 split-after gap added — index 2 gets `mr(space.inline.md)` when `length == 6`.
- [x] FIXED Active slot now uses `color.accent.border` (was `accent.base`), matching Svelte; only the invalid case overrides slot colors (Valid/Pending branches removed).
- accepted: No real input / paste / autofill / autocomplete / slot-click — visual slot grid only; interaction (typing, paste, focus movement) lives in the preview event loop.
- accepted: No hint/label rendering — the `Field` wrapper (label/hint) is a runtime composition concern; the component renders the slot row + error label.
- accepted: no ARIA channel; interaction lives in preview event loop.
- note (runtime gap): monospace font-family + text caret are engine gaps (Jetstream has no font-family/caret control) — cells, distributed value, and active-slot highlight render; the caret/monospace do not.

## Specimen parity

- Svelte covers: Default (6-digit interactive w/ hint + completion), Masked, Alphanumeric (`numbersOnly=false`), With error, Disabled (4-digit), Sizes, Densities (`CodeInputSpecimen.svelte`).
- GPUI covers: 6-digit code (interactive), 4-digit masked (interactive), With error, Disabled, Sizes, Densities. — missing: **Alphanumeric** variant (unsupported in spec).
- Jetstream covers: Partial (3 of 6), Complete, Invalid, 4-digit Masked, Disabled. — missing: **hint/label** rendering, **Alphanumeric**, **Sizes**, **Densities**; static only (no interaction).

## Notes

- `consv=fixed`: the stale slot geometry (now square `2.25rem` md + square per-size table), default label (`Authenticator code`), root-gap token, slot font-weight, 3+3 split-after, and validation-derived slot colors are all reconciled to Svelte. Remaining gpui/jetstream todos are code-side.
- Both Rust targets render visual slot grids only; real-input ownership (paste, autofill, one-time-code, slot-click caret placement) is a documented runtime concern but means Tier-1 auto-advance/backspace/onComplete parity is only partially met (GPUI via key events, Jetstream not at all).
- `numbersOnly={false}` (alphanumeric) is now supported on both Rust targets — `CodeInputSpec.numbers_only` (default true) drives `sanitized_chars()`; GPUI's key handler also accepts alphanumeric in that mode.
