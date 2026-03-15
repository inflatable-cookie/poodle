# g10.003 — Structural and Layout Primitive Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.002
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for all structural primitives
- [ ] each specimen demonstrates variants matching the Svelte reference

## Execution Checklist

- [ ] create `box.rs` specimen — Box with padding variants (None/Sm/Md/Lg)
  and overflow demonstrations
- [ ] create `stack.rs` specimen — Stack with vertical/horizontal direction,
  gap sizes, and alignment variants (Start/Center/End/Stretch)
- [ ] create `grid.rs` specimen — Grid approximation with column count and
  gap (emulated via nested row/column panels)
- [ ] create `surface.rs` specimen — Surface showing all tone variants
  (Default, Subtle, Elevated, Overlay) with border options
- [ ] create `separator.rs` specimen — Separator in horizontal and vertical
  orientations with tone variants
- [ ] create `scroll_shell.rs` specimen — ScrollShell with overflowing
  content demonstrating vertical scroll
- [ ] create `banner.rs` specimen — Banner showing info, warning, error,
  success tones with dismiss button
- [ ] create `callout.rs` specimen — CallOut with tone variants, title,
  and content
- [ ] create `inline.rs` specimen — Inline horizontal flow with wrapped
  children
- [ ] create `spacer.rs` specimen — Spacer between flex items demonstrating
  flexible space fill
- [ ] register all specimen modules and wire slug routing
- [ ] verify all 10 specimens render without panic

## Acceptance Criteria

- [ ] all 10 structural specimens render correctly in the preview app
- [ ] tone variants produce visually distinct backgrounds and borders
- [ ] scroll specimen allows vertical scrolling of overflow content
- [ ] `cargo check` passes

## Next Task

Open `g10.004` and build action and input specimens.
