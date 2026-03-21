//! TextInput — Jetstream text input backed by TextInputSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::TextInputSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_text_input(spec: &TextInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let height = resolve_px(theme, spec.control_height_token());
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let pad_x = resolve_px(theme, spec.horizontal_padding_token());
    let text_color = resolve_color(theme, spec.text_color_token());

    let display_value = spec.value.as_deref()
        .or(Some(spec.default_value.as_str()))
        .unwrap_or("");

    let mut el = ui_element::label(display_value)
        .h(height)
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x)
        .text_color(text_color)
        .text_size(13.0)
        .flex_row()
        .items_center();

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
