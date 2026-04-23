//! DatePicker — Jetstream date picker backed by DatePickerSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DatePickerSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_date_picker(spec: &DatePickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let fill = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");

    let height = rem_to_px(control_height_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let font_size = rem_to_px(size_font_rem(effective_size));

    let display = spec.value.as_deref().unwrap_or("Select date...");
    let display_color = if spec.value.is_some() { text_color } else { muted };

    ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .h(height).pl(pad_x).pr(pad_x)
        .flex_row().items_center().gap(rem_to_px(0.25))
        .child(ui_element::label(display).text_color(display_color).text_size(font_size).grow())
        .child(ui_element::label("📅").text_size(font_size))
}
