<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
# Parity: TimeInput

> Pass (Rust): Resolved the cross-cutting `validation_state` issue — removed the unsourced `ValidationState` field from `TimeFieldSpec` and the GPUI `validation_state()` builder; `TimeFieldSpec::border_token()` now always returns `COLOR_BORDER_DEFAULT` (contract + Svelte have no validation/invalid concept). GPUI: per-size font via `size_font_rem` (was flat `body_size_token()`); focus ring now uses the shared token-backed `focus_ring_shadow()` helper (dropped the `0.28` magic alpha + `px(2.0)` spread). Jetstream: dropped `cursor_pointer()` on the idle field (contract has a text caret, not a pointer); placeholder aligned to GPUI's `HH:MM` (was `--:--`). Specimens add min/max + Densities groups and the full xs..xl size ladder. Editing/keyboard/min-max enforcement remain preview-loop / runtime (no native input — contract §12 Known Delta); GPUI is build-verified only, Jetstream probe-verified.

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/time-input.md`
- Svelte (authoritative): `packages/svelte/components/src/TimeInput.svelte`
- GPUI: `packages/gpui/preview/src/specimens/time_input.rs` via `poodle_render::time_input`
- Jetstream: `packages/jetstream/preview/src/specimens/time_input.rs` via `poodle_render::time_input`
- Spec: `packages/contracts/components/src/time_input.rs` (`TimeInputSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/TimeInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/time_input.rs` · jetstream `packages/jetstream/preview/src/specimens/time_input.rs`

Filename mapping: contract, Svelte, React, spec, and renderer all use `time-input` / `TimeInput` / `TimeInputSpec`.

## Contract ↔ Svelte

Svelte faithfully implements the contract's native-`<input type="time">` model: every contract prop (`id`, `value`, `defaultValue`, `min`, `max`, `step` default `60`, `size`, `sizeRole` default `"control"`, `density`, `disabled`, `ariaLabel`, `describedBy`) is present with matching defaults (TimeInput.svelte:6-36), and the `onValueChange` callback fires on native `input` (TimeInput.svelte:54-64). No 12/24h, segment (hh/mm/ss/period), placeholder, invalid, or clearable props exist on either side — correct: the contract delegates segment editing, placeholder, and AM/PM entirely to the platform native picker (contract §2, §6, §12).

Divergences:

- [x] FIXED Contract size table (§8) omitted `sm` font-size; Svelte sets `sm font-size: 0.8125rem` (TimeInput.svelte:110). Added the `sm` font-size row to §8 (noted it equals the md body-size baseline).
- [x] FIXED Contract `min-height` as `calc()` vs Svelte literal rem (`xs 1.5rem`, `sm 1.75rem`, `lg 2.75rem`, `xl 3.25rem`, TimeInput.svelte:109-118). Documented the literal-rem choice in §8 (kept `calc()` as the intent; noted Svelte's literal resolution breaks token re-theming until it switches to `calc()`). Svelte-side cleanup left for code.
- [x] FIXED Density padding rows: Svelte emits `compact`/`comfortable` padding overrides (TimeInput.svelte:121-122); §8 had no density rows. Added a density adjustment table (horizontal padding only, per orthogonality rule).
- [x] FIXED (spec, not contract↔Svelte) **Rust spec `validation_state: ValidationState`** removed from `TimeFieldSpec`; `border_token()` now always returns `COLOR_BORDER_DEFAULT`. Both Rust borders no longer recolor on a state the contract/Svelte never defined. The GPUI `validation_state()` builder and the unused contracts-crate test usage were dropped.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED **Per-size font-size now applied** — body text size resolves from `size_font_rem(effective_size)` (xs 0.6875rem … xl 0.9375rem ladder), replacing the flat `body_size_token()`.
- [x] FIXED **Focus ring uses the token-backed helper** — `.focus(|s| s.border_color(focus_ring).shadow(focus_ring_shadow(focus_ring)))`. The `0.28` alpha multiplier and `px(2.0)` spread literals are gone; the ring now matches every other GPUI control.
- [x] FIXED **`validation_state` border removed** — field dropped from `TimeFieldSpec`; `border_token()` returns the default border. No more unsourced recolor.
- accepted (preview-loop / runtime): custom HH:MM editing, min/max clamping, HH:MM:SS, and direct digit entry are interaction the doc itself frames as runtime; GPUI is build-verified only here, so editing isn't wired in this pass.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`described_by`/`min`/`max`/`step` stored on spec but not surfaced (runtime-limited).
- accepted: GPUI provides custom text-display editing instead of a native picker (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- accepted (preview-loop): **No editing / keyboard / spin in the component** — `js_time_field` renders a static time-display (no native input; contract §12 Known Delta). Segment editing, arrow-key increment, and `onValueChange` are preview-loop concerns; not wired in this pass.
- [x] FIXED **`cursor_pointer()` dropped on the idle field** — the enabled branch leaves the cursor unchanged (contract input has a text caret, not a pointer). Disabled branch keeps `opacity` + `disabled(true)`.
- [x] FIXED **Placeholder aligned to `HH:MM`** (was `--:--`), matching the GPUI build.
- [x] FIXED **`validation_state` border removed** (same spec change as GPUI) — `border_token()` returns the default border.
- accepted: **`min`/`max`/`step` unused** — spec carries them but there's no in-component editor to enforce them; flag for when editing is wired.
- accepted: no ARIA channel (`aria_label`/`described_by` not surfaced; documented runtime limit).
- accepted: no native `input[type="time"]`; static display is the contract §12 Known Delta substitute. Border width `rem_to_px(0.0625)` (time_field.rs:65) is the contract literal `0.0625rem`, not a token violation.

## Specimen parity

- **Svelte covers** (`TimeInputSpecimen.svelte`): Default (empty + live value readout), With default value (`14:30`), With min/max (`09:00`/`08:00`/`18:00`), Disabled (`12:00`), plus **Sizes** and **Densities** tabs via SpecimenLayout snippets.
- **GPUI covers** (`time_field.rs`): Default (interactive, value readout via `on_change`), With default value, With min/max constraints, Disabled, plus Sizes and Densities tabs. — missing: nothing notable; closest parity of the three (it wires `on_change` for two examples).
- **Jetstream covers** (`time_field.rs`): With value (`14:30`), Placeholder, With min/max constraints (`09:00` / `08:00` / `18:00`), Sizes (full xs..xl ladder), Densities (compact/default/comfortable), Disabled (`16:45`). Value-readout/interaction stays preview-loop (no native input). Specimen parity reached.

## Notes

- The biggest cross-cutting issue is `validation_state` / `ValidationState` living in `TimeFieldSpec` and both Rust borders while being absent from contract and Svelte — it is unsourced surface, not a Svelte-parity gap. Resolve at the contract level before either Rust impl keeps rendering it.
- GPUI's per-size font omission and shadow-based focus ring are the two real visual-parity bugs; everything else in GPUL is an accepted runtime delta.
- Jetstream's component is render-only (no editing wired anywhere), which is the broadest functional gap of the three but consistent with the "no native input" Known Delta — only the missing specimen coverage and `cursor_pointer` are clear bugs.
- Border-width and radius resolve from tokens in both Rust impls; the only hardcoded-literal violations are GPUI's focus-ring `0.28` alpha + `px(2.0)` spread (time_field.rs:170-174). Jetstream has none.
