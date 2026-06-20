<!-- parity consv=fixed gpui=3 jetstream=6 specimen=gap -->
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

- [ ] Token violation: bubble box-shadow hardcodes `hsla(0.0, 0.0, 0.0, 0.30)` / `hsla(0.0, 0.0, 0.0, 0.20)` and raw rem literals `rem_to_px(0.5)`, `rem_to_px(1.25)`, `rem_to_px(0.125)`, `rem_to_px(0.375)` (`tooltip.rs:166-178`) — resolve from an elevation/shadow token, not raw HSLA + magic rems.
- [ ] Placement `*-start` / `*-end` modifiers partially collapse: `TopStart`/`BottomStart` route to the same branch as `Top`/`Bottom` (`floating_overlay.rs:48,63`), so start-aligned vs centered differ from Svelte's `left:0`/`transform:none` modifier rules (contract §8 alignment modifiers).
- [ ] Anchor height is a hardcoded Md baseline `control_height_rem(ControlSize::Md)` (`tooltip.rs:200-201`) because `TooltipSpec` has no size; left/right placement will misalign for non-Md triggers. Note as estimate-driven.
- accepted: no ARIA (gpui has no accessibility API) — `role="tooltip"` / `aria-describedby` not emitted; help-text exposure deferred (contract §10).
- accepted: open/dismiss + 300ms delay live in the preview overlay-state machine (`specimens/tooltip.rs` via `overlay_state::schedule_toggle_if`), not the component — consistent with the contract's render-only component model.
- note: bubble padding/font-size/max-width/radius all resolve from `TooltipSpec` token methods (`tooltip.rs:155-183`); radius correctly = `radius.control − 0.125rem`.

## Jetstream gap (vs Svelte + contract)

- [ ] Wrong radius source: uses `resolve_radius("radius.surface")` (`tooltip.rs:16`) — contract §8 requires `calc(radius.control − 0.125rem)`. `TooltipSpec::radius_inset_rem()` exists but is unused. Resolve `radius.control` and subtract the inset.
- [ ] No border — contract §8 + Svelte require a `0.0625rem` border (`border-default 72%`); `js_tooltip` never calls `.border_*` (`tooltip.rs:27-38`). Add the border.
- [ ] Elevation mismatch: uses `.shadow_sm()` (`tooltip.rs:32`) instead of the contract `elevation-overlay` two-layer shadow. Resolve from the overlay elevation token.
- [ ] No placement handling — `js_tooltip` only calls `.overlay()` (`tooltip.rs:33`); `spec.placement` is never read. Contract §8 placement families/modifiers and Svelte's `resolveOverlayPosition` are unimplemented.
- [ ] No trigger / open / delay / dismiss model — `js_tooltip` always renders a bare bubble with no trigger wrapping, no `current_open()` gate, no hover/focus delay. Contract §4 (closed/pending/open) is absent. No tooltip hover wiring in preview `main.rs` event loop (grep: none).
- [ ] Background uses raw elevated fill (`spec.fill_token()` → `color.background.elevated`, `tooltip.rs:14`) — contract §8 requires `color-mix(elevated 98%, panel)`; no panel mix applied.
- accepted: no ARIA channel (`role="tooltip"` / `aria-describedby` not emitted).
- note: padding/font-size/max-width resolve from `TooltipSpec` token methods correctly (`tooltip.rs:21-24`).

## Specimen parity

- Svelte covers: Default (top placement, secondary trigger), Placements 2×2 grid (top/bottom/left/right, ghost triggers), Sizes snippet, Densities snippet (`TooltipSpecimen.svelte`).
- GPUI covers: Default (top, secondary "Hover me"), Placements (top/bottom/left/right ghost triggers), Sizes, Densities — with real triggers and open/delay state machine. — missing: nothing material vs contract §13.
- Jetstream covers: Default, Longer content, Short label — **all bare bubbles with no trigger and no placement** (`specimens/tooltip.rs`). — missing: **trigger element**, **all four cardinal Placements** (contract §13 requires the 2×2 grid), hover/open interaction. Specimen invents content-length cases the contract never specifies.

## Notes

- Biggest gap: Jetstream tooltip is a static bubble — no trigger, no placement, no open/delay/dismiss — so it fails contract §4 states, §6 dismissal, and §13 Placements specimen wholesale.
- `consv=gap` driver: contract §2 trigger-wrapper anatomy and §8 `position`/box-shadow values describe a model Svelte abandoned (`display:contents` host, `position:fixed` JS-positioned bubble, literal rgba shadow).
- Both Rust targets keep open/delay state out of the component (GPUI in preview overlay-state, Jetstream nowhere) — acceptable for GPUI per the render-only model, but Jetstream has no event loop wiring at all.
