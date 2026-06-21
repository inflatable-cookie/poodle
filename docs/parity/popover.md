<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap | pass: PopoverSpec gains disabled/block/surface_width/surface_min/max_width_rem (additive) + border token methods; GPUI wires disabled (blocks open), surfaceWidth=trigger, min/max overrides, initialFocus focusable branch; Jetstream resolves panel padding + border-subtle@74% + min/max width from tokens (shadow approximated, placement/trigger/open-close = preview-loop) -->
# Parity: Popover

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/popover.md`
- Svelte (authoritative): `packages/svelte/components/src/Popover.svelte`
- GPUI: `packages/gpui/components/src/primitives/popover.rs`
- Jetstream: `packages/jetstream/components/src/popover.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PopoverSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/popover.rs` · jetstream `packages/jetstream/preview/src/specimens/popover.rs`

## Contract ↔ Svelte

Svelte exposes props the contract omits, and the contract's surface token table (border/background/shadow) disagrees with what Svelte ships. Svelte is authoritative — update the contract.

- FIXED — `disabled?: boolean` (default `false`) added to contract §3 props, §6 disabled-trigger semantics (`data-disabled`/`aria-disabled`/`tabindex=-1`/blocks open), and a §8 `.popover__trigger[data-disabled]` `cursor: not-allowed` table.
- FIXED — `surfaceWidth?: "content" | "trigger"` (default `"content"`) promoted to a documented §3 prop, §7 sizing prose, a §8 `data-surface-width="trigger"` table (`width:100%`/`min-width:100%`/`box-sizing:border-box`), and a `data-surface-width` data-attr.
- FIXED — `surfaceMinWidth` / `surfaceMaxWidth` (`string | null`, default `null`) added to §3 props and the §8 CSS Custom Properties table; surface min/max-width now expressed as `var(--poodle-popover-surface-min-width, 14rem)` / `var(--poodle-popover-surface-max-width, min(24rem, 90vw))`.
- FIXED — surface **border**: §8 now `0.0625rem solid var(--poodle-treatment-surface-elevated-border, color-mix(border-subtle 74%, transparent))` (was wrong token `border-default` at 72%).
- FIXED — surface **background**: §8 now plain `var(--poodle-color-background-elevated)` (was the `elevated 98%, panel` mix Svelte does not do); `--poodle-surface` matched.
- FIXED — surface **box-shadow**: §8 now documents Svelte's literal 3-layer stack (inset highlight + two drop shadows); §11 Tier-2 line updated. Svelte authoritative.
- FIXED — surface **radius**: §8 now `var(--poodle-treatment-surface-elevated-radius, var(--poodle-radius-surface))` (treatment-elevated override noted).
- `block` prop already present in both — OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- accepted: shadow already resolves from `elevation_overlay_shadow()` (token-resolved elevation stack), not raw HSLA/px. (Svelte's exact inset-highlight + two-drop stack is an internal rendering delta — Tier-3.)
- [x] Border token: now resolves via `spec.surface_border_token()` (`color.border.subtle`) × `spec.surface_border_alpha()` (0.74) and bg via `spec.surface_fill_token()` (plain `elevated`) — matches the reconciled contract §8.
- [x] `surfaceWidth`/`surfaceMinWidth`/`surfaceMaxWidth` now supported: builder methods `surface_width`/`surface_min_width_rem`/`surface_max_width_rem`; min/max-width resolve from `spec.effective_surface_{min,max}_width_rem()` (14rem/24rem defaults); `surfaceWidth="trigger"` sets the surface to `w_full().min_w(100%)`.
- [x] `disabled` now supported: builder `disabled(bool)`; a disabled trigger ignores the click handler and the surface is suppressed (`current_open() && !disabled`). (aria-disabled/not-allowed cursor remain GPUI ARIA deltas.)
- [x] `initialFocus` honored: surface is `.focusable()` only when `initial_focus != None`; `"content"` focuses the surface, otherwise the parent focuses the first focusable child. (first-focusable vs content child distinction stays a GPUI focus-model delta.)
- accepted: no ARIA — `role="dialog"`, `aria-expanded`, `aria-controls` not expressible on GPUI native elements (documented in file header).
- accepted: `dismiss_on_outside_interact` — no window-level outside-click interceptor; Escape-to-close wired instead. Spec field stored, unused.

## Jetstream gap (vs Svelte + contract)

- [x] Panel padding now `resolve_px(theme, "space.panel.x")` / `space.panel.y` (contract §8), matching GPUI — no raw rem literals.
- [x] Border token now `tint(spec.surface_border_token() = border-subtle, spec.surface_border_alpha() = 0.74)` — matches Svelte/contract §8; border-width from `border.width.default`.
- [x] min-width/max-width now applied from `spec.effective_surface_{min,max}_width_rem()` (14rem / 24rem defaults; overridable). The `min(…,90vw)` clamp is viewport-relative and not expressible — 24rem arm used (note).
- accepted: `shadow_md()` retained — JsEl has no token-resolved box-shadow primitive; the contract `elevation-overlay` 3-layer stack is approximated (JsEl delta).
- accepted: `placement` anchoring lives in the preview event loop (`overlay()` lifts the surface; the loop positions it). The component renders the open panel at current state — no in-component collision engine.
- accepted: trigger rendering + open/close + dismiss/Escape live in the preview `main.rs` event loop, not the component (Jetstream has no DOM/overlay-positioning engine in the component layer).
- accepted: `disabled`/`onOpenChange`/`initialFocus` are interaction concerns handled by the preview loop; spec now carries `disabled`/`surface_width`/min/max for callers that build the surface.
- accepted: no ARIA channel (`role="dialog"`, `aria-expanded`, `aria-controls`) — Jetstream has no accessibility API.

## Specimen parity

- Svelte covers: Default (bottom-start) with Button trigger + heading/paragraph content, Top placement with Button trigger + paragraph — both interactive (open/dismiss/Escape) (`PopoverSpecimen.svelte`). Matches contract §13 exactly.
- GPUI covers: Default (bottom-start) + Top placement, real Button triggers, `on_open_change` wired through `overlay_state`, open/close interactive (`gpui/.../popover.rs`). — matches Svelte; closest parity of the three.
- Jetstream covers: "With text content", "With rich content" (divider + options), "Empty (no content)" — **surface-only, no trigger, no placement, not interactive** (`jetstream/.../popover.rs`). — missing: **Button trigger**, **bottom-start vs top placement** groups, **open/dismiss interaction**. Demonstrates content shells rather than the contract's anchored-overlay behavior. Largest specimen gap.

## Notes

- `consv=fixed`: former drivers (undocumented `disabled`/`surfaceWidth`/`surfaceMinWidth`/`surfaceMaxWidth` props, plus §8 surface tokens — border token+%, background mix, shadow, radius) reconciled into the contract per "Svelte is parity authority".
- GPUI is the only target with a working trigger→surface→dismiss flow. Jetstream `js_popover` is a passive surface shell; the anchored-overlay contract (placement, offset, outside-dismiss) is effectively unimplemented there.
- Both Rust shadow treatments (GPUI hardcoded HSLA stack, Jetstream `shadow_md()`) bypass token resolution — once an `elevation-overlay`/`elevation-popover` token exists, both should consume it.
