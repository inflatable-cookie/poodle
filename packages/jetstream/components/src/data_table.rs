//! DataTable — Jetstream data table backed by DataTableSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::DataTableSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_data_table(spec: &DataTableSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let header_font = rem_to_px(size_font_rem(effective_size) - 0.0625); // headers slightly smaller
    let body_font = rem_to_px(size_font_rem(effective_size));
    let cell_gap = rem_to_px(0.5);
    let cell_px = rem_to_px(control_space_x_rem(spec.density));
    let header_py = rem_to_px(panel_space_y_rem(spec.density));
    let row_py = rem_to_px(panel_space_y_rem(spec.density) - 0.125);

    let fill = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let header_fill = resolve_color(theme, spec.header_fill_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let radius = resolve_radius(theme, "radius.surface");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .overflow_hidden();

    // Header row
    let mut header = ui_element::div()
        .bg(header_fill)
        .flex_row().gap(cell_gap)
        .pl(cell_px).pr(cell_px).pt(header_py).pb(header_py);

    for col in &spec.columns {
        header = header.child(
            ui_element::label(&col.label)
                .text_color(text_secondary).text_size(header_font).text_weight(600)
                .grow()
        );
    }
    el = el.child(header);

    // Data rows
    for row in &spec.rows {
        let mut row_el = ui_element::div()
            .flex_row().gap(cell_gap)
            .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
            .border(1.0).border_color(border);

        for (_key, value) in &row.cells {
            row_el = row_el.child(
                ui_element::label(value)
                    .text_color(text_primary).text_size(body_font)
                    .grow()
            );
        }
        el = el.child(row_el);
    }

    el
}
