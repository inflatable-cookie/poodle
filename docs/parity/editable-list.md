<!-- parity consv=fixed gpui=4 jetstream=11 specimen=gap -->
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

- [x] FIXED Row anatomy composes real primitives: handle = `grip-vertical` 6-dot grip `Icon` sized to the contract handle-size square; content = label text (ellipsis); remove = ghost `IconButton` (icon `x`, `Chrome` size role), shown only when `is_editable || is_removable`.
- [x] FIXED Add row composes the real `TextInput` (placeholder, size/density, disabled) + primary `Button` (`add_label`), not hand-rolled input/button divs.
- [x] FIXED Workflow header composes real secondary/primary `Button` primitives (own their geometry/typography/fill/foreground) — removed hardcoded `gpui::white()` submit text and the hand-rolled action row.
- [x] FIXED Size/density geometry resolves from token-exact scales `presentation::editable_list_{handle_size,item_x,item_y,font,list_gap,item_gap}_rem` (contract §8) — no inline `rem_to_px(...)` scale. Sm handle corrected `0.875` → `1.0`.
- [x] FIXED Removed hardcoded dirty-dot `px(6.0)` cluster and `label_size * 0.92` literal; container gaps use contract-exact rem (`0.75`/`0.5`), panels use `radius.surface` + `0.875rem` font.
- [x] FIXED Counter shows only when `maxItems` set, rendering `"N/M"` (dropped non-contract `"N item(s)"`).
- [x] FIXED Item border `0.0625rem solid transparent`, `radius.control`, transparent background per contract.
- New: `editable(bool)` / `removable(bool)` builders added so the remove control and add row can render per contract.

## Jetstream gap (vs Svelte + contract)

- [ ] Static render — NO callbacks, no add/remove/reorder interaction.
- [ ] No drag handle rendered at all — `reorderable` ignored entirely; no drag/drop/grab states.
- [ ] Remove button rendered unconditionally on every row (`editable_list.rs:80-86`) — contract shows remove only when `editable||removable`.
- [ ] No live region, no `<ul>`/role/aria, no header/workflow, no error/info banners, no window-nav, no long-list warning. No ARIA.
- [ ] Drops most props: no aria_label, dirty, submitting, error/info, longList, window, submit/cancel, editable/removable distinction.
- [ ] Hardcoded disabled opacity `.opacity(0.48)` at `editable_list.rs:61` — contract requires `state-opacity-disabled` token (CLAUDE.md violation).
- [ ] Hardcoded size scale `rem_to_px(1.0|1.125|1.25|1.375|1.5)` remove sizes at `editable_list.rs:17-23` — diverges from BOTH contract (xs=0.875) and GPUI; resolve from tokens.
- [ ] Hardcoded font `rem_to_px(0.8125)` at `editable_list.rs:14`, remove-icon `.text_size(rem_to_px(0.75))` at `:84`, remove radius `.rounded(rem_to_px(0.25))` at `:85`.
- [ ] Hardcoded border width `.border(1.0)` at `editable_list.rs:99,114` and `.text_weight(500)` at `:119` — resolve from tokens.
- [ ] Density tuple literals inline at `editable_list.rs:40-44` — resolve from density tokens.
- accepted: no ARIA.
- accepted: interaction (add/remove/reorder/submit) would live in preview event loop; absent here.

## Specimen parity

- Svelte covers: Editable+reorderable, Max items, Removable-only, Disabled, Dirty, Submitting, Error, Info, plus size + density snippets. Full interactive drag/keyboard reorder.
- GPUI covers: Editable+reorderable, Max items, Removable-only (faked via max==count), Disabled, Dirty, Submitting, Error, Info (8 static groups). — missing: drag-to-reorder, workflow-via-callbacks, windowed, size/density variants.
- Jetstream covers: With items, Empty, Disabled (3 groups). — missing: **reorderable/drag, max-counter, workflow, windowed, error/info, dirty/submitting, size/density** (weakest coverage).

## Notes

- Three-way size-scale disagreement: contract handle xs/sm/md/lg/xl = `0.875/1/1/1.125/1.25`; GPUI remove = `0.875/0.875/1.0/1.125/1.25` (sm wrong); Jetstream remove = `1.0/1.125/1.25/1.375/1.5` (entirely larger). None agree — needs a single tokenized source.
- Both Rust targets are presentational shells: zero callbacks, zero ARIA, zero drag/keyboard, no `<ul>`/role semantics, no live region. The reorder behavior that defines this component is unimplemented in both.
- Jetstream is weakest: no handle, unconditional remove button, hardcoded `0.48` disabled opacity.
