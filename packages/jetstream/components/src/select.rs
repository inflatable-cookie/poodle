//! Select — Jetstream select dropdown backed by SelectSpec.
//!
//! Contract: `docs/contracts/foundation/select.md`
//! Reference: `packages/svelte/primitives/src/Select.svelte`
//!
//! Uses SVG chevron icon and overlay() for the dropdown panel.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::SelectSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_select(spec: &SelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border_color = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");
    let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
    let height = resolve_px(theme, "semantic.size.control.height");
    let pad_x = resolve_px(theme, "semantic.space.control.x");
    let label_size = resolve_px(theme, "semantic.typography.body.size");

    // Hover state
    let border_c: Color = border_color.into();
    let text_primary: Color = text_color.into();
    let hover_border = border_c.mix(text_primary, 0.78);

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());
    let display = selected
        .and_then(|v| spec.options.iter().find(|o| o.value == v).map(|o| o.label.as_str()))
        .or(spec.placeholder.as_deref())
        .unwrap_or("Select...");

    let display_color = if selected.is_some() { text_color } else { muted };

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border_color)
        .rounded(radius)
        .h(height)
        .pl(pad_x).pr(pad_x - 4.0)
        .flex_row().items_center()
        .gap(4.0)
        .focusable()
        .cursor_pointer()
        .hover(|s| s.border_color(hover_border));

    el = el.child(
        ui_element::label(display)
            .text_color(display_color)
            .text_size(label_size)
            .grow()
    );

    // SVG chevron indicator
    el = el.child(
        ui_element::icon("chevron-down")
            .w(16.0).h(16.0)
            .text_color(icon_muted)
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
