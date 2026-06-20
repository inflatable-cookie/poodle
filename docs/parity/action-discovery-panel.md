<!-- parity consv=gap gpui=6 jetstream=9 specimen=gap -->
# Parity: ActionDiscoveryPanel

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/action-discovery-panel.md`
- Svelte (authoritative): `packages/svelte/components/src/ActionDiscoveryPanel.svelte`
- GPUI: `packages/gpui/components/src/composites/action_discovery_panel.rs`
- Jetstream: `packages/jetstream/components/src/action_discovery_panel.rs`
- Spec: `packages/contracts/components/src/action_discovery_panel.rs` · `CommandActionItem`/`ActionDiscoverySection`/`DiscoveryState` in `composite_types.rs:586-691`
- Specimens: svelte `packages/svelte/preview/src/specimens/ActionDiscoveryPanelSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/action_discovery.rs` · jetstream `packages/jetstream/preview/src/specimens/action_discovery_panel.rs`

## Contract ↔ Svelte

Several token-table values in contract §9 do not match the Svelte CSS. Svelte is authoritative — update the contract.

- Badge background/color: contract §9 (shared badge+kbd table) shows `background: background-surface 76%` + `color: text-secondary` for both chips. Svelte gives the **badge** its own accent treatment (`background: accent-base 16%`, `color: accent-base`, `ActionDiscoveryPanel.svelte:285-288`). **Fix: split the contract badge table from kbd; document accent badge styling.**
- Chip `min-height`: contract §9 says `1.5rem`; Svelte default is `1.375rem` (`--poodle-action-discovery-chip-height`, line 172) and varies per size (1.125/1.25/1.5/1.75rem). **Fix: contract value is wrong; document the per-size chip-height table.**
- Chip `padding` / `font-size`: contract §9 says `0 0.5rem` and `0.75rem` flat; Svelte resolves both from per-size custom props (`chip-x`, `chip-font-size`, lines 190-219). **Fix: document size-driven chip padding + font-size.**
- Chip typography: contract omits it; Svelte adds `font-family: label-family`, `font-weight: 600`, `letter-spacing: 0.03em`, `text-transform: uppercase` (kbd resets letter-spacing/transform, lines 277-294). **Fix: add to contract.**
- Skeleton row layout: contract §9 says `grid` with `grid-template-columns: minmax(0,1fr) auto`, `gap: space.inline.md`, `padding: 0.875rem`, `border-radius`, tinted `background`. Svelte skeleton-row is a plain `flex` `justify-content: space-between` with `gap` only and NO padding/background/radius — the pad/tint moved to the `__skeletons` wrapper (`0.875rem` pad, lines 302-313). **Fix: rewrite contract skeleton tables to match Svelte.**
- State region: contract §3 shows skeleton/EmptyState directly under root; Svelte wraps loading in `__state` (`min-height: 10rem`, `place-items: center`, lines 296-300) but renders error/empty/no-results EmptyState WITHOUT the `__state` wrapper (lines 110-125). **Fix: document the `__state` wrapper only for loading.**
- Eyebrow heading: contract anatomy §3 names `[Eyebrow]`; Svelte uses the `Eyebrow` component (line 129). Matches — no fix.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded `.p(px(14.0))` skeleton-row padding at `action_discovery_panel.rs:138` — resolve from a token (Svelte `__skeletons` pad is `0.875rem`); no float px literal.
- [ ] Hardcoded `.bg(...).opacity(0.72)` skeleton fill at `:140` and `.opacity(0.12)` badge fill at `:245` — Svelte uses `color-mix(... 72%/16% ...)`; route through `color_mix()` helper, not raw `.opacity()`.
- [ ] Hardcoded `.py(px(1.0))` badge padding at `:243` and `.h(px(1.0))` separator at `:267` — resolve from tokens / drop. The separator line itself is **not in Svelte** (groups separated by gap only) — remove it.
- [ ] No active-item state: Svelte tints the active `li` (`accent-base 18%` bg + inset accent ring, `ActionDiscoveryPanel.svelte:254-258`). GPUI only has `hover(bg elevated)` (`:231`) — no `aria-selected`/active background or inset ring. Add active styling driven by an active-id.
- [ ] Error state text mismatch: GPUI renders literal `"Failed to load actions"` (`:161`); Svelte/contract EmptyState title is `"Could not load actions"` with a message body. Use EmptyState equivalent + correct copy. Empty renders `spec.empty_message` fallback `"No actions found"` vs contract `"No actions available"`.
- [ ] No kbd/shortcut monospace + chip styling: shortcut is rendered as plain muted text (`:254-260`), not a `code-family` kbd chip with surface bg/border-radius. Badge lacks uppercase label typography. Build proper badge + kbd chips per §9.
- accepted: no ARIA (gpui has no accessibility API) — `role="listbox"`/`role="option"`/`aria-selected` not emitted.
- accepted: keyboard nav (`moveActive`/`moveToBoundary`/`selectActive`) is parent-driven in Svelte; GPUI exposes none and uses `.focus()` per row instead.

## Jetstream gap (vs Svelte + contract)

`js_action_discovery_panel` is a **stub** — it renders only section title labels. Nearly every part is missing.

- [ ] No action items rendered: loop at `action_discovery_panel.rs:28-30` emits one label per section title and never iterates `section.actions`. Render each action row (title + badge + shortcut) via ListCard-equivalent.
- [ ] No state handling: `spec.state` is ignored — loading/error/empty/no-results all render nothing. Add 5 skeleton rows (loading) and EmptyState copy (error/empty/no-results) per contract §7.
- [ ] No badge chip — `action.badge` never read. Add accent badge chip.
- [ ] No shortcut/kbd chip — `action.shortcut` never read. Add monospace (`code-family`) kbd chip.
- [ ] No active-item highlight — no active-id plumbing or accent-tinted row.
- [ ] No section description rendering — `section.description` unused (GPUI renders it at `:196-204`).
- [ ] No row hover / disabled treatment — `action.is_disabled` unused; no `disabled_opacity` dimming.
- [ ] Hardcoded `text_weight(600)` literal at `:29` — resolve weight from a typography token.
- [ ] `_font_size`/`_text_primary` computed but unused (`:13`,`:20`) — dead bindings flag the stub; wire real item rendering.
- accepted: no ARIA channel (`role`/`aria-selected`).
- accepted: keyboard navigation lives in the preview event loop — not wired for this component in `main.rs` (grep: no match).

## Specimen parity

- Svelte covers: Grouped actions, With descriptions and badges, Empty state, Sizes (xs–xl via `sizes` snippet), Densities (via `densities` snippet) — `ActionDiscoveryPanelSpecimen.svelte`.
- GPUI covers: Grouped, With descriptions and badges, Empty, Semantic presentation (compact + prominent). — missing: explicit per-size sweep; loading/error/no-results states never shown.
- Jetstream covers: Default (two empty sections), Empty. — missing: **action items entirely** (sections have `vec![]`), badges, shortcuts, descriptions, sizes, densities, loading/error states. Specimen is effectively non-functional until `js_action_discovery_panel` renders items.

## Notes

- Data-model divergence (accepted, not a contract bug): Svelte takes a flat `items: CommandActionItem[]` grouped at render by `item.group`; Rust spec takes pre-grouped `sections: Vec<ActionDiscoverySection>`. Both express the same anatomy; the Rust shape just front-loads grouping. Spec also lacks `active_id`/`size`-override-for-chips plumbing exposed to the component.
- The `summary_tone()` helper on the spec (`action_discovery_panel.rs:58`) has no Svelte/contract analogue — Rust-only convenience, harmless.
- Biggest gap: Jetstream is a stub (9 todos) and its specimen renders empty sections; GPUI is functional but missing the active-item state and has token-literal violations.
