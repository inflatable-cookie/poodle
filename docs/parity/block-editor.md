<!-- parity consv=fixed gpui=1 jetstream=2 specimen=gap -->
<!-- pass 36: Jetstream block-editor rebuilt to match GPUI — placeholder "Block N" content +
     static type label + unicode glyphs removed; composed js_select TypeSelect (block_type +
     block_types options) + AddSelect (value-less + plus tool button); move/remove tool buttons
     with real icons (grip-vertical/arrow-up/arrow-down/plus/x), first/last + single-block
     gating, posture via mode/allow_*; per-type content (heading/quote/code/list/paragraph);
     opacity 0.48 → state.opacity.disabled; root border/radius/padding dropped per §8. 6 probe
     tests (incl no-unicode-glyphs); suite 128. Remaining jetstream: italic/monospace (JsEl gaps,
     approximated) + specimen. -->
# Parity: BlockEditor

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/block-editor.md`
- Svelte (authoritative): `packages/svelte/components/src/BlockEditor.svelte`
- GPUI: `packages/gpui/components/src/composites/block_editor.rs`
- Jetstream: `packages/jetstream/components/src/block_editor.rs`
- Spec: `packages/contracts/components/src/block_editor.rs` (`BlockEditorSpec`, `EditorBlock`, `BlockTypeDefinition`, `BlockEditorMode`)
- Specimens: svelte `packages/svelte/preview/src/specimens/BlockEditorSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/block_editor_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/block_editor.rs`

## Contract ↔ Svelte

Contract §8 root table is stale — Svelte moved the border/padding/radius off the root onto each block. Svelte is authoritative — update the contract.

- [x] FIXED Root border/padding/radius: §8 `.block-editor` root table dropped `border`, `border-radius`, and the `shell-y shell-x` padding; now `background`, `padding: 0`, `display:flex`, `flex-direction:column`, `gap` — matching Svelte (`BlockEditor.svelte:438-443`). Added a note that shell-x/y custom props are declared-but-unused on the root. §7 sizing line updated (no root padding/border/radius).
- Block background: contract §8 `.block-editor__block` and Svelte match (`elevated 42%`, active `72%`, drag-over ring `0 0 0 0.125rem accent`, dragging `opacity 0.4`). No fix.
- `data-size`/`data-density` size table: contract §8 matches Svelte (lines 445-463). No fix.
- `blockTypeItems` (flat or grouped `BlockTypeGroup[]`): present in both. No fix.
- Snippets (`block`, `typePicker`, `addPicker`): contract §3 and Svelte match. No fix.
- [x] FIXED Type-select inset: documented `.block-editor__type-select--inset` (margin-left `calc(content-x + input-x − toolbar-x)` when `!canReorder`) in §8 + noted it on the TypeSelect part in §2 (`BlockEditor.svelte:328,562-568`).

## GPUI gap (vs Svelte + contract)

GPUI now renders the contract anatomy: per-block toolbar with drag grip + ghost TypeSelect (left) / move-up, move-down, plus-icon + ghost AddSelect, remove (right), and a type-aware content area. The fabricated hover-reveal strip and bottom add button are gone.

- [x] FIXED Anatomy/toolbar rebuilt to contract §2: drag-grip + TypeSelect (left) / move-up, move-down, AddSelect, RemoveBtn (right), one per block (`block_editor.rs`). Real GPUI `Select` (ghost, `menuMinWidth=10rem`) composed for both TypeSelect (value = `block.block_type`) and AddSelect (value-less picker).
- [x] FIXED Removed the fabricated bottom "Add block" button — add is now per-block via AddSelect.
- [x] FIXED Type is a ghost Select seeded from `block_types`, not static caption text.
- [x] FIXED Toolbar control size resolves from the contract `control-size` table (`1.25…2.25rem` by size), not fixed 24px.
- [x] FIXED All spacing resolves from the contract density recipe (`toolbar-y/x`, `content-x/y`, `stack-gap`, `toolbar-gap`) via `rem_to_px`. Non-contract separator + root `min_h(120)` dropped.
- [x] FIXED Hover-reveal removed; disabled tool buttons use the contract `0.3` opacity. Move buttons disabled at first/last; remove hidden when 1 block.
- [x] FIXED Root has no border/radius/padding (matches current Svelte §8) — flex column, surface bg, `stack-gap`.
- [x] FIXED RemoveBtn hidden when `block_count <= 1` (and gated by `can_remove`); add/reorder/type-change gated by mode + allow_* overrides; single-posture TypeSelect inset (`content-x + input-x − toolbar-x`) applied.
- [ ] No active / drag-over / dragging block states (contract §4) — these are interaction-driven (focus/drag), preview-event-loop bound; blocks render at static elevated-42% bg.
- accepted: no ARIA (gpui has no accessibility API) — `role="group"`, aria-labels not emitted.
- accepted: per-option icons not rendered in the type/add menus — `ChoiceOption` carries no icon field (Select-primitive gap, not block-editor-specific).
- accepted: AddSelect uses a plus tool button ahead of the ghost picker (Select has no trigger-slot override) — closest faithful subset of the contract's trigger-slot pattern.
- accepted: editing / type-change / add / remove / reorder are preview-event-loop bound; controls render at current spec state, no callbacks wired.

## Jetstream gap (vs Svelte + contract)

`js_block_editor` renders placeholder blocks with a closer-to-contract toolbar shape, but block content + type select are stubs and spacing uses inline float literals.

- [ ] No real block content: renders `"Block {n}"` placeholder labels (`block_editor.rs:122-126`) and ignores `block.content` / consumer block payload. Render actual block content (or the consumer slot equivalent).
- [ ] Type select is a static `"paragraph"` label (`:76-79`) — not a ghost Select bound to block type, and ignores `spec.block_types` / `block.block_type`. Wire to actual type.
- [ ] Drag grip is an empty sized div (`:70-74`) — no grip icon. Add `grip-vertical` icon.
- [ ] Move/add/remove buttons use unicode glyphs `↑ ↓ + ×` (`:88-112`) instead of icon-registry icons (`arrow-up`/`arrow-down`/`plus`/`x`). Use icons.
- [ ] Hardcoded `tint(elevated, 0.42)` block bg (`:36`) and `.opacity(0.48)` disabled (`:48`) — disabled should resolve `state.opacity.disabled` token (contract §8 uses `--poodle-state-opacity-disabled`), not a raw `0.48`.
- [ ] Hardcoded `.border(1.0)` root (`:41`) plus many `rem_to_px(0.125/0.75/0.8125/0.875/1.5)` inline float literals (`:62,:68,:78,:86,:91-110,:121,:125`) — border-width and the toolbar-gap/font-size/min-height values should resolve from tokens, not inline rems.
- [ ] Root still has `border` + `rounded(radius.surface)` (`:41-42`) matching the STALE contract, not current Svelte (no root border/radius). Once contract §8 is fixed, drop these.
- [ ] No active / drag-over / dragging block states (contract §4) — placeholder blocks are static.
- accepted: no ARIA channel (`role="group"`, aria-labels).
- accepted: interaction (add/remove/move/type-change/drag) would live in preview event loop — not wired in `main.rs` (grep: no match).

## Specimen parity

- Svelte covers: Consumer-driven block types (with custom `block` slot per type: heading input, code/quote textareas, divider hr), Single posture with grouped type picker. Full add/remove/reorder/type-change interactive. — `BlockEditorSpecimen.svelte`.
- GPUI covers: Default blocks (legacy `with_child`), Custom blocks (legacy `with_child` callout), Consumer-driven block types (spec `with_blocks` + `with_block_types`, exercising heading/paragraph/quote/code/list rendering + the TypeSelect/AddSelect toolbar), Single posture (TypeSelect-only, inset). — interactive add/remove/type-change remain preview-loop bound (controls render, no mutation); two of four groups still use the legacy child API.
- Jetstream covers: With blocks (4 placeholders), Empty (1 placeholder), Disabled. — missing: consumer block types / real content, single posture, sizes, densities; blocks are `"Block n"` placeholders.

## Notes

- Spec `EditorBlock` is `Eq`/`Hash`-friendly (`String` fields only) so it drops Svelte's opaque `data: unknown` and `[key: string]: unknown` payload — Rust targets can only carry `content: Option<String>`. Accepted modeling delta; consumer-owned rich payloads aren't expressible in the Rust spec.
- `with_block_count(n)` (`block_editor.rs:187`) is a legacy placeholder-block helper used by the Jetstream specimen; prefer `with_blocks`. The GPUI `with_child` path (`:41`) is also legacy and should migrate to spec blocks.
- Biggest gap: neither Rust target renders the contract's TypeSelect or AddSelect controls — block type-switching and add-block (the editor's core chrome) are absent in both; GPUI additionally fabricates a non-contract hover toolbar + bottom add button.
