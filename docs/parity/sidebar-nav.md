<!-- parity consv=gap gpui=6 jetstream=11 specimen=gap -->
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

- **Class names**: Svelte uses `poodle-sidebar-nav`, `poodle-sidebar-nav__group`, `poodle-sidebar-nav__group-title`, `poodle-sidebar-nav__list`, `poodle-sidebar-nav__item`, `poodle-sidebar-nav__item--active` (lines 40,48,53,56,61,72). Contract §2/§8 omit the `poodle-` prefix everywhere (e.g. `sidebar-nav`, `sidebar-nav__item`). **Fix: prefix every class name in contract §2 (Parts table) and §8 (all CSS selector headings) with `poodle-`.**
- **Callback name**: contract §3 "Controlled And Uncontrolled" + §5 callbacks table name the event `onValueChange` — Svelte matches (`onValueChange`, lines 17,35). But §5 prose and §9 are consistent; the only naming wart is contract §1 nowhere documents `onValueChange` as a prop in §3 Public Props table (it's listed only under "Controlled And Uncontrolled" + §5). Svelte declares `onValueChange` in `Props` (line 17). **Fix: add `onValueChange` row to §3 Public Props table for completeness.**
- **`value` binding**: Svelte makes `value` `$bindable` (line 22) and mutates it internally on activation (line 34) in addition to firing `onValueChange`. Contract §3 says "Active item is controlled via `value`" implying pure-controlled. Svelte is effectively two-way bindable. **Fix: note in contract §3 that `value` is bindable (two-way) in the Svelte target, mutated on activation.**
- **No anatomy/state divergence otherwise**: all parts (root nav, group section, group title h2, list ul, item a/button), all states (active accent fill + left border + inset shadow, hover, disabled, focus-visible), and all ARIA (`aria-current="page"`, group `aria-label`, native `disabled`) are present in Svelte exactly per contract §2/§4/§6. Size/density token tables (§8) match Svelte CSS line-for-line.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded item-radius offset `px(2.0)` at `sidebar_nav.rs:135` (`control_radius - px(2.0)`) — contract item radius is `calc(radius-control - 0.125rem)`; resolve `0.125rem` via `rem_to_px(0.125)`, not raw `2.0`.
- [ ] Hardcoded section/separator magic numbers at `sidebar_nav.rs:163,167,171,189` — group internal gap `px(2.0)`, separator `mt px(2.0)`, `pt px(group_gap - 2.0)`, list gap `px(2.0)`. Contract: group gap `0.3125rem`, list gap `0.125rem`, separator margin-top `0.125rem`, pad-top `calc(group-gap - 0.125rem)`. Replace each `2.0` with `rem_to_px(...)` of the contract rem value; group gap should be `rem_to_px(0.3125)` not `2.0`.
- [ ] Hardcoded active-rail indicator literals at `sidebar_nav.rs:249-254` — `left(px(2.0))`, `top(px(4.0))`, `bottom(px(4.0))`, `w(px(3.0))`, `rounded(px(999.0))`. Width should be `rem_to_px(0.1875)` (contract 3px left border); the inset/pill (4px insets, 999 radius) is a GPUI-only invention not in contract — either token-resolve or document as Known Delta.
- [ ] Missing active inset box-shadow — contract §8 `.sidebar-nav__item--active` requires `box-shadow: inset 0 0 0 0.0625rem accent@20%`; GPUI applies only `bg(active_bg)` + rail (`sidebar_nav.rs:217-222`), no inset border shadow. Add an inset ring (accent at 0.20 alpha).
- [ ] Active background alpha hardcoded `* 0.10` and hover `* 0.60` at `sidebar_nav.rs:138,140` and separator `* 0.54` at `:171` — these alpha factors are magic float literals; contract derives them from `color-mix` percentages (10%, 60%, 54%). Acceptable as literal-from-contract but flag: lift to named consts so they trace to contract values.
- [ ] Active rail uses absolute-positioned child pill instead of a left border on the item itself — contract §8 note + §10 GPUI Notes mandate "active indicator is a left border (not a pseudo-element)". GPUI renders a separate `absolute` div (`sidebar_nav.rs:246-256`). Switch to `border_l` + `border_color` on the item to match contract intent (also removes the `top/bottom px(4.0)` magic insets).
- accepted: no ARIA (gpui has no accessibility API) — `aria-current="page"`, nav role, group `aria-label`, native `disabled` not emitted.
- accepted: letter-spacing for group titles dropped (`title_spacing` discarded at `sidebar_nav.rs:123`; GPUI has no letter-spacing API).

## Jetstream gap (vs Svelte + contract)

- [ ] **Item height wrong across all 5 sizes** — `sidebar_nav.rs:15` uses `control_height_rem(effective_size)` (Md=2.25rem, xs=1.5, sm=1.75, lg=2.75, xl=3.25) but contract §8 Size Variants item-height is Md=1.875, xs=1.375, sm=1.625, lg=2.125, xl=2.375. Add a `sidebar_nav_item_height_rem(size)` helper (or inline match) using the contract values; do NOT reuse `control_height_rem`.
- [ ] **Group gap wrong / not density-scaled** — `sidebar_nav.rs:24` uses `resolve_px(theme, "space.stack.md")` (single fixed token) but contract §8 Density table requires group-gap = 0.625/0.75/0.875rem per compact/default/comfortable. Resolve per-density from the density value, not a fixed stack token.
- [ ] **Missing horizontal nav padding** — `sidebar_nav.rs:42-46` root sets only `pt/pb(pad_y)`, no horizontal padding. Contract §8 root `padding: var(--space-panel-y) 0.375rem`. Add `pl/pr(rem_to_px(0.375))`.
- [ ] **Group internal gap wrong** — `sidebar_nav.rs:49` uses `gap(rem_to_px(0.3125))` (correct value) but item-list gap at `:73` uses `item_gap = rem_to_px(0.125)` (correct). OK — but title→list gap is collapsed: title uses `mb(title_mb=0.1875)` (`:26,68`) which is hardcoded for default density only; contract title-gap is density-scaled (0.125/0.1875/0.25rem). Scale `title_mb` by density.
- [ ] **Separator pad/margin hardcoded + wrong shape** — `sidebar_nav.rs:53-58` renders a standalone 1px divider child with `mb(rem_to_px(0.25))`, no top spacing/pad. Contract §8 separator = `margin-top 0.125rem` + `padding-top calc(group-gap - 0.125rem)` + `border-top` on the group itself. Reimplement as top border + top padding on the group element (matching Svelte adjacent-sibling rule), not a floating divider div.
- [ ] **Missing active inset box-shadow** — `sidebar_nav.rs:88-94` active branch sets bg + left border only; contract requires `box-shadow inset 0 0 0 0.0625rem accent@20%`. Add inset ring if JsEl supports it.
- [ ] **Missing hover state** — no `.hover(...)` treatment; contract §4/§8 hover = text-primary + elevated@60% bg. Jetstream items have no hover branch (`sidebar_nav.rs:102-106` only handles disabled/enabled focusable). Add hover styling.
- [ ] **Missing focus ring** — contract §6/§8 focus-visible = `border-width-focus solid accent-focusRing`, offset 0.125rem. Jetstream sets `.focusable()` (`:105`) but no focus-ring shadow/outline. Add focus-ring resolution (cf. GPUI `focus_ring_shadow`).
- [ ] **`tint(accent, 0.10)` / `tint(separator, 0.54)` alpha literals** at `sidebar_nav.rs:92,57` — magic factors; trace to contract color-mix percentages (10%, 54%) via named consts.
- [ ] **No `value` / no `onValueChange` wiring** — `js_sidebar_nav` renders only; selection is not flipped and no callback exists. The specimen (`jetstream/.../sidebar_nav.rs`) passes a static `with_value("projects")` and never updates it — selection is not interactive. Note whether the preview main.rs event loop is expected to own click→reselect; currently absent.
- [ ] **No item radius parity check on left-border item** — active item uses `border_l(2.0)` (`:93`) but contract left border is `0.1875rem` (3px), and inactive items get no transparent left border (so active/inactive shift horizontally by 2px). Use `rem_to_px(0.1875)` and reserve a transparent left border on inactive items to prevent layout shift.
- accepted: no ARIA channel (`aria-current`, nav role, group `aria-label`) — documented platform limit.
- accepted: interaction (click→select) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers (`SidebarNavSpecimen.svelte`): single-group plain list, multi-group with titles + separator, active item, sizes snippet (all 5 via SpecimenLayout), densities snippet (all 3), interactive `onValueChange` reselection. **No disabled-item specimen** — contract §12 "Disabled Items" specimen is not demonstrated in Svelte either.
- GPUI covers (`gpui/.../sidebar_nav.rs`): single-group catalogue, grouped verification nav, active item. — missing: **sizes**, **densities**, **disabled item**, interactive reselection (no `on_select` wired in specimen).
- Jetstream covers (`jetstream/.../sidebar_nav.rs`): grouped (Workspace/Settings) with active item, no-selection variant, **one disabled item** (Team). — missing: **single untitled group / plain list**, **sizes**, **densities**, interactive reselection.

## Notes

- `specimen=gap`: GPUI and Jetstream both under-cover Svelte (no size/density grids); Svelte itself omits the contract's disabled-item specimen (only Jetstream demonstrates disabled). All three diverge from the contract's §12 specimen set in different directions.
- No component references icons, badges, or counts — the prompt's "may compose sub-parts (icons/badges/counts)" does not apply; SidebarNav anatomy is label-only per contract §2.
- Biggest single Jetstream bug: item height pulled from `control_height_rem` instead of the contract's sidebar-specific height table — every size is too tall (Md 2.25rem vs 1.875rem).
- The `consv=gap` driver is cosmetic/documentation only (`poodle-` class prefix + missing `onValueChange`/bindable-`value` rows in contract §3). Anatomy, states, ARIA, and token tables all match Svelte. Lower-severity than the Button audit's missing-surface gaps.
