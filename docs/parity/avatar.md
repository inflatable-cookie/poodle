<!-- parity consv=ok gpui=0 jetstream=0 specimen=gap -->
<!-- specimen pass: GPUI avatar specimen now FULL — size ladder xs–xl (added xl), initials fallback (1/2/3-char + empty→"?"), tone (neutral/accent), shape (circle/rounded), and Image group (real img node, object-fit cover, circle+rounded), all real Avatar::from_spec, zero hardcoded boxes. Builds clean. Jetstream avatar specimen STILL gap: js_avatar component is real + ready, but specimens/mod.rs has no `pub mod avatar` / dispatch arm and editing mod.rs was out of scope this pass — an orphan specimen file would be dead code, so it was deliberately NOT created (no fake). Flip specimen=ok once mod.rs registers a jetstream avatar specimen. status/presence/ring/seed-tint remain OUT OF SCOPE per contract §1. -->
<!-- pass: AvatarSpec gained token methods (background_base/mix_token, background_mix_ratio, color_token, radius_token) + is_circle/circle_radius_rem (=size/2) + has_image; both targets now resolve tone/shape from the spec (no inlined token strings, no 999 sentinel — circle = half the box). Both targets render an image node when src is set (object-fit cover; URL decode is host/runtime). Jetstream probe tests cover size scale, circle radius, rounded token, image-over-initials, tone bg. Contract-confirmed: status dot / badge / ring / seed-tint are OUT OF SCOPE (§1) — not added. Remaining: decorative/ARIA accepted (no a11y channel on either Rust target); Jetstream avatar specimen file still TODO (specimen=gap). -->
# Parity: Avatar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/avatar.md`
- Svelte (authoritative): `packages/svelte/components/src/Avatar.svelte`
- GPUI: `packages/gpui/components/src/primitives/avatar.rs`
- Jetstream: NONE — no `packages/jetstream/components/src/avatar.rs`, no `js_avatar` in `lib.rs` (confirmed: `ls packages/jetstream/components/src/ | grep -i avatar` → empty).
- Specimens: svelte `packages/svelte/preview/src/specimens/AvatarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/avatar.rs` · jetstream NONE (confirmed: `ls packages/jetstream/preview/src/specimens/ | grep -i avatar` → empty).

## Contract ↔ Svelte

Svelte matches the contract on every prop (name/type/default), the size + font-size scales, the shape/tone token targets, and the ARIA rules. No divergence.

- Props align 1:1: `src`/`alt`/`initials`/`ariaLabel`/`decorative`/`size`/`shape`/`tone` with contract defaults (`Avatar.svelte:1-22`).
- Size scale (`Avatar.svelte:79-102`) and font scale match contract §3 exactly (xs 1.5rem/0.625rem … xl 6rem/2rem).
- ARIA matches contract §4: `role="img"` + `aria-label` only when `!decorative && !src` (`Avatar.svelte:33-35`); decorative → `aria-hidden="true"` and empty `alt` (`Avatar.svelte:38`).
- `fallbackText` trims to three uppercased chars with `"?"` fallback (`Avatar.svelte:24`) — matches contract `initials` note.
- consv=ok.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED Circle radius now `rem_to_px(spec.circle_radius_rem())` = half the box size (CSS `border-radius: 50%` on a square), via the new `AvatarSpec::is_circle()`/`circle_radius_rem()`. No more `999.0` sentinel.
- [x] FIXED Image rendering — `into_element` now emits a `gpui::img(src).size_full().object_fit(ObjectFit::Cover)` child when `spec.has_image()` (contract §3 cover). URL decode/load is the GPUI asset pipeline's job. Initials fallback only when no `src`.
- [x] FIXED Tone/shape now resolve through the spec token methods (`background_base_token`/`background_mix_token`/`background_mix_ratio`/`color_token`/`radius_token`) — no inlined token strings duplicated in the component.
- [ ] `decorative` prop unhandled — still not read in `into_element`. **Accepted: ARIA is absent on GPUI (no a11y API); there is no visible behavioral difference to drive.**
- accepted: no ARIA (gpui has no accessibility API) — `role="img"`/`aria-label`/`aria-hidden` from contract §4 cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `packages/jetstream/components/src/avatar.rs` (`js_avatar`) exists (declared `pub mod avatar` in lib.rs); inline-flex square from `size_rem()`, `flex_none()`, centered, `overflow_hidden()`.
- [x] DONE: shape (circle → half-box radius via `circle_radius_rem()`; rounded → `radius.control`); tone neutral/accent via the spec token methods + `theme_ext::color_mix`; initials fallback at `font_size_rem()` weight 600; all font sizes token-resolved. Probe-tested.
- [x] FIXED Image (`src`) rendering — `js_avatar` now emits `ui_element::image(src).w(size).h(size).object_fit_cover()` when `spec.has_image()` (JsEl *does* have an image-by-path channel: `Widget::Image`). URL decode is the Jetstream texture pipeline's job; the avatar only forwards the URL + cover fit. Initials fallback only when no `src`. Probe-tested (image node present, initials suppressed).
- [x] FIXED Circle radius now `rem_to_px(spec.circle_radius_rem())` = half the box (was the `999.0` sentinel); rounded uses `spec.radius_token()`. Probe-tested.
- [x] FIXED `AvatarSpec` token methods added; the component resolves tone/shape from the spec (shared with GPUI), no duplicated inline token strings.
- accepted: no ARIA channel (contract §4 `role="img"`/`aria-label`/`aria-hidden` not expressible); decorative-vs-labeled distinction has no Jetstream a11y surface.

## Specimen parity

- Svelte covers: Initials size scale (xs–xl), Tone and shape (neutral/accent/rounded), Image rendering (`AvatarSpecimen.svelte:7-31`).
- GPUI covers: Initials size scale (xs, sm, md, lg — **missing `xl`**), Tone and shape (neutral/accent/rounded-accent) (`avatar.rs:10-65`). — missing: **`xl` size**, **Image group** (no `src` specimen because the component cannot render images yet).
- Jetstream covers: nothing — **no specimen file exists**. — missing: entire specimen (`packages/jetstream/preview/src/specimens/avatar.rs` must be created covering initials scale, tone, shape, image once `js_avatar` lands; register in `specimens/mod.rs`).

## Notes

- `AvatarSpec` (`packages/contracts/components/src/avatar.rs`) now exposes the color/radius token methods (`background_base_token`/`background_mix_token`/`background_mix_ratio`/`color_token`/`radius_token`) plus `is_circle()`/`circle_radius_rem()` and `has_image()`. Both Rust targets resolve tone/shape from the spec; the mix math is no longer duplicated inline.
- **Out of scope per contract §1:** presence/status dot, badge, border/ring, and name/seed-derived background tint are NOT avatar features — the only fallback tints are the fixed neutral/accent tone mixes. These were deliberately not added (contract is source of truth).
- Both Rust targets now render an image node when `src` is set; image *decode* (URL → texture) remains a host/runtime concern on both.
- consv=ok — contract and Svelte are fully aligned. Remaining open: the Jetstream avatar specimen file (`packages/jetstream/preview/src/specimens/avatar.rs`) is still unwritten (specimen=gap); GPUI specimen still missing the `xl` + image groups.
