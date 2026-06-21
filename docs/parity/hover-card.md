<!-- parity consv=ok gpui=1 jetstream=0 specimen=ok -->
<!-- pass 41: spec defaults fixed (180/120/Top, were 400/150/Bottom). GPUI trigger
     focus ring added (focusRing border + ring shadow, same convention as Button);
     shadow already token-resolved (elevation_overlay_shadow) — no change. Jetstream
     surface rebuilt: panel-x/y padding tokens, border 0.72 alpha, bg color-mix(elevated
     98%, panel), min-width(size.menu.minWidth)/max-width(size.hoverCard.maxWidth); shadow
     stays shadow_md() JsEl approximation of elevation-overlay (noted). Probe tests added
     (surface+content, min-width, fill mix). gpui=1 = placement (accepted runtime delta). -->
# Parity: HoverCard

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/hover-card.md`
- Svelte (authoritative): `packages/svelte/components/src/HoverCard.svelte`
- GPUI: `packages/gpui/components/src/primitives/hover_card.rs`
- Jetstream: `packages/jetstream/components/src/hover_card.rs`
- Spec: `packages/contracts/components/src/hover_card.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/HoverCardSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/hover_card.rs` · jetstream `packages/jetstream/preview/src/specimens/hover_card.rs`

## Contract ↔ Svelte

Svelte matches the contract on every prop (name/type/default), anatomy part, state, role and ARIA attribute. No divergence.

- Props: `open` (null), `defaultOpen` (false), `openDelayMs` (180), `closeDelayMs` (120), `placement` ("top"), `ariaLabel` (null), `onOpenChange` — all match contract §3 exactly.
- Anatomy: root/trigger/surface all `<span>`; trigger `role="button" tabindex="0"` with `aria-expanded`/`aria-controls`; surface `role="dialog" tabindex="-1"` with `aria-label` — matches §2/§6.
- Surface tokens (z-index, min/max-width, padding, border color-mix 72%, bg color-mix 98%, radius, elevation) match §8 verbatim. Svelte additionally references `--poodle-treatment-surface-elevated-*` fallback vars; that is an implementation refinement of the same target, not a contract divergence.
- Escape/timer/hover-continuity behavior matches §4.

Note: the **Rust spec** (`hover_card.rs`) defaults `open_delay_ms=400`, `close_delay_ms=150`, `placement=Bottom` — these disagree with contract/Svelte (180/120/Top). That is a spec bug, not a contract↔Svelte divergence; flagged under both target gaps.

## GPUI gap (vs Svelte + contract)

- [x] Shadow already token-resolved — `into_element` uses `elevation_overlay_shadow()` (`hover_card.rs:158`), not raw HSLA/px. (Prior literal-shadow flag was stale; no change needed.)
- [x] Focus ring on trigger added — trigger wrapper is now `.focusable()` with `.focus(|s| s.border_color(focusRing).shadow(focus_ring_shadow(...)))`, the same approximation Button uses (contract §8; outline-offset is the documented GPUI delta).
- [x] Spec defaults fixed — `HoverCardSpec` now defaults `open_delay_ms=180 close_delay_ms=120 placement=Top` (`packages/contracts/components/src/hover_card.rs`), matching contract/Svelte.
- [ ] Placement is render-only: `placement_id` computed but never used for positioning; surface always stacks below trigger via `flex_col`. **Accepted runtime delta** — contract §12 makes anchored placement host-driven; the surface visual tokens all resolve correctly.
- accepted: no ARIA (gpui has no accessibility API) — role/aria-expanded/aria-controls/aria-label not emitted.
- accepted: delay timers + viewport clamping live in host event loop, not the component (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] Padding now token-resolved — `resolve_px("space.panel.x")` / `space.panel.y`, no hardcoded rem (contract §8).
- [x] Border opacity fixed — `tint(border-default, 0.72)` applies the 0.72 alpha multiplier, matching GPUI / contract `color-mix(border-default 72%, transparent)`.
- [x] Background mixed — `color_mix(elevated, panel, 0.98)` per contract `color-mix(elevated 98%, panel)`.
- [~] Shadow: still `shadow_md()` — JsEl has no per-token shadow channel; `shadow_md()` is the nearest elevated preset and `spec.shadow_token()` carries the intended `elevation-overlay` token. **JsEl approximation, noted.**
- [x] min-width/max-width added — `min_w(size.menu.minWidth)` (14rem) / `max_w(size.hoverCard.maxWidth)` (22rem); the `90vw` clamp is host-driven (contract §7).
- [ ] No placement handling — `spec.placement` ignored; surface is a bare `overlay()`. **Accepted preview-loop delta** (contract §12) — anchored positioning + delay timers live in the preview event loop.
- accepted: no ARIA channel (role/dialog/aria-label).
- accepted: hover/delay interaction lives in preview `main.rs` event loop, not the component.

## Specimen parity

- Svelte covers: Default (top placement), Bottom placement — both with realistic trigger link + content (name/bio, repo/stats).
- GPUI covers: top + bottom placement groups with content (135 lines). — ok.
- Jetstream covers: With content, Minimal content, Empty — all content variants but no explicit placement variant; renders surface inline (no live hover). — ok-ish; placement coverage thinner than Svelte but states are demonstrated.

## Notes

- consv=ok: Svelte is a faithful contract implementation. The only delta is the Rust `HoverCardSpec` defaults, which diverge from contract/Svelte — counted as a todo under each Rust target, not as a contract↔Svelte gap.
- Both Rust targets render placement as a static stack rather than anchored JS positioning. Real anchoring + delay timing is host-driven (contract §12 Known Delta), so this is accepted at the component layer — but the surface visual tokens (padding/border/bg/shadow/width) must still resolve correctly, and Jetstream's do not.
