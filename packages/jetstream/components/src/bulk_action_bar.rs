//! BulkActionBar — Jetstream bulk action bar backed by BulkActionBarSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::BulkActionBarSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_bulk_action_bar(spec: &BulkActionBarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density) - 0.375);
    let gap = rem_to_px(0.5);
    let btn_px = rem_to_px(control_space_x_rem(spec.density));

    let fill = resolve_color(theme, "color.background.elevated");
    let text_color = resolve_color(theme, "color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .flex_row().items_center().gap(gap)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y);

    // Selected count
    let count_text = format!("{} selected", spec.selection_count);
    el = el.child(ui_element::label(&count_text).text_color(text_color).text_size(font_size).text_weight(500));

    // Actions
    for action in &spec.actions {
        el = el.child(
            ui_element::button(&action.label)
                .text_color(text_color).text_size(font_size)
                .pl(btn_px).pr(btn_px)
                .focusable()
        );
    }

    el
}
