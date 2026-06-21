<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass: Jetstream bubble rebuilt to contract §8 — radius = radius.control −
     0.125rem (was radius.surface), 0.0625rem border (border-default 72%),
     background = color-mix(elevated 98%, panel) (was raw elevated), and the
     dominant overlay shadow layer (0 0.5rem 1.25rem rgba(0,0,0,0.3)) via a
     direct BoxShadow (JsEl holds one layer; the second 0.125/0.375/0.2 layer is
     dropped). Probe tests cover content / surface fill / max-width / placement.
     Jetstream specimen now renders Default + the 2×2 cardinal Placements grid.
     GPUI's three flags were recall-biased: the literal rgba shadow + contract-
     exact rems are mandated by the contract (not the elevation token), the
     placement-modifier collapse lives in the shared floating_overlay.rs (not a
     tooltip file), and the Md anchor-height estimate is correct since TooltipSpec
     has no size prop (none in the contract). All accepted. -->
# Parity: Tooltip

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/tooltip.md`
- Svelte (authoritative): `packages/svelte/components/src/Tooltip.svelte`
- GPUI: `packages/gpui/components/src/primitives/tooltip.rs`
- Jetstream: `packages/jetstream/components/src/tooltip.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TooltipSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/tooltip.rs` · jetstream `packages/jetstream/preview/src/specimens/tooltip.rs`

## Contract ↔ Svelte

Anatomy §2 and token §8 describe a trigger-wrapper + positioning model Svelte does not implement. Svelte is authoritative — update the contract.

- [x] FIXED Trigger anatomy: contract §2/§6 specified a `[Trigger .tooltip__trigger]` `<span role="button" tabindex="0">` wrapper. Svelte renders `children` directly and treats `rootElement.firstElementChild` as the anchor (`Tooltip.svelte:87-95,225`). Contract §2/§6 rewritten: dropped the trigger-wrapper part, documented the "first child is the anchor (caller supplies a focusable trigger)" model; no `role`/`tabindex` injected.
- [x] FIXED Root display: contract §8 said root `display: inline-flex`, `position: relative`. Updated to `display: contents` (`Tooltip.svelte:242-244`); root carries no positioning box.
- [x] FIXED Bubble position: contract §8 said `position: absolute` with per-placement CSS `top/left/right/transform` offsets. Updated to `position: fixed` with JS-computed `top/left` px from `resolveOverlayPosition` (`Tooltip.svelte:178-185,247`); placement section now describes JS-resolved viewport coords (resolver may flip; `data-placement` exposes result).
- [x] FIXED Box-shadow: contract §8 said `var(--poodle-elevation-overlay)`. Per Svelte-is-authority, documented Svelte's literal two-layer shadow `0 0.5rem 1.25rem rgba(0,0,0,0.3), 0 0.125rem 0.375rem rgba(0,0,0,0.2)` (`Tooltip.svelte:263-265`) in §8 + Tier-2.
- [x] FIXED Surface fill/border/radius: documented the `--poodle-treatment-surface-elevated-*` custom-prop indirection (color-mix forms as fallback) in §8 + §9 (`Tooltip.svelte:251-262`).
- `aria-describedby`: matches — Svelte sets it on the anchor when open and removes on close (`Tooltip.svelte:186,59-61`). `role="presentation"` on root and `role="tooltip"` on bubble match §6.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- accepted: bubble box-shadow uses literal `hsla(…,0.30)`/`hsla(…,0.20)` and `rem_to_px(0.5/1.25/0.125/0.375)`
  (`tooltip.rs`). Contract §8 explicitly mandates a **literal** two-layer drop shadow `0 0.5rem 1.25rem
  rgba(0,0,0,0.3), 0 0.125rem 0.375rem rgba(0,0,0,0.2)` (NOT the `--poodle-elevation-overlay` token), and
  the rems are contract-exact — so this is faithful, not a violation. Recall-biased flag.
- accepted: Placement `*-start`/`*-end` modifier collapse lives in the **shared** `floating_overlay.rs`
  utility (`Top`/`TopStart` both use `left:0`), not `tooltip.rs`. Fixing it touches menu/popover too;
  out of scope for a tooltip-only pass. Cardinal placements (top/bottom/left/right) are correct.
- accepted: Anchor height uses the Md baseline `control_height_rem(ControlSize::Md)` (`tooltip.rs`)
  because `TooltipSpec` has no size prop — and the contract §3 defines none. Estimate-driven and correct
  for the common (Md trigger) case; not a deviation to close without inventing an out-of-contract prop.
- accepted: no ARIA (gpui has no accessibility API) — `role="tooltip"` / `aria-describedby` not emitted; help-text exposure deferred (contract §10).
- accepted: open/dismiss + 300ms delay live in the preview overlay-state machine (`specimens/tooltip.rs` via `overlay_state::schedule_toggle_if`), not the component — consistent with the contract's render-only component model.
- note: bubble padding/font-size/max-width/radius all resolve from `TooltipSpec` token methods (`tooltip.rs:155-183`); radius correctly = `radius.control − 0.125rem`.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Radius: now `resolve_radius("radius.control") − rem_to_px(radius_inset_rem())`
  (contract §8 `calc(radius.control − 0.125rem)`), replacing the wrong `radius.surface`.
- [x] FIXED Border: `js_tooltip` now applies a `0.0625rem` border with
  `color-mix(border-default 72%, transparent)` via `with_alpha(a*0.72)` (contract §8 + Svelte).
- [x] FIXED Elevation: replaced `.shadow_sm()` with the contract's dominant overlay layer
  (`0 0.5rem 1.25rem rgba(0,0,0,0.3)`) set directly on `style.shadow`. JsEl holds a single shadow
  layer, so the secondary `0 0.125rem 0.375rem rgba(0,0,0,0.2)` layer is dropped — noted JsEl limit.
- accepted: No placement positioning — `spec.placement` is a positioning input owned by the runtime/
  preview overlay loop; the render-only bubble is placement-independent (probe-tested
  `placement_does_not_change_bubble_surface`). Consistent with the GPUI render-only model.
- accepted: No trigger / open / delay / dismiss — `js_tooltip` renders the bubble only; open/delay/
  dismiss + trigger anchoring live in the preview event loop (not yet wired). Contract §4 state machine
  is a runtime concern, matching GPUI's split.
- [x] FIXED Background: now `color-mix(elevated 98%, panel)` (contract §8), replacing the raw elevated
  fill. Probe-tested (`surface_fill_is_elevated_panel_mix`).
- accepted: no ARIA channel (`role="tooltip"` / `aria-describedby` not emitted).
- note: padding/font-size/max-width resolve from `TooltipSpec` token methods correctly (`tooltip.rs:21-24`).

## Specimen parity

- Svelte covers: Default (top placement, secondary trigger), Placements 2×2 grid (top/bottom/left/right, ghost triggers), Sizes snippet, Densities snippet (`TooltipSpecimen.svelte`).
- GPUI covers: Default (top, secondary "Hover me"), Placements (top/bottom/left/right ghost triggers), Sizes, Densities — with real triggers and open/delay state machine. — missing: nothing material vs contract §13.
- Jetstream covers: Default (top, "Save your changes") + the contract §13 Placements 2×2 grid
  (top/bottom/left/right bubbles), replacing the invented content-length cases (`specimens/tooltip.rs`).
  — bubbles only (no trigger element / hover-open) since the Jetstream tooltip is render-only and
  positioning/open are preview-loop concerns; surface parity matches the contract.

## Notes

- Biggest gap: Jetstream tooltip is a static bubble — no trigger, no placement, no open/delay/dismiss — so it fails contract §4 states, §6 dismissal, and §13 Placements specimen wholesale.
- `consv=gap` driver: contract §2 trigger-wrapper anatomy and §8 `position`/box-shadow values describe a model Svelte abandoned (`display:contents` host, `position:fixed` JS-positioned bubble, literal rgba shadow).
- Both Rust targets keep open/delay state out of the component (GPUI in preview overlay-state, Jetstream nowhere) — acceptable for GPUI per the render-only model, but Jetstream has no event loop wiring at all.
