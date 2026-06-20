<!-- parity consv=gap gpui=11 jetstream=11 specimen=gap -->
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

Svelte has props the contract does not document, and the contract size/state tables are stale. Svelte authoritative.

- Svelte adds `embeddedHandle?: boolean` (default false, `EditableList.svelte:21`) — not in contract §3. **Fix: add to contract.**
- Svelte adds `showWorkflowChrome?: boolean` (default true, `EditableList.svelte:34`) — not in contract §3. **Fix: add to contract.**
- Item background is `transparent` (`EditableList.svelte:593`); contract token table says `background-surface`. Hover is `background-elevated 82%` (line 605) vs contract 52%. Drop-target tint `accent-base 56% border / 10% bg` (lines 620-621) vs contract 8%. **Fix: update contract §8 values to Svelte.**
- Contract §10 "GPUI Notes: Not yet implemented" is stale — GPUI now has a real composite. **Fix: rewrite §10.**
- All 27 contract props + 6 callbacks (onReorder/onAdd/onRemove/onChange/onSubmit/onCancel), full anatomy (live region, header, error/info, window-nav, `<ul role=listbox>`/`<li role=option>`, 6-dot handle, remove, add row, counter), keyboard grab/move/drop/cancel, and ARIA all present.

## GPUI gap (vs Svelte + contract)

- [ ] Static render — NO callbacks (onReorder/onAdd/onRemove/onChange/onSubmit/onCancel all absent). No add/remove/reorder interaction.
- [ ] No drag-and-drop or keyboard grab/move — `reorderable` shows a `grip-vertical` icon only (`editable_list.rs:204-219`); no drop-target/grabbed/dragging states.
- [ ] No live region / sr announcer; no `<ul role=listbox>` / `<li role=option>` semantics (plain divs); no ARIA at all.
- [ ] Handle uses `grip-vertical` icon; contract specifies a 6-dot grip SVG.
- [ ] No window-nav (Previous/label/Next), no long-list warning, no `windowSize`/`longListThreshold`/`longListWarningText`.
- [ ] No `editable`/`removable`/`addPlaceholder`/`submitLabel`/`cancelLabel` builders; workflow chrome gated on `dirty||submitting` (`editable_list.rs:411`) instead of contract's onSubmit/onCancel presence.
- [ ] Counter shows non-contract `"N item(s)"` when no max (`editable_list.rs:373`); contract counter shows only when `maxItems` set.
- [ ] Hardcoded size/density rem scales inline `rem_to_px(0.875|0.5|0.625|1.0|1.125|0.75|1.25)` at `editable_list.rs:131-156` — resolve from tokens, not literal scale. Note Sm handle = `0.875` (line 132) vs contract sm = `1.0`.
- [ ] Hardcoded dirty-dot `.w(px(6.0)).h(px(6.0)).rounded(px(3.0))` at `editable_list.rs:355`; control-height multiplied by literal `0.92` at line 177.
- [ ] Hardcoded color `gpui::white()` for submit button text at `editable_list.rs:437` — resolve from a token.
- accepted: no ARIA (gpui has no accessibility API) — listbox/option roles, live region, aria-labels.

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
