//! SegmentedControl — Jetstream segmented control backed by SegmentedControlSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SegmentedControlSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_segmented_control(spec: &SegmentedControlSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let selected_fill = resolve_color(theme, spec.selected_fill_token());
    let bg = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let text_muted = resolve_color(theme, "semantic.color.text.secondary");
    let radius = resolve_radius(theme, "semantic.radius.control");

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());

    let mut el = ui_element::div()
        .bg(bg)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center()
        .h(32.0);

    for option in &spec.options {
        let is_selected = selected == Some(option.value.as_str());
        let mut seg = ui_element::button(&option.label)
            .text_size(13.0).text_weight(500)
            .text_color(if is_selected { text_color } else { text_muted })
            .pl(12.0).pr(12.0).pt(4.0).pb(4.0)
            .rounded(radius)
            .focusable();

        if is_selected {
            seg = seg.bg(selected_fill);
        }

        el = el.child(seg);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
