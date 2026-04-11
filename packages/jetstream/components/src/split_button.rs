//! SplitButton — Jetstream split button backed by SplitButtonSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SplitButtonSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_split_button(spec: &SplitButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let separator_h = rem_to_px(control_height_rem(effective_size) * 0.56);
    let trigger_px = rem_to_px(0.375);

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let separator = resolve_color(theme, spec.separator_token());
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");

    let label = spec.label.as_deref().unwrap_or("");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center()
        .h(height);

    // Primary action
    el = el.child(
        ui_element::button(label)
            .text_color(text_color).text_size(font_size).text_weight(500)
            .pl(pad_x).pr(pad_x)
            .focusable()
    );

    // Separator
    el = el.child(ui_element::div().w(1.0).h(separator_h).bg(separator));

    // Dropdown trigger
    el = el.child(
        ui_element::button("")
            .child(ui_element::icon("chevron-down").w(icon_size).h(icon_size).text_color(text_color))
            .pl(trigger_px).pr(trigger_px)
            .focusable()
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
