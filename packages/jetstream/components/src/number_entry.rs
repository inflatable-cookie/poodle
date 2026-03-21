//! NumberEntry — Jetstream number input backed by NumberEntrySpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::NumberEntrySpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_number_entry(spec: &NumberEntrySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "semantic.radius.control");
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let value_text = format!("{}", spec.value);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .h(36.0)
        .flex_row().items_center();

    // Decrement button
    el = el.child(ui_element::button("−").text_size(14.0).pl(8.0).pr(4.0).focusable());

    // Value display
    el = el.child(
        ui_element::label(&value_text)
            .text_color(text_color).text_size(13.0)
            .grow().text_align_center()
    );

    // Increment button
    el = el.child(ui_element::button("+").text_size(14.0).pl(4.0).pr(8.0).focusable());

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
