<!-- parity consv=ok gpui=4 jetstream=8 specimen=gap -->
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

- [ ] Hardcoded circle radius literal `px(rem_to_px(999.0))` at `avatar.rs:42` — circle should use a `50%`/pill radius derived from the resolved `size`, not a raw `999.0` rem sentinel. Add `AvatarSpec` shape-radius handling instead of the magic number.
- [ ] No image (`src`) rendering — `into_element` only ever emits `fallback_text()` (`avatar.rs:58`); `spec.src`/`spec.alt`/object-fit cover (contract §3) are ignored. Render an image element when `spec.src.is_some()`.
- [ ] `tone`/`shape`/`size` resolve colors via raw token strings + inline `color_mix` (`avatar.rs:33-43`) — `AvatarSpec` exposes no token methods (`background`/`color`/`radius`), only `size_rem`/`font_size_rem`. Add `AvatarSpec::background_token()`/`color_token()`/`radius_token()` so resolution is spec-driven, not duplicated in the component.
- [ ] `decorative` prop unhandled — `spec.decorative`/`accessible_label()` are never read in `into_element`; no behavioral difference (accepted that ARIA itself is absent, but decorative should still gate any future label/alt logic).
- accepted: no ARIA (gpui has no accessibility API) — `role="img"`/`aria-label`/`aria-hidden` from contract §4 cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No Jetstream implementation exists — create `packages/jetstream/components/src/avatar.rs` (`js_avatar(spec: &AvatarSpec, theme: &JetstreamThemeProvider) -> JsEl`) resolving all tokens from `AvatarSpec`, and register it in `packages/jetstream/components/src/lib.rs`.
- [ ] Root: inline-flex square — `div().w(size).h(size)` from `spec.size_rem()` resolved to px, `flex_none()`, centered (`items_center`/`justify_center`), `overflow_hidden()` (contract §3, `Avatar.svelte:45-59`).
- [ ] Shape: `circle` → pill/`50%` radius; `rounded` → `radius.control` (contract §3, `Avatar.svelte:61-67`). Do not hardcode a `999.0` sentinel.
- [ ] Tone: `neutral` → bg `color-mix(background.surface 82%, text.primary)` / fg `text.secondary`; `accent` → bg `color-mix(accent.base 76%, background.surface)` / fg `text.primary` (`Avatar.svelte:69-77`).
- [ ] Initials fallback: render `spec.fallback_text()` at `font_size_rem()`-resolved size, weight 600, line-height 1 (contract §3, `Avatar.svelte:40,55-57`).
- [ ] Image (`src`): render an image element filling the square with object-fit cover when `spec.src.is_some()` (contract §3, `Avatar.svelte:37-38,104-109`) — note Jetstream image-source support may need a runtime channel; if unsupported, document as a runtime limit, do not fake it.
- [ ] Font tokens: resolve font-size from `spec.font_size_rem()` per size — zero hardcoded px (contract §3).
- [ ] Add `AvatarSpec` token methods (`background_token`/`color_token`/`radius_token`) so both Rust targets resolve identically rather than duplicating the `color_mix` math.
- accepted: no ARIA channel (contract §4 `role="img"`/`aria-label`/`aria-hidden` not expressible); decorative-vs-labeled distinction has no Jetstream a11y surface.

## Specimen parity

- Svelte covers: Initials size scale (xs–xl), Tone and shape (neutral/accent/rounded), Image rendering (`AvatarSpecimen.svelte:7-31`).
- GPUI covers: Initials size scale (xs, sm, md, lg — **missing `xl`**), Tone and shape (neutral/accent/rounded-accent) (`avatar.rs:10-65`). — missing: **`xl` size**, **Image group** (no `src` specimen because the component cannot render images yet).
- Jetstream covers: nothing — **no specimen file exists**. — missing: entire specimen (`packages/jetstream/preview/src/specimens/avatar.rs` must be created covering initials scale, tone, shape, image once `js_avatar` lands; register in `specimens/mod.rs`).

## Notes

- `AvatarSpec` (`packages/contracts/components/src/avatar.rs`) is complete for props and exposes `fallback_text()`/`accessible_label()`/`size_rem()`/`font_size_rem()`, but lacks color/radius token methods — both Rust targets currently inline the token math. Adding those methods is the cleanest unblock for the Jetstream build and the GPUI cleanup.
- Avatar is the top-priority gap: the entire Jetstream target (component + specimen) is missing, while GPUI is initials-only.
- consv=ok — contract and Svelte are fully aligned; all open work is on the Rust targets.
