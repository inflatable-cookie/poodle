# Poodle GPUI Developer Guide

Poodle renders native components from shared Rust specs. `poodle-render`
produces a renderer-neutral node tree; `poodle-gpui-node-backend` interprets
that tree as GPUI elements.

Poodle's Rust crates are pre-1.0 source previews and are not published to
crates.io. Use path or workspace dependencies.

## Add Dependencies

```toml
[dependencies]
gpui = "0.2.2"
poodle-gpui = { path = "../poodle/packages/gpui/adapter" }
poodle-gpui-node-backend = { path = "../poodle/packages/gpui/node-backend" }
poodle-render = { path = "../poodle/packages/render" }
poodle-specs = { path = "../poodle/packages/contracts/components" }
poodle-tokens = { path = "../poodle/packages/contracts/tokens" }
```

Adjust paths for a vendored checkout or Cargo workspace.

## Render a Component

Create a theme, build the component spec, render a node, and convert it at the
GPUI boundary:

```rust
use gpui::IntoElement;
use poodle_gpui::GpuiThemeProvider;
use poodle_render::RenderContext;
use poodle_specs::{ButtonSpec, ButtonTone, ButtonVariant};

let theme = GpuiThemeProvider::new()
    .with_theme(&poodle_tokens::themes::ECLIPSE)
    .with_density(&poodle_tokens::density::DEFAULT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_SM);

// The construction context carries the borrowed theme plus the effective
// presentation defaults (root: md size scale, default density).
let ctx = RenderContext::new(&theme);

let spec = ButtonSpec::new()
    .with_label("Save changes")
    .with_variant(ButtonVariant::Primary)
    .with_tone(ButtonTone::Default);

let node = poodle_render::button(&spec, &ctx, None);
let element = poodle_gpui_node_backend::to_gpui(&node);

element.into_element()
```

Interactive render functions accept typed handlers. For example, a button
click handler is an `Arc<dyn Fn() + Send + Sync>`:

```rust
use std::sync::Arc;

let on_save = Arc::new(|| {
    // Send an application action or update application state.
});

let node = poodle_render::button(&spec, &ctx, Some(on_save));
```

The exact render signature is part of the Rust API. Check the function in
`packages/render/src/` and the matching
[component contract](../contracts/components/README.md) for a component with
multiple handlers or content slots.

## Wire the Window Root

`to_gpui` converts one node tree. Two behaviours are window-level rather than
component-level, and an application has to opt into them **once**, at its root:

- **Tab and Shift+Tab traversal.** GPUI owns sequential focus
  (`Window::focus_next`/`focus_prev`) but binds no key to it, exactly as a
  browser's Tab is a document-level default action rather than a control's.
- **Overlay dismissal and drag cleanup.** Escape dismisses the innermost open
  layer, a pointer press outside a layer dismisses it, and an unfinished
  payload drag ends on mouse-up or Escape.

Both live on `attach_overlay_host`, whose name predates the traversal it now
also carries. Its companion, `overlay_frame_begin`, marks the frame boundary
the layer registry, painted bounds, and focus queue are rebuilt at. Defer
`overlay_frame_end` to the end of the same effect cycle so a removed control
cancels in the removal frame.

```rust
use gpui::{div, Context, IntoElement, ParentElement, Render, Styled, Window};

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Once per rendered frame, before any node is converted.
        poodle_gpui_node_backend::overlay_frame_begin();
        cx.defer(|_cx| poodle_gpui_node_backend::overlay_frame_end());
        // Restart the generated-id counter so a node that declares no id keeps
        // the same ElementId across the frames a real click spans.
        poodle_gpui_node_backend::reset_element_ids();

        // Once per window, around the one root element.
        poodle_gpui_node_backend::attach_overlay_host(
            div()
                .size_full()
                .child(poodle_gpui_node_backend::to_gpui(&self.node_tree())),
        )
    }
}
```

Once per **window**, not once per component: wrapping individual components
would register the same window-level listeners many times over. A root that
skips this still renders and still takes pointer input, but nothing in it is
reachable by keyboard traversal and no overlay dismisses.

Sequential traversal follows `Interaction::focusable` and `NodeA11y::tab_index`
the way the DOM follows the absence or presence of `tabindex`: a focusable node
with no declared index is a tab stop, an index of `0` or more orders it, and
`-1` keeps it programmatically focusable and out of the sequence. Components
declare that; applications do not manage focus by hand.

## Theme and Scale

`GpuiThemeProvider` implements Poodle's renderer-neutral `ThemeProvider` trait.
It resolves semantic colors, spacing, radii, borders, and opacity into typed
values used by `poodle-render`.

```rust
let theme = GpuiThemeProvider::new()
    .with_theme(&poodle_tokens::themes::GRAPHITE)
    .with_density(&poodle_tokens::density::COMPACT)
    .with_control_size(&poodle_tokens::density::CONTROL_SIZE_MD)
    .with_contrast(0.65)
    .with_scale_factor(2.0);
```

Available theme constants are in `poodle_tokens::themes`; density and control
size constants are in `poodle_tokens::density`. Use these generated constants
instead of constructing theme values in application code.

## Ownership by Layer

| Layer | Owns |
| --- | --- |
| `poodle-specs` | Renderer-neutral component inputs and state |
| `poodle-tokens` | Generated themes and semantic values |
| `poodle-render` | Shared native composition, appearance, and interaction intent |
| `poodle-node` | Renderer-neutral output vocabulary |
| `poodle-gpui` | GPUI theme and shared style mapping |
| `poodle-gpui-node-backend` | Node interpretation, GPUI input, and event plumbing |
| Application | Data, routing, persistence, domain state, and orchestration |

Do not create a GPUI-only component implementation when the semantics are
shared with Jetstream. Add the spec and contract first, implement the component
in `poodle-render`, then extend the node backend only if the node vocabulary
cannot express it.

## Slots and Composition

Simple content is carried in specs. Rich child content and component handlers
are additional render-function arguments. Ordinary slots are
`poodle_node::Node` values, so components can compose without depending on
GPUI types. The exception is a composite whose web pair wraps host content in
an internal `UiPresentationProvider` (AppHeader, Field, FilterToolbar,
MediaPreview, PageHeader, BlockEditor): those slots are `SlotBuilder` closures
the component invokes immediately inside its scoped context, so the host
child inherits the scope instead of arriving prebuilt.

Convert the completed tree to GPUI once, near the view boundary. Avoid
converting child nodes individually and then trying to insert GPUI elements
back into shared rendering.

## Presentation Scopes (UiPresentationProvider)

`UiPresentationProvider` is a construction-time boundary, not a painted node.
`poodle_render::ui_presentation_provider` derives a nested
`RenderContext`, builds its child through an immediate closure, and returns
that child unchanged — no wrapper layout, paint, focus target, or
accessibility entry.

Component specs keep omission in the type system: semantic `size` and
`density` inputs are `Option`. A renderer resolves an omitted input from the
context, then applies the component's `size_role`. An explicit value always
wins — including an explicit `md` or `default` inside a non-default scope.
Root defaults are `md` / `default`; nested providers replace both defaults
only for construction inside their closure.

```rust
use poodle_render::ui_presentation_provider;
use poodle_specs::{ControlDensity, ControlSize, UiPresentationProviderSpec};

let scope = UiPresentationProviderSpec::new()
    .with_size_scale(ControlSize::Xl)
    .with_density(ControlDensity::Comfortable);

let node = ui_presentation_provider(&scope, &ctx, |scoped| {
    // Omitted inputs inherit xl / comfortable here.
    poodle_render::button(&ButtonSpec::new().with_label("Save"), scoped, None)
    // An explicit `.with_size(ControlSize::Md)` would stay md in this scope.
    // A nested `ui_presentation_provider(&inner, scoped, ...)` would replace
    // both defaults for its own closure only.
});
```

See the
[UiPresentationProvider contract](../contracts/components/ui-presentation-provider.md)
for the resolution tables and the breaking-migration notes.

## Run the Preview

From the Poodle repository root:

```sh
bun install
effigy gpui:preview
```

The preview exercises the component catalogue across theme, density, and
control-size modes. Use the component contract and parity artifacts for
semantic review; specimen output alone is not proof of parity.

## Add or Change a Component

1. Update the contract under `docs/contracts/components/`.
2. Update or add the spec in `poodle-specs`.
3. Put reusable interaction logic in `poodle-headless` when appropriate.
4. Implement `Spec + RenderContext -> Node` in `poodle-render`.
5. Extend `poodle-node` only for a genuinely reusable rendering capability.
6. Add GPUI backend behavior only for GPUI-specific interpretation.
7. Add preview coverage and update parity evidence.

Keep application actions outside Poodle. A component reports intent; the host
decides what saving, navigation, upload, or deletion means.

## Related Documentation

- [System architecture](../architecture/001-poodle-system-shape.md)
- [Token architecture](../architecture/002-token-system-and-package-layout.md)
- [Component contracts](../contracts/components/README.md)
- [GPUI adapter reference](../../packages/gpui/adapter/README.md)
- [Jetstream developer guide](jetstream-developer-guide.md)
