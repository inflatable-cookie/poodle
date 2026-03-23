//! TextArea — Jetstream multi-line text area backed by TextAreaSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::TextAreaSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_text_area(spec: &TextAreaSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let display_value = spec.value.as_deref()
        .or(Some(spec.default_value.as_str()))
        .unwrap_or("");

    let line_height = 20.0;
    let min_height = line_height * spec.rows as f32;

    ui_element::label(display_value)
        .min_h(min_height)
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(8.0).pr(8.0).pt(6.0).pb(6.0)
        .text_color(text_color)
        .text_size(13.0)
}
