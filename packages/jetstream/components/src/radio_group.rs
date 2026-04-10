//! RadioGroup — Jetstream radio group backed by RadioGroupSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{Orientation, RadioGroupSpec};

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_radio_group(spec: &RadioGroupSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(control_height_rem(effective_size) * 0.5);
    let dot_size = rem_to_px(control_height_rem(effective_size) * 0.22);
    let gap = rem_to_px(0.75);
    let item_gap = rem_to_px(0.375);

    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.default");
    let text_color = resolve_color(theme, "color.text.primary");
    let selected_value = spec.value.as_deref().or(spec.default_value.as_deref());

    let mut el = match spec.orientation {
        Orientation::Horizontal => ui_element::div().flex_row().gap(gap),
        Orientation::Vertical => ui_element::div().flex_col().gap(gap),
    };

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = if is_selected { accent } else { glam::Vec4::ZERO };

        // Radio indicator: proportional circle with inner dot when selected
        let mut indicator = ui_element::div()
            .w(indicator_size).h(indicator_size)
            .rounded(indicator_size * 0.5)
            .border(1.0).border_color(indicator_color)
            .items_center().justify_center();

        if is_selected {
            indicator = indicator.child(
                ui_element::div()
                    .w(dot_size).h(dot_size)
                    .rounded(dot_size * 0.5)
                    .bg(indicator_bg)
            );
        }

        let mut row = ui_element::div().flex_row().items_center().gap(item_gap).cursor_pointer();
        row = row.child(indicator);
        row = row.child(ui_element::label(&option.label).text_color(text_color).text_size(font_size));
        el = el.child(row);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
