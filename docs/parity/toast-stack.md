<!-- parity consv=ok gpui=8 jetstream=11 specimen=gap -->
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
- [ ] No accent-bar tone-mix — contract §8 accent is `color-mix(toast-tone 82%, white 6%)`; GPUI fills the bar with the raw tone color (toast_stack.rs:198). Border (34% mix) and gradient background (12% tint) mixes are also dropped — toast uses flat `fill`/`border_color` (toast_stack.rs:191-194).
- [ ] No dismiss `aria-label` and dismiss is a raw `×` glyph div, not an Icon(name="x") (toast_stack.rs:170-176); contract §2 requires the Icon primitive + `aria-label="Dismiss {title}"`.
- [ ] Action affordance is a bare text div, not a `Button variant="secondary"` (toast_stack.rs:149-156); contract §2 anatomy + Svelte (line 55) require the Button primitive.
- [ ] Message font size wrong — contract §8 message is `0.8125rem` (smaller than title); GPUI sets message `text_size(body_size)` = same as title (toast_stack.rs:138). Per-size message scale ladder (§8 size table) not applied.
- [ ] Hardcoded `text_size(px(12.0))` + `mt(px(4.0))` on the action element (toast_stack.rs:152,155) — resolve from tokens.
- [ ] Hardcoded `pl(px(8.0))` on dismiss (toast_stack.rs:175) and `w(px(360.0))` container width (toast_stack.rs:103) — raw literals, not token/rem-derived.
- [ ] No size ladder for dismiss button dimensions — contract §8 sizes dismiss 1rem→1.75rem and inset across xs/sm/lg/xl; GPUI has no per-size dismiss sizing.
- [ ] No density padding ladder — contract §8 density scales toast padding ×0.75 / ×1.25; GPUI applies flat `padding` both axes (toast_stack.rs:207-208), density unused for spacing.
- accepted: no ARIA / live-region (gpui has no accessibility API) — danger `aria-live="assertive"` escalation cannot be expressed.

## Jetstream gap (vs Svelte + contract)

- [ ] No accent bar at all — `js_toast_stack` renders no left tone stripe (toast_stack.rs:27-33); contract §2 marks AccentBar **required**. `tone_color`/`tone` is never read, so tone is visually invisible.
- [ ] No dismiss button — contract §2 marks DismissButton **required**; `js_toast_stack` emits no dismiss control or Icon (toast_stack.rs:27-33).
- [ ] No title — only `toast.message` is rendered (toast_stack.rs:33); contract requires `<strong>` title (required) + optional message. Title (required field) is dropped entirely; message-only toasts show blank.
- [ ] No action affordance — `action_label` never read; contract §2 Actions/ActionButton absent.
- [ ] No `onDismiss`/`onAction` callbacks — function signature takes only `(spec, theme)` (toast_stack.rs:11); interaction would live in preview `main.rs` but no wiring exists.
- [ ] No tone-based border/background/accent mixes — flat `fill`/`border` only (toast_stack.rs:28-29); contract §8 gradient + 34% border mix + accent mix dropped.
- [ ] No box-shadow/elevation — contract §8 `box-shadow: var(--poodle-elevation-overlay)`; `shadow_token()` exists on spec (toast_stack.rs:109) but is never applied.
- [ ] Hardcoded `item_gap = rem_to_px(0.5)` at toast_stack.rs:17 — contract toast internal gap is `space-stack-sm` (a token); resolve it, drop the `0.5` literal.
- [ ] Vertical padding hack `panel_space_y_rem(spec.density) - 0.25` at toast_stack.rs:15 subtracts a magic `0.25` rem — not contract-derived; density also wrongly drives vertical padding here (density must not affect vertical padding per repo rules).
- [ ] No size ladder (dismiss dims, message font) and no overflow:hidden / radius-minus-0.125rem treatment.
- [ ] No `role="list"`/`listitem` semantic mapping note — accepted no ARIA, but transient-notification meaning per contract §6 GPUI-native note is unaddressed.
- accepted: no ARIA channel (documented across jetstream impls).
- accepted: interaction (dismiss/action clicks) lives in preview event loop — but no handlers are plumbed through the component signature, so it can't be wired yet.

## Specimen parity

- Svelte covers: Interactive stack (success / info+action / warning), "Add toast" cycling info/success/warning/danger with live dismiss+action removal, **Sizes** snippet (xs–xl, info + success+action), **Densities** snippet (compact/default/comfortable, warning + danger+action) (`ToastStackSpecimen.svelte`).
- GPUI covers: Sizes group (xs–xl, single info toast each) + Interactive stack (success / info+action / warning) with decorative "Add toast" button. — missing: **Densities** group, **danger tone** in interactive set, real dismiss/action interactivity (button has no handler).
- Jetstream covers: Multiple toasts (success/warning/danger) + Single info toast. — missing: **Sizes** group, **Densities** group, **action toast**, **dismiss affordance** (component renders none), titles (component renders message only).

## Notes

- Jetstream `js_toast_stack` is the largest gap in this pair: it renders only a bordered box with the message string — no title, no accent bar, no dismiss, no action, no tone color, no elevation. It is effectively a placeholder and per CLAUDE.md "worse than no specimen" risk applies; treat the 11 todos as one coherent rebuild against the contract.
- GPUI info-tone token bug (`color.accent.base` vs `color.status.info`) is a concrete correctness defect, not just a missing mix — info toasts render the accent hue, not the status-info blue.
- Rust `ToastStackSpec` exposes `shadow_token()` and full tone/color token methods (spec toast_stack.rs:97-140) that both impls partially or fully ignore — the spec surface is ahead of both renderers.
- Light theme: contract §8 says "None"; nothing to verify.
