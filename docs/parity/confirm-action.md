<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- pass 44: specimens backfilled with REAL components. GPUI confirm_action_specimen.rs
     already covered default-trigger(danger)/warning/custom-ghost-trigger/body-content
     interactively — left intact (full coverage). Jetstream confirm_action.rs rebuilt: closed
     default trigger (danger + default tones) + open confirm dialog (danger + default tones),
     all via js_confirm_action (secondary trigger Button / delegated js_alert_dialog). Custom
     trigger + body slot remain AlertDialog-side limits, not faked. Both previews build clean. -->
<!-- pass 19: GPUI rebuilt — was hand-rolled trigger div + hand-rolled dialog (literal
     shadows, min_w(360), white() text). Now composes a real Button trigger + delegates
     the open state entirely to the AlertDialog primitive (tone/labels/size/density/body),
     so backdrop/escape dismiss + Button states work. No literals remain. Build clean.
     Jetstream still hand-rolls (same pass could apply js_alert_dialog there). -->
<!-- pass 41: Jetstream rebuilt to match GPUI — was hand-rolled trigger button + inline
     dialog (rem_to_px padding, max_w(28rem), ad-hoc title size, white-ish confirm text,
     hand-rolled cancel/confirm buttons). Now composes a secondary js_button trigger (derived
     tone) + delegates the open state entirely to js_alert_dialog (Dialog + tone-driven
     Buttons), so every dialog/button visual resolves through those primitives' tokens. No
     literals remain. Probe tests added (closed trigger, open title/desc/confirm/cancel,
     warning-tone routing, size/density). gpui=0 (verified pass-19 build still composes,
     stale literal flags cleared). body-slot remains an AlertDialog-side limit (noted). -->
# Parity: ConfirmAction

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/confirm-action.md`
- Svelte (authoritative): `packages/svelte/components/src/ConfirmAction.svelte`
- GPUI: `packages/gpui/components/src/composites/confirm_action.rs`
- Jetstream: `packages/jetstream/components/src/confirm_action.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ConfirmActionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/confirm_action_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/confirm_action.rs`

## Contract ↔ Svelte

Aligned. Props (title/description/tone/triggerLabel/confirmLabel/cancelLabel/onConfirm/onCancel/size/sizeRole/density), defaults, snippets (`trigger`, `children`), and the trigger-tone derivation (`danger→danger`, else `default`, line 48) all match. Default trigger is a `variant="secondary"` Button with derived tone (line 86); custom trigger is wrapped in `<span role="presentation">` with Enter/Space keydown + preventDefault (lines 70–84). AlertDialog prop mapping matches §8. No divergence.

## GPUI gap (vs Svelte + contract)

All closed in pass 19 (re-verified pass 41 — `composites/confirm_action.rs` composes a secondary `Button` trigger with derived tone and delegates the open state entirely to `AlertDialog::from_spec`; no inline dialog, no literals).

- [x] Composes AlertDialog + Button — default trigger is `Button::new().variant(Secondary).tone(derived)`; open delegates to `AlertDialog::from_spec(...)` (tone/labels/size/density + body via `.with_content`).
- [x] No shadow/width/text literals — all delegated to the composed AlertDialog/Button (shadows, dialog width, confirm text color resolve through their tokens).
- [x] Backdrop dismiss / Escape — handled by the composed AlertDialog (Dialog primitive owns backdrop + escape).
- [x] No ad-hoc gap multiplier — gaps come from the composed primitives.
- accepted: no ARIA (gpui has no accessibility API) — `role="alertdialog"`, `aria-labelledby`, `aria-describedby`, focus trap not emitted.

## Jetstream gap (vs Svelte + contract)

Rebuilt in pass 41 to mirror GPUI — `js_confirm_action` composes a secondary `js_button` trigger + delegates the open state entirely to `js_alert_dialog`.

- [x] Composes AlertDialog + Button — closed renders `js_button(Secondary, derived tone)`; open returns `js_alert_dialog(...)` (Dialog + tone-driven cancel/confirm Buttons).
- [x] No hardcoded padding/width — dialog padding, width and surface all delegated to the composed `js_dialog`/`js_button`, which resolve from tokens.
- [x] Title size — now the AlertDialog/Dialog title resolves through the composed primitive, no ad-hoc `+0.1875` offset.
- [x] Trigger tone derivation — `tone === "danger" ? "danger" : "default"` feeds `js_button`'s tone, so danger renders a danger secondary Button and non-danger a default secondary Button (matching Svelte/GPUI).
- [x] Confirm/cancel buttons — now real composed `js_button`s (via `js_alert_dialog`) with their hover/active/focus treatment.
- [x] Backdrop dismiss / Escape — owned by the composed `js_dialog` (gated by working state); no hand-rolled backdrop.
- [~] `children`/body-content slot — `js_confirm_action` still takes no content arg. The composed `js_alert_dialog` exposes only the item-detail row, not an arbitrary body element; full body-slot parity is an **AlertDialog-side capability**, noted as a remaining limit (out of scope here).
- accepted: no ARIA channel; open/close interaction lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Default trigger (danger), Warning tone, **Custom trigger slot** (ghost Button), **With body content** (code block), Last-action readout.
- GPUI covers: Default trigger (danger), Warning tone, Custom trigger slot (ghost), With body content (code block), Last action — broad, interactive open/close. — note: all four use `with_trigger` custom Buttons; the no-trigger default-Button path is never exercised.
- Jetstream covers: Neutral (closed), Destructive (closed), Open state (static). — missing: **Warning tone** as distinct, **Custom trigger slot**, **With body content** (no body arg exists), interactive open/close, last-action readout.

## Notes

- The dominant cross-target gap is architectural: the contract defines ConfirmAction as a **composite over AlertDialog + Button**, but both Rust targets reimplement an inline dialog. This duplicates AlertDialog styling and is the source of every hardcoded-literal flag here — fixing it means routing through the AlertDialog/Button components, which already resolve their own tokens.
- Specimen-wise the GPUI suite mirrors Svelte closely; Jetstream is the laggard (no custom-trigger, no body content, no warning differentiation).
