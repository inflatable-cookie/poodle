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
    // Vertical inset: 0.125rem top+bottom so label fills (height - 0.25rem), matching Svelte.
    let seg_py = rem_to_px(0.125);
    // Inner container padding and gap between segments: 0.125rem (matches Svelte gap/padding).
    let inner = rem_to_px(0.125);

    let selected_fill = resolve_color(theme, spec.selected_fill_token());
    let bg = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let radius = resolve_radius(theme, "radius.control");
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

    let selected = spec.current_value();

    let mut el = ui_element::div()
        .bg(bg)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center()
        .p(inner)
        .gap(inner)
        .h(height);

    for option in &spec.options {
        let is_selected = selected == Some(option.value.as_str());
        // Per-segment disabled only when the individual option is disabled but
        // the whole control is not (the container handles the whole-control case).
        let is_option_disabled = !spec.is_disabled && option.is_disabled;

        let text_color = if is_selected { text_inverse } else { text_muted };

        let mut seg = ui_element::button(&option.label)
            .text_size(font_size).text_weight(600)
            .text_color(text_color)
            .pl(seg_px).pr(seg_px).pt(seg_py).pb(seg_py)
            .rounded(radius)
            .focusable();

        if is_selected {
            seg = seg.bg(selected_fill);
        }

        if spec.equal_width {
            seg = seg.grow();
        }

        if is_option_disabled {
            seg = seg.opacity(disabled_opacity).disabled(true);
        }

        el = el.child(seg);
    }

    // Whole-control disabled: wrap in an additional opacity layer so the border
    // and background also dim, not just the individual segments.
    if spec.is_disabled {
        el = el.opacity(disabled_opacity);
    }

    el
}
