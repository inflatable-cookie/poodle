<!-- parity consv=fixed gpui=4 jetstream=5 specimen=gap -->
# Parity: TimeInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/time-input.md`
- Svelte (authoritative): `packages/svelte/components/src/TimeInput.svelte`
- GPUI: `packages/gpui/components/src/primitives/time_field.rs`
- Jetstream: `packages/jetstream/components/src/time_field.rs`
- Spec: `packages/contracts/components/src/time_field.rs` (`TimeFieldSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/TimeInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/time_field.rs` · jetstream `packages/jetstream/preview/src/specimens/time_field.rs`

Filename mapping: contract + Svelte use `time-input`/`TimeInput`; both Rust impls and the spec use the legacy `time_field.rs` / `TimeFieldSpec` name. Same component (noted in spec file header and contract §"Rust Spec Note").

## Contract ↔ Svelte

Svelte faithfully implements the contract's native-`<input type="time">` model: every contract prop (`id`, `value`, `defaultValue`, `min`, `max`, `step` default `60`, `size`, `sizeRole` default `"control"`, `density`, `disabled`, `ariaLabel`, `describedBy`) is present with matching defaults (TimeInput.svelte:6-36), and the `onValueChange` callback fires on native `input` (TimeInput.svelte:54-64). No 12/24h, segment (hh/mm/ss/period), placeholder, invalid, or clearable props exist on either side — correct: the contract delegates segment editing, placeholder, and AM/PM entirely to the platform native picker (contract §2, §6, §12).

Divergences:

- [x] FIXED Contract size table (§8) omitted `sm` font-size; Svelte sets `sm font-size: 0.8125rem` (TimeInput.svelte:110). Added the `sm` font-size row to §8 (noted it equals the md body-size baseline).
- [x] FIXED Contract `min-height` as `calc()` vs Svelte literal rem (`xs 1.5rem`, `sm 1.75rem`, `lg 2.75rem`, `xl 3.25rem`, TimeInput.svelte:109-118). Documented the literal-rem choice in §8 (kept `calc()` as the intent; noted Svelte's literal resolution breaks token re-theming until it switches to `calc()`). Svelte-side cleanup left for code.
- [x] FIXED Density padding rows: Svelte emits `compact`/`comfortable` padding overrides (TimeInput.svelte:121-122); §8 had no density rows. Added a density adjustment table (horizontal padding only, per orthogonality rule).
- [ ] (spec, not contract↔Svelte) **Rust spec `validation_state: ValidationState`** swaps the border to danger/success/accent (time_field.rs:73-75). Neither contract §3 nor Svelte has any validation concept — this is unsourced Rust surface. Per "never invent contract surface Svelte lacks," the contract correctly omits it; resolve in code (drop the spec field, or add `invalid?` to Svelte + contract first). Left for Rust pass.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Per-size font-size not applied** — GPUI uses the flat `body_size_token()` for all sizes (time_field.rs:126,163) and never reads a per-size font. Contract §8 requires `xs 0.75rem`, `lg 0.9375rem`, `xl 1rem`; Svelte and Jetstream both vary it. Resolve a per-size font (mirror `size_font_rem` / Jetstream's `time_font_size_rem`).
- [ ] **Focus ring is a shadow, not the contract outline** — GPUI draws a 2px box-shadow at `focus_ring.a * 0.28` (time_field.rs:166-176). Contract §8 focus spec is `outline: border-width-focus solid accent-focusRing` + `outline-offset: 0.125rem`. The `0.28` alpha multiplier and `spread_radius: px(2.0)` are hardcoded literals with no token backing. Resolve focus width/offset from tokens; drop the `0.28` magic alpha.
- [ ] **`validation_state` border applied but unsourced from Svelte** — `border_token()` lets Invalid/Valid/Pending recolor the border (via spec), but Svelte has no such state; this renders a divergence that the contract doesn't define. Gate behind a contract decision (see Contract↔Svelte note) or do not expose it.
- [ ] **Custom editing is up-only/down-only by step, parses HH:MM only** — key handler (time_field.rs:189-214) ignores `min`/`max` clamping (spec carries `min`/`max` but they're never enforced) and never handles HH:MM:SS or direct digit entry. Contract §6 keyboard + §10 require min/max/step honored in the custom editor.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`described_by`/`min`/`max`/`step` stored on spec but not surfaced to an accessibility tree (contract §10 wants them exposed; runtime-limited here).
- accepted: GPUI provides custom text-display editing instead of a native picker (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] **No editing / keyboard / spin at all** — `js_time_field` renders a static `button(display_text)` (time_field.rs:75-88); there is no segment editing, arrow-key increment, or `onValueChange`. Unlike GPUI (which wires arrow keys in the component) and Svelte (native), Jetstream has zero value mutation, and none is wired in preview `main.rs`/specimen either. Contract §6 keyboard + §5 callback unmet.
- [ ] **Renders as `button(...)` element, not an input** — uses the button builder + `cursor_pointer()` (time_field.rs:75,95); anatomy is a single time-display, acceptable as the no-native-input substitute, but `cursor_pointer` on an idle field is wrong (contract has no pointer cursor; native is text caret). Drop `cursor_pointer()` for the non-disabled branch.
- [ ] **Placeholder glyph differs** — empty value shows `"--:--"` (time_field.rs:70) where GPUI shows `"HH:MM"` (time_field.rs:136) and Svelte defers to the platform placeholder. Pick one placeholder convention; align with GPUI or document.
- [ ] **`validation_state` border (same as GPUI)** — `border_token()` honors validation state with no Svelte/contract backing. Gate behind contract decision or drop.
- [ ] **`min`/`max`/`step` unused** — spec carries them but `js_time_field` never reads them and there's no editor to enforce them. Accepted only insofar as there's no editing; flag for when editing is added.
- accepted: no ARIA channel (`aria_label`/`described_by` not surfaced; documented runtime limit).
- accepted: no native `input[type="time"]`; static display is the contract §12 Known Delta substitute. Border width `rem_to_px(0.0625)` (time_field.rs:65) is the contract literal `0.0625rem`, not a token violation.

## Specimen parity

- **Svelte covers** (`TimeInputSpecimen.svelte`): Default (empty + live value readout), With default value (`14:30`), With min/max (`09:00`/`08:00`/`18:00`), Disabled (`12:00`), plus **Sizes** and **Densities** tabs via SpecimenLayout snippets.
- **GPUI covers** (`time_field.rs`): Default (interactive, value readout via `on_change`), With default value, With min/max constraints, Disabled, plus Sizes and Densities tabs. — missing: nothing notable; closest parity of the three (it wires `on_change` for two examples).
- **Jetstream covers** (`time_field.rs`): With value (`14:30`), Placeholder, Sizes (Sm/Md/Lg only), Disabled. — **missing: With min/max constraints group**, **Densities** group, **xs and xl sizes** (only Sm/Md/Lg shown), and any value-readout/interaction. `specimen=gap` driven by Jetstream.

## Notes

- The biggest cross-cutting issue is `validation_state` / `ValidationState` living in `TimeFieldSpec` and both Rust borders while being absent from contract and Svelte — it is unsourced surface, not a Svelte-parity gap. Resolve at the contract level before either Rust impl keeps rendering it.
- GPUI's per-size font omission and shadow-based focus ring are the two real visual-parity bugs; everything else in GPUL is an accepted runtime delta.
- Jetstream's component is render-only (no editing wired anywhere), which is the broadest functional gap of the three but consistent with the "no native input" Known Delta — only the missing specimen coverage and `cursor_pointer` are clear bugs.
- Border-width and radius resolve from tokens in both Rust impls; the only hardcoded-literal violations are GPUI's focus-ring `0.28` alpha + `px(2.0)` spread (time_field.rs:170-174). Jetstream has none.
