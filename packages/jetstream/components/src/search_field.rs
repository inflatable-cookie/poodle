//! SearchField — Jetstream search input backed by SearchFieldSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SearchFieldSpec;

use crate::theme_ext::resolve_color;

pub fn js_search_field(spec: &SearchFieldSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let icon_color = resolve_color(theme, spec.search_icon_color_token());
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.default");

    let display = spec.value.as_deref()
        .or(Some(spec.default_value.as_str()))
        .unwrap_or("");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(6.0)
        .pl(8.0).pr(8.0)
        .h(36.0)
        .flex_row()
        .items_center()
        .gap(6.0);

    // Search icon (SVG)
    el = el.child(ui_element::icon("search").w(16.0).h(16.0).text_color(icon_color));

    // Value
    el = el.child(ui_element::label(display).text_color(text_color).text_size(13.0).grow());

    el
}
