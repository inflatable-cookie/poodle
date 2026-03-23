//! NumberEntry — Jetstream number input backed by NumberEntrySpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::NumberEntrySpec;

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

    // Decrement button (SVG icon)
    el = el.child(
        ui_element::button("").pl(8.0).pr(4.0).focusable().cursor_pointer()
            .child(ui_element::icon("minus").w(14.0).h(14.0).text_color(text_color))
    );

    // Value display
    el = el.child(
        ui_element::label(&value_text)
            .text_color(text_color).text_size(13.0)
            .grow().text_align_center()
    );

    // Increment button (SVG icon)
    el = el.child(
        ui_element::button("").pl(4.0).pr(8.0).focusable().cursor_pointer()
            .child(ui_element::icon("plus").w(14.0).h(14.0).text_color(text_color))
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
