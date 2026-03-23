//! DatePicker — Jetstream date picker backed by DatePickerSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::DatePickerSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_date_picker(spec: &DatePickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");

    let display = spec.value.as_deref().unwrap_or("Select date...");
    let display_color = if spec.value.is_some() { text_color } else { muted };

    ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .h(36.0).pl(12.0).pr(8.0)
        .flex_row().items_center().gap(4.0)
        .child(ui_element::label(display).text_color(display_color).text_size(13.0).grow())
        .child(ui_element::label("📅").text_size(14.0))
}
