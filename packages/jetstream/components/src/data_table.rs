//! DataTable — Jetstream data table backed by DataTableSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CheckboxSpec, CheckState, DataTableSpec, TableSortDirection};

use crate::checkbox::js_checkbox;
use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::skeleton::js_skeleton;
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_data_table(spec: &DataTableSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let header_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
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
    let accent = resolve_color(theme, "color.accent.base");
    let surface = resolve_color(theme, "color.background.surface");
    let radius = resolve_radius(theme, "radius.surface");
    let header_hover = tint(accent, 0.10);

    // Zebra stripe tint for even-indexed rows
    let stripe_tint = tint(surface, 0.04);
    // Selected row tint (accent at 8% opacity — mirrors Svelte `color-mix 8%`)
    let selected_tint = tint(accent, 0.08);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col()
        .overflow_hidden();

    // ── Header row ────────────────────────────────────────────────
    let mut header = ui_element::div()
        .bg(header_fill)
        .flex_row().gap(cell_gap)
        .pl(cell_px).pr(cell_px).pt(header_py).pb(header_py);

    // Selectable: "select all" checkbox column header
    if spec.selectable {
        let select_all_state = spec.select_all_state();
        let cb_spec = match select_all_state {
            CheckState::Checked => CheckboxSpec::new().with_checked(true),
            CheckState::Mixed => CheckboxSpec::new().with_mixed(true),
            CheckState::Unchecked => CheckboxSpec::new(),
        };
        header = header.child(js_checkbox(&cb_spec, theme));
    }

    for col in spec.visible_columns() {
        let is_sorted = spec.sort_column_id.as_deref() == Some(&*col.id);

        let mut col_cell = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.25))
            .grow();

        col_cell = col_cell.child(
            ui_element::label(&col.label)
                .text_color(text_secondary)
                .text_size(header_font)
                .text_weight(600)
        );

        if col.is_sortable {
            if is_sorted {
                let icon_name = match spec.sort_direction {
                    TableSortDirection::Asc => "arrow-up",
                    TableSortDirection::Desc => "arrow-down",
                };
                col_cell = col_cell.child(
                    ui_element::icon(icon_name)
                        .w(rem_to_px(0.75))
                        .h(rem_to_px(0.75))
                        .text_color(accent)
                );
            } else {
                // Unsorted sortable column: neutral sort indicator
                col_cell = col_cell.child(
                    ui_element::label("⇅")
                        .text_color(text_secondary)
                        .text_size(header_font)
                );
            }
            // Sortable columns show a pointer cursor and accent tint on hover.
            col_cell = col_cell
                .cursor_pointer()
                .focusable()
                .hover(|s| s.bg(header_hover));
        }

        header = header.child(col_cell);
    }

    el = el.child(header);

    // ── Body ──────────────────────────────────────────────────────
    if spec.rows.is_empty() {
        // Empty state
        let empty_msg = spec
            .empty_message
            .as_deref()
            .unwrap_or("No results");
        el = el.child(
            ui_element::div()
                .flex_row().items_center().justify_center()
                .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
                .child(
                    ui_element::label(empty_msg)
                        .text_color(text_secondary)
                        .text_size(body_font)
                )
        );
    } else {
        let visible_cols: Vec<_> = spec.visible_columns().collect();

        for (row_index, row) in spec.rows.iter().enumerate() {
            let is_selected = spec.selected_row_ids.iter().any(|id| id == &row.id);

            // Row background: selected > default/stripe
            let row_bg = if is_selected {
                selected_tint
            } else if spec.striped && row_index % 2 == 0 {
                stripe_tint
            } else {
                // Transparent — use a zero-alpha version of surface
                tint(surface, 0.0)
            };

            let mut row_el = ui_element::div()
                .flex_row().gap(cell_gap)
                .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
                .border(1.0).border_color(border)
                .bg(row_bg);

            // Row-selection checkbox
            if spec.selectable {
                let cb_spec = CheckboxSpec::new().with_checked(is_selected);
                row_el = row_el.child(js_checkbox(&cb_spec, theme));
            }

            for col in &visible_cols {
                let value = row
                    .cells
                    .iter()
                    .find(|(k, _)| k == &col.id)
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("—");

                row_el = row_el.child(
                    ui_element::label(value)
                        .text_color(text_primary)
                        .text_size(body_font)
                        .grow()
                );
            }

            el = el.child(row_el);

            // Expanded row summary
            if spec.is_row_expanded(&row.id) {
                if let Some(ref summary) = row.summary {
                    el = el.child(
                        ui_element::div()
                            .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
                            .border(1.0).border_color(border)
                            .child(
                                ui_element::label(summary)
                                    .text_color(text_secondary)
                                    .text_size(body_font)
                            )
                    );
                }
            }
        }
    }

    el
}

/// Render a loading skeleton for the data table body (used when data is in-flight).
/// Renders `row_count` skeleton rows, each with a skeleton per visible column.
pub fn js_data_table_loading(
    spec: &DataTableSpec,
    theme: &JetstreamThemeProvider,
    row_count: usize,
) -> JsEl {
    use poodle_specs::SkeletonSpec;

    let cell_gap = rem_to_px(0.5);
    let cell_px = rem_to_px(control_space_x_rem(spec.density));
    let row_py = rem_to_px(panel_space_y_rem(spec.density) - 0.125);
    let border = resolve_color(theme, "color.border.subtle");
    let skel_spec = SkeletonSpec::new();

    let mut el = ui_element::div().flex_col();

    for _ in 0..row_count {
        let mut row_el = ui_element::div()
            .flex_row().gap(cell_gap).items_center()
            .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
            .border(1.0).border_color(border);

        if spec.selectable {
            row_el = row_el.child(js_skeleton(&skel_spec, theme));
        }
        for _ in spec.visible_columns() {
            row_el = row_el.child(js_skeleton(&skel_spec, theme).grow());
        }
        el = el.child(row_el);
    }

    el
}
