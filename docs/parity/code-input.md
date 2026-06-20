<!-- parity consv=fixed gpui=5 jetstream=7 specimen=gap -->
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

- [ ] Hardcoded rem-float font literals `rem_to_px(0.8125/0.875/1.0/1.125/1.25)` at `code_input.rs:157-161` and density gaps `px(rem_to_px(0.25/0.375))` at `code_input.rs:167-168` — resolve from typography/space tokens, not raw rem floats.
- [ ] Slot sizing uses `control_height_rem` for both width and height (`code_input.rs:153-155`), so md = control-height square. Svelte md is fixed `2.25rem` square; verify control-height md == 2.25rem or the slots drift from Svelte at every size.
- [ ] No real input / paste / autofill / one-time-code autocomplete (contract §5/§6) — only a focusable group with `on_key_down` digit handling (`code_input.rs:301-343`). Auto-advance + backspace-retreat are approximated via key events; clicking a slot to place caret is absent.
- [ ] No slot-click-to-focus or in-place replacement (contract §5) — slots are non-interactive `div`s; `active_index` is preview-driven (`code_input.rs:124`), not click-driven.
- [ ] Numbers-only is hardcoded — `digits` filters `is_ascii_digit()` (`code_input.rs:209-213`/`329`); spec has no `numbers_only`/alphanumeric path, so contract `numbersOnly={false}` (alphanumeric) is unsupported.
- accepted: no ARIA (gpui has no accessibility API) — `role="group"`, `aria-label`, `aria-invalid`, `aria-disabled` not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded literals: `border_width = rem_to_px(0.0625)` `code_input.rs:70`, font `* 1.5` magic multiplier `code_input.rs:69`, gap `* 0.5` heuristic `code_input.rs:71`, error `text_size(rem_to_px(0.75))` `code_input.rs:131`, error gap `rem_to_px(0.5)` `code_input.rs:136` — resolve from tokens, drop magic multipliers.
- [ ] Slot width is an ad-hoc `slot_width_rem` table (xs `1.25`→xl `3.0`, lines 21-29) that matches **neither** Svelte (square, md `2.25`) nor the contract. Slots are non-square and undersized. **Fix: use Svelte's square per-size values.**
- [ ] Slot height = `control_height + size_height_offset` (`code_input.rs:66-67`); combined with the separate width table this yields non-square slots unlike Svelte's square slots.
- [ ] Font size is `size_font_rem × 1.5` (`code_input.rs:69`); contract slot font is `1rem` at md with a per-size table. The `×1.5` heuristic does not track the contract size ladder.
- [ ] No 3+3 split-after gap for 6-digit codes (Svelte `--split-after` at index 2); all slots evenly spaced.
- [ ] No real input / paste / autofill / autocomplete / slot-click — visual slot grid only (component header acknowledges runtime ownership, lines 42-43). Auto-advance/backspace/onComplete not present in the component.
- [ ] No hint/label rendering — only the slot row and (on error) an error label (`code_input.rs:129-147`); contract composes a `Field` wrapper with label/hint. Active slot uses `color.accent.base` (`code_input.rs:103`) where Svelte uses `color.accent.border`.
- accepted: no ARIA channel; interaction (typing, paste, focus movement) lives in preview event loop.

## Specimen parity

- Svelte covers: Default (6-digit interactive w/ hint + completion), Masked, Alphanumeric (`numbersOnly=false`), With error, Disabled (4-digit), Sizes, Densities (`CodeInputSpecimen.svelte`).
- GPUI covers: 6-digit code (interactive), 4-digit masked (interactive), With error, Disabled, Sizes, Densities. — missing: **Alphanumeric** variant (unsupported in spec).
- Jetstream covers: Partial (3 of 6), Complete, Invalid, 4-digit Masked, Disabled. — missing: **hint/label** rendering, **Alphanumeric**, **Sizes**, **Densities**; static only (no interaction).

## Notes

- `consv=fixed`: the stale slot geometry (now square `2.25rem` md + square per-size table), default label (`Authenticator code`), root-gap token, slot font-weight, 3+3 split-after, and validation-derived slot colors are all reconciled to Svelte. Remaining gpui/jetstream todos are code-side.
- Both Rust targets render visual slot grids only; real-input ownership (paste, autofill, one-time-code, slot-click caret placement) is a documented runtime concern but means Tier-1 auto-advance/backspace/onComplete parity is only partially met (GPUI via key events, Jetstream not at all).
- `numbersOnly={false}` (alphanumeric) is unsupported on both Rust targets — the spec lacks the flag and both hardcode digit filtering.
