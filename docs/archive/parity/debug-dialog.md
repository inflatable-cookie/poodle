<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok --><!-- pass: GPUI specimen done; Jetstream pending engine recovery --><!-- pass: Jetstream specimen created+registered (debug-dialog slug) mirroring GPUI groups (with value/custom trigger/hidden-when-null) via real js_debug_dialog→js_button+js_code; both previews build clean -->
<!-- pass: GPUI rebuilt to compose Dialog primitive (title/close/lg-width) wrapping a Code block; gap tokenized (space.stack.md); value-as-String reclassified accepted (Rust has no `unknown`). max_height wired into both targets' Code blocks via new DebugDialogSpec::max_height_px(). -->
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

Now composes the Dialog primitive: trigger Button + Dialog (title / close button / lg width) wrapping a JSON Code block. Renders nothing when value is null.

- [x] DONE: **Dialog container.** `debug_dialog.rs` rebuilt — trigger `Button`
  (variant + size from spec) stacked above a `Dialog::from_spec` (`title`,
  `width=Lg`, `show_close_button`, `close_label`) whose content slot is the JSON
  `Code` block. Matches Svelte's Button + `<Dialog width="lg">` structure. Open
  state is parent/preview-loop owned (`default_open(true)` so the surface is
  built for verification).
- [x] DONE: **Hardcoded gap `px(12.0)` removed** — the trigger/surface stack gap
  now resolves from `space.stack.md`.
- [x] DONE: **`maxHeight` wired** — `DebugDialogSpec::max_height_px()` (new
  additive helper) parses the rem term of the CSS string (default
  `"min(60vh, 32rem)"` → 512px) and feeds `Code::with_max_height`; the vh term is
  viewport-relative and owned by the centering parent.
- accepted (architectural): Spec stores `value: Option<String>` rather than
  contract `unknown`. Rust has no `unknown` type; the idiomatic channel is a
  pre-serialized JSON string supplied by the caller (matches the Jetstream side).
  A `serde_json::Value` field would couple `poodle-specs` to serde for one
  component — out of scope. Callers serialize via `JSON.stringify`-equivalent
  before constructing the spec.
- accepted: no ARIA / no Dialog focus-trap/aria-modal (gpui has no accessibility API) — the standing gpui delta, inherited from the Dialog primitive.

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `js_debug_dialog(spec, theme)` created — trigger Button (`triggerVariant`/`triggerSize`) + JSON Code block of `value`; renders nothing without a value. Registered in lib.rs, probe-tested. (Open/close dialog behavior lives in the preview event loop, like other Jetstream interactions.)
- [x] DONE: Added the Jetstream specimen `packages/jetstream/preview/src/specimens/debug_dialog.rs` (created + registered: pub mod + dispatch arm + `debug-dialog` registry entry) covering the Svelte/GPUI states (with value, custom trigger, hidden when null) via real `js_debug_dialog`.

## Specimen parity

- Svelte covers: with debug value (object → JSON), custom trigger (`triggerVariant="secondary"`, `triggerSize="xs"`, custom `maxHeight`), hidden-when-null.
- GPUI covers: labeled groups — "With debug value" (title + trigger-label + nested-array value), "Custom trigger" (`trigger_variant=Secondary`, `trigger_size=Xs`, `max_height="18rem"`), "Hidden when null" (renders nothing). **GPUI specimen complete** — mirrors all three Svelte groups; real Button + Dialog + Code composition (the component now renders the Dialog surface with `default_open(true)`), no fakes.
- Jetstream covers: nothing — no specimen exists.

## Notes

- `consv=ok`: contract and Svelte match exactly.
- GPUI's structural gap (no Dialog) is the higher-severity GPUI issue; the `px(12.0)` literal is cleanup.
- Jetstream is a from-scratch build — depends on Jetstream `dialog` + `code` components already existing (both present).
