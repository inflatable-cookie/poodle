<!-- parity consv=fixed gpui=2 jetstream=2 specimen=gap -->
<!-- pass 43: Jetstream rebuilt to match GPUI — composes js_dialog (role=AlertDialog, width=Sm) +
     js_buttons (ghost cancel / primary tone-driven confirm: Danger→Danger, Warning→Default);
     js_alert_dialog_working entry (label swap + buttons disabled + dismissal gated); item-detail
     row. Additive AlertDialogSpec.item_label/item_value (+with_item_detail) — GPUI re-verified.
     3 probe tests; specs 61, jet 156, gpui clean. Click/dismiss = preview-loop. -->
<!-- pass 19: GPUI rebuilt — was a hand-rolled card with hsla/px literals; now composes
     the real Dialog (role=AlertDialog, width=Sm) + two composed Buttons (ghost cancel /
     primary confirm). Tone-driven confirm (danger→Danger), working state (label swap +
     buttons disabled + dismissal suppressed), item_detail row. No literals remain.
     Build clean. Remaining GPUI: ARIA (accepted, no a11y channel). -->
# Parity: AlertDialog

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/alert-dialog.md`
- Svelte (authoritative): `packages/svelte/components/src/AlertDialog.svelte`
- GPUI: `packages/gpui/components/src/primitives/alert_dialog.rs`
- Jetstream: `packages/jetstream/components/src/alert_dialog.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/AlertDialogSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/alert_dialog.rs` · jetstream `packages/jetstream/preview/src/specimens/alert_dialog.rs`

## Contract ↔ Svelte

Both surfaces are close, but the warning-tone mapping diverges and the contract is missing one width detail.

- [x] FIXED Warning tone → confirm Button tone. Svelte derives `confirmTone = tone === "danger" ? "danger" : "default"` (line 57); warning resolves to Button tone `"default"`. Svelte is the parity authority and Button-tone selection is a deliberate value choice (not an a11y/feature requirement), so updated contract §8 mapping + §4 state to `warning → "default"`. The §2 anatomy already showed `tone={confirmTone}`.
- [x] FIXED `width="sm"` Dialog passthrough. Documented the fixed `width="sm"` in §2 anatomy + the §8 Dialog-props table so Rust targets resolve a Dialog `sm` width token instead of guessing rem.
- [x] FIXED `itemLabel`/`itemValue` detail row. Added `[Item Detail .alert-dialog__item-detail]` to §2 anatomy + part table and added the item-detail CSS table to §8 (margin `0 0 0.75rem`, text-secondary, line-height 1.5, strong → text-primary).
- `onOpenChange`, controlled/uncontrolled `open`, internal `working` gating of escape/backdrop/close (lines 113-115), async-confirm-keeps-open (lines 74-78): all match contract. `ok`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Backdrop color literal `hsla(0.0, 0.0, 0.0, 0.5)` at `alert_dialog.rs:60` — resolve from `spec.backdrop_fill_token()` (Jetstream already does), not a raw HSLA. Field is dead anyway (`_backdrop_fill`), since GPUI renders inline with no overlay.
- [ ] Cancel hover fill literal `hsla(0.0, 0.0, 0.5, 0.1)` at `alert_dialog.rs:176` — resolve a ghost-hover token, not a raw HSLA.
- [ ] Dialog shadow color + offset literals `hsla(0.0,0.0,0.0,0.12)` / `0.08` and `px(8.0)/px(24.0)/px(2.0)/px(8.0)` at `alert_dialog.rs:235-244` — resolve from a surface-shadow token, not raw HSLA + px floats.
- [ ] Card width literal `px(rem_to_px(26.25))` at `alert_dialog.rs:218` — resolve from a Dialog `width="sm"` token once contract documents it; drop the hardcoded `26.25`.
- [ ] No `tone`-driven confirm fill: `confirm_fill_token()` is resolved once (line 66) with no warning branch — confirm fill is identical for danger and warning. Match Svelte/contract tone→variant mapping.
- [ ] No `itemLabel`/`itemValue` support — spec builder absent; the detail-item row (Svelte lines 140-144) never renders.
- [ ] No `working` state: `on_confirm`/`on_cancel` are fire-and-forget (lines 128-136); no internal working bool, no escape/backdrop/close suppression, no `workingLabel` swap, no Promise await. Contract §4 working state + §11 Tier-1 "working state suppresses dismiss" unmet.
- [ ] No Dialog composition: renders a bespoke inline card (lines 213-261), not `PoodleDialog`/`DialogKind::AlertDialog` per contract §10. No backdrop overlay, no focus trap, no `showCloseButton`, no real modality.
- [ ] No `density` plumbed to a real Dialog — padding uses `panel_space_*` directly (lines 57-58); acceptable only until Dialog composition lands.
- accepted: no ARIA (gpui has no accessibility API) — `alertdialog` role / `aria-modal` / `aria-label` not emitted.
- accepted: inline render instead of absolute overlay is documented in-file ("specimen page handles its own layout").

## Jetstream gap (vs Svelte + contract)

- [ ] Button padding literals `rem_to_px(0.75)` / `rem_to_px(0.375)` at `alert_dialog.rs:41-42` — resolve from a control padding token, not raw rem floats.
- [ ] Panel min-width literal `rem_to_px(20.0)` at `alert_dialog.rs:50` — resolve from the Dialog `width="sm"` token once documented; drop hardcoded `20.0`.
- [ ] Separator height literal `.h(1.0)` at `alert_dialog.rs:73` — resolve from a hairline/border-width token, not `1.0`.
- [ ] Border width literal `.border(1.0)` at `alert_dialog.rs:46` — resolve from a border-width token.
- [ ] Separator above actions (line 71-75) is not in Svelte or contract anatomy — Svelte Dialog has no rule between description and actions. Remove or justify; it is an invented part.
- [ ] No `tone`-driven confirm fill: `confirm_fill_token()` resolved once (line 36); warning and danger render identical confirm fill. Apply tone→variant mapping.
- [ ] No `itemLabel`/`itemValue` support — detail-item row never rendered.
- [ ] No `working` state: no working bool, no `workingLabel`, no escape/backdrop suppression, no Promise await. Component has no `density` builder either (spec field exists but no `.with_density`).
- [ ] No focus trap / dismiss-route handling — overlay renders backdrop (line 106-110) but backdrop click + escape gating are absent; contract §4/§6 working-state suppression unmet.
- [ ] Confirm/cancel buttons have no click wiring in the component (`.focusable()` only, lines 90/100). Interaction must live in preview event loop — confirm none exists; flag once.
- accepted: no ARIA channel (`alertdialog` role / aria-modal not emitted).
- accepted: interaction (click handler) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Danger tone (default), Warning tone, Async confirm with body-content user card + working label, Last-action readout (`AlertDialogSpecimen.svelte`). Interactive open/close via trigger buttons.
- GPUI covers: Danger, Warning, "Async confirm callback" trigger, Last action. Triggers toggle open via specimen state. — missing: **body content / item-detail render** (the async group opens a plain dialog, no user card), **working-state visual** (label swap / disabled buttons), **async resolution** (no real Promise, close is synchronous).
- Jetstream covers: Danger tone, Warning tone, No-description. — missing: **Async confirm + body content** group, **working/disabled state**, **Last-action readout**. Dialogs render always-open (no trigger), so open/close + working transitions are not demonstrated.

## Notes

- Both Rust targets render confirm fill identically across tones because neither reads `tone` into the fill — the warning/danger visual distinction is absent in GPUI and Jetstream even though the spec carries `AlertDialogTone`.
- GPUI does not compose a Dialog at all (contract §10 expects `PoodleDialog` + `DialogKind::AlertDialog`); it hand-rolls a card. This blocks focus trap, modality, and close-button parity until Dialog composition lands — the single largest structural gap.
- `consv=gap` driver: warning-tone confirm-Button mapping (Svelte maps warning→default, contract says warning→warning) plus the undocumented `width="sm"` passthrough.
