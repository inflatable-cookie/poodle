//! Code — Jetstream code block backed by CodeSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::CodeSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_code(spec: &CodeSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let border = resolve_color(theme, spec.border_token());
    let font_size = resolve_px(theme, spec.font_size_token());
    let radius = resolve_radius(theme, "semantic.radius.surface");

    ui_element::label(&spec.content)
        .bg(fill)
        .text_color(text_color)
        .text_size(font_size)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0)
}
