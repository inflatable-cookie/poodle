<!-- parity consv=ok gpui=0 jetstream=1 specimen=gap -->
<!-- pass 20: GPUI action buttons now composed Button::from_spec (ghost cancel /
     primary submit), submit label flips to "Submitting…" + both disabled while
     submitting; dropped px(12/6)/white()/manual-disabled-fill/hand-rolled-spinner
     literals; body gaps → space.stack.md. Build clean. Remaining GPUI: width/aria/
     controlled-open. -->
<!-- pass 41: FormDialogSpec gained `width: Option<DialogWidth>` (+ with_width).
     GPUI: added `from_spec` consuming FormDialogSpec; wired width/aria_label/size/
     density to the composed Dialog; columns now passed to FormLayout; submitting
     blocks Escape+backdrop dismiss. Jetstream: REBUILT to compose js_dialog (shell
     + backdrop + header + close + separator) + js_button (ghost cancel / primary
     submit) + js_form_actions (footer rail, top-separation off) + js_form_layout
     (body). Dropped hand-rolled panel/backdrop, Color::TRANSPARENT/WHITE, btn_px/py,
     border 1.0 / h 1.0, accent.mix(_,0.5), rem literals → tokens/Button. Width via
     DialogWidth preset; aria/size/density forwarded. 6 probe tests (title/body slot/
     default actions/submitting/error/custom-width/bare). Remaining Jetstream:
     real Field/TextInput body in specimen + controlled-open (preview-loop). -->
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

- [x] `from_spec` constructor added — `FormDialog::from_spec(&FormDialogSpec, theme)` copies title/labels/submitting/error/success/aria_label/width/columns/bare/size/density; `open` stays host-owned (immediate mode).
- [x] `width` support added — `FormDialog::width(DialogWidth)` (+ spec field) forwarded to `Dialog::width()`; contract §3/§8 custom width reachable via preset.
- [x] `ariaLabel` plumbing added — `FormDialog::aria_label()` forwarded to `Dialog::aria_label()`.
- [x] `columns` now passed through — `FormLayout::new(theme).columns(self.columns as usize)` (contract §7).
- [x] Cancel/Submit are composed `Button`s (ghost/primary) — fixed pass 20; reconfirmed.
- [x] Button paddings/gaps/colors/disabled-opacity — fixed pass 20 (Button tokens + `space.inline.sm` + no white/0.5 literals).
- [x] Submitting now blocks Escape + backdrop dismiss via `Dialog::dismiss_on_escape(!submitting)` / `dismiss_on_backdrop(!submitting)` (contract §6).
- accepted: no ARIA (gpui has no accessibility API) — `aria_modal`/`aria-labelledby`/`aria-describedby` and Escape/backdrop dismiss are delegated to/handled by the Dialog primitive, not FormDialog.
- accepted: controlled `open` / `onOpenChange` round-trip lives in the host app, not the component (matches GPUI immediate-mode pattern).

## Jetstream gap (vs Svelte + contract)

- [x] Now composes `js_dialog` for the shell — backdrop + panel + header (title/description) + close button + action separator all come from the Dialog primitive. Hand-rolled backdrop/panel/separator removed.
- [x] Now uses `js_button` — Cancel (ghost) + Submit (primary) composed from `js_button`; tone/variant/tokens match the Button contract. Submit label flips to "Submitting…"; both disabled while submitting (cancel via `is_submitting`, contract parity with Svelte).
- [x] `width` support added — `FormDialogSpec.width: Option<DialogWidth>` (+ `with_width`); forwarded to `DialogSpec::with_width()`. Probe-verified (Xl surface wider than default).
- [x] `aria_label` / `size` / `density` now forwarded to the composed `DialogSpec`; `columns` reaches `FormLayoutSpec`.
- [x] Raw-float gaps removed — body/section gap now derives from the size-font scale; the Dialog primitive owns panel padding/gaps from tokens.
- [x] Button paddings/colors/disabled-opacity now resolved inside `js_button` (no `btn_px`/`btn_py`, no `Color::TRANSPARENT`/`Color::WHITE`, no `accent.mix(_,0.5)` literal).
- [x] Panel border + separator come from `js_dialog` (no local `.border(1.0)`/`.h(1.0)` literals).
- accepted: no ARIA channel (no accessibility API in jetstream runtime).
- accepted: open/close + Escape + backdrop-click dismiss not wired in the preview loop — submitting-blocks-dismiss intent is encoded on the `DialogSpec` (`dismiss_on_escape`/`dismiss_on_backdrop`), but the host preview loop owns the actual dismiss path. Specimen renders the dialog inline, always-open.
- [ ] Specimen still uses a hand-rolled `field()` stub rather than real `Field`/`TextInput` — borderline mockup per repo "no fakes" rule; swap for composed components (specimen-only, not the component).

## Specimen parity

- Svelte covers (`FormDialogSpecimen.svelte`): Basic (trigger Button → open, async submit → close), With error state (async error callout), Shell mode with custom actions (`subtitle`, `width="40rem"`, `showDefaultActions=false`, custom `FormActions` snippet, async success). Interactive via trigger Buttons and `open` state.
- GPUI covers (`form_dialog_specimen.rs`): Basic, With error, Submitting state, Shell mode w/ custom actions (3 buttons incl. danger Reset), **Bare mode**. — missing: custom `width` demo; no interactive trigger/open (rendered inline, always-open); no async submit→close flow.
- Jetstream covers (`specimens/form_dialog.rs`): Default, With subtitle+error, Submitting state, Success confirmation, No-default-actions custom slot. — missing: **bare mode**, custom `width` demo, interactive open/close; fields are hand-rolled `div` stubs (`form_dialog.rs:16-26` `field()` helper hardcodes `text_size(11.0)`, `rem_to_px(2.0)` height, `border_1`) rather than real `Field`/`TextInput` components — borderline mockup per repo "no fakes" rule.

## Notes

- `consv=ok`: Svelte faithfully implements the contract; only a cosmetic class-name prefix mismatch (`poodle-form-dialog__*` in Svelte vs `.form-dialog__*` in contract §8) — optional contract touch-up, not a behavior gap.
- Both Rust targets diverge structurally from "compose the Dialog primitive": GPUI does compose `Dialog` (good) but bypasses the `Button` primitive for actions; Jetstream composes neither `js_dialog` nor `js_button`, hand-rendering the whole shell — the largest parity risk because surface/backdrop/dismiss semantics drift from the Dialog contract.
- The Jetstream specimen's hand-rolled `field()` helper hardcodes pixel values and is not a real `Field`/`TextInput`; per CLAUDE.md "no mockups", it should use real composed components or be left out.
- Both Rust specimens render the dialog inline and always-open (no trigger/controlled `open`), so the `closed` state and async submit→close lifecycle from the Svelte specimen are untested in the previews.
