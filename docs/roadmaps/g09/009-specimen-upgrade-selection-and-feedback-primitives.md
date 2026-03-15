# g09.009 — Specimen Upgrade: Selection and Feedback Primitives

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.003
Primary repos: `pug`

## Goals

- [ ] replace all mockup feedback and selection specimens with real Pug
  component instances
- [ ] ensure every variant and interactive state is demonstrated

## Execution Checklist

- [ ] rewrite `meter.rs` to use `PugMeter` showing value ranges, semantic
  color thresholds (low/medium/high), and labeled variants
- [ ] rewrite `rating.rs` to use `PugRating` with interactive star selection,
  read-only display, half-star support, and different max values
- [ ] rewrite `skeleton.rs` to use `PugSkeleton` showing text line, avatar,
  card, and paragraph presets
- [ ] rewrite `pill.rs` to use `PugPill` for tag display with optional
  remove action; add `PugEyebrow` demonstration in same or separate file
- [ ] rewrite `temporal.rs` (time-ago, duration-input) to use `PugTimeAgo`
  and `PugDurationInput` with formatted values
- [ ] rewrite `code.rs` to use `PugCode` with elevated container, monospace
  rendering, and optional line numbers
- [ ] rewrite `range_slider.rs` to use `PugRangeSlider` with dual thumbs,
  value labels, and different ranges
- [ ] update `mod.rs` slug routing for `eyebrow` to have its own specimen
  file if separated from `pill`
- [ ] verify all specimen slugs render without panic

## Acceptance Criteria

- [ ] zero hand-built mockup code remains in feedback specimen files
- [ ] rating specimen supports interactive click-to-rate
- [ ] meter specimen shows at least 3 value ranges with color changes
- [ ] skeleton specimen renders distinct shape presets
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.010` and upgrade overlay and date/time specimens.
