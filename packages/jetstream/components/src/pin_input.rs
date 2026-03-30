//! PinInput — Jetstream PIN code input backed by PinInputSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::PinInputSpec;

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_pin_input(spec: &PinInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let cell_size = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size) + 0.1875);
    let gap = rem_to_px(0.375);

    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div().flex_row().gap(gap);

    for i in 0..spec.length {
        let ch = Some(&spec.value)
            .and_then(|v| v.chars().nth(i as usize))
            .map(|c| if spec.is_masked { "●".to_string() } else { c.to_string() })
            .unwrap_or_default();

        el = el.child(
            ui_element::label(&ch)
                .bg(fill)
                .border(1.0).border_color(border)
                .rounded(radius)
                .w(cell_size).h(cell_size)
                .text_color(text_color)
                .text_size(font_size)
                .flex_row().items_center().justify_center()
        );
    }

    el
}
