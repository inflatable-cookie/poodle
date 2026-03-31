//! NumberInput — Jetstream number input backed by NumberInputSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::NumberInputSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_number_input(spec: &NumberInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let btn_gap = rem_to_px(0.25);

    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "semantic.radius.control");
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let value_text = format!("{}", spec.value);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .h(height)
        .flex_row().items_center();

    // Decrement button (SVG icon)
    el = el.child(
        ui_element::button("").pl(pad_x).pr(btn_gap).focusable().cursor_pointer()
            .child(ui_element::icon("minus").w(icon_size).h(icon_size).text_color(text_color))
    );

    // Value display
    el = el.child(
        ui_element::label(&value_text)
            .text_color(text_color).text_size(font_size)
            .grow().text_align_center()
    );

    // Increment button (SVG icon)
    el = el.child(
        ui_element::button("").pl(btn_gap).pr(pad_x).focusable().cursor_pointer()
            .child(ui_element::icon("plus").w(icon_size).h(icon_size).text_color(text_color))
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
