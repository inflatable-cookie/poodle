# g08.008 Focus Rings And ARIA Attributes

Status: complete (documented as known platform delta)
Owner: Poodle Core
Depends on: g08.005, g08.006, g08.007

## Contract Check

Verified the focus ring token (`semantic.color.accent.focusRing`) and ARIA
requirements across interactive component contracts.

## Investigation Summary

GPUI's element builder API does **not** expose methods for:

1. **Focus rings / outline**: No `.outline()`, `.focus_visible()`, or equivalent
   on `Div` or `Stateful<Div>`. The `GpuiStyle` struct has `focus_ring_color`
   and `focus_ring_width` fields, but these are only accessible through the
   adapter layer's `render_structural` / `render_informational` codepaths —
   not through the fluent builder used by components.

2. **ARIA attributes**: No `.aria_label()`, `.role()`, `.aria_checked()`,
   `.aria_expanded()`, `.aria_disabled()`, `.aria_busy()` or similar methods
   on any GPUI element type. The `order_by.rs` component resolves a focus ring
   token but discards it (`let _ = focus_ring_color`), confirming no API exists
   to apply it.

3. **`:focus-visible` equivalent**: GPUI supports `.on_focus_in()` callbacks
   but has no style modifier for focus-visible state (analogous to CSS
   `:focus-visible` pseudo-class).

## What IS Available

- `.id()` — element identification (already applied on interactive components)
- `.on_click()` — click handlers (already applied)
- `.cursor_pointer()` — cursor style (already applied)
- `.hover()` / `.active()` — visual state callbacks (already applied)
- `.on_focus_in()` / `.on_focus_out()` — focus event callbacks (used in text_input)
- `spec.label` / `spec.aria_label` — labels stored on specs for future use

## Resolution

These are documented as **known GPUI platform deltas**:

- **Focus rings**: Cannot be implemented until GPUI exposes outline or
  focus-visible styling on the fluent builder API. All interactive components
  already resolve the focus ring color token — applying it is blocked on GPUI.
- **ARIA attributes**: Cannot be applied until GPUI exposes ARIA attribute
  methods on element builders. Component specs already store aria_label values.

When GPUI adds these APIs, applying them will be a straightforward pass across
all interactive components since the token resolution and spec storage are
already in place.

## Acceptance Criteria

- [x] Investigated GPUI API for focus ring and ARIA support
- [x] Confirmed platform limitation — no fluent builder methods exist
- [x] Documented as known delta
- [x] All interactive components have `.id()` and event handlers in place
- [x] Token resolution for focus ring color is ready (used in adapter layer)
