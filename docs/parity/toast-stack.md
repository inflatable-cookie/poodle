<!-- parity consv=ok gpui=0 jetstream=3 specimen=gap pass=fixed-pass-42 -->
<!-- pass-42: GPUI toast bg now the contract 90deg tone→elevated linear-gradient (was flat tint) — last GPUI gap closed. Jetstream 3 remaining all preview-loop/ARIA (dismiss/action handlers, real Button slot, role=list) — accepted. -->
# Parity: ToastStack

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/toast-stack.md`
- Svelte (authoritative): `packages/svelte/components/src/ToastStack.svelte`
- GPUI: `packages/gpui/components/src/composites/toast_stack.rs`
- Jetstream: `packages/jetstream/components/src/toast_stack.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ToastStackSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/toast_stack_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/toast_stack.rs`

## Contract ↔ Svelte

Props, anatomy, tone set, ARIA, size/density tables all match Svelte. Minor notes only:

- Class names: contract §2/§8 use `.toast-stack`, `.toast`, `.toast__dismiss`, etc.; Svelte renders `.poodle-toast-stack`, `.poodle-toast`, `.poodle-toast__dismiss` (lines 33, 35, 42). Svelte authoritative. **Fix: prefix all selectors in contract §2/§8 with `poodle-`.**
- Live-region posture: contract §6 says stack is `aria-live="polite"`, `aria-atomic="false"`; toast `aria-atomic="true"` and danger escalates to `aria-live="assertive"`. Svelte matches exactly (lines 33, 39-40). `ok`.
- `position` prop: Rust `ToastStackSpec` carries a `position` field (spec toast_stack.rs:64) but Svelte has no `position` prop — positioning is owned by ToastHost in the web target. Not a contract prop; **leave** (Rust-only field for standalone placement; note in §3).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] Info tone uses wrong token — FIXED: `ToastTone::Info` now maps to `COLOR_STATUS_INFO` (new `color.status.info` token), not `COLOR_ACCENT_BASE`. (The accent *fallback* read at toast_stack.rs:100 is a separate concern only hit for unknown tones.)
- [x] DONE (pass 41): accent-bar + tone mixes — accent bar `color_mix(tone, white, 0.94)` (≈82%/white-6% intent), border `color_mix(tone 34%, border-default)`.
- [x] DONE (pass 42): background is now the contract §8 `linear-gradient(90deg, color-mix(tone 12%, elevated) → elevated 18%)` via `gpui::linear_gradient`/`linear_color_stop` (was a flat `color_mix(tone 12%, elevated)` tint). Matches Svelte + Jetstream. The contract's two transparent-composited layers collapse to this single gradient on GPUI (no multi-layer alpha background channel). Bar is absolute, full-height.
- [x] DONE: dismiss is the `Icon` primitive (`name="x"`) with `aria_label("Dismiss {title}")`, absolute top-right, sized square.
- [x] DONE: action affordance is the real `Button` primitive (variant Secondary, size/density forwarded), wired to `on_action`.
- [x] DONE: per-size title + message font ladder (§8 size table); message is the smaller body scale, title the heading scale.
- [x] DONE: action/dismiss/container literals removed — action sits in the Button; dismiss/inset/container width are rem-derived (`22.5rem` container, per-size dismiss dims + inset).
- [x] DONE: per-size dismiss dimension + top/right inset ladder (1rem→1.75rem; 0.25→0.5rem).
- [x] DONE: density padding ladder — toast padding scaled ×0.75 / ×1.25 (contract-justified uniform scale, matches Svelte `padding` shorthand). Elevation shadow now applied (`elevation_overlay_shadow()`); radius = surface − 0.125rem; `overflow_hidden`.
- accepted: no ARIA / live-region (gpui has no accessibility API) — danger `aria-live="assertive"` escalation cannot be expressed.

## Jetstream gap (vs Svelte + contract)

- [x] DONE: built out the toast body — leading tone **accent bar** (`tone_color`, so Info now shows the status-info blue), required **title** (was message-only — message-only toasts rendered blank before), optional message, optional **action** label, and a **dismiss** `×` affordance. Probe-tested (title/message/action/dismiss present).
- [x] DONE (pass 41): tone-based mixes — accent bar `color_mix(tone, white, 0.94)`, border `color_mix(tone 34%, border-default)`, background `color_mix(tone 12%, elevated)` flat tint plus a 90° `bg_gradient_linear` (tone-tint → elevated at 18%). Probe-tested (info accent uses `status.info`; danger bg tint applied).
- [x] DONE: box-shadow — `shadow_md()` applied. Note: JsEl has no token-driven box-shadow, so `elevation.overlay` is approximated by the runtime medium-shadow preset (offset 0,4 / blur 8). (note)
- [x] DONE: `item_gap` resolves `spec.gap_token()` (`space-stack-sm`) — `0.5` literal dropped; stack gap uses the same token.
- [x] DONE: vertical-padding hack removed — toast padding is `space-panel-x × density-scale` on all sides (the magic `-0.25` and `panel_space_y_rem` are gone). Density-scaled padding is the contract-justified case (Svelte `padding` shorthand).
- [x] DONE: size ladder (per-size dismiss square + title/message font), `overflow_hidden`, and radius = surface − 0.125rem all applied.
- [ ] No `onDismiss`/`onAction` callbacks — `js_toast_stack(spec, theme)` still has no handler slot; dismiss/action clicks live in the preview loop (**preview-loop**), un-plumbed.
- [ ] Action affordance approximates a secondary button as a bordered tone-neutral chip label — the real `Button` primitive isn't composed (no nested-component slot in the immediate JsEl pass). (note)
- [ ] `role="list"`/`listitem` transient-notification semantics unaddressed — accepted: no ARIA channel.
- accepted: no ARIA channel (documented across jetstream impls).
- accepted: interaction (dismiss/action clicks) lives in preview event loop — no handlers plumbed through the component signature yet.

## Specimen parity

- Svelte covers: Interactive stack (success / info+action / warning), "Add toast" cycling info/success/warning/danger with live dismiss+action removal, **Sizes** snippet (xs–xl, info + success+action), **Densities** snippet (compact/default/comfortable, warning + danger+action) (`ToastStackSpecimen.svelte`).
- GPUI covers: Sizes group (xs–xl, single info toast each) + Interactive stack (success / info+action / warning) with decorative "Add toast" button. — missing: **Densities** group, **danger tone** in interactive set, real dismiss/action interactivity (button has no handler).
- Jetstream covers: Multiple toasts (success/warning/danger) + Single info toast. — missing: **Sizes** group, **Densities** group, **action toast**, **dismiss affordance** (component renders none), titles (component renders message only).

## Notes

- Jetstream `js_toast_stack` is the largest gap in this pair: it renders only a bordered box with the message string — no title, no accent bar, no dismiss, no action, no tone color, no elevation. It is effectively a placeholder and per CLAUDE.md "worse than no specimen" risk applies; treat the 11 todos as one coherent rebuild against the contract.
- GPUI info-tone token bug (`color.accent.base` vs `color.status.info`) is a concrete correctness defect, not just a missing mix — info toasts render the accent hue, not the status-info blue.
- Rust `ToastStackSpec` exposes `shadow_token()` and full tone/color token methods (spec toast_stack.rs:97-140); GPUI uses `elevation_overlay_shadow()` and Jetstream approximates via `shadow_md()` (no token-driven box-shadow on JsEl). `shadow_token()` itself is still unread surface (both targets derive the shadow from a preset/helper, not the token string).
- Pass 42: GPUI background brought to gradient parity; the only remaining open items are the 3 Jetstream preview-loop/ARIA entries (dismiss+action click handlers, real `Button` slot for the action affordance, `role=list` transient-notification semantics) — all accepted runtime/no-channel limits, not representable in the immediate JsEl pass.
- Light theme: contract §8 says "None"; nothing to verify.
