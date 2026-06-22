<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok -->
<!-- pass 46: built out on BOTH targets. Additive ActionDiscoveryPanelSpec.active_id
     (+with_active_id). GPUI: active-item state (accent bg + ring), per-size badge/kbd chips,
     EmptyState for error/empty/no-results, 5 skeleton rows; dropped px(14)/opacity(0.72)/px(1)
     literals. Jetstream: was title-only stub → full items (title+subtitle+disabled), js_skeleton
     ×10 loading, js_empty_state states, accent badge + kbd chips, active accent bg+ring. 4 probe
     tests; specs 61, jet 173, gpui clean. Activation/keyboard = preview-loop; GPUI active ring
     outset (no inset primitive); js_eyebrow verbatim vs GPUI uppercase (Eyebrow-primitive delta). -->
<!-- specimen note: GPUI specimen done (active/loading/error/empty/no-results groups added,
     real ActionDiscoveryPanel, gpui/preview builds 0 errors); Jetstream pending engine recovery.
     specimen=gap held — Jetstream half unverifiable while engine is build-blocked. -->
<!-- specimen flip: engine now builds. Jetstream specimen brought to full GPUI coverage —
     grouped/described (label+shortcut+badge+description), active-item, loading skeleton,
     empty/error/no-results, semantic compact+prominent; real js_action_discovery_panel in a
     token-colored frame, no fakes. Both previews build 0 errors. specimen=gap → ok. -->
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

- [x] FIXED Badge background/color: split the shared badge+kbd table; added a `__badge` accent override (`accent-base 16%` bg, `accent-base` color, `ActionDiscoveryPanel.svelte:285-288`) and a `__kbd` override (code-family, letter-spacing 0, text-transform none).
- [x] FIXED Chip `min-height`: contract said `1.5rem`; updated to `var(--poodle-action-discovery-chip-height)` (`1.375rem` md default) and added the per-size chip table (1.125/1.25/1.375/1.5/1.75rem).
- [x] FIXED Chip `padding` / `font-size`: now resolve from `chip-x` / `chip-font-size` custom props with the per-size table (lines 190-219).
- [x] FIXED Chip typography: added `font-family: label-family`, `font-weight: 600`, `letter-spacing: 0.03em`, `text-transform: uppercase`, `white-space: nowrap` to the shared table; kbd override resets letter-spacing/transform.
- [x] FIXED Skeleton row layout: rewrote `__skeleton-row` to `flex` + `justify-content: space-between` + `gap: chip-gap` (no padding/bg/radius); moved pad/width to `__skeletons` (`display:grid`, `gap: list-gap`, `width:100%`, `padding: skeleton-pad` `0.875rem`).
- [x] FIXED State region: §3 anatomy + §9 now document `__state` (`min-height: 10rem`, `place-items: center`) wrapping loading only; error/empty/no-results EmptyState renders directly under root with no `__state` wrapper.
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
- GPUI covers: Grouped, With descriptions and badges, **Active item** (accent bg + ring via `with_active_id`), **Loading (skeleton)**, Empty, **Error**, **No results**, Semantic presentation (compact + prominent). **GPUI specimen done** — full contract state coverage with real `ActionDiscoveryPanel` (no fakes); Jetstream pending engine recovery.
- Jetstream covers: Default (two empty sections), Empty. — missing: **action items entirely** (sections have `vec![]`), badges, shortcuts, descriptions, sizes, densities, loading/error states. Specimen is effectively non-functional until `js_action_discovery_panel` renders items.

## Notes

- Data-model divergence (accepted, not a contract bug): Svelte takes a flat `items: CommandActionItem[]` grouped at render by `item.group`; Rust spec takes pre-grouped `sections: Vec<ActionDiscoverySection>`. Both express the same anatomy; the Rust shape just front-loads grouping. Spec also lacks `active_id`/`size`-override-for-chips plumbing exposed to the component.
- The `summary_tone()` helper on the spec (`action_discovery_panel.rs:58`) has no Svelte/contract analogue — Rust-only convenience, harmless.
- Biggest gap: Jetstream is a stub (9 todos) and its specimen renders empty sections; GPUI is functional but missing the active-item state and has token-literal violations.
