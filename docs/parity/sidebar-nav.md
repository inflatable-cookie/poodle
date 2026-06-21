<!-- parity consv=fixed gpui=0 jetstream=1 specimen=ok pass=41 -->
# Parity: SidebarNav

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/sidebar-nav.md`
- Svelte (authoritative): `packages/svelte/components/src/SidebarNav.svelte`
- GPUI (composite): `packages/gpui/components/src/composites/sidebar_nav.rs`
- Jetstream: `packages/jetstream/components/src/sidebar_nav.rs`
- Spec (poodle-specs): `packages/contracts/components/src/sidebar_nav.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SidebarNavSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/sidebar_nav.rs` · jetstream `packages/jetstream/preview/src/specimens/sidebar_nav.rs`

SidebarNav is a self-contained composite. It does NOT reference a separate nav-item component — items are rendered inline as `<a>`/`<button>` (Svelte) or `div`/`button` builders (Rust). No icons, badges, or counts exist in the contract or any implementation (anatomy is label-only).

## Contract ↔ Svelte

Class-name prefix divergence + one callback-naming divergence. Svelte is authoritative.

- [x] FIXED: prefixed every class name in contract §2 (anatomy tree + Parts table) and §8 (all CSS selector headings) with `poodle-` to match Svelte (`poodle-sidebar-nav`, `poodle-sidebar-nav__group`, `poodle-sidebar-nav__group-title`, `poodle-sidebar-nav__list`, `poodle-sidebar-nav__item`, `poodle-sidebar-nav__item--active`). CSS custom properties already carried the prefix.
- [x] FIXED: added `onValueChange` row to §3 Public Props table (payload = item `value`, fires on activation of a non-disabled item) — Svelte declares it in `Props` (line 17).
- [x] FIXED: §3 "Controlled And Uncontrolled" now states `value` is two-way bindable (`$bindable`) in the Svelte target, mutated internally on activation alongside firing `onValueChange` (Svelte lines 22,34).
- **No anatomy/state divergence otherwise**: all parts (root nav, group section, group title h2, list ul, item a/button), all states (active accent fill + left border + inset shadow, hover, disabled, focus-visible), and all ARIA (`aria-current="page"`, group `aria-label`, native `disabled`) are present in Svelte exactly per contract §2/§4/§6. Size/density token tables (§8) match Svelte CSS line-for-line.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED item-radius offset — `control_radius - px(rem_to_px(0.125))` (contract `calc(radius-control - 0.125rem)`); raw `2.0` gone.
- [x] FIXED section/separator magic numbers — group internal gap `rem_to_px(0.3125)`, list gap `rem_to_px(0.125)`, separator `mt rem_to_px(0.125)` + `pt (group_gap - separator_mt)`. All keyed off the spec's contract §8 density helpers (`group_gap_rem`, etc.).
- [x] FIXED active rail — now a 3px (`rem_to_px(0.1875)`) **left border** on the item itself via `border_l_3()` + `border_color(accent)`, with a transparent 3px left border reserved on inactive items. The absolute pill (4px insets, 999 radius) is gone.
- [x] FIXED active inset box-shadow — contract `inset 0 0 0 0.0625rem accent@20%` rendered as a full-bleed absolutely-positioned `inset_0` child with a 1px border at accent@20% (GPUI `BoxShadow` has no inset variant — this is the faithful substitute).
- [x] FIXED alpha factors — `ACTIVE_BG_ALPHA 0.10`, `ACTIVE_RING_ALPHA 0.20`, `HOVER_BG_ALPHA 0.60`, `SEPARATOR_ALPHA 0.54` are now named consts tracing to contract color-mix percentages; hover/active/separator fills resolve from `hover_fill_token`/`active_fill_token`/`separator_color_token`.
- [x] FIXED size table keys off raw `data-size` (Svelte CSS behavior), not the chrome-resolved size — was rendering Sm geometry for a Md sidebar. Item/font/title resolve from `spec.item_height_rem`/`item_font_rem`/`title_font_rem` (raw size).
- accepted: no ARIA (gpui has no accessibility API) — `aria-current="page"`, nav role, group `aria-label`, native `disabled` not emitted.
- accepted: letter-spacing for group titles dropped (GPUI has no letter-spacing API).

## Jetstream gap (vs Svelte + contract)

- [x] DONE item height across all 5 sizes — now `spec.item_height_rem()` (xs 1.375 / sm 1.625 / md 1.875 / lg 2.125 / xl 2.375), promoted to a shared `poodle-specs` helper keyed off raw `data-size` (both targets use it). Probe-verified md = 30px, not control-height 36px.
- [x] FIXED group gap density-scaled — `rem_to_px(spec.group_gap_rem())` (0.625/0.75/0.875rem), not a fixed `space.stack.md` token.
- [x] FIXED horizontal nav padding — root now `pl/pr(rem_to_px(0.375))` (contract `padding: var(--space-panel-y) 0.375rem`); panel-y density-driven.
- [x] FIXED title→list gap density-scaled — title `mb(rem_to_px(spec.title_gap_rem()))` (0.125/0.1875/0.25rem).
- [x] FIXED separator shape — reimplemented as `border_t_1()` + `mt(0.125rem)` + `pt(group_gap - 0.125rem)` on the group element (Svelte adjacent-sibling rule), not a floating divider div.
- [x] FIXED active inset box-shadow — emulated as a 1px all-side border at accent@20% (`tint(accent, 0.20)`) plus the 3px accent left rail; JsEl has no inset shadow.
- [x] FIXED hover state — `.hover(|s| s.text_color(text-primary).bg(elevated@60%))` (contract §4/§8).
- [x] FIXED focus ring — `.active(|s| s.border_color(accent-focusRing))` resolves the focus-ring color (probe captures layout only, so verified by construction; runtime focus styling owned by the loop).
- [x] FIXED alpha literals — `ACTIVE_BG_ALPHA 0.10`, `ACTIVE_RING_ALPHA 0.20`, `HOVER_BG_ALPHA 0.60`, `SEPARATOR_ALPHA 0.54` named consts tracing to contract color-mix percentages.
- [x] FIXED left-border parity — active rail `border_l(rem_to_px(0.1875))` (3px) + `border_color_left(accent)`; inactive items reserve a transparent 3px left border (no shift). Per-side border widths are draw-only in Jetstream layout, so no geometry shift regardless.
- [ ] No `value` / `onValueChange` wiring — `js_sidebar_nav` renders the active state for the current `value` but click→reselect lives in the preview `main.rs` event loop (still absent). Note.
- accepted: no ARIA channel (`aria-current`, nav role, group `aria-label`) — documented platform limit.
- accepted: interaction (click→select) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers (`SidebarNavSpecimen.svelte`): single-group plain list, multi-group with titles + separator, active item, sizes snippet (all 5 via SpecimenLayout), densities snippet (all 3), interactive `onValueChange` reselection. **No disabled-item specimen** — contract §12 "Disabled Items" specimen is not demonstrated in Svelte either.
- GPUI covers (`gpui/.../sidebar_nav.rs`): single-group catalogue, grouped verification nav, active item. — missing: **sizes**, **densities**, **disabled item**, interactive reselection (no `on_select` wired in specimen).
- Jetstream covers (`jetstream/.../sidebar_nav.rs`): **plain list** (single untitled group, active item), grouped (Workspace/Settings) with active item + **one disabled item** (Team), no-selection variant. Probe tests cover item-height (sidebar table, not control-height), group-title uppercasing + item labels, active accent@10% background, title-font < item-font, and disabled item presence. — still missing: **sizes**/**densities** grids, interactive reselection (preview-loop).

## Notes

- `specimen=gap`: GPUI and Jetstream both under-cover Svelte (no size/density grids); Svelte itself omits the contract's disabled-item specimen (only Jetstream demonstrates disabled). All three diverge from the contract's §12 specimen set in different directions.
- No component references icons, badges, or counts — the prompt's "may compose sub-parts (icons/badges/counts)" does not apply; SidebarNav anatomy is label-only per contract §2.
- Biggest single Jetstream bug: item height pulled from `control_height_rem` instead of the contract's sidebar-specific height table — every size is too tall (Md 2.25rem vs 1.875rem).
- `consv=fixed`: the cosmetic/documentation drivers are resolved (`poodle-` class prefix added throughout §2/§8; `onValueChange` and bindable-`value` documented in §3). Anatomy, states, ARIA, and token tables all match Svelte.
- Pass 41: promoted the size/density tables to additive `SidebarNavSpec` helpers so both targets resolve from one place — `item_height_rem`/`item_font_rem`/`title_font_rem` (keyed off raw `data-size`, matching Svelte CSS), `group_gap_rem`/`item_pad_inline_rem`/`item_pad_block_rem`/`title_gap_rem`, plus token methods (`hover_fill`/`active_fill`/`disabled_opacity`) and `effective_size`. Unit-tested in poodle-specs. No token gaps. Both targets rebuilt: GPUI active rail = real left border + inset-ring overlay; Jetstream uses per-side border widths/colors for rail + ring. Known Delta: neither target supports a native CSS-style inset box-shadow, so the ring is a 1px border substitute.
