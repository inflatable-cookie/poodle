---
title: SegmentedControl's hover fill erased the selected accent
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, poodle-render, segmented-control, bug]
---

## Symptom

Hovering the selected segment turned it dark instead of keeping its accent
highlight — the selection visually disappeared under the pointer.

## Cause

`packages/render/src/segmented_control.rs` attached the hover patch to *every*
enabled segment:

```rust
if is_enabled {
    s.descriptor.cursor = CursorHint::Pointer;
    s.hover = Some(StylePatch { background: Some(hover_fill), .. });
}
```

A `StylePatch` replaces the background outright rather than blending with it, so
on the selected segment `hover_fill` (a surface-into-elevated mix) simply
overwrote `selected_fill` (accent). The contract's selected state is "accent
background, inverse text, inset highlight shadow", so the hover treatment was
cancelling the one state the component exists to show.

## Why It Was Wrong Rather Than A Trade

The hover fill is an old-GPUI-tier invention with no counterpart anywhere else:

- The contract's §4 states table lists unselected, selected, focus, and the two
  disabled states. There is no hover state.
- `packages/styles/src/segmented-control.css` has no `:hover` rule for segments
  at all.

So nothing was owed to the selected segment on hover, and the fix is simply not
to patch it. The affordance stays on unselected segments, where it conflicts
with nothing, and the pointer cursor stays on every enabled segment including
the selected one — re-picking the current segment still fires.

## Checked For The Same Shape Elsewhere

Only one other component pairs a selected fill with a hover patch that replaces
the background: `sidebar_nav`. That one is correct as it stands — in
`packages/styles/src/sidebar-nav.css`, `.item:hover:not(:disabled)` has higher
specificity than `.item--active`, so hover legitimately wins on the web target
too, and the Rust recipe matches it. The distinction is that SegmentedControl's
web target has no hover rule to win.

## Verification

- New regression test `hovering_the_selected_segment_keeps_its_accent_fill`
  pins both halves: the selected segment carries no hover patch and keeps its
  accent background, an unselected one still has its patch, and the selected
  segment keeps its pointer cursor. `poodle-render`: 109 → 110.
- The existing hover test now asserts over the unselected segments only, with
  the selected one named as deliberately excluded.
- Visually: with the click driver parking the pointer on the selected segment —
  the same region that was black in the report — it keeps its accent fill.

Green: `poodle-render` 110, both preview builds, `effigy drift:handlers`,
`git diff --check`.
