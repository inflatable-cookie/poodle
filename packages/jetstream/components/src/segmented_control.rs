//! SegmentedControl — Jetstream segmented control backed by SegmentedControlSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SegmentedControlSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_segmented_control(spec: &SegmentedControlSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let seg_px = rem_to_px(control_space_x_rem(spec.density));
    let seg_py = rem_to_px(0.25);

    let selected_fill = resolve_color(theme, spec.selected_fill_token());
    let bg = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let text_color = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let radius = resolve_radius(theme, "radius.control");

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());

    let mut el = ui_element::div()
        .bg(bg)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center()
        .h(height);

    for option in &spec.options {
        let is_selected = selected == Some(option.value.as_str());
        let mut seg = ui_element::button(&option.label)
            .text_size(font_size).text_weight(500)
            .text_color(if is_selected { text_color } else { text_muted })
            .pl(seg_px).pr(seg_px).pt(seg_py).pb(seg_py)
            .rounded(radius)
            .focusable();

        if is_selected {
            seg = seg.bg(selected_fill);
        }

        el = el.child(seg);
    }

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
