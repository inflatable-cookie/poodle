//! SearchInput — Jetstream search input backed by SearchInputSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SearchInputSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_search_input(spec: &SearchInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let gap = rem_to_px(0.375);

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
        .rounded(rem_to_px(0.375))
        .pl(pad_x).pr(pad_x)
        .h(height)
        .flex_row()
        .items_center()
        .gap(gap);

    // Search icon (SVG)
    el = el.child(ui_element::icon("search").w(icon_size).h(icon_size).text_color(icon_color));

    // Value
    el = el.child(ui_element::label(display).text_color(text_color).text_size(font_size).grow());

    el
}
