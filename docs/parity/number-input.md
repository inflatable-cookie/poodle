<!-- parity consv=gap gpui=5 jetstream=6 specimen=gap -->
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

- **`role="spinbutton"` + `aria-valuenow`/`aria-valuemin`/`aria-valuemax`**: contract §2 requires these on the root input. Svelte renders `<input type="text" inputmode="decimal">` with NO `role` and NO `aria-value*` (lines 282–318). Divergence. **Fix: either add spinbutton role + value ARIA to Svelte (it is genuine contract-specified a11y functionality the reference lacks), or downgrade the contract. Per "Svelte is parity authority", prefer adding to Svelte since this is missing contract functionality, not invented surface.**
- **`aria-disabled="true"` on root**: contract §2. Svelte uses the native `disabled` attribute only (line 290), no `aria-disabled`. **Fix: contract should accept native `disabled` as equivalent, or Svelte adds `aria-disabled`.**
- **Stepper `aria-label="Increment"`/`"Decrement"`**: contract §2. Svelte stepper `<button>`s (lines 322–327) have no `aria-label` — only an `Icon`, so they are unlabeled to AT. **Fix: add `aria-label` to Svelte steppers (contract-specified, reference lacks it).**
- **`Home`/`End` → min/max**: contract §2 keyboard list. Svelte `onkeydown` handles only `Enter`, `ArrowUp`, `ArrowDown` (lines 303–317); no `Home`/`End`. **Fix: add `Home`/`End` to Svelte, or mark "when supported" in contract as not-yet-implemented.**
- **Stepper glyph**: contract calls them "stepper buttons" (icon unspecified). Svelte uses `plus`/`minus` icons (lines 323/326). Not a contract conflict, but the cross-target reference point for Rust impls. **Note: Svelte = plus/minus, not chevrons.**
- **`aria-describedby` link to validation message**: contract §2 wires `aria-describedby` to a validation message element. Svelte exposes `describedBy` prop (passed through, line 294) but renders NO validation-message element to point at and does not auto-generate one. **Fix: document that the message element is consumer-supplied, or add an internal message region to Svelte.**
- Props/callbacks otherwise match contract Core Props + Callbacks lists exactly (value/min/max/step/precision/prefix/suffix/validate/validationState/showSteppers/standard control props; onValueChange/onValidationChange/onSubmit/onIncrement/onDecrement/onFocus/onBlur). No undocumented Svelte surface here.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Stepper glyph mismatch**: GPUI uses `chevron-up`/`chevron-down` (`number_input.rs:192,218`); Svelte authoritative uses `plus`/`minus`. Switch to plus/minus to match the reference.
- [ ] **Steppers always rendered**: GPUI always emits the stepper column (`number_input.rs:317-318`); spec/Svelte gate steppers behind `showSteppers` (Svelte line 320). Spec has no `show_steppers` field — add it and gate, or accept always-on as a documented GPUI delta.
- [ ] **No min/max clamping on stepper press**: increment/decrement handlers live entirely in the specimen; the component's `on_increment`/`on_decrement` are opaque callbacks (`number_input.rs:103-123`) with no built-in clamp/step/`disabled-at-bound` logic. Svelte `adjust()` clamps + snaps to step (Svelte lines 255–267). Bound-disabled stepper state (present in Jetstream) is absent here.
- [ ] **No `precision`/prefix/suffix divider parity check**: prefix/suffix render as inline text (`number_input.rs:267-280`) without the bordered affix chrome Svelte gives them (Svelte `.poodle-number-input__prefix` has border + surface bg, lines 344–354). GPUI affixes are plain text-secondary spans — visual gap vs Svelte's boxed affix.
- [ ] **No read-only / required / placeholder-empty handling beyond `value==0` heuristic**: placeholder only shows when `display_value=="0" && value==0.0` (`number_input.rs:245-246`); `is_read_only`/`is_required` spec fields are ignored.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but never emitted; no spinbutton role.
- accepted: focus ring spread `px(2.0)` at `number_input.rs:314` and `px(0.0)` offsets/blur are structural box-shadow geometry, not color/size token targets — Svelte focus ring is also fixed `0 0 0 <focus-width>`. Border-width token (`--poodle-border-width-focus`) is the only tokenizable value; consider resolving spread from it.
- accepted: `px(rem_to_px(1.25))` stepper width (`:164`), `px(rem_to_px(0.125))` inner-radius inset (`:165`), `px(1.0)` stepper padding (`:236`), `px(rem_to_px(0.5))` gap (`:263`) are all contract-fixed rem constants mirroring Svelte literals (stepper width `calc(icon-md + 0.5rem)`, padding `0.0625rem`, gap `0.5rem`) — rem-based, not raw px; acceptable.

## Jetstream gap (vs Svelte + contract)

- [ ] **Stepper glyph mismatch**: uses `plus`/`minus` — matches Svelte. OK. (No action; recorded for contrast with GPUI.)
- [ ] **Steppers always rendered**: `js_number_input` always appends `dec_btn`/`inc_btn` (`number_input.rs:76,130`); no `showSteppers` gate. Same spec-field gap as GPUI.
- [ ] **Steppers are non-interactive**: `js_number_input` builds buttons with no click/keyboard wiring; increment/decrement/clamp must live in the preview event loop. Confirm the preview main.rs drives them — current specimen (`preview/src/specimens/number_input.rs`) only renders static specs, so steppers do nothing.
- [ ] **`btn_gap = rem_to_px(0.25)` hardcoded** (`number_input.rs:23`): the inner stepper gap is an ad-hoc `0.25rem` constant with no token target. Resolve from a spacing token (e.g. a content-gap / `space.control.x` derivation) rather than a bare rem literal.
- [ ] **Affix divider `w(1.0)` hardcoded** (`number_input.rs:81,119`): 1px separator width is a raw literal; use a border-width token (`border.width.hairline`/equivalent) resolved to px. Also Svelte affixes use a full bordered box + surface bg (Svelte lines 344–354), not a single divider line — visual anatomy gap.
- [ ] **No validation border color**: validation state only swaps in a trailing `alert-circle` icon for `Invalid` (`number_input.rs:102-114`); the root border stays `border_token()`. Svelte recolors the field border per validation state (Svelte lines 369–379) and GPUI does too (`number_input.rs:285-290`). Jetstream omits the border recolor — add it.
- [ ] **No focus ring**: no `focus(...)` treatment on the root; Svelte has `:focus-within` ring (Svelte lines 381–385), GPUI has it (`number_input.rs:306-316`). Jetstream root has none.
- accepted: `.border(1.0)` at `number_input.rs:71` is a structural border-presence flag (width), paired with `border_color(border)` token — matches Svelte `0.0625rem` hairline; acceptable as a width literal pending a border-width token.
- accepted: no ARIA channel (documented engine limit); `aria_label` unused.
- accepted: interaction (click/key handling) lives in preview event loop, not the component.

## Specimen parity

- **Svelte covers**: Numeric value, With steppers (step+precision), String-form binding (prefix), Disabled, Invalid, Sizes snippet, Densities snippet (`NumberInputSpecimen.svelte`).
- **GPUI covers**: Default, With steppers, Disabled, Invalid, Prefix (currency), Suffix (unit), Precision (3dp), plus Sizes + Densities via `specimen_layout`. Broadest of the three. — missing: nothing material vs Svelte; actually exceeds it (suffix, precision groups). Steppers always-on so "With steppers" group is not isolating `showSteppers` behavior.
- **Jetstream covers**: Default(50), At min(0), At max(100), Disabled, Invalid, With prefix($), With suffix(px) (`preview/src/specimens/number_input.rs`). — missing: **Sizes** group and **Densities** group (Svelte + GPUI both show them; Jetstream specimen omits both). No precision-isolation group. → specimen=gap driven by Jetstream's missing size/density coverage.

## Notes

- `NumberInputSpec` (`packages/contracts/components/src/number_input.rs`) has `is_read_only`, `is_required`, `placeholder` fields and `formatted_value()`/`clamped_value()` helpers, but **no `show_steppers` field** — both Rust targets render steppers unconditionally as a result. Adding `show_steppers` to the spec is the cleanest fix for the always-on stepper gap in both targets.
- Spec lacks a `validate`/validation-callback surface entirely (it is a render-only spec); async validation, `onValidationChange`, and `onSubmit` are Svelte-only and out of scope for the Rust targets by design — not counted as gaps.
- The big `consv=gap` driver is the contract's accessibility section (spinbutton role, `aria-value*`, stepper `aria-label`, `Home`/`End`, `aria-describedby` message wiring) that the authoritative Svelte does not implement. These are contract-specified functionality the reference lacks, so the fix direction is "add to Svelte", not "delete from contract".
- Stepper glyph divergence: GPUI uses chevrons, Svelte + Jetstream use plus/minus. GPUI is the odd one out vs the reference.
