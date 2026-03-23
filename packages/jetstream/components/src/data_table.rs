//! DataTable — Jetstream data table backed by DataTableSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_composites::DataTableSpec;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_data_table(spec: &DataTableSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let header_fill = resolve_color(theme, spec.header_fill_token());
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .overflow_hidden();

    // Header row
    let mut header = ui_element::div()
        .bg(header_fill)
        .flex_row().gap(8.0)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0);

    for col in &spec.columns {
        header = header.child(
            ui_element::label(&col.label)
                .text_color(text_secondary).text_size(12.0).text_weight(600)
                .grow()
        );
    }
    el = el.child(header);

    // Data rows
    for row in &spec.rows {
        let mut row_el = ui_element::div()
            .flex_row().gap(8.0)
            .pl(12.0).pr(12.0).pt(6.0).pb(6.0)
            .border(1.0).border_color(border);

        for (_key, value) in &row.cells {
            row_el = row_el.child(
                ui_element::label(value)
                    .text_color(text_primary).text_size(13.0)
                    .grow()
            );
        }
        el = el.child(row_el);
    }

    el
}
