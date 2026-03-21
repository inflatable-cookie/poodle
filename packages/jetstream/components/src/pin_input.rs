//! PinInput — Jetstream PIN code input backed by PinInputSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::PinInputSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_pin_input(spec: &PinInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div().flex_row().gap(6.0);

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
                .w(36.0).h(36.0)
                .text_color(text_color)
                .text_size(16.0)
                .flex_row().items_center().justify_center()
        );
    }

    el
}
