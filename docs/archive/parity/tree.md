<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok -->
<!-- pass 41: tree render gaps closed. GPUI row-radius now control_radius − rem_to_px(0.125)
     (no raw px(2.0)). Jetstream rename-box border now accent-base (selected_fill_token),
     and the drop indicator (before/after accent line + inside accent-12% fill) now renders
     from spec.drop_target_value/drop_position, mirroring GPUI. Remaining items are accepted
     Known Deltas (selected inset ring, virtual scroll) or preview-loop interaction. Probe
     tests added (jetstream): indentation, twisty glyph, selected fill, rename border, drop
     indicator. consv stays ok; both Rust targets build/test clean. -->
# Parity: Tree

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/tree.md`
- Svelte (authoritative): `packages/svelte/components/src/Tree.svelte`
- GPUI: `packages/gpui/components/src/composites/tree.rs`
- Jetstream: `packages/jetstream/components/src/tree.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TreeSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/tree.rs` · jetstream `packages/jetstream/preview/src/specimens/tree.rs`

## Contract ↔ Svelte

Recently overhauled — all four target files are new/uncommitted. Contract and Svelte are in tight agreement. Props, defaults, anatomy, ARIA, and keyboard all line up. No divergences found.

- Props match 1:1: `nodes`, `selectedValues`, `expandedValues` (default `null`), `defaultExpandedValues`, `checkedValues`, `loadingValues`, `editingValue`, `ariaLabel`, `showGuides`/`showIcons` (default `true`), `showCheckboxes`/`reorderable` (default `false`), `virtualized`/`virtualHeight` (Svelte-only), `size`/`sizeRole` (`"chrome"`)/`density`. Svelte `$props()` (lines 44–71) is identical to contract §3.
- Callbacks match: `onSelectionChange`, `onExpandedChange`, `onCheckedChange`, `onLoadChildren`, `onRenameCommit`, `onRenameCancel`, `onContextMenu`, `onReorder`, `onActivate` — all present (lines 31–39) and documented in §3/§5.
- Anatomy match: root `role="tree"` + `aria-multiselectable` + `data-size`/`data-density`/`data-size-role` (lines 464–476); `treeitem` carries roving `tabindex`, `aria-level`, `aria-selected`, `aria-expanded` (branch only), `aria-disabled`, `data-branch`, `data-selected` (lines 552–573); Row / Indent (with `data-guide`) / Twisty / Checkbox / Icon / Label / RenameInput / LoadingRow / Group all present.
- Keyboard match: Down/Up, Right/Left, Home/End, Enter (select+activate), Space (toggle), F2 (rename), Alt+↑/↓ (move sibling), Shift+↑/↓ (extend) — `handleKeydown` (lines 389–461) implements the full §6 table.
- Branch rule match: `isBranch || children.length > 0` (line 89–91) == contract §3.
- Token targets match: row height/font, twisty/chevron sizing (`*1.5`/`*0.85`), indent/gap/pad-inline per density, selected fill (accent 10% + inset ring 20%), hover (elevated 60%), guide (border-subtle 54%), focus ring, disabled opacity — Svelte `<style>` (lines 640–863) matches §8 exactly. Note: contract §8 row background prose says canvas mix in the *tri-state* doc only; Tree's CSS-var defaults match the §8 table.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- accepted: Selected inset ring omitted — `render_row` applies only the accent fill (`tree.rs:840-842`), no `inset 0 0 0 0.0625rem accent-20%` box-shadow. "Selected inset ring is Svelte-only" Known Delta (§12).
- [x] **DONE:** `row_radius` was `control_radius - px(2.0)` (raw literal); now `control_radius - px(rem_to_px(0.125))` for the contract's `- 0.125rem`, matching the Jetstream form.
- accepted: drag preview chip colors/metrics are raw px literals (`px(8.0)`/`py(2.0)`/`rounded(px(4.0))`, `tree.rs:95-97`) — internal floating-cursor preview, not a contract-specified surface.
- accepted: no ARIA (gpui has no accessibility API) — role/level/selected/expanded conveyed visually only (§6 + Known Delta).
- accepted: no virtual scrolling (no windowing primitive; Known Delta) — renders all visible rows.
- accepted: drop indicator before/after line uses `h(px(2.0))`/`top(px(-1.0))`/`bottom(px(-1.0))` (`tree.rs:978-984`) — matches Svelte's `0.125rem` / `-0.0625rem` literally (2px / -1px at 16px root), an exact translation not a hardcode drift.

## Jetstream gap (vs Svelte + contract)

- [x] **DONE:** Inline-rename editor box border was `m.focus_ring`; now `m.selected_fill` (accent-base, `selected_fill_token`) per contract `.tree__rename`.
- accepted: Inline-rename editor renders a fake caret by appending `"|"` to the text — cosmetic stand-in for the immediate-mode runtime (no real input cursor primitive). Token-driven caret element would be a follow-up.
- accepted: Selected inset ring omitted — accent fill only; same Svelte-only Known Delta as GPUI (§12).
- [x] **DONE:** Drop indicator now renders — `js_tree` reads `spec.drop_target_value` / `spec.drop_position`: before/after draws an absolute `0.125rem` accent line at the row top/bottom (`-0.0625rem` inset); inside applies an `accent-base 12%` fill. Mirrors GPUI; drag/drop state owned by the preview loop.
- accepted: interaction (click/keyboard/drag) lives in the preview event loop + shell token routing (`tree:` / `tree-twisty:` / `tree-check:` ids), not the component — documented in the file header and specimen.
- accepted: no ARIA (immediate-mode runtime has no a11y tree; Known Delta).
- accepted: no virtual scrolling (Known Delta).
- accepted: focus ring via uniform transparent 1px border that turns accent when focused (`tree.rs:191-209`) — avoids layout jitter; equivalent to the contract's outline.

## Specimen parity

- Svelte covers: File explorer, Multi-select (Ctrl/Cmd + Shift), No guides/no icons, Checkbox cascade, Lazy/async children, Virtualized (1260 rows), Rename + context-menu + reorder, Sizes (xs–xl), Densities (compact/default/comfortable) — `TreeSpecimen.svelte`.
- GPUI covers: File explorer (interactive: click/Ctrl/Cmd/Shift/arrows/Space), Checkbox cascade, Rename + context-menu + Alt+↑/↓ reorder, Lazy/async, No guides/no icons, Sizes, Densities — `gpui/.../tree.rs`. Missing only Virtualized (accepted Known Delta). At parity.
- Jetstream covers: File explorer (interactive via event loop), Checkbox cascade, Lazy/async, No guides/no icons, Sizes, Densities, context-menu overlay — `jetstream/.../tree.rs`. Missing only Virtualized (accepted Known Delta). At parity.

## Notes

- This component was recently overhauled and is in strong shape. The two `consv` axes (contract vs Svelte) are genuinely aligned — no invented gaps.
- The remaining open todos are minor: GPUI's `- px(2.0)` radius literal, and Jetstream's rename-box border token + missing drop indicator. None are token-system violations of the resolved-color/size variety — sizing flows through `rem_to_px` + size/density tables and colors flow through `resolve_color(theme, spec.*_token())` in both Rust targets.
- Selected inset ring and virtual scroll are pre-approved Svelte-only deltas (§12); they are listed for traceability but are not parity bugs.
- The `1.5` / `0.85` twisty/chevron multipliers and the per-size/per-density rem tables are duplicated as literals across Svelte CSS, the spec is silent on them, and both Rust impls hardcode the same tables. This is the contract's stated size/density scale (§8), so it is faithful translation, not drift — but a shared spec helper (`twisty_size_rem`, etc.) would remove the triple-maintenance risk.
