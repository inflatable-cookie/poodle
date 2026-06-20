<!-- parity consv=gap gpui=9 jetstream=8 specimen=gap -->
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

- Root border/padding/radius: contract §8 `.block-editor` table says `border: 0.0625rem solid border-default`, `border-radius: radius-surface`, `padding: shell-y shell-x`, `background: background-surface`. Svelte root has only `background: background-surface; padding: 0; display:flex; gap` — **no border, no radius, no padding** (`BlockEditor.svelte:438-443`). The `shell-x/y` custom props are declared but unused on the root. **Fix: contract §8 root table is wrong; remove border/radius/padding from root.**
- Block background: contract §8 `.block-editor__block` and Svelte match (`elevated 42%`, active `72%`, drag-over ring `0 0 0 0.125rem accent`, dragging `opacity 0.4`). No fix.
- `data-size`/`data-density` size table: contract §8 says xs `1.25` / sm `1.5` / md `1.75` / lg `2` / xl `2.25rem`; Svelte matches (lines 445-463). No fix.
- `blockTypeItems` (flat or grouped `BlockTypeGroup[]`): present in both contract §3 and Svelte (lines 51, 95). No fix.
- Snippets (`block`, `typePicker`, `addPicker`): contract §3 and Svelte match. No fix.
- Type-select inset: Svelte adds `.block-editor__type-select--inset` margin when `!canReorder` (lines 328, 562-568) — minor, contract §2 doesn't mention it. **Fix: note the inset behaviour in contract §2/§8 (low priority).**

## GPUI gap (vs Svelte + contract)

GPUI uses a legacy `with_child` API plus a parallel spec-blocks path; the toolbar is a fabricated hover-reveal design that diverges from the contract anatomy.

- [ ] Anatomy mismatch — toolbar is wrong: contract §2 toolbar = drag-grip + TypeSelect (left) / move-up, move-down, AddSelect, RemoveBtn (right), one per block. GPUI builds a hover-revealed icon strip (`grip-vertical`, `square`, separator, `chevron-up/down` | `plus`, `trash-2`) wrapping each child (`block_editor.rs:161-243`). No TypeSelect, no AddSelect — type switching and add-block are absent. Rebuild toolbar to match.
- [ ] Bottom "Add block" button (`:248-271`) is **not in the contract** — Svelte adds via the per-block AddSelect plus icon. Remove the fabricated bottom button.
- [ ] Type label rendered as static caption text above content (`:123-138`) instead of a ghost Select bound to `block.block_type`. No type-change control.
- [ ] Hardcoded toolbar-icon `.w(px(24.0)).h(px(24.0))` (`:72-73`) — contract `control-size` is size-driven (`1.25…2.25rem`); resolve from token, not fixed 24px (`_effective_size` computed then discarded at `:86`).
- [ ] Hardcoded `.gap(px(2.0))` (`:126,:175,:230`), separator `.w(px(1.0)).h(px(14.0)).mx(px(2.0))` (`:195`), `.min_h(px(120.0))` root (`:156`) — float px literals; resolve from tokens or remove (separator + min_h have no Svelte analogue).
- [ ] Hardcoded `.opacity(0.0)` hover-reveal (`:176,:231`) and `.opacity(0.35)` disabled-move (`:201,:216`) — the hover-reveal pattern itself is non-contract; disabled tool-btn opacity in Svelte is `0.3` (contract §8 `.block-editor__tool-btn:disabled`).
- [ ] Root has `border_1` + `rounded(radius.surface)` + `px/py(pad)` (`:146-156`) — matches the STALE contract but NOT current Svelte (root has no border/radius/padding). Once contract §8 is fixed, drop these.
- [ ] No remove-on-single-block guard surfaced visually: contract hides RemoveBtn when 1 block; GPUI always shows `trash-2` (`:236`).
- [ ] No drag-over / dragging / active-block visual states (contract §4) — blocks have no `role="group"`, no active background bump, no drag ring.
- accepted: no ARIA (gpui has no accessibility API) — `role="group"`, aria-labels not emitted.
- accepted: drag-and-drop reorder mechanics may use move buttons (contract §10 GPUI note).

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
- GPUI covers: Default blocks (legacy `with_child`), Custom blocks (legacy `with_child` callout), Consumer-driven block types (spec `with_blocks` + `with_block_types`). — missing: single-posture demo; no interactive add/remove/type-change (no controls render); two of three groups use the legacy child API.
- Jetstream covers: With blocks (4 placeholders), Empty (1 placeholder), Disabled. — missing: consumer block types / real content, single posture, sizes, densities; blocks are `"Block n"` placeholders.

## Notes

- Spec `EditorBlock` is `Eq`/`Hash`-friendly (`String` fields only) so it drops Svelte's opaque `data: unknown` and `[key: string]: unknown` payload — Rust targets can only carry `content: Option<String>`. Accepted modeling delta; consumer-owned rich payloads aren't expressible in the Rust spec.
- `with_block_count(n)` (`block_editor.rs:187`) is a legacy placeholder-block helper used by the Jetstream specimen; prefer `with_blocks`. The GPUI `with_child` path (`:41`) is also legacy and should migrate to spec blocks.
- Biggest gap: neither Rust target renders the contract's TypeSelect or AddSelect controls — block type-switching and add-block (the editor's core chrome) are absent in both; GPUI additionally fabricates a non-contract hover toolbar + bottom add button.
