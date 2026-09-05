<!-- parity consv=gap gpui=1 jetstream=2 specimen=ok | pass 41: show_steppers gate, plus/minus glyph, boxed affixes, validation border (jet), tri-state/number token resolution; specimen sizes+densities added -->
# Parity: NumberInput

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/number-input.md`
- Svelte (authoritative): `packages/svelte/components/src/NumberInput.svelte`
- GPUI: `packages/gpui/components/src/primitives/number_input.rs`
- Jetstream: `packages/jetstream/components/src/number_input.rs`
- Spec struct: `packages/contracts/components/src/number_input.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/NumberInputSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/number_input.rs` · jetstream `packages/jetstream/preview/src/specimens/number_input.rs`

## Contract ↔ Svelte

Contract over-specifies accessibility the authoritative Svelte does not implement, and the two disagree on stepper anatomy. Svelte wins on behavior; contract a11y items are aspirational. Resolve each explicitly.

CONTRACT LEFT INTACT (per task rule: these are contract-specified a11y the
reference lacks — Svelte-side gaps, fix direction is "add to Svelte", never
weaken the contract). Each below is a Svelte TODO, not a contract edit:

- [ ] SVELTE GAP — **`role="spinbutton"` + `aria-valuenow`/`aria-valuemin`/`aria-valuemax`**: contract §2 requires these on the root input. Svelte renders `<input type="text" inputmode="decimal">` with NO `role` and NO `aria-value*` (lines 282–318). Contract kept; Svelte must add spinbutton role + value ARIA.
- [ ] SVELTE GAP — **`aria-disabled="true"` on root**: contract §2. Svelte uses the native `disabled` attribute only (line 290), no `aria-disabled`. Contract kept; native `disabled` is *close* but the contract's explicit `aria-disabled` requirement stands as a Svelte TODO.
- [ ] SVELTE GAP — **Stepper `aria-label="Increment"`/`"Decrement"`**: contract §2. Svelte stepper `<button>`s (lines 322–327) have no `aria-label` — only an `Icon`, so they are unlabeled to AT. Contract kept; Svelte must add `aria-label`.
- [ ] SVELTE GAP — **`Home`/`End` → min/max**: contract §2 keyboard list. Svelte `onkeydown` handles only `Enter`, `ArrowUp`, `ArrowDown` (lines 303–317); no `Home`/`End`. Contract already hedges "when supported"; Svelte must add `Home`/`End`.
- [ ] SVELTE GAP — **`aria-describedby` link to validation message**: contract §2 wires `aria-describedby` to a validation message element. Svelte exposes `describedBy` (passed through, line 294) but renders NO validation-message element and does not auto-generate one. Contract kept; either Svelte supplies an internal message region or documents the element as consumer-supplied.
- Note (no action): **Stepper glyph** — Svelte uses `plus`/`minus` icons (lines 323/326). Contract leaves the glyph unspecified; this is the cross-target reference point (GPUI uses chevrons — GPUI-side delta, tracked below). No contract change.
- Props/callbacks otherwise match contract Core Props + Callbacks lists exactly. No undocumented Svelte surface, no stale contract numeric/token value Svelte superseded → no contract edits warranted.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED **Stepper glyph mismatch**: GPUI now uses `plus`/`minus` (matches Svelte).
- [x] FIXED **Steppers always rendered**: added additive `show_steppers` field to `NumberInputSpec`; GPUI gates the stepper column on it (`with_steppers(true)` in the specimen's "With steppers" group).
- [ ] preview-loop **No min/max clamping on stepper press**: increment/decrement handlers live in the specimen; the component's `on_increment`/`on_decrement` stay opaque callbacks. Clamp/step/`disabled-at-bound` are driven by the preview event loop by design (render-only spec). `is_read_only`/`is_required` are likewise non-visual here (only affect editing, which is preview-loop) — accepted.
- [x] FIXED **Affix chrome**: prefix/suffix now render as boxed affixes (border-default box + surface bg + muted text, control radius) via the new `affix_box` builder, matching Svelte's `.poodle-number-input__prefix`.
- accepted: placeholder still uses the `value==0` heuristic (Svelte ::placeholder shows on empty string; the f64-typed spec has no empty state, so the heuristic is the faithful render-only approximation).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but never emitted; no spinbutton role.
- accepted: focus ring spread `px(2.0)` at `number_input.rs:314` and `px(0.0)` offsets/blur are structural box-shadow geometry, not color/size token targets — Svelte focus ring is also fixed `0 0 0 <focus-width>`. Border-width token (`--poodle-border-width-focus`) is the only tokenizable value; consider resolving spread from it.
- accepted: `px(rem_to_px(1.25))` stepper width (`:164`), `px(rem_to_px(0.125))` inner-radius inset (`:165`), `px(1.0)` stepper padding (`:236`), `px(rem_to_px(0.5))` gap (`:263`) are all contract-fixed rem constants mirroring Svelte literals (stepper width `calc(icon-md + 0.5rem)`, padding `0.0625rem`, gap `0.5rem`) — rem-based, not raw px; acceptable.

## Jetstream gap (vs Svelte + contract)

- ok **Stepper glyph**: uses `plus`/`minus` — matches Svelte. (No action.)
- [x] FIXED **Steppers always rendered**: `js_number_input` now gates `dec_btn`/`inc_btn` behind `spec.show_steppers` (off by default, matching Svelte `showSteppers=false`).
- [ ] preview-loop **Steppers non-interactive**: stepper buttons carry stable ids (`poodle-number-input-{inc,dec}`) but no click/key wiring inside the component; increment/decrement/clamp are driven by the preview event loop by design (immediate-mode runtime). Accepted.
- [x] FIXED **`btn_gap` token**: inner stepper gap now resolves from `space.inline.xs` via `spec.stepper_gap_token()` (was a bare `rem_to_px(0.25)`); affix/field border width resolves from `border.width.default` via `spec.border_width_token()`.
- [x] FIXED **Boxed affixes**: prefix/suffix now render as bordered boxes (border-default + surface bg + muted text, full control height) via the `affix_box` helper, replacing the single `w(1.0)` divider line — matches Svelte's boxed affix anatomy.
- [x] FIXED **Validation border color**: the root field border now recolors per validation state (danger/success/accent), matching Svelte + GPUI; the trailing `alert-circle` icon is retained for `Invalid`.
- [ ] JsEl-gap **No focus ring**: the immediate-mode runtime has no `:focus-within` style hook, so the root focus ring cannot be expressed. Steppers are `.focusable()`. Accepted as a documented JsEl limitation.
- accepted: `.border(1.0)` at `number_input.rs:71` is a structural border-presence flag (width), paired with `border_color(border)` token — matches Svelte `0.0625rem` hairline; acceptable as a width literal pending a border-width token.
- accepted: no ARIA channel (documented engine limit); `aria_label` unused.
- accepted: interaction (click/key handling) lives in preview event loop, not the component.

## Specimen parity

- **Svelte covers**: Numeric value, With steppers (step+precision), String-form binding (prefix), Disabled, Invalid, Sizes snippet, Densities snippet (`NumberInputSpecimen.svelte`).
- **GPUI covers**: Default, With steppers, Disabled, Invalid, Prefix (currency), Suffix (unit), Precision (3dp), plus Sizes + Densities via `specimen_layout`. Broadest of the three. — missing: nothing material vs Svelte; actually exceeds it (suffix, precision groups). Steppers always-on so "With steppers" group is not isolating `showSteppers` behavior.
- **Jetstream covers**: Default(50), At min(0), At max(100), Disabled, Invalid, With prefix($), With suffix(px), **With steppers**, **Sizes (xs–xl)**, **Densities (compact/default/comfortable)** (`preview/src/specimens/number_input.rs`). Sizes + densities + steppers groups added this pass → specimen=ok.

## Notes

- `NumberInputSpec` (`packages/contracts/components/src/number_input.rs`) now has an additive `show_steppers: bool` field (default `false`, `with_steppers()` builder) — both Rust targets gate the stepper column on it. Also added `stepper_gap_token()` (`space.inline.xs`), `border_width_token()` (`border.width.default`), and `affix_fill_token()`/`affix_border_token()`/`affix_text_token()` for the boxed affix chrome.
- Token gap: Svelte affix text uses `--poodle-color-text-muted`; the Rust semantic set has no `text.muted` token, so `affix_text_token()` falls back to `color.text.secondary` (closest available). Add a `text.muted` semantic token to close this exactly.
- Spec lacks a `validate`/validation-callback surface entirely (it is a render-only spec); async validation, `onValidationChange`, and `onSubmit` are Svelte-only and out of scope for the Rust targets by design — not counted as gaps.
- `consv=gap` (intentionally held): the contract's accessibility section (spinbutton role, `aria-value*`, stepper `aria-label`, `Home`/`End`, `aria-describedby` message wiring) is contract-specified functionality the authoritative Svelte does not implement. Per the no-weakening rule the contract was LEFT INTACT — the fix direction is "add to Svelte" (code), so the divergence cannot be closed by a contract edit. `consv` stays `gap` until Svelte ships the a11y, NOT because the contract is wrong. No contract file changes were made for number-input.
- Stepper glyph divergence: GPUI uses chevrons, Svelte + Jetstream use plus/minus. GPUI is the odd one out vs the reference.
