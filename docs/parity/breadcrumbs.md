<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass 42: Jetstream specimen backfilled to full contract §12 coverage — Basic, Deep path, Collapsed (max 3), Size ladder (xs..xl), Density ladder — via real js_breadcrumbs + BreadcrumbsSpec builders; GPUI already full. Leading/home-icon group not represented (no BreadcrumbItem icon field in API/contract). Both previews build clean. -->
<!-- pass 41: both targets — ellipsis truncation, density-gap ladder, breadcrumbs font ladder (md=body-size), dropped current weight-bump, chevron-icon separator at 0.4 on Jetstream; spec gained force_last_item_current + visible_items()/is_current_at() helpers + ELLIPSIS_VALUE. Remaining: href→anchor (GPUI router-hook gap, accepted), click/nav (preview-loop). -->
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
- [x] FIXED (Rust spec) `BreadcrumbsSpec` now has `force_last_item_current` (default true) plus `visible_items()` / `is_current_at()` helpers and the `ELLIPSIS_VALUE` sentinel; both Rust impls thread it through (`is_current_at`) so the last-item-current behavior is opt-out-able, matching Svelte.
- Svelte separator uses the `Icon` component `name="chevron-right"` (line 60), contract §2/§9 confirm. No divergence — flagged for Rust parity below.
- Otherwise contract↔Svelte align (nav/ol/li anatomy, aria-current, aria-hidden separators+ellipsis, size/density tables). **Action: extend contract + spec for `forceLastItemCurrent`; no Svelte change.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Truncation now inserts the ellipsis — `into_element` calls `spec.visible_items()` which builds `first + … + last N-1` (matching Svelte); the `…` crumb is rendered non-interactive (no `on_click`).
- [x] FIXED Dropped the `FontWeight::MEDIUM` bump on current — current is now color-only (`current_text_color`), matching Svelte CSS lines 102-104.
- [x] FIXED Density folded into the gap calc — `breadcrumbs_density_gap_rem` overrides the size gap when density != default (compact 0.25, comfortable 0.75rem).
- [x] FIXED `href` items differentiated — items with `href` no longer attach `on_navigate` (anchor navigation is a router concern GPUI lacks); only no-href items fire the callback. (router-hook gap noted in code.)
- accepted: Separator `opacity(0.4)` is the contract-pinned constant (contract §8 separator opacity 0.4); no `separator-opacity` token exists, so the raw float matches the contract value.
- accepted: no ARIA (gpui has no accessibility API) — no `<nav>`/`aria-label`/`aria-current`/`aria-hidden`; semantics carried visually only.
- accepted: hover uses `color.accent.base` (`spec.hover_color_token()`, `breadcrumbs.rs:163`) — Svelte relies on browser default link hover; accent hover is an acceptable native affordance.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Separator is now the `chevron-right` Icon (via `js_icon`) wrapped at opacity 0.4, not a `"/"` label — matches contract §2/§9.
- [x] FIXED Gap resolves from the size ladder (`breadcrumbs_gap_rem`) with density override (`breadcrumbs_density_gap_rem`); md = `space.inline.sm` (0.5rem).
- [x] FIXED Truncation implemented via `spec.visible_items()` — `first + … + last N-1`, middle items collapse to the ellipsis crumb.
- [x] FIXED Dropped the weight `600` bump on current — current is color-only.
- [x] FIXED Separator now renders at opacity 0.4 (contract value).
- [x] FIXED size/density honored — font from `breadcrumbs_font_rem` (md=body-size), gap from the size/density ladder.
- accepted: `href` vs callback not modeled — interaction (link vs button) lives in the preview event loop; the component renders crumbs only (Jetstream emits no clickable affordance).
- accepted: no ARIA channel (no accessibility API).
- accepted: click/navigation handled by preview event loop, not the component.

## Specimen parity

- Svelte covers: Basic (+ onNavigate readout), Deep path, Collapsed (`maxVisibleItems=3`), Size ladder (xs..xl), Density ladder (compact/default/comfortable). (`BreadcrumbsSpecimen.svelte`)
- GPUI covers: Basic, Deep path, Collapsed (max 3), Size ladder + Density ladder (via `specimen_layout` closures). — missing: onNavigate readout (interactive), but state hook is wired (`on_navigate` exists). Effectively full coverage; collapsed group will visibly differ until the ellipsis todo lands.
- Jetstream now covers: Basic, Deep path, Collapsed (max 3), Size ladder (xs..xl), Density ladder (`breadcrumbs.rs`) — built from real `js_breadcrumbs` + `BreadcrumbsSpec` builders, matching contract §12 and the Svelte/GPUI specimen sets. No fakes. `specimen=ok`.
- Not represented on any target: a "with leading/home icon" group. `BreadcrumbItem` has no icon field and the contract anatomy §2 carries no per-item icon (only the separator chevron), so a real leading-icon crumb is unrepresentable without inventing API — skipped per CLAUDE.md "no fakes".

## Notes

- `consv=gap` is driven solely by undocumented `forceLastItemCurrent` (Svelte) + the spec's hardcoded equivalent. Add the prop to contract §3 and to `BreadcrumbsSpec`.
- Both Rust impls hardcode "last item = current"; once `force_last_item_current` lands on the spec, thread it through `breadcrumbs.rs:148` and `breadcrumbs_comp.rs:23`.
- Jetstream's `"/"` separator and missing opacity make it the visually furthest from Svelte; GPUI is close except the dropped ellipsis and the spurious medium weight on current.
