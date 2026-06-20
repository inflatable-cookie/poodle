<!-- parity consv=ok gpui=3 jetstream=9 specimen=gap -->
<!-- pass 20: GPUI action buttons now composed Button::from_spec (ghost cancel /
     primary submit), submit label flips to "Submitting…" + both disabled while
     submitting; dropped px(12/6)/white()/manual-disabled-fill/hand-rolled-spinner
     literals; body gaps → space.stack.md. Build clean. Remaining GPUI: width/aria/
     controlled-open. -->
# Parity: FormDialog

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/form-dialog.md`
- Svelte (authoritative): `packages/svelte/components/src/FormDialog.svelte`
- GPUI: `packages/gpui/components/src/composites/form_dialog.rs`
- Jetstream: `packages/jetstream/components/src/form_dialog.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/FormDialogSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/form_dialog_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/form_dialog.rs`

## Contract ↔ Svelte

Svelte matches the contract on every prop, default, snippet, callback, state, and token. No divergence found.

- Props (`FormDialog.svelte:11-36`) match contract §3 exactly: `open`/`title`/`subtitle`/`description`/`submitLabel`/`cancelLabel`/`submitting`/`error`/`success`/`ariaLabel`/`width`/`columns`/`showDefaultActions`/`bare`/`size`/`sizeRole`/`density`. Defaults match (`submitLabel="Submit"`, `cancelLabel="Cancel"`, `columns=6`, `showDefaultActions=true`, `bare=false`, `sizeRole="control"`).
- `resolvedShowActions = bare ? false : showDefaultActions` (`:70`) matches contract §3/§9 "bare auto-sets showDefaultActions false".
- `resolvedDescription = subtitle ?? description` (`:71`) matches contract §9 "subtitle takes precedence".
- Subtitle snippet present → `description={null}` on Dialog (`:110`) matches contract §6 "avoid duplicate description announcement".
- `dismissOnEscape={!submitting}` / `dismissOnBackdrop={!submitting}` (`:115-116`) match contract §6.
- `showCloseButton={true}` always (`:117`) matches contract §6.
- Submit text `submitting ? "Submitting..." : submitLabel` (`:170`) matches contract §4.
- Custom width via `--poodle-form-dialog-width` + `.form-dialog__surface` `min(...,100%)` (`:72-73,177-179`) matches contract §8. (Note: Svelte class is `poodle-form-dialog__*`; contract §8 lists `.form-dialog__*` without the `poodle-` prefix — cosmetic naming mismatch in the contract, not a behavior gap. Optional: align contract class names.)
- Callbacks `onSubmit`/`onCancel`/`onOpenChange` and cancel-suppression-during-submitting (`:98-104`) match contract §5/§9.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Does not consume `FormDialogSpec` — hand-rolled builder (`form_dialog.rs:12-145`). Spec exists (`contracts/components/src/form_dialog.rs`) with all props; GPUI ignores it, so `open`/`columns`/`aria_label`/`size`/`size_role`/`density`/`width` have no path. Refactor to a `from_spec` constructor.
- [ ] No `width` support — Dialog primitive exposes `.width(DialogWidth)` (`primitives/dialog.rs:121`) but FormDialog never calls it; contract §3/§8 custom width unreachable.
- [ ] No `ariaLabel` plumbing — Dialog primitive exposes `.aria_label()` (`primitives/dialog.rs:105`); FormDialog never forwards it.
- [ ] `columns` accepted but discarded — `let _ = self.columns;` at `form_dialog.rs:208`; never passed to `FormLayout`. Contract §7 requires passthrough.
- [ ] Cancel/Submit are hand-rolled `div`s (`form_dialog.rs:219-268`), not the `Button` primitive. Contract anatomy §2 requires ghost Cancel + primary Submit `Button`s; reuse `Button` for variant/tone/token parity.
- [ ] Hardcoded button paddings `.px(px(12.0))`/`.py(px(6.0))` at `form_dialog.rs:224-225,253-254` — resolve from Button/control padding tokens.
- [ ] Hardcoded gaps `.gap(px(12.0))` (body, `:172`,`:195`) and `.gap(px(6.0))` (submit spinner row, `:260`) — resolve from `space.*` tokens.
- [ ] Hardcoded color `gpui::white()` for submit-button text (`form_dialog.rs:250`) and spinner mix base (`:263`) — resolve from an on-accent text token, not a raw white literal.
- [ ] Disabled/submitting uses raw `.opacity(0.5)` for cancel (`form_dialog.rs:228`) and `color_mix(accent, panel_bg, 0.5)` for submit fill (`:241-244`) — use `disabled_opacity_token()` (`state.opacity.disabled`), not a hardcoded `0.5`.
- accepted: no ARIA (gpui has no accessibility API) — `aria_modal`/`aria-labelledby`/`aria-describedby` and Escape/backdrop dismiss are delegated to/handled by the Dialog primitive, not FormDialog.
- accepted: controlled `open` / `onOpenChange` round-trip lives in the host app, not the component (matches GPUI immediate-mode pattern).

## Jetstream gap (vs Svelte + contract)

- [ ] Does not compose `js_dialog` despite the module doc claiming "Composes js_dialog + js_form_layout" (`form_dialog.rs:6`). It hand-renders its own backdrop + panel + separator (`:142-192`). Result: dialog surface/backdrop/close-button/focus-trap diverge from the Dialog contract. Compose `js_dialog` for the shell.
- [ ] No `js_button` — Cancel/Submit are raw `ui_element::button` divs (`form_dialog.rs:98-122`). Contract §2 requires ghost Cancel + primary Submit `Button`s; use `js_button` so tone/variant/tokens match.
- [ ] No `width` support — `FormDialogSpec` has no `width` field and the panel uses a fixed `min_w(rem_to_px(25.0))` (`form_dialog.rs:149`); contract §3/§8 custom width is unreachable. Add a width field/token and wire it.
- [ ] No `aria_label` / `columns` / `size` / `density` applied to the dialog shell — `columns` reaches `FormLayoutSpec` (`:71`) but `aria_label` is never read and `size`/`density` only size the body font, not the (absent) Dialog shell.
- [ ] Hardcoded gaps via raw floats: `title_gap = rem_to_px(0.5)` (`:50`), `section_gap = rem_to_px(1.0)` (`:51`) — resolve from `space.stack.*` tokens, not literal rem.
- [ ] Hardcoded button paddings `btn_px = rem_to_px(0.75)` / `btn_py = rem_to_px(0.375)` (`form_dialog.rs:94-95`) — resolve from control padding tokens.
- [ ] Hardcoded `.border(1.0)` panel border width (`:145`) and `.h(1.0)` separator (`:179`) — resolve from a border-width token.
- [ ] Hardcoded color literals `Color::TRANSPARENT` cancel bg (`:103`) and `Color::WHITE` submit text (`:119`) — resolve cancel ghost bg + on-accent text from tokens.
- [ ] Disabled/submitting submit fill uses raw `accent.mix(panel_bg, 0.5)` (`form_dialog.rs:107-111`) — the `0.5` is a literal; cancel button is never disabled/dimmed during submitting (Svelte disables both, `FormDialog.svelte:159,168`).
- accepted: no ARIA channel (no accessibility API in jetstream runtime).
- accepted: open/close + Escape + backdrop-click dismiss not wired — `jetstream/preview/src/main.rs` Escape only exits the app or dismisses the tree context menu (`main.rs:411,449`); no FormDialog dismiss path. Specimen renders the dialog inline, always-open.

## Specimen parity

- Svelte covers (`FormDialogSpecimen.svelte`): Basic (trigger Button → open, async submit → close), With error state (async error callout), Shell mode with custom actions (`subtitle`, `width="40rem"`, `showDefaultActions=false`, custom `FormActions` snippet, async success). Interactive via trigger Buttons and `open` state.
- GPUI covers (`form_dialog_specimen.rs`): Basic, With error, Submitting state, Shell mode w/ custom actions (3 buttons incl. danger Reset), **Bare mode**. — missing: custom `width` demo; no interactive trigger/open (rendered inline, always-open); no async submit→close flow.
- Jetstream covers (`specimens/form_dialog.rs`): Default, With subtitle+error, Submitting state, Success confirmation, No-default-actions custom slot. — missing: **bare mode**, custom `width` demo, interactive open/close; fields are hand-rolled `div` stubs (`form_dialog.rs:16-26` `field()` helper hardcodes `text_size(11.0)`, `rem_to_px(2.0)` height, `border_1`) rather than real `Field`/`TextInput` components — borderline mockup per repo "no fakes" rule.

## Notes

- `consv=ok`: Svelte faithfully implements the contract; only a cosmetic class-name prefix mismatch (`poodle-form-dialog__*` in Svelte vs `.form-dialog__*` in contract §8) — optional contract touch-up, not a behavior gap.
- Both Rust targets diverge structurally from "compose the Dialog primitive": GPUI does compose `Dialog` (good) but bypasses the `Button` primitive for actions; Jetstream composes neither `js_dialog` nor `js_button`, hand-rendering the whole shell — the largest parity risk because surface/backdrop/dismiss semantics drift from the Dialog contract.
- The Jetstream specimen's hand-rolled `field()` helper hardcodes pixel values and is not a real `Field`/`TextInput`; per CLAUDE.md "no mockups", it should use real composed components or be left out.
- Both Rust specimens render the dialog inline and always-open (no trigger/controlled `open`), so the `closed` state and async submit→close lifecycle from the Svelte specimen are untested in the previews.
