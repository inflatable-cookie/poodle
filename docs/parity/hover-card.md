<!-- parity consv=ok gpui=4 jetstream=6 specimen=ok -->
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

- [ ] Hardcoded shadow literals at `hover_card.rs:160-170` — two `BoxShadow` with raw `hsla(0.0,0.0,0.0,0.10)`/`0.06`, `px(16.0)`, `px(4.0)`. Contract §8 specifies `box-shadow: var(--poodle-elevation-overlay)`; resolve from the elevation-overlay token (spec already exposes `shadow_token()`), not raw HSLA/px.
- [ ] No focus ring on trigger — contract §8 requires `outline: border-width-focus solid accent.focusRing; outline-offset: 0.125rem`; trigger wrapper (`hover_card.rs:120-128`) applies none.
- [ ] Spec default mismatch: `HoverCardSpec` defaults `open_delay_ms=400 close_delay_ms=150 placement=Bottom` (`packages/contracts/components/src/hover_card.rs:25-27`); contract/Svelte are 180/120/Top — fix spec defaults.
- [ ] Placement is render-only: `placement_id` computed (`hover_card.rs:84-97`) but never used for positioning; surface always stacks below trigger via `flex_col`. Accept as runtime delta only if documented; contract §7 expects JS-computed anchored placement.
- accepted: no ARIA (gpui has no accessibility API) — role/aria-expanded/aria-controls/aria-label not emitted.
- accepted: delay timers + viewport clamping live in host event loop, not the component (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded padding `pad_x = rem_to_px(0.75)`, `pad_y = rem_to_px(0.5)` at `hover_card.rs:19-20` — contract §8 surface padding is `var(--poodle-space-panel-y) var(--poodle-space-panel-x)`; resolve `space.panel.x`/`space.panel.y` tokens, do not hardcode rem.
- [ ] Border at full opacity: `resolve_color(theme, "color.border.default")` (`hover_card.rs:15`) — contract requires `color-mix(border-default 72%, transparent)`; apply 0.72 alpha multiplier (GPUI does this).
- [ ] Background not mixed: uses `spec.fill_token()` = raw `color.background.elevated` (`hover_card.rs:14`) — contract requires `color-mix(elevated 98%, panel)`.
- [ ] `shadow_md()` instead of elevation token (`hover_card.rs:28`) — contract §8 box-shadow is `elevation-overlay`; spec exposes `shadow_token()` (unused). Resolve from token.
- [ ] No min-width/max-width — contract §7 requires `min-width: 14rem`, `max-width: min(22rem, 90vw)`; `js_hover_card` sets neither.
- [ ] No placement handling — `spec.placement` ignored; surface is a bare `overlay()` with no anchored positioning.
- accepted: no ARIA channel (role/dialog/aria-label).
- accepted: hover/delay interaction lives in preview `main.rs` event loop, not the component.

## Specimen parity

- Svelte covers: Default (top placement), Bottom placement — both with realistic trigger link + content (name/bio, repo/stats).
- GPUI covers: top + bottom placement groups with content (135 lines). — ok.
- Jetstream covers: With content, Minimal content, Empty — all content variants but no explicit placement variant; renders surface inline (no live hover). — ok-ish; placement coverage thinner than Svelte but states are demonstrated.

## Notes

- consv=ok: Svelte is a faithful contract implementation. The only delta is the Rust `HoverCardSpec` defaults, which diverge from contract/Svelte — counted as a todo under each Rust target, not as a contract↔Svelte gap.
- Both Rust targets render placement as a static stack rather than anchored JS positioning. Real anchoring + delay timing is host-driven (contract §12 Known Delta), so this is accepted at the component layer — but the surface visual tokens (padding/border/bg/shadow/width) must still resolve correctly, and Jetstream's do not.
