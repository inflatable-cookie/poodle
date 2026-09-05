<!-- parity consv=fixed gpui=4 jetstream=3 specimen=ok | specimen backfill (2026-06-21): both Rust targets now cover editable+reorderable (handle+content+remove+add), empty (add-row-only), max-items+counter, at-max (add hidden), removable-only, reorderable handles, workflow chrome dirty+saving, error banner, info banner, disabled, plus size+density sweeps — real EditableList/js_editable_list + Button/TextInput/IconButton primitives + tokens, no fakes (GPUI removable-only now uses .removable(true) not the max==count fake). Windowed mode + long-list warning SKIPPED on both: feature-scope unbuilt in the Rust components (window/long-list spec fields not consumed). gpui+jetstream preview build clean. -->
<!-- finalize pass (2026-06-21): re-confirmed every remaining todo against contract §2/§4/§5/§6. NO representable visual/structural gap remains on either target — anatomy (handle, content, conditional remove, header, error/info banners, add row, counter) is fully built and token-resolved on BOTH. The gpui=4/jet=3 open items are ALL genuine non-representable limits: callbacks + drag/keyboard reorder + dragging/drop-target/grabbed states = PREVIEW-LOOP bound (these states exist only during live interaction; no static form); window-nav/long-list = FEATURE-SCOPE (spec fields exist, windowing logic unbuilt); ARIA = ACCEPTED (no a11y API); dirty||submitting chrome gate = ACCEPTED divergence (no callbacks to gate on). Reclassified, not closeable here. Jetstream probe tests (13) all passed at run time before an external `jetstream-renderer` engine WIP regression broke the lib build (`sdf.rs:90` Vec3::xz, outside Poodle, not from this change). No code change to editable-list this pass — confirm-only. -->
# Parity: EditableList

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/editable-list.md`
- Svelte (authoritative): `packages/svelte/components/src/EditableList.svelte`
- GPUI: `packages/gpui/components/src/composites/editable_list.rs`
- Jetstream: `packages/jetstream/components/src/editable_list.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EditableListSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/editable_list_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/editable_list.rs`

## Contract ↔ Svelte

`consv=fixed`. Undocumented Svelte props, stale token values, composed-primitive anatomy, and stale §10 all reconciled.

- [x] FIXED `embeddedHandle?: boolean` (default false, `EditableList.svelte:21,54`) added to contract §3 + anatomy (`--embedded-handle` padding 0, handle omitted).
- [x] FIXED `showWorkflowChrome?: boolean` (default true, `EditableList.svelte:34,67`) added to contract §3; chrome gate documented as `showWorkflowChrome && (onSubmit||onCancel)`.
- [x] FIXED Item token values updated to Svelte: background `transparent` (`:593`), hover `elevated 82% / transparent` (`:605`), focus-visible `accent-focusRing` border + box-shadow (`:608-611`), drop-target/grabbed/last-moved `accent-base 56%` border + `accent-base 10%` bg (`:620-621`); transition `120ms ease`. §4 states table synced.
- [x] FIXED (extra) Anatomy/§8 reconciled to composed primitives: Remove = ghost `IconButton` (`--danger-on-hover`), Add Input = `TextInput` wrapper, Add Button = primary `Button` wrapper; add-row gap `0.375rem` → `0.5rem`; raw-element token tables replaced with delegation + wrapper styling.
- [x] FIXED Contract §10 rewritten — GPUI composite exists but is presentational (no callbacks/DnD/keyboard/live-region/`<ul>` semantics).
- All 27+ contract props + 6 callbacks (onReorder/onAdd/onRemove/onChange/onSubmit/onCancel), full anatomy (live region, header, error/info, window-nav, `<ul role=listbox>`/`<li role=option>`, 6-dot handle, remove, add row, counter), keyboard grab/move/drop/cancel, and ARIA all present.

## GPUI gap (vs Svelte + contract)

Buildout pass (2026-06-20): row anatomy now composes the real primitives, all
geometry resolves from token-exact size/density scales, and the workflow header
+ add row use the real `Button`/`TextInput`/`IconButton` primitives. Remaining
open items are interaction/a11y/feature-scope, not renderable-anatomy gaps.

- [ ] Static render — NO callbacks (onReorder/onAdd/onRemove/onChange/onSubmit/onCancel all absent). No add/remove/reorder interaction. (preview-event-loop bound)
- [ ] No drag-and-drop or keyboard grab/move; no drop-target/grabbed/dragging visual states. The `grip-vertical` handle renders at rest. (preview-event-loop bound)
- [ ] No window-nav (Previous/label/Next), no long-list warning, no `windowSize`/`longListThreshold`/`longListWarningText`. (feature scope — not yet built)
- [ ] Workflow chrome gated on `dirty||submitting` rather than contract's onSubmit/onCancel presence (no callbacks in GPUI to gate on). (accepted divergence)
- accepted: no ARIA (gpui has no accessibility API) — `<ul role=listbox>`/`<li role=option>`, live region, aria-labels.
- accepted: remove `--danger-on-hover` wrapper recolor — GPUI's ghost IconButton owns its own hover; wrapper-level override not expressible via spec. Real ghost IconButton (icon `x`, chrome role) is composed.

### Resolved in buildout pass

- [x] FIXED (banner pass 2026-06-21) Error/info banners now match contract §8: error panel `border color-mix(danger 40%, transparent)` + `bg color-mix(danger 8%, surface)`; info panel `border color-mix(accent 22%, transparent)` + `bg color-mix(accent 6%, surface)`; previously the panels had padding+radius+text only (no border, no fill). Info text color corrected `text-secondary` → `text-primary` (`EditableListSpec::info_color_token`, contract §8 Info table). Workflow header gains its contract bottom border `0.0625rem solid color-mix(border-default 76%, transparent)` (`border_b` + `pb 0.5rem`).
- [x] FIXED Row anatomy composes real primitives: handle = `grip-vertical` 6-dot grip `Icon` sized to the contract handle-size square; content = label text (ellipsis); remove = ghost `IconButton` (icon `x`, `Chrome` size role), shown only when `is_editable || is_removable`.
- [x] FIXED Add row composes the real `TextInput` (placeholder, size/density, disabled) + primary `Button` (`add_label`), not hand-rolled input/button divs.
- [x] FIXED Workflow header composes real secondary/primary `Button` primitives (own their geometry/typography/fill/foreground) — removed hardcoded `gpui::white()` submit text and the hand-rolled action row.
- [x] FIXED Size/density geometry resolves from token-exact scales `presentation::editable_list_{handle_size,item_x,item_y,font,list_gap,item_gap}_rem` (contract §8) — no inline `rem_to_px(...)` scale. Sm handle corrected `0.875` → `1.0`.
- [x] FIXED Removed hardcoded dirty-dot `px(6.0)` cluster and `label_size * 0.92` literal; container gaps use contract-exact rem (`0.75`/`0.5`), panels use `radius.surface` + `0.875rem` font.
- [x] FIXED Counter shows only when `maxItems` set, rendering `"N/M"` (dropped non-contract `"N item(s)"`).
- [x] FIXED Item border `0.0625rem solid transparent`, `radius.control`, transparent background per contract.
- New: `editable(bool)` / `removable(bool)` builders added so the remove control and add row can render per contract.

## Jetstream gap (vs Svelte + contract)

Buildout pass (2026-06-21): `js_editable_list` rebuilt to mirror the GPUI
composite anatomy. Row anatomy now renders the drag handle (when reorderable),
content label, and a conditional ghost remove `IconButton`; the workflow header,
error/info banners, add row, and counter are all present and composed from the
real `js_button` / `js_text_input` / `js_icon_button` primitives. All geometry
resolves from the new token-exact `presentation::editable_list_*` scales (mirrors
GPUI exactly). 13 `render_probe` tests cover the closed gaps. Remaining open
items are interaction/a11y/feature-scope, not renderable-anatomy gaps.

- [ ] Static render — NO callbacks (onReorder/onAdd/onRemove/onChange/onSubmit/onCancel). No add/remove/reorder interaction; the grip renders at rest, no drag/drop/grab visual states. (preview-event-loop bound)
- [ ] Workflow chrome gated on `dirty||submitting` rather than contract's onSubmit/onCancel presence (no callbacks in Jetstream to gate on). (accepted divergence)
- [ ] No window-nav (Previous/label/Next), no long-list warning, no `windowSize`/`longListThreshold`/`longListWarningText`. (feature scope — not yet built)
- accepted: no ARIA — the `JsEl` builder has no accessibility sink; `<ul role=listbox>`/`<li role=option>`, live region, and aria-labels are omitted.
- accepted: interaction (add/remove/reorder/submit) would live in preview event loop; absent here.
- accepted (JsEl limit): the workflow header's contract `border-bottom` is not expressible — `JsEl::border` is all-sides only (no per-side border), so the Jetstream header omits the bottom rule. GPUI renders it via `border_b`.

### Resolved in buildout pass

- [x] FIXED (banner pass 2026-06-21) Error/info banners now carry the contract §8 tinted border + background (error: `color-mix(danger 40%, transparent)` border, `color-mix(danger 8%, surface)` bg; info: `color-mix(accent 22%, transparent)` border, `color-mix(accent 6%, surface)` bg) — previously padding+radius+text only. Info text color corrected to `text-primary`. 2 `render_probe` tests now assert a tinted panel `bg` fill (`a > 0`), not just the message text.
- [x] FIXED Drag handle now rendered — `grip-vertical` 6-dot grip Icon, sized to the contract handle-size square, shown only when `reorderable`.
- [x] FIXED Remove button now conditional — ghost `IconButton` (icon `x`, chrome size role), shown only when `editable || removable` (was unconditional).
- [x] FIXED Workflow header composes real secondary/primary `js_button` (cancel + submit, submit disabled unless dirty, "Saving…" while submitting); error/info banners rendered (`role` semantics N/A in Jetstream).
- [x] FIXED Add row composes the real `js_text_input` (placeholder, size/density, disabled) + primary `js_button` — not hand-rolled input/button divs; hidden at max-items.
- [x] FIXED Counter shows only when `maxItems` set, rendering `"N/M"`.
- [x] FIXED Disabled opacity now `state.opacity.disabled` token (was hardcoded `0.48`).
- [x] FIXED Size/density geometry resolves from new `presentation::editable_list_{handle_size,item_x,item_y,font,list_gap,item_gap}_rem` (contract §8) — mirrors GPUI; the old larger remove-size scale and inline density tuples are gone.
- [x] FIXED Item border `0.0625rem solid transparent`, `radius.control`, transparent background per contract — no hardcoded font/radius/border-width/weight literals (delegated to composed primitives).

## Specimen parity

- Svelte covers: Editable+reorderable, Max items, Removable-only, Disabled, Dirty, Submitting, Error, Info, plus size + density snippets. Full interactive drag/keyboard reorder.
- GPUI covers (rebuilt 2026-06-21): Editable+reorderable (handle+content+remove+add row), Empty (add-row-only), Max items (5)+counter, At-max (3/3, add hidden), Removable-only (`.removable(true)` — no longer the max==count fake), Reorderable (handles only), Workflow chrome dirty, Workflow chrome saving, Error banner, Info banner, Disabled, plus size + density sweeps. — skipped: windowed mode + long-list warning (feature-scope unbuilt in `EditableList`).
- Jetstream covers (rebuilt 2026-06-21): Editable+reorderable, Empty (add-row-only), Max items (5)+counter, At-max (3/3), Removable-only, Reorderable (handles), Workflow chrome dirty, Workflow chrome saving, Error banner, Info banner, Disabled, plus size + density sweeps. — skipped: windowed mode + long-list warning (feature-scope unbuilt in `js_editable_list`). (was 3 groups — now full renderable coverage.)

## Notes

- Size-scale now reconciled: both Rust targets share the contract handle scale `0.875/1/1/1.125/1.25` via their `presentation::editable_list_handle_size_rem` helpers (GPUI + Jetstream mirror each other). The old Jetstream `1.0/1.125/1.25/1.375/1.5` remove scale is gone.
- Both Rust targets are presentational shells: zero callbacks, zero ARIA, zero drag/keyboard, no `<ul>`/role semantics, no live region. The reorder behavior that defines this component is unimplemented in both — preview-event-loop bound.
- Jetstream and GPUI now have parity in renderable anatomy (handle, conditional remove, header, banners, add row, counter); the gap is interaction/a11y only.
