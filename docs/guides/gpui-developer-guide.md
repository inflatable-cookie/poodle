# Poodle GPUI Developer Guide

End-to-end guide for building UI with Poodle components in a GPUI application.

For the Jetstream target, see [Jetstream Developer Guide](./jetstream-developer-guide.md).
For the Svelte target, see [Svelte Developer Guide](./svelte-developer-guide.md).
For component contracts, see `docs/contracts/components/`.

---

## Quick Start

### 1. Add dependencies

In your GPUI app's `Cargo.toml`:

```toml
[dependencies]
poodle-gpui            = { path = "<path-to-poodle>/packages/gpui/adapter" }
poodle-gpui-components = { path = "<path-to-poodle>/packages/gpui/components" }
poodle-specs           = { path = "<path-to-poodle>/packages/contracts/components" }
poodle-tokens          = { path = "<path-to-poodle>/packages/contracts/tokens" }
gpui                   = "0.2.2"
```

If Poodle is a workspace sibling, use workspace path dependencies. If it is
vendored, point to the vendored paths.

### 2. Create a theme provider

```rust
use poodle_gpui::GpuiThemeProvider;
use poodle_tokens::{ThemeDefinition, DensityDefinition, ControlSizeDefinition};

let theme = GpuiThemeProvider::new()
    .with_theme(&ThemeDefinition::Dark)
    .with_density(&DensityDefinition::Default)
    .with_control_size(&ControlSizeDefinition::Md);
```

The theme provider resolves semantic token paths to the typed values each
component's rendering needs. Pass it by reference into every component
constructor.

### 3. Render a component

GPUI components are structs that implement `IntoElement`. Build them with
either the fluent builder or `from_spec`:

```rust
use gpui::prelude::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::button::Button;
use poodle_specs::{ButtonSpec, ButtonVariant, ButtonTone, ControlSize};

// Via fluent builder
let button = Button::new(&theme)
    .label("Save changes")
    .variant(ButtonVariant::Solid)
    .tone(ButtonTone::Accent)
    .size(ControlSize::Md)
    .on_click(|_event, _window, _app| { /* handler */ });

// Via spec (when spec is computed or data-driven)
let spec = ButtonSpec::new()
    .with_label("Save changes")
    .with_variant(ButtonVariant::Solid)
    .with_tone(ButtonTone::Accent);
let button = Button::from_spec(spec, &theme);

// Compose into a GPUI view
div()
    .flex_row()
    .gap(rems(0.5))
    .child(button)
    .into_element()
```

### Run the preview app

```sh
cargo run -p poodle-gpui-preview \
    --manifest-path packages/gpui/preview/Cargo.toml
```

The preview app renders specimens for all 96 implemented components across
theme, density, and control-size permutations. Use it to validate visual
correctness before shipping.

---

## Token Resolution

All visual properties in Poodle resolve from the semantic token system. No
component hardcodes a pixel value or color — every property traces back to a
token.

### The token path system

Token paths are dot-separated semantic names:

```
color.background.canvas
color.background.surface
color.background.panel
color.background.elevated
color.background.overlay
color.text.primary
color.text.secondary
color.text.tertiary
color.text.inverse
color.border.subtle
color.border.default
color.border.strong
color.accent.base
color.accent.hover
color.accent.focusRing
color.status.success
color.status.warning
color.status.danger
color.icon.primary
color.icon.muted
size.control.height
size.control.minWidth
size.icon.sm / .md / .lg
space.control.x / .y
space.panel.x / .y
space.inline.xs / .sm / .md / .lg
space.stack.sm / .md / .lg
radius.control
radius.surface
radius.pill
border.width.default
border.width.focus
state.opacity.disabled
state.opacity.muted
typography.body.size / .lineHeight
typography.label.size / .lineHeight
typography.heading.size / .lineHeight
typography.code.size / .lineHeight
```

### Resolving tokens in component code

Use the helpers from `poodle_gpui_components::theme_ext`:

```rust
use poodle_gpui_components::theme_ext::*;

// Color → Hsla (GPUI's native color type)
let fill   = resolve_color(&theme, "color.background.surface");
let text   = resolve_color(&theme, "color.text.primary");
let accent = resolve_color(&theme, "color.accent.base");
let ring   = resolve_color(&theme, "color.accent.focusRing");

// Space / size / radius → Pixels
let height = resolve_px(&theme, "size.control.height");
let pad    = resolve_px(&theme, "space.control.x");
let radius = resolve_radius(&theme, "radius.control");

// Opacity → f32 (0.0..1.0)
let dim    = resolve_opacity(&theme, "state.opacity.disabled");
```

Additional helpers:

```rust
// Color blending (sRGB space)
let mixed      = color_mix(a, b, 0.5);
let darkened   = color_mix_black(color, 0.15);

// Focus ring shadow
let shadows = focus_ring_shadow(ring_color);

// Hex color parsing (for values from token strings)
let parsed = parse_hex_color("#2d86f3");  // Option<Hsla>

// Brand-raised gradient fills
let primary_fill       = brand_raised_primary_fill(accent);
let primary_fill_hover = brand_raised_primary_fill_hover(accent);
```

### Presentation helpers

`poodle_gpui_components::presentation` provides size/density resolution:

```rust
use poodle_gpui_components::presentation::*;

// Height in rem for a given control size
let h    = rem_to_px(control_height_rem(ControlSize::Md));    // 36px

// Font size
let fs   = rem_to_px(size_font_rem(ControlSize::Md));         // 13px

// Horizontal padding by density
let padx = rem_to_px(control_space_x_rem(ControlDensity::Default));

// Icon size is one stop smaller than control (clamped at Xs)
let icon = resolve_supporting_visual_size(ControlSize::Md);   // → Sm
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

### How GPUI components work

```
ComponentSpec  (data — props, state, content)
    + GpuiThemeProvider  (resolves token paths → typed values)
        ↓
    Component::from_spec(spec, theme)  or  Component::new(theme).builder_methods()
        (GPUI struct implementing IntoElement)
        ↓
    component.into_element()
        (called by GPUI during the render pass → AnyElement → layout → draw)
```

Unlike Jetstream's pure render functions, GPUI components are structs that hold
their spec and theme, and implement GPUI's `IntoElement` / `RenderOnce` traits.
This integrates cleanly with GPUI's view system and `cx.listener()` event
handling pattern.

### Two construction patterns

**Fluent builder** — when building a component in-line within a view:

```rust
Button::new(&theme)
    .label("Cancel")
    .variant(ButtonVariant::Ghost)
    .size(ControlSize::Sm)
    .on_click(cx.listener(|this, _, window, cx| {
        this.close_dialog(window, cx);
    }))
```

**`from_spec`** — when the spec is computed, stored, or data-driven:

```rust
let spec = ButtonSpec::new()
    .with_label("Submit")
    .with_variant(ButtonVariant::Solid)
    .with_disabled(self.is_submitting);

Button::from_spec(spec, &theme)
    .on_click(cx.listener(|this, _, window, cx| {
        this.submit(window, cx);
    }))
```

Both produce the same element. The fluent builder forwards its calls to the
underlying spec — they are equivalent paths to the same result.

### Composites

Composites follow the same patterns but tend to carry richer state. Use
`from_spec` for composites when their spec is built from view state:

```rust
// CommandPalette example
let spec = CommandPaletteSpec::new(self.actions.clone())
    .with_query(&self.query)
    .with_open(self.is_open);

CommandPalette::from_spec(spec, &theme)
    .on_select(cx.listener(|this, action_id: &str, window, cx| {
        this.execute_action(action_id, window, cx);
    }))
    .on_query_change(cx.listener(|this, query: &str, window, cx| {
        this.set_query(query, cx);
    }))
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

Rebuild the provider when the user switches themes:

```rust
fn update_theme(&mut self, theme_def: ThemeDefinition, cx: &mut ViewContext<Self>) {
    self.theme = GpuiThemeProvider::new()
        .with_theme(&theme_def)
        .with_density(&self.density)
        .with_control_size(&self.control_size);
    cx.notify();
}
```

Because `GpuiThemeProvider` is cheap to construct and all rendering is
stateless, switching themes triggers a normal GPUI re-render with the new
provider.

### DPI scaling

```rust
let theme = GpuiThemeProvider::new()
    .with_scale_factor(2.0);  // Retina / 200% display zoom
```

All space tokens returned by the provider are multiplied by the scale factor.

### Brand-raised variant

Some components support a visually elevated appearance with gradient fills.
The adapter exposes the brand-raised flag:

```rust
// Enable via theme provider fields
// brand_raised: bool — raises primary accent fills to gradient
```

---

## Size and Density

Size and density are orthogonal axes. Getting this right matters for
maintaining visual consistency across the system.

**Size** controls intrinsic dimensions:
- Component height, vertical padding, font size, icon size, border radius
- Changing size makes a component physically larger or smaller

**Density** controls spacing between siblings:
- Horizontal padding, gaps between items, margins between list rows
- Changing density makes a layout tighter or looser without changing height

**Density must never affect vertical padding or component height.** If a
density variant in your component code changes `padding-block`, `min-height`,
or `height`, that is a bug — those properties belong to size variants.

Per-component size is set on the spec:

```rust
ButtonSpec::new().with_size(ControlSize::Sm)
```

A component can also query the global control size from the theme context when
no explicit size is given.

---

## Writing a New Component

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
contract. If props are missing, add them to the spec crate first.

### Step 3: Implement the component struct

Create `packages/gpui/components/src/primitives/<component>.rs`:

```rust
use gpui::prelude::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::theme_ext::*;
use poodle_gpui_components::presentation::*;
use poodle_specs::MyComponentSpec;

pub struct MyComponent {
    spec: MyComponentSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl MyComponent {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MyComponentSpec::new(),
            theme: theme.clone(),
            on_click: None,
        }
    }

    pub fn from_spec(spec: MyComponentSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_click: None }
    }

    // Forward spec builder methods
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.spec = self.spec.with_label(label.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for MyComponent {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // 1. Resolve all visual properties from tokens
        let fill   = resolve_color(&self.theme, self.spec.fill_token());
        let text   = resolve_color(&self.theme, self.spec.text_token());
        let height = resolve_px(&self.theme, self.spec.height_token());
        let radius = resolve_radius(&self.theme, self.spec.radius_token());

        // 2. Build anatomy matching the contract
        let label_el = div()
            .text_color(text)
            .text_size(rem_to_px(size_font_rem(self.spec.size())))
            .child(self.spec.label().to_string());

        // 3. Build root element
        let mut root = div()
            .flex_row()
            .items_center()
            .h(px(height))
            .px(rem_to_px(control_space_x_rem(self.spec.density())))
            .rounded(px(radius))
            .bg(fill)
            .child(label_el);

        // 4. Handle disabled state
        if self.spec.disabled() {
            root = root.opacity(resolve_opacity(&self.theme, "state.opacity.disabled"));
        }

        // 5. Wire events
        if let Some(handler) = self.on_click {
            root = root.on_click(handler);
        }

        root.into_any_element()
    }
}
```

### Step 4: Implementation checklist

- [ ] Every dimension resolves from a token — ZERO hardcoded px values
- [ ] Every color resolves from a token via spec token methods
- [ ] Anatomy matches the contract (all parts present, correct nesting)
- [ ] All props from the contract are supported in the spec
- [ ] Disabled/loading states use `resolve_opacity(theme, "state.opacity.disabled")`
- [ ] Focus ring implemented where the contract requires it (`focus_ring_shadow()`)
- [ ] ARIA attributes applied (role, aria-label, aria-expanded as applicable)
- [ ] Both `new()` and `from_spec()` constructors present

### Step 5: Write a specimen

Add a specimen in `packages/gpui/preview/src/specimens/<component>.rs` showing
all states the contract defines.

### Step 6: Cross-reference Svelte

The Svelte implementation (`packages/svelte/components/src/`) is the visual
reference. When the contract is ambiguous, check the Svelte component. Do not
use the Jetstream implementation as a reference (it may have its own deltas).

### Step 7: Update the parity report

After the component is complete, add it to `packages/gpui/cross-runtime-parity-report.json`
with its parity tier and any approved native adaptations.

---

## Parity

Poodle maintains parity across Svelte, GPUI, and Jetstream targets using a
three-tier model.

### Parity tiers

| Tier | Name | Meaning |
|---|---|---|
| 1 | Full | Strict visual and behavioral parity with the Svelte reference |
| 2 | Partial | Visual parity with documented, approved native adaptations |
| 3 | Skip | Out of scope for this runtime |

Tier 2 deltas require an explicit entry in the parity report with a rationale.
Undocumented deviations from the Svelte reference are bugs, not features.

### Known intentional deltas (GPUI)

Five approved native adaptations are documented in the parity report:

1. **Table narration** — screen reader row/cell announcement differs from web ARIA patterns
2. **Overlay focus scope** — GPUI focus trapping uses native focus system, not a web overlay model
3. **Media renderer** — video/audio playback backed by native APIs, not `<video>`/`<audio>`
4. **Announcement timing** — live region announcements use native AT timing
5. **Workstation dock** — dock resize behavior uses GPUI window constraints

These are features of the native target, not gaps to close.

### Parity report

```
packages/gpui/cross-runtime-parity-report.json
```

Current status (g09.018): 96 component exports across 64 primitives, 17
composites, and 15 workstation components. 73 specimen files.

---

## Reference

### Key crates

| Crate | Path | Role |
|---|---|---|
| `poodle-gpui` | `packages/gpui/adapter` | `GpuiThemeProvider`, `map_layout()`, `map_style()`, adapter manifest |
| `poodle-gpui-components` | `packages/gpui/components` | Component structs implementing `IntoElement` |
| `poodle-specs` | `packages/contracts/components` | Component spec structs |
| `poodle-workstation` | `packages/contracts/workstation` | Workstation spec structs |
| `poodle-tokens` | `packages/contracts/tokens` | `ThemeDefinition`, `DensityDefinition`, `ControlSizeDefinition` |
| `poodle-adapter` | `packages/contracts/adapter` | `ThemeProvider` trait |
| `poodle-layout` | `packages/contracts/layout` | `LayoutIntent`, layout types |
| `poodle-style` | `packages/contracts/style` | `StyleDescriptor`, style IR types |

### Implemented components

**Primitives (64):** accordion, alert_dialog, breadcrumbs, bulk_action_bar,
button, bx, calendar, callout, card, checkbox, code, code_input,
collapse_toggle, collapsible, color_picker, context_menu, date_picker,
date_range_picker, date_time_picker, date_time_range_picker,
date_time_zone_picker, detail_item, dialog, drawer, duration_input,
editable_label, eyebrow, field, field_set, file_upload, floating_overlay,
form_actions, grid, hover_card, icon, icon_button, list_card,
list_card_counter, list_grid, menu, menubar, meta_bar, meta_item, meter,
nav_card, navigation_menu, number_input, order_by, pagination,
pagination_summary, password_requirements, pill, popover, progress,
radio_group, range_slider, rating, region, resize_handle, scroll_shell,
segmented_control, select, separator, skeleton, slider, spacer, spinner,
split_button, stack, status_bar, status_indicator, surface, switch,
tab_strip, table, tabs, text_input, time_ago, time_field, time_zone_select,
toggle_group, toolbar, tooltip, tri_state_switch

**Composites (17):** action_discovery_panel, app_header, audio_player,
block_editor, card_radio_group, command_palette, confirm_action, data_table,
detail_section, filter_toolbar, form_dialog, form_layout, list_container,
media_browse_panel, media_picker, page_header, picker_shell

**Workstation (15):** detail_shell, dock_region, split_view, and others
defined in `poodle-workstation`

### Related documentation

- Component contracts: `docs/contracts/components/`
- Architecture: `docs/architecture/001-poodle-system-shape.md`
- Token system: `docs/architecture/002-token-system-and-package-layout.md`
- Size/density rules: `docs/architecture/` and `CLAUDE.md`
- Jetstream guide: `docs/guides/jetstream-developer-guide.md`
- Svelte guide: `docs/guides/svelte-developer-guide.md`
