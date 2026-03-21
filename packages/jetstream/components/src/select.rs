//! Select — Jetstream select dropdown backed by SelectSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::SelectSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_select(spec: &SelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());
    let display = selected
        .and_then(|v| spec.options.iter().find(|o| o.value == v).map(|o| o.label.as_str()))
        .or(spec.placeholder.as_deref())
        .unwrap_or("Select...");

    let display_color = if selected.is_some() { text_color } else { muted };

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .h(36.0)
        .pl(12.0).pr(8.0)
        .flex_row().items_center()
        .gap(4.0);

    el = el.child(ui_element::label(display).text_color(display_color).text_size(13.0).grow());
    el = el.child(ui_element::label("▾").text_color(muted).text_size(10.0));

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
