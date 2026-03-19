# Add event handling and sRGB color support to JsEl

## Context

Pug (the UI component library) is unifying its architecture across GPUI and Jetstream targets. Both use Taffy-backed fluent element builders, but Jetstream's `JsEl` currently lacks two things GPUI has: interactive event handling and proper color representation.

## 1. Event system for JsEl

JsEl needs interactive state support. Currently it's a static layout description — interactions are handled externally by parsing element IDs in the shell. We need first-class event support on the builder.

**Add these methods to JsEl:**

```rust
// Click handler — fires on pointer up within element bounds
.on_click(handler: impl Fn(&ClickEvent) + 'static)

// Hover style modifier — applies style changes when pointer is over element
.hover(modifier: impl Fn(JsStyleOverride) -> JsStyleOverride + 'static)

// Active/pressed style modifier — applies while pointer is down on element
.active(modifier: impl Fn(JsStyleOverride) -> JsStyleOverride + 'static)

// Cursor style
.cursor_pointer()
```

`JsStyleOverride` should be a subset of the style properties that can be overridden on hover/active — at minimum: `bg`, `border_color`, `text_color`, `opacity`. The render pipeline needs to track pointer position and resolve these overrides during draw.

`ClickEvent` should carry at minimum the element ID and pointer position.

**How Pug uses this (example from GPUI button):**

```rust
let hover_fill = color_mix(fill, elevated, 0.84);
let active_fill = color_mix(fill, elevated, 0.72);

el = el
    .cursor_pointer()
    .hover(|s| s.bg(hover_fill).border_color(hover_border))
    .active(|s| s.bg(active_fill))
    .on_click(move |event| handler(event));
```

## 2. sRGB color type

JsEl currently uses `glam::Vec4` for colors, which is ambiguous (linear? sRGB? premultiplied?). Pug is standardizing on an explicit sRGB `Rgba` type at the contract boundary.

**Add a `Color` type** (or use Pug's shared one from `pug-adapter` once it exists):

```rust
/// sRGB color with alpha, matching CSS/token color space.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32, // 0.0–1.0, sRGB
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

**Update JsEl methods** to accept this type instead of `Vec4`:

- `.bg(color: Color)`
- `.text_color(color: Color)`
- `.border_color(color: Color)`

The sRGB→linear conversion should happen at render time inside the draw pipeline, not at the API boundary. This keeps the component code working in the same color space as CSS and the token system.

Provide `From<Vec4>` and `Into<Vec4>` impls for migration, but new code should use `Color` directly.

## 3. Additional builder methods to align with GPUI

These GPUI methods don't have JsEl equivalents yet. Adding them would allow Pug components to use near-identical rendering code across both targets:

```rust
.flex()              // enable flex layout (JsEl currently uses .flex_row()/.flex_col())
.flex_shrink_0()     // prevent shrinking
.flex_1()            // flex: 1
.flex_grow()         // flex-grow: 1
.flex_wrap()         // flex-wrap: wrap
.relative()          // position: relative
.absolute()          // position: absolute
.top(px)             // top offset
.left(px)            // left offset
.ml(px)              // margin-left
.w_full()            // width: 100%
.h_full()            // height: 100%
.shadow_sm()         // small box shadow
.shadow_md()         // medium box shadow
.text_xs()           // font-size: 0.75rem
.text_sm()           // font-size: 0.875rem
.text_lg()           // font-size: 1.125rem
.font_weight(weight) // accept FontWeight enum or u16
```

## Priority

1. **Color type** — smallest change, biggest impact on correctness
2. **Hover/active style overrides** — needed for visual parity with GPUI/Svelte
3. **on_click** — needed for interactive components
4. **Additional builder methods** — nice-to-have for API alignment

## Non-goals

- Don't add ARIA/accessibility methods yet (GPUI doesn't support them either)
- Don't change how JsEl trees are consumed by `render_immediate()` — just extend the data model
- Don't worry about keyboard focus handling beyond what `.focusable()` already does

## Notes

Adjust the `ClickEvent` and `JsStyleOverride` types to fit however Jetstream's input/pointer system currently works. The key thing is that the API shape matches what Pug components expect so we can write near-identical rendering code for both targets.
