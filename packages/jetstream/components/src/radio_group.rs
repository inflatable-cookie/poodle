//! RadioGroup — Jetstream radio group backed by RadioGroupSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::{Orientation, RadioGroupSpec};

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

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let indicator_color = if is_selected { accent } else { border };

        let indicator = if is_selected { "◉" } else { "○" };
        let mut row = ui_element::div().flex_row().items_center().gap(6.0);
        row = row.child(ui_element::label(indicator).text_color(indicator_color).text_size(14.0));
        row = row.child(ui_element::label(&option.label).text_color(text_color).text_size(13.0));
        el = el.child(row);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
