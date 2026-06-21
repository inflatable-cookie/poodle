<!-- parity consv=ok gpui=1 jetstream=1 specimen=ok -->
<!-- pass 85: added the contract's 5th AspectRatio variant Auto (was Square/Landscape/Portrait/
     Video only). aspect_ratio_pair maps Auto → landscape placeholder (no real media in Rust);
     is_auto() lets impls choose content-driven height. Only one exhaustive match (aspect_ratio_
     pair) — all other sites are constructors. 1 spec test; specs 116, gpui clean, jet 739.
     Remaining 1/1 = radial-gradient frame (no gradient primitive) — accepted runtime delta. -->
# Parity: MediaThumbnail

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/media-thumbnail.md`
- Svelte (authoritative): `packages/svelte/components/src/MediaThumbnail.svelte`
- GPUI: `packages/gpui/components/src/composites/media_thumbnail.rs`
- Jetstream: `packages/jetstream/components/src/media_thumbnail.rs`
- Spec: `packages/contracts/components/src/media_thumbnail.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MediaThumbnailSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/media_thumbnail_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/media_thumbnail.rs`

## Contract ↔ Svelte

Contract and Svelte agree on all 16 props, data attributes, aspect ratios, fallback icons, and state defaults. `consv=ok`.

- All props (`kind`, `state`, `aspectRatio`, `title`, `badge`, `meta`, `ariaLabel`, `stateTitle`, `stateMessage`, `presentation`, `fit`, `frameWidth`, `frameMinHeight`, `frameMaxHeight`) match `MediaThumbnail.svelte:14-48`.
- Svelte additionally accepts `kind="pdf"`/`"other"` and folds them to `"document"` (`MediaThumbnail.svelte:51`) — undocumented but a defensive alias, not a contract surface. Minor: could note in contract §2 that pdf/other coerce to document.
- `aria-busy`, `data-*` attributes, fallback-icon map, state-title defaults all match.

## GPUI gap (vs Svelte + contract)

GPUI renders a fixed-px frame with a bracketed kind label (`[Image]`) instead of the fallback Icon; several anatomy parts missing.

- [ ] Hardcoded px aspect-ratio frame sizes: `px(160.0)`/`px(220.0)`/`px(148.0)`/`px(157.0)`/`px(280.0)` (`media_thumbnail.rs:110-115`) — contract uses CSS `aspect-ratio` + token-driven frame width; these are magic dimensions. Resolve frame width from `frameWidth`/tokens, derive height from ratio.
- [ ] Hardcoded px literals: `top(px(10.0))`, `right(px(10.0))`, `py(px(2.0))` (badge, `:192-194`), `gap(px(2.0))` (caption, `:207`) — resolve from spacing tokens.
- [ ] Badge background uses raw `accent` + `gpui::white()` text (`:196-198`); contract badge bg is `color-mix(background-surface 74%)` with `text-primary` + uppercase/letter-spacing/blur. Wrong tokens + missing text-transform.
- [ ] No fallback Icon — renders text `[{kind_label}]` (`:179`) instead of the `kind`→icon map (image/music/play/file-text/external-link). Contract §9 fallback-icon table unimplemented.
- [ ] No play indicator for audio/video (contract §3 Play Indicator part absent).
- [ ] No `presentation` (compact) support — `MediaThumbnailSpec` has no `presentation` field; compact gap/padding/caption-hiding rules unimplemented.
- [ ] No `fit` (cover/contain) support — spec lacks `fit`; object-fit pass-through to image absent.
- [ ] No `frameWidth`/`frameMinHeight`/`frameMaxHeight` — spec lacks these fields.
- accepted: no ARIA (`<figure>`/`aria-busy`/`aria-label` not emitted).

## Jetstream gap (vs Svelte + contract)

Skeletal — a fixed box with optional title only (`media_thumbnail.rs:15-26`). Most of the contract is unimplemented.

- [ ] Hardcoded px literals: frame `w(rem_to_px(7.5)).h(rem_to_px(5.0))` (`:19`), caption `text_size(rem_to_px(0.6875))` (`:23`) — magic dimensions; no aspect-ratio resolution, no token-driven size.
- [ ] Uses `radius.surface` (`:12`); contract frame radius is `calc(radius.surface − 0.125rem)`. Wrong radius.
- [ ] No state display — loading/error/empty postures entirely absent (only ready-box renders).
- [ ] No badge overlay, no play indicator, no fallback icon (`kind` ignored).
- [ ] No aspect-ratio variation — single fixed 7.5×5 box regardless of `aspect_ratio`.
- [ ] No meta line in caption (`spec.meta` ignored).
- [ ] No `presentation`/`fit`/`frameWidth`/`frameMinHeight`/`frameMaxHeight` (spec-level gap shared with GPUI).
- [ ] No spinner on loading.
- accepted: no ARIA / interaction (none required for this component).

## Specimen parity

- Svelte covers: Image thumbnails (square, badge, meta, video kind), Compact presentation (document + audio), Loading state, Contained image (`fit="contain"`, `aspectRatio="auto"`) (`MediaThumbnailSpecimen.svelte`).
- GPUI covers (2026-06-21, `specimen=ok`): Kinds (image/video/audio/document/embed fallback icons), Badge + meta + play indicator, Aspect ratios (square/landscape/portrait/video), Compact presentation, States (loading/error/empty), Fit: contain (`media_thumbnail_specimen.rs`). All `MediaThumbnail::from_spec`, real fallback icons resolved by the component. `cargo build` clean.
- Jetstream covers (2026-06-21, `specimen=ok`): same six groups via `js_media_thumbnail(&spec)` (`media_thumbnail.rs`). All token-resolved; retry-loop build clean.

## Notes

- RESOLVED (2026-06-21): `MediaThumbnailSpec` gained additive `presentation`
  (`MediaPresentation`), `fit` (`MediaFit`), `frame_width` (`MediaFrameWidth`),
  `frame_min_height`, `frame_max_height` plus helper/token accessors
  (`fallback_icon`, `shows_play_indicator`, `aspect_ratio_pair`,
  `frame_height_for_width`, badge/play/frame/placeholder token methods). Both
  Rust targets rebuilt against tokens: ratio-derived frame (no magic px),
  fallback Icon per kind, placeholder, badge (uppercased, surface-mix fill,
  control radius), play indicator (audio/video), loading grid spinner, error/
  empty state postures, compact caption-hiding, non-compact caption with meta.
  Jetstream covered by 6 render_probe tests.
- Residual (preview-loop only): the frame's radial+panel gradient and the light-
  theme overrides are approximated by the flat panel fill on both Rust targets
  (no radial-gradient primitive); the loading spinner animation runs only in the
  preview loop. ARIA remains accepted-out (not required for this component).
- The Rust `AspectRatio` enum has no `auto` variant; `aspect_ratio_pair()` maps
  every variant to a concrete ratio. `auto` is a Svelte/contract-only value and
  was intentionally left out of the shared enum (shared module, out of scope).
