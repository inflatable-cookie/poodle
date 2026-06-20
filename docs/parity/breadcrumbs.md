<!-- parity consv=fixed gpui=5 jetstream=7 specimen=gap -->
# Parity: Breadcrumbs

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/breadcrumbs.md`
- Svelte (authoritative): `packages/svelte/components/src/Breadcrumbs.svelte`
- GPUI: `packages/gpui/components/src/primitives/breadcrumbs.rs`
- Jetstream: `packages/jetstream/components/src/breadcrumbs_comp.rs`
- Spec: `packages/contracts/components/src/breadcrumbs.rs` (`BreadcrumbsSpec`, `BreadcrumbItem`)
- Specimens: svelte `packages/svelte/preview/src/specimens/BreadcrumbsSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/breadcrumbs_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/breadcrumbs.rs`

## Contract ↔ Svelte

Most props match (contract §3 `items`/`ariaLabel`/`maxVisibleItems`/`size`/`sizeRole`/`density`, callback `onNavigate`, anatomy §2). One Svelte prop is undocumented; Svelte is authoritative.

- [x] FIXED Svelte `forceLastItemCurrent?: boolean` (default `true`, `Breadcrumbs.svelte:11`) added to contract §3 props; §4 current state + §6 aria-current note updated to reference the prop instead of "or last item in the list".
- (Rust spec todo, not a contract edit) `BreadcrumbsSpec` hardcodes the equivalent of `forceLastItemCurrent=true` — both Rust impls treat `i == last_idx` as current with no opt-out (`breadcrumbs.rs:148`, `breadcrumbs_comp.rs:23`). Add `force_last_item_current` to the spec to match Svelte.
- Svelte separator uses the `Icon` component `name="chevron-right"` (line 60), contract §2/§9 confirm. No divergence — flagged for Rust parity below.
- Otherwise contract↔Svelte align (nav/ol/li anatomy, aria-current, aria-hidden separators+ellipsis, size/density tables). **Action: extend contract + spec for `forceLastItemCurrent`; no Svelte change.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Truncation drops the ellipsis entry — `into_element` builds `first + last N-1` with no `…` item (`breadcrumbs.rs:101-114`); contract §3 truncation + §11 require `first + ellipsis + last N-1`. Insert a non-interactive `…` crumb.
- [ ] Current item adds `FontWeight::MEDIUM` at `breadcrumbs.rs:153` — Svelte applies no weight change to current (only `color: text-primary`, CSS line 102-104); drop the weight bump for visual parity.
- [ ] Density gap ignored — gap is size-only (`breadcrumbs.rs:50-55`); contract §8 density table (compact `0.25`, comfortable `0.75rem`) and `spec.density` are unused. Fold density into the gap calc.
- [ ] `href` items not differentiated — all non-current items render as clickable `div` with `on_navigate` (`breadcrumbs.rs:157-174`); contract §2/§5 want `href` → anchor navigation, no-href → callback. (GPUI has no anchor; note as router hook gap.)
- [ ] Separator `opacity(0.4)` literal at `breadcrumbs.rs:138` — matches contract value but is a raw float; source from a `separator-opacity` token if one exists, else accept as contract-pinned constant.
- accepted: no ARIA (gpui has no accessibility API) — no `<nav>`/`aria-label`/`aria-current`/`aria-hidden`; semantics carried visually only.
- accepted: hover uses `color.accent.base` (`spec.hover_color_token()`, `breadcrumbs.rs:163`) — Svelte relies on browser default link hover; accent hover is an acceptable native affordance.

## Jetstream gap (vs Svelte + contract)

- [ ] Separator is a literal `"/"` text label at `breadcrumbs_comp.rs:34` — contract §2/§9 separator is the `chevron-right` Icon at opacity 0.4; render the icon, not a slash.
- [ ] Hardcoded gap `rem_to_px(0.25)` at `breadcrumbs_comp.rs:13` — ignores both the size table (`space.inline.sm`/0.375 at md) and density; resolve gap from `spec.gap_token()` and apply the size/density ladder.
- [ ] No truncation — `max_visible_items` and the ellipsis collapse (contract §3) are unimplemented; iterates all items (`breadcrumbs_comp.rs:22-36`).
- [ ] Current item adds weight `600` at `breadcrumbs_comp.rs:30` — Svelte changes color only, no weight; drop the weight bump.
- [ ] No separator opacity — separator label uses full `text.secondary` (`breadcrumbs_comp.rs:34`); contract separator opacity is 0.4.
- [ ] `href` vs callback not modeled — items render as plain labels; no `onNavigate` path (interaction would live in preview event loop, but the component emits no clickable affordance).
- [ ] No `size`/`density` honored for font/gap beyond `size_font_rem` font size (`breadcrumbs_comp.rs:11-12`); density unused entirely.
- accepted: no ARIA channel (no accessibility API).
- accepted: click/navigation handled by preview event loop, not the component.

## Specimen parity

- Svelte covers: Basic (+ onNavigate readout), Deep path, Collapsed (`maxVisibleItems=3`), Size ladder (xs..xl), Density ladder (compact/default/comfortable). (`BreadcrumbsSpecimen.svelte`)
- GPUI covers: Basic, Deep path, Collapsed (max 3), Size ladder + Density ladder (via `specimen_layout` closures). — missing: onNavigate readout (interactive), but state hook is wired (`on_navigate` exists). Effectively full coverage; collapsed group will visibly differ until the ellipsis todo lands.
- Jetstream covers: a single "3-item breadcrumb" group (`breadcrumbs.rs:18-21`). — missing: **Deep path**, **Collapsed**, **Size ladder**, **Density ladder**. Largest specimen gap; also the only specimen exercising `href` items, which the component ignores.

## Notes

- `consv=gap` is driven solely by undocumented `forceLastItemCurrent` (Svelte) + the spec's hardcoded equivalent. Add the prop to contract §3 and to `BreadcrumbsSpec`.
- Both Rust impls hardcode "last item = current"; once `force_last_item_current` lands on the spec, thread it through `breadcrumbs.rs:148` and `breadcrumbs_comp.rs:23`.
- Jetstream's `"/"` separator and missing opacity make it the visually furthest from Svelte; GPUI is close except the dropped ellipsis and the spurious medium weight on current.
