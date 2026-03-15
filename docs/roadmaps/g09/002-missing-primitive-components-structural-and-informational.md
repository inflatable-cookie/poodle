# g09.002 — Missing Primitive Components: Structural and Informational

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.001
Primary repos: `pug`

## Goals

- [ ] implement first-class `Pug*` component structs for all structural and
  informational primitives that exist in Svelte but are missing from GPUI
- [ ] ensure each component resolves tokens, implements `IntoElement`, and
  follows the established builder pattern

## Execution Checklist

- [ ] create `PugBanner` component backed by `BannerSpec` — tone-colored
  container with icon, title, message, and optional dismiss button
- [ ] create `PugCallOut` component backed by `CallOutSpec` — tone-colored
  container with title and content
- [ ] create `PugEyebrow` component backed by `EyebrowSpec` — small uppercase
  label with secondary text color
- [ ] create `PugPill` component backed by `PillSpec` — small rounded tag
  with optional remove action
- [ ] create `PugCode` component backed by `CodeSpec` — code display block
  with elevated background, monospace font, and optional line numbers
- [ ] create `PugSkeleton` component backed by `SkeletonSpec` — placeholder
  loading shapes (text lines, circles, rectangles) with opacity animation
- [ ] create `PugMeter` component backed by `MeterSpec` — horizontal bar
  with value display and semantic color thresholds
- [ ] create `PugRating` component backed by `RatingSpec` — star or icon
  rating display with optional interactive selection
- [ ] create `PugHoverCard` component backed by `HoverCardSpec` — hoverable
  popover with rich content, arrow placement
- [ ] create `PugTriStateSwitch` component backed by `TriStateSwitchSpec` —
  three-state toggle (on/off/indeterminate)
- [ ] create `PugEditableLabel` component backed by `EditableLabelSpec` —
  click-to-edit inline text with commit/cancel
- [ ] create `PugInline` layout helper — horizontal inline flow wrapper
- [ ] create `PugSpacer` layout helper — flexible space filler
- [ ] register all new modules in `lib.rs` with `mod` and `pub use`
- [ ] verify all new components compile with `cargo check`

## Acceptance Criteria

- [ ] all listed components have `Pug*` structs implementing `IntoElement`
- [ ] each component resolves spec tokens via `GpuiThemeProvider`
- [ ] each component supports builder methods (`with_id`, callbacks) where
  applicable
- [ ] `cargo check` passes with zero errors for `pug_gpui_components`
- [ ] `lib.rs` exports all new components

## Next Task

Open `g09.003` and implement missing input and temporal primitives.
