# Jetstream Engine Feature Handoff — Pug Component Parity

## Context

Pug has 86 Jetstream component implementations that compile and resolve tokens
from the theme system. However, many components are currently **structurally
incomplete** because the Jetstream engine (`jetstream-runtime`) lacks rendering
features that GPUI provides natively. This document specifies exactly what the
engine needs so that every Pug component can render with full visual fidelity.

The GPUI implementation is the reference. If GPUI can do it, Jetstream must
be able to do it.

## What Already Works

JsEl already supports:
- Full CSS flexbox (direction, wrap, grow, shrink, align, justify, gap)
- Padding, margin (all sides)
- Background color (solid)
- Borders (uniform + per-side width, uniform color)
- Border radius (uniform + per-corner)
- Box shadow (sm/md/lg presets)
- Text rendering (size, weight, color, alignment, ellipsis, line-height)
- Opacity
- Overflow hidden/scroll with scissor clipping
- Absolute/relative positioning with insets
- Click events, mouse down events
- Hover and active state modifiers
- Focus ring (automatic on focusable nodes)
- Cursor hints (pointer, not-allowed, col-resize, row-resize, etc.)
- Disabled state (opacity + event blocking)
- Property transitions (opacity, translate, rotate, scale, width, height, bg color, radius, border)
- Keyframe animations (loop, ping-pong, easing)
- Scroll state preservation
- Text input with cursor, selection, clipboard

## What's Missing — Required Engine Features

### 1. SVG / Icon Rendering

**Used by:** button (spinner), icon, icon_button, and indirectly by ~40 components that show icons

**What GPUI does:** `svg().path("assets/icons/chevron-down.svg").size(px(16.0)).text_color(color)`

**What Jetstream needs:**

Option A — **SVG rasterizer**: Add an SVG loading + rasterization pipeline. Load SVG
files from disk, rasterize them at the requested size into the glyph atlas or a
texture atlas, and render as a quad with the text color applied as a tint. This
is the highest-fidelity approach.

Option B — **Icon font / sprite atlas**: Pre-render all icons into a sprite atlas
at build time. Add a `JsEl::icon(name, size)` method that looks up the icon in
the atlas and renders it as a textured quad. This avoids runtime SVG parsing.

**Minimum API needed on JsEl:**
```rust
/// Render an icon by name at the given size, tinted by the current text_color.
fn icon(name: &str) -> JsEl
// OR
fn svg(path: &str) -> JsEl
```

**Priority:** HIGH — icons appear in buttons, inputs, navigation, status indicators,
toggles, menus, breadcrumbs, cards, and nearly every interactive component.

---

### 2. Per-Side Border Colors

**Used by:** tabs (underline indicator = colored bottom border), tab_strip,
segmented_control, data_table (header bottom border)

**What GPUI does:** `.border_b_2().border_color(accent)` — a 2px bottom-only
border in the accent color, while other sides have no border or a different color.

**What Jetstream has:** Per-side border *widths* exist (`border_l`, `border_r`,
`border_t_1`, `border_b_1`, etc.) but border color is uniform for all sides.

**What Jetstream needs:** Per-side border colors, or at minimum a way to set a
different color on the bottom border.

```rust
/// Set border color for individual sides
fn border_color_top(color: Color) -> Self
fn border_color_bottom(color: Color) -> Self
fn border_color_left(color: Color) -> Self
fn border_color_right(color: Color) -> Self
```

**Priority:** MEDIUM — needed for tab underline indicators and table headers.

---

### 3. Color Mixing Utility

**Used by:** button (hover/active states, danger×secondary blend, primary border
darkening), checkbox (hover), card (hover), badge (tinted fill), callout, banner,
switch, accordion, nav_card, list_card, and ~30 other components

**What GPUI does:** `color_mix(a, b, fraction)` to linearly interpolate between
two colors. Used extensively for:
- Hover: `fill 84% + elevated 16%`
- Active: `fill 72% + elevated 28%`
- Hover border: `border 78% + text-primary 22%`
- Tinted fills: `accent 18% + surface 82%`

**What Jetstream has:** The component crate has a local `mix_colors()` helper
using `Vec4` math, but this should be an engine-level utility since it's needed
everywhere, and ideally the hover/active color computation should happen in the
engine's hover/active state system rather than requiring each component to
pre-compute all state colors.

**What Jetstream needs:** A `Color::mix(other, fraction)` method or a
`color_mix(a, b, fraction)` function at the engine level.

```rust
impl Color {
    /// Linear interpolation: self * fraction + other * (1 - fraction)
    pub fn mix(self, other: Color, fraction: f32) -> Color
}
```

**Priority:** HIGH — used by nearly every component's hover/active state styling.

---

### 4. Gradient Backgrounds

**Used by:** Not directly used by Pug components currently, but the engine already
supports linear and radial gradients in `UiDrawCommand`. The JsEl builder just
doesn't expose them.

**What Jetstream needs:** Expose gradient methods on JsEl:

```rust
fn bg_gradient_linear(angle_deg: f32, stops: &[(Color, f32)]) -> Self
fn bg_gradient_radial(center: [f32; 2], radius: f32, stops: &[(Color, f32)]) -> Self
```

**Priority:** LOW — no immediate Pug use, but useful for future components.

---

### 5. Image / Texture Rendering

**Used by:** media_thumbnail, media_preview, file_upload (preview), avatar/leading
elements in list_card, nav_card

**What GPUI does:** Components create placeholder containers. Image rendering is
handled by GPUI's native image loading.

**What Jetstream needs:** An image loading and rendering pipeline.

```rust
/// Load and render an image from a file path or URL
fn image(path: &str) -> JsEl
/// Set object-fit behavior
fn object_fit_cover() -> Self
fn object_fit_contain() -> Self
```

The engine needs:
1. Image loading (from disk path or byte buffer)
2. GPU texture upload
3. Rendering as a textured quad within the UI tree
4. Aspect ratio preservation (contain/cover modes)

**Priority:** MEDIUM — needed for media components and any component with visual
content (avatars, thumbnails, previews).

---

### 6. Drag Interaction

**Used by:** slider (thumb drag), range_slider (two thumb drags), resize_handle
(panel resize drag), reorderable_list (item drag-and-drop)

**What GPUI does:** Mouse down + mouse move tracking with delta computation.

**What Jetstream needs:**

```rust
/// Start tracking drag on mouse down, fire handler on each mouse move
fn on_drag(handler: impl Fn(DragEvent)) -> Self

struct DragEvent {
    delta_x: f32,
    delta_y: f32,
    position: [f32; 2],
    phase: DragPhase, // Start, Move, End
}
```

The engine needs:
1. Mouse move event dispatch (currently only click/mouse_down exist)
2. Drag state tracking (capture element on mouse down, dispatch moves until mouse up)
3. Delta computation between frames

**Priority:** HIGH — sliders, resize handles, and reorderable lists are core
interactive components.

---

### 7. Pointer Move / Hover Detection

**Used by:** hover_card, tooltip (trigger on hover), context_menu (highlight on hover)

JsEl already has `.hover()` for style changes, but there's no `on_hover_start` /
`on_hover_end` event for triggering visibility of related elements (like showing
a tooltip or hover card).

**What Jetstream needs:**

```rust
fn on_pointer_enter(handler: impl Fn()) -> Self
fn on_pointer_leave(handler: impl Fn()) -> Self
```

**Priority:** MEDIUM — needed for tooltip, hover_card, and menu item highlighting.

---

### 8. Overlay / Portal Rendering

**Used by:** popover, tooltip, hover_card, context_menu, select dropdown, combobox
dropdown, dialog backdrop, drawer, command_palette, date_picker calendar popup,
color_picker popup

**What GPUI does:** Overlays render on top of all other content using GPUI's
built-in stacking/overlay system.

**What Jetstream has:** z_index support exists in draw command collection, but
there's no "portal" concept where a child element escapes its parent's clip rect
and renders at the top of the stacking context.

**What Jetstream needs:**

```rust
/// Render this element as an overlay, escaping parent clip rects
/// and rendering on top of all normal content.
fn overlay() -> Self
/// Position the overlay relative to an anchor element
fn anchor_to(element_id: &str) -> Self
```

The engine needs:
1. An overlay layer that renders after all normal content
2. Overlay elements escape parent scissor rects
3. Positioning relative to an anchor element (above, below, left, right with
   automatic flip if near viewport edge)

**Priority:** HIGH — needed for all dropdown, popover, tooltip, dialog, and
picker components. Without this, these components render inline which breaks
the expected UX.

---

### 9. Rich Text Runs

**Used by:** code (syntax highlighting — different colors per token), markdown
content, breadcrumbs (some segments clickable, some not)

**What GPUI does:** Multiple child elements with different text colors/weights.

**What Jetstream has:** Single text color/weight per label element. To achieve
multi-colored text, you must use multiple label children in a flex row — this
works but is less efficient and doesn't support line-wrapping across runs.

**What Jetstream could add (nice to have):**

```rust
/// Create a text element with multiple styled runs
fn rich_text(runs: &[(text, TextStyle)]) -> JsEl
```

**Priority:** LOW — the flex-row-of-labels workaround is adequate for most
cases. Only a real issue for code highlighting with line wrapping.

---

### 10. Scroll Events

**Used by:** scroll_shell, data_table (virtual scrolling), list-based components

**What Jetstream has:** Scroll handling exists for `list()` widgets with
`overflow_scroll()`. However, there's no scroll event callback.

**What Jetstream needs:**

```rust
fn on_scroll(handler: impl Fn(ScrollEvent)) -> Self

struct ScrollEvent {
    scroll_top: f32,
    scroll_height: f32,
    client_height: f32,
}
```

**Priority:** LOW — basic scrolling works; events are only needed for
infinite scroll / virtual scrolling patterns.

---

## Summary: Priority Order

| Priority | Feature | Impact | Components Affected |
|----------|---------|--------|---------------------|
| **HIGH** | SVG/Icon rendering | Icons everywhere | ~40 |
| **HIGH** | Overlay/portal rendering | All dropdowns, dialogs, popovers | ~15 |
| **HIGH** | Drag interaction | Sliders, resize, reorder | ~5 |
| **HIGH** | Color mixing utility | Hover/active states | ~30 |
| **MEDIUM** | Per-side border colors | Tab indicators | ~5 |
| **MEDIUM** | Image/texture rendering | Media components | ~5 |
| **MEDIUM** | Pointer enter/leave events | Tooltips, hover cards | ~3 |
| **LOW** | Gradient backgrounds | Future use | 0 |
| **LOW** | Rich text runs | Code highlighting | ~2 |
| **LOW** | Scroll events | Virtual scrolling | ~2 |

## Files to Modify

All engine changes are in `jetstream/crates/jetstream-runtime/`:

| File | Changes |
|------|---------|
| `src/ui_element.rs` | New JsEl builder methods: `icon()`, `image()`, `overlay()`, `on_drag()`, `on_pointer_enter/leave()`, `on_scroll()`, per-side border colors |
| `src/game_ui.rs` | SVG rasterizer integration, overlay rendering pass, drag state machine, pointer tracking, image texture pipeline, color mix utility |

Pug component files to update after engine features land:
- `packages/jetstream/components/src/` — all 86 components, updating icon rendering from text glyphs to real SVG icons, adding proper overlay rendering for dropdowns/dialogs, implementing drag for sliders/resize
