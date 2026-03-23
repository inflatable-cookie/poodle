//! RadioGroup — Jetstream radio group backed by RadioGroupSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::{Orientation, RadioGroupSpec};

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

pub fn js_radio_group(spec: &RadioGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let gap = resolve_px(theme, spec.option_gap_token());
    let accent = resolve_color(theme, "semantic.color.accent.base");
    let border = resolve_color(theme, "semantic.color.border.default");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let selected_value = spec.value.as_deref().or(spec.default_value.as_deref());

    let mut el = match spec.orientation {
        Orientation::Horizontal => ui_element::div().flex_row().gap(gap),
        Orientation::Vertical => ui_element::div().flex_col().gap(gap),
    };

    let label_size = resolve_px(theme, "semantic.typography.label.size");

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = if is_selected { accent } else { glam::Vec4::ZERO };

        // Radio indicator: 18px circle with inner dot when selected
        let mut indicator = ui_element::div()
            .w(18.0).h(18.0)
            .rounded(9.0) // circle
            .border(1.0).border_color(indicator_color)
            .items_center().justify_center();

        if is_selected {
            // Inner dot (contract: 8px filled circle)
            indicator = indicator.child(
                ui_element::div()
                    .w(8.0).h(8.0)
                    .rounded(4.0)
                    .bg(indicator_bg)
            );
        }

        let mut row = ui_element::div().flex_row().items_center().gap(6.0).cursor_pointer();
        row = row.child(indicator);
        row = row.child(ui_element::label(&option.label).text_color(text_color).text_size(label_size));
        el = el.child(row);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
