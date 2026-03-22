//! SplitButton — Jetstream split button backed by SplitButtonSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::SplitButtonSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_split_button(spec: &SplitButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let separator = resolve_color(theme, spec.separator_token());
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let label = spec.label.as_deref().unwrap_or("");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center()
        .h(36.0);

    // Primary action
    el = el.child(
        ui_element::button(label)
            .text_color(text_color).text_size(13.0).text_weight(500)
            .pl(12.0).pr(12.0)
            .focusable()
    );

    // Separator
    el = el.child(ui_element::div().w(1.0).h(20.0).bg(separator));

    // Dropdown trigger
    el = el.child(
        ui_element::button("")
            .child(ui_element::icon("chevron-down").w(14.0).h(14.0).text_color(text_color))
            .pl(6.0).pr(6.0)
            .focusable()
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
