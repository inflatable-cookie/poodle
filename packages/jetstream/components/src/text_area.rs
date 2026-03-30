//! TextArea — Jetstream multi-line text area backed by TextAreaSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::TextAreaSpec;

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_text_area(spec: &TextAreaSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(0.375);

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let display_value = spec.value.as_deref()
        .or(Some(spec.default_value.as_str()))
        .unwrap_or("");

    let line_height = rem_to_px(1.25);
    let min_height = line_height * spec.rows as f32;

    ui_element::label(display_value)
        .min_h(min_height)
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .text_color(text_color)
        .text_size(font_size)
}
