# Poodle Jetstream Developer Guide

End-to-end guide for building UI with Poodle components in a Jetstream
game-engine application.

For the GPUI target, see [GPUI Developer Guide](./gpui-developer-guide.md).
For the Svelte target, see [Svelte Developer Guide](./svelte-developer-guide.md).
For component contracts, see `docs/contracts/components/`.

---

## Quick Start

### 1. Add dependencies

In your Jetstream app's `Cargo.toml`:

```toml
[dependencies]
poodle-jetstream            = { path = "<path-to-poodle>/packages/jetstream/adapter" }
poodle-jetstream-components = { path = "<path-to-poodle>/packages/jetstream/components" }
poodle-specs                = { path = "<path-to-poodle>/packages/contracts/components" }
poodle-tokens               = { path = "<path-to-poodle>/packages/contracts/tokens" }
jetstream-runtime           = { ... }
```

If Poodle is a workspace sibling, use workspace path dependencies. If it is
vendored, point to the vendored paths.

### 2. Create a theme provider

```rust
use poodle_jetstream::JetstreamThemeProvider;
use poodle_tokens::{ThemeDefinition, DensityDefinition, ControlSizeDefinition};

let theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark)
    .with_density(&DensityDefinition::Default)
    .with_control_size(&ControlSizeDefinition::Md);
```

The theme provider is the runtime bridge between semantic token names and
resolved values. Pass it (by reference) into every component render function.

### 3. Render a component

```rust
use jetstream_runtime::ui_element::*;
use poodle_jetstream_components::button::js_button;
use poodle_specs::{ButtonSpec, ButtonVariant, ButtonTone, ControlSize};

let el = js_button(
    &ButtonSpec::new()
        .with_label("Save changes")
        .with_variant(ButtonVariant::Solid)
        .with_tone(ButtonTone::Accent)
        .with_size(ControlSize::Md),
    &theme,
);

let root = div().flex_col().p(16.0).child(el);
game_ui.render_immediate(&root);
```

### Run the preview app

```sh
cargo run -p poodle-jetstream-preview \
    --manifest-path packages/jetstream/preview/Cargo.toml
```

The preview app renders specimens for all 117 implemented components across
theme, density, and control-size permutations. Use it to validate visual
correctness before shipping.

---

## Token Resolution

All visual properties in Poodle resolve from the semantic token system. No
component hardcodes a pixel value or color — every property traces back to a
token.

### The token path system

Token paths are dot-separated semantic names that match the system defined in
`poodle-tokens`:

```
color.background.surface
color.text.primary
color.border.default
color.accent.base
size.control.height.md
space.inline.sm
space.panel.x
radius.control
state.opacity.disabled
```

### Resolving tokens in component code

Use the helpers from `poodle_jetstream_components::theme_ext`:

```rust
use poodle_jetstream_components::theme_ext::*;

// Color → glam::Vec4 (sRGB, linear-converted for rendering)
let fill     = resolve_color(&theme, "color.background.surface");
let text     = resolve_color(&theme, "color.text.primary");
let border   = resolve_color(&theme, "color.border.default");

// Space / size / radius → f32 (logical pixels, scale-factor applied)
let height   = resolve_px(&theme, "size.control.height.md");
let gap      = resolve_px(&theme, "space.inline.sm");
let radius   = resolve_px(&theme, "radius.control");

// Opacity → f32 (0.0..1.0)
let disabled = resolve_opacity(&theme, "state.opacity.disabled");
```

Direct methods on `JetstreamThemeProvider` are also available:

| Method | Return type | Notes |
|---|---|---|
| `resolve_color(token)` | `ColorValue` (r,g,b,a f32) | sRGB |
| `resolve_linear_color(token)` | `glam::Vec4` | Linear-space RGB, unchanged A |
| `resolve_space(token)` | `f32` | Logical px × scale_factor |
| `resolve_radius(token)` | `f32` | Delegates to resolve_space |
| `resolve_border_width(token)` | `f32` | Delegates to resolve_space |
| `resolve_opacity(token)` | `f32` | 0.0..1.0 |

The `theme_ext` helpers (`resolve_color`, `resolve_px`, `resolve_radius`,
`resolve_opacity`) are preferred in component code — they return types that
compose directly with `JsEl` builder methods.

### Presentation helpers

`poodle_jetstream_components::presentation` provides size/density resolution
for components that need to compute dimensions based on the current context:

```rust
use poodle_jetstream_components::presentation::*;

// Height in rem for a given control size (converted to px via rem_to_px)
let h = rem_to_px(control_height_rem(ControlSize::Md));    // 36px

// Font size in rem
let fs = rem_to_px(size_font_rem(ControlSize::Md));        // 13px

// Horizontal padding by density
let px = rem_to_px(control_space_x_rem(ControlDensity::Default));

// Icon size is one stop smaller than control size (clamped)
let icon_size = resolve_supporting_visual_size(ControlSize::Md); // → Sm
```

Size table (rem values, 1rem = 16px):

| Size | Height | Font | Min-width |
|---|---|---|---|
| Xs | 1.5rem (24px) | 0.6875rem (11px) | 3.75rem (60px) |
| Sm | 1.75rem (28px) | 0.75rem (12px) | 4.25rem (68px) |
| Md | 2.25rem (36px) | 0.8125rem (13px) | 5.0rem (80px) |
| Lg | 2.75rem (44px) | 0.875rem (14px) | 5.75rem (92px) |
| Xl | 3.25rem (52px) | 0.9375rem (15px) | 6.5rem (104px) |

---

## Component Pipeline

### How rendering works

```
ComponentSpec  (data — props, state, content)
    + JetstreamThemeProvider  (resolves token paths → typed values)
        ↓
    js_<component>(spec, theme) → JsEl
        (JsEl is a value tree: layout + visual style + children + events)
        ↓
    game_ui.render_immediate(&root_el)
        (materializes JsEl → UiTree → Taffy layout → draw commands)
```

The render function is pure: given the same spec and theme, it always produces
the same `JsEl` tree. State changes (hover, active, selection) are handled
inside the `JsEl` via `.hover()` and `.active()` style overrides and event
handlers — the render function re-runs each frame.

### Spec builders

Every component spec has a builder API:

```rust
// All specs have ::new() and with_* methods
let spec = ButtonSpec::new()
    .with_label("Save")
    .with_variant(ButtonVariant::Solid)
    .with_tone(ButtonTone::Accent)
    .with_size(ControlSize::Md)
    .with_disabled(false)
    .with_loading(false)
    .with_leading_icon(Some("save"));
```

Spec structs live in `poodle-specs` (`poodle_specs::` prefix).
Workstation specs live in `poodle-workstation`.

### Calling a render function

```rust
use poodle_jetstream_components::checkbox::js_checkbox;
use poodle_specs::{CheckboxSpec, CheckState};

let el = js_checkbox(
    &CheckboxSpec::new()
        .with_label("Enable notifications")
        .with_state(CheckState::Checked),
    &theme,
);
```

Every render function is `fn js_<name>(spec: &<Name>Spec, theme: &JetstreamThemeProvider) -> JsEl`.

---

## Layout

Layout in Poodle Jetstream is flexbox-based, powered by Taffy. The `JsEl`
fluent builder exposes layout properties directly on every element.

### Sizing

```rust
// Fixed dimensions
div().w(200.0).h(48.0)

// Grow to fill available space on both axes (flex-grow: 1 + align-self: stretch)
div().grow()

// Grow on the main axis only (flex-grow: 1, no cross-axis stretch)
div().flex_grow()

// Fill 100% of parent width
div().w_full()

// Constrain min/max
div().min_w(120.0).max_w(480.0)

// Allow shrinking past content size (sets min_size: 0)
div().min_w_0()
```

Key rule: containers default to `min_size: 0`, not `auto`. This means
containers can be constrained by their parents rather than expanding to fit
content.

### Flex direction and alignment

```rust
// Horizontal row (default)
div().flex_row().items_center().justify_between()

// Vertical column
div().flex_col().items_stretch()

// Wrapping
div().flex_row().flex_wrap()
```

### Spacing

```rust
// Gap between children
div().gap(8.0)

// Padding — uniform, axis, or individual sides
div().p(16.0)
div().px(12.0).py(8.0)
div().pt(4.0).pb(4.0).pl(8.0).pr(8.0)

// Margin
div().mx(8.0).my(4.0)
```

### Overflow and scrolling

```rust
// Clip content
div().overflow_hidden()

// Scrollable list — converts element to List widget kind
div().overflow_scroll()
```

### Positioning

```rust
// Absolute positioning
div().absolute().top(0.0).left(0.0).right(0.0).bottom(0.0)
div().absolute().inset_0()

// Relative (default)
div().relative()
```

### `LayoutIntent` and `map_layout()`

When working with Poodle's renderer-agnostic layout types:

```rust
use poodle_jetstream::map_layout;
use poodle_layout::LayoutIntent;

let taffy_style: taffy::Style = map_layout(&my_layout_intent);
```

Component render functions call `map_layout` internally — application code
rarely needs this directly.

---

## Fluent Builder Reference

`JsEl` is constructed via entry-point functions from `jetstream_runtime::ui_element`:

### Entry points

| Function | Widget kind | Notes |
|---|---|---|
| `div()` | Panel | Generic container |
| `label(text)` | Label | Text display |
| `button(label)` | Button | Interactive button |
| `list()` | List | Scrollable list container |
| `slider(value, min, max)` | Slider | Range control |
| `progress(value)` | ProgressBar | Progress indicator |
| `text_input(text, placeholder)` | TextInput | Editable text field |
| `image(path)` | Image | Image from file path |
| `icon(name)` | Icon | SVG icon by name (e.g. `"chevron-down"`) |

### Visual

```rust
// Colors
.bg(color)                             // Background
.bg_opt(option_color)                  // Conditional background
.bg_gradient_linear(angle_deg, stops)  // Linear gradient
.bg_gradient_radial(center, r, stops)  // Radial gradient
.text_color(color)                     // Text color
.border_color(color)                   // Border color

// Border
.border(width)                         // All sides
.border_1()                            // 1px preset
.border_l(width)                       // Individual sides: l, r, t, b

// Corner radius
.rounded(radius)                       // All corners
.rounded_tl(r).rounded_tr(r)          // Individual corners

// Effects
.opacity(value)                        // 0.0..1.0
.shadow(box_shadow)
```

### Typography

```rust
.text_size(px)                         // Font size in px
.text_weight(weight)                   // Font weight (400, 500, 700, etc.)
.text_color(color)
.text_align_center()
.text_right()
.text_ellipsis()                       // Truncate with ellipsis
.whitespace_nowrap()
.line_height(multiplier)
```

### Interaction and state

```rust
// Hover and active style overrides
.hover(|s| s.bg(hover_color).text_color(hover_text))
.active(|s| s.bg(active_color))

// Events
.on_click(|event| { /* handler */ })
.on_pointer_enter(|event| { /* handler */ })
.on_pointer_leave(|event| { /* handler */ })
.on_drag(|event| { /* handler */ })
.on_scroll(|event| { /* handler */ })

// Focus and cursor
.focusable()
.cursor_pointer()

// Disabled state
.disabled(is_disabled)
```

### Composition

```rust
.child(el)                             // Add one child
.children(vec![el1, el2, el3])        // Add multiple children
.id("stable-id")                      // Stable ID for state preservation
```

---

## Themes and Context

### Available themes

| `ThemeDefinition` | Description |
|---|---|
| `ThemeDefinition::Dark` | Default dark |
| `ThemeDefinition::Light` | Light |
| `ThemeDefinition::LoopholeStudio` | Loophole Studio branded dark |

### Density

Controls horizontal spacing between and within components (does NOT affect
component height — that is owned by size):

| `DensityDefinition` | Effect |
|---|---|
| `DensityDefinition::Compact` | Tighter horizontal gaps and padding |
| `DensityDefinition::Default` | Standard spacing |
| `DensityDefinition::Comfortable` | Looser spacing |

### Control size

Controls intrinsic dimensions (height, font size, vertical padding, radii):

| `ControlSizeDefinition` | Height |
|---|---|
| `ControlSizeDefinition::Xs` | 24px |
| `ControlSizeDefinition::Sm` | 28px |
| `ControlSizeDefinition::Md` | 36px |
| `ControlSizeDefinition::Lg` | 44px |
| `ControlSizeDefinition::Xl` | 52px |

### Switching theme at runtime

```rust
// Rebuild the provider with the new theme
let dark_theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark);
let light_theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Light);

// Pass the active theme into render functions each frame
let el = js_button(&spec, &active_theme);
```

Because all render functions are pure and the provider is cheap to construct
from cached definitions, switching themes only requires passing a different
provider — no state to flush.

### DPI scaling

```rust
let theme = JetstreamThemeProvider::from_theme(&ThemeDefinition::Dark)
    .with_scale_factor(2.0);  // Retina / 200% display zoom
```

All space tokens returned by the provider are multiplied by the scale factor.

---

## Writing a New Component

This is the full workflow for implementing a new Jetstream component, following
the contract-first discipline defined in `CLAUDE.md`.

### Step 1: Read the contract

Before writing any code, read the full contract:

```
docs/contracts/components/<component>.md
```

Every implementation decision must trace to a contract requirement. The
contract defines anatomy, props, states, token targets, accessibility rules,
and specimen definitions. If something is not in the contract, do not invent
it.

### Step 2: Verify the spec struct

Check that `poodle-specs` has a Spec struct covering every prop in the
contract. If props are missing, add them to the spec crate before implementing
the component.

### Step 3: Implement the render function

Create `packages/jetstream/components/src/<component>.rs`:

```rust
use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;
use poodle_jetstream_components::presentation::*;
use poodle_specs::MyComponentSpec;

pub fn js_my_component(spec: &MyComponentSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // 1. Resolve all visual properties from tokens — zero hardcoded values
    let fill    = resolve_color(theme, spec.fill_token());
    let text    = resolve_color(theme, spec.text_token());
    let height  = rem_to_px(control_height_rem(spec.size()));
    let radius  = resolve_px(theme, spec.radius_token());
    let gap     = resolve_px(theme, spec.gap_token());

    // 2. Build anatomy matching the contract
    // (every part the contract defines must be present)
    let label_el = label(spec.label())
        .text_size(rem_to_px(size_font_rem(spec.size())))
        .text_color(text);

    // 3. Build the root element
    let mut root = div()
        .flex_row()
        .items_center()
        .h(height)
        .px(resolve_px(theme, spec.padding_x_token()))
        .gap(gap)
        .bg(fill)
        .rounded(radius)
        .child(label_el);

    // 4. Handle states
    if spec.disabled() {
        root = root.opacity(resolve_opacity(theme, "state.opacity.disabled"));
    }

    // 5. Hover / active overrides
    root = root
        .hover(|s| s.bg(resolve_color(theme, spec.fill_hover_token())))
        .active(|s| s.bg(resolve_color(theme, spec.fill_active_token())));

    root
}
```

### Step 4: Check the implementation checklist

- [ ] Every dimension resolves from a token (height, padding, gap, radius, font-size) — ZERO hardcoded px values
- [ ] Every color resolves from a token via the spec's token methods
- [ ] Anatomy matches the contract (all parts present, correct nesting)
- [ ] All props from the contract are supported in the spec
- [ ] Disabled and loading states apply `state.opacity.disabled` via token
- [ ] Focus ring implemented where the contract requires it
- [ ] ARIA/accessibility attributes noted (Jetstream runtime level — document deviations in parity report)

### Step 5: Write a specimen

Add a specimen file at `packages/jetstream/preview/src/specimens/<component>.rs`
showing all states the contract defines:

```rust
use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::my_component::js_my_component;
use poodle_specs::MyComponentSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    div().flex_col().gap(16.0)
        // Default state
        .child(js_my_component(&MyComponentSpec::new().with_label("Default"), theme))
        // Disabled state
        .child(js_my_component(&MyComponentSpec::new().with_label("Disabled").with_disabled(true), theme))
        // ... all other contract states
}
```

### Step 6: Cross-reference Svelte

The Svelte implementation (`packages/svelte/components/src/`) is the visual
reference. When the contract is ambiguous, check the Svelte component. The GPUI
implementation may have deviations and should not be used as a reference.

### Step 7: Update the parity report

After the component is complete, update `cross-runtime-parity-report.json` with
the new component's parity tier and any approved deviations.

---

## Parity

Poodle maintains parity across Svelte, GPUI, and Jetstream targets using a
three-tier model.

### Parity tiers

| Tier | Name | Meaning |
|---|---|---|
| 1 | Full | Strict visual and behavioral parity with the Svelte reference |
| 2 | Partial | Visual parity with documented, approved native adaptations |
| 3 | Skip | Out of scope for this runtime (currently none in Jetstream) |

Tier 2 deltas require an explicit entry in the parity report with a rationale.
Undocumented deviations from the Svelte reference are bugs, not features.

### Parity report

```
packages/jetstream/cross-runtime-parity-report.json
```

Current status (g10.014): 109 full parity, 8 partial parity, 165 adapter tests
passing, 100% component coverage (117/117).

---

## Reference

### Key crates

| Crate | Path | Role |
|---|---|---|
| `poodle-jetstream` | `packages/jetstream/adapter` | `JetstreamThemeProvider`, `map_layout()`, adapter manifest |
| `poodle-jetstream-components` | `packages/jetstream/components` | `js_<component>()` render functions |
| `poodle-specs` | `packages/contracts/components` | Component spec structs |
| `poodle-workstation` | `packages/contracts/workstation` | Workstation spec structs |
| `poodle-tokens` | `packages/contracts/tokens` | `ThemeDefinition`, `DensityDefinition`, `ControlSizeDefinition` |
| `poodle-adapter` | `packages/contracts/adapter` | `ThemeProvider` trait |
| `poodle-layout` | `packages/contracts/layout` | `LayoutIntent`, layout types |
| `jetstream-runtime` | (external) | `JsEl`, `div()`, `label()`, etc. |

### Related documentation

- Component contracts: `docs/contracts/components/`
- Architecture: `docs/architecture/001-poodle-system-shape.md`
- Token system: `docs/architecture/002-token-system-and-package-layout.md`
- Size/density rules: `docs/architecture/` and `CLAUDE.md`
- GPUI guide: `docs/guides/gpui-developer-guide.md`
- Svelte guide: `docs/guides/svelte-developer-guide.md`
