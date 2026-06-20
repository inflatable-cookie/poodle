<!-- parity consv=ok gpui=3 jetstream=2 specimen=gap -->
# Parity: DebugDialog

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/debug-dialog.md`
- Svelte (authoritative): `packages/svelte/components/src/DebugDialog.svelte`
- GPUI: `packages/gpui/components/src/composites/debug_dialog.rs`
- Jetstream: **ABSENT** — no `packages/jetstream/components/src/debug_dialog.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/DebugDialogSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/debug_dialog_specimen.rs` · jetstream **ABSENT**

## Contract ↔ Svelte

Perfect parity. All eight contract props present in Svelte with identical names/types/defaults: `value` (`null`), `title` (`"Debug data"`), `triggerLabel` (`"View debug data"`), `maxHeight` (`"min(60vh, 32rem)"`), `triggerVariant` (`"ghost"`), `triggerSize` (`"sm"`), `showCloseButton` (`true`), `closeLabel` (`"Close debug dialog"`). Component hides when `value` is null/undefined, per contract. No divergence.

## GPUI gap (vs Svelte + contract)

Renders Button + Code side-by-side; does **not** wrap them in a Dialog.

- [ ] No Dialog container — `debug_dialog.rs:29-45` is a flex div holding Button + Code; contract requires the JSON code block inside a dialog opened by the trigger. Wrap in the `dialog` primitive.
- [ ] Hardcoded gap `px(12.0)` at `debug_dialog.rs:32` — resolve from a spacing token.
- [ ] Spec stores `value: Option<String>` (`packages/contracts/components/src/debug_dialog.rs`) — contract `value` is `unknown`; pre-serializing to String loses non-JSON-string inputs. Accept structured/serializable value, serialize at render.
- accepted: no ARIA / no Dialog focus-trap/aria-modal (gpui has no accessibility API) — follows from the missing Dialog wrapper above; ARIA itself is the standing gpui delta.

## Jetstream gap (vs Svelte + contract)

- [ ] **Entire component missing.** Implement `js_debug_dialog()` in `packages/jetstream/components/src/debug_dialog.rs` per contract: trigger Button (`triggerVariant`/`triggerSize`) opening a Dialog containing a Code block of the serialized `value`, all values token-resolved. This is the single biggest gap for this component.
- [ ] Add the Jetstream specimen `packages/jetstream/preview/src/specimens/debug_dialog.rs` covering the Svelte states (with value, custom trigger, hidden when null).

## Specimen parity

- Svelte covers: with debug value (object → JSON), custom trigger (`triggerVariant="secondary"`, `triggerSize="xs"`, custom `maxHeight`), hidden-when-null.
- GPUI covers: "Asset payload" (title + trigger-label + value) and default-hidden. — missing: **custom trigger variant/size**, **maxHeight customization**, and (blocked) the dialog-open path since no Dialog is rendered.
- Jetstream covers: nothing — no specimen exists.

## Notes

- `consv=ok`: contract and Svelte match exactly.
- GPUI's structural gap (no Dialog) is the higher-severity GPUI issue; the `px(12.0)` literal is cleanup.
- Jetstream is a from-scratch build — depends on Jetstream `dialog` + `code` components already existing (both present).
