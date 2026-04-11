//! SelectionSummary — Jetstream selection summary backed by SelectionSummarySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SelectionSummarySpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_selection_summary(spec: &SelectionSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let chip_px = rem_to_px(control_space_x_rem(spec.density));
    let chip_py = rem_to_px(panel_space_y_rem(spec.density) - 0.625);
    let gap = resolve_px(theme, spec.gap_token());
    let chip_radius = rem_to_px(0.25);

    let text_color = resolve_color(theme, "color.text.primary");
    let chip_bg = resolve_color(theme, "color.background.surface");
    let chip_border = resolve_color(theme, "color.border.subtle");

    let mut el = ui_element::div().flex_row().items_center().gap(gap).flex_wrap();

    for item in &spec.items {
        el = el.child(
            ui_element::label(&item.label)
                .text_color(text_color).text_size(font_size)
                .pl(chip_px).pr(chip_px).pt(chip_py).pb(chip_py)
                .rounded(chip_radius)
                .bg(chip_bg)
                .border(1.0).border_color(chip_border)
        );
    }

    el
}
