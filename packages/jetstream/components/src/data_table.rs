//! DataTable — Jetstream data table backed by DataTableSpec.
//!
//! Contract: `docs/contracts/components/data-table.md`
//! Svelte reference: `packages/svelte/components/src/DataTable.svelte`
//!
//! Anatomy covered (contract §3): Toolbar (export + column-visibility buttons),
//! Header row (select-all + column labels + active sort icon + actions header),
//! Filter chip row, Body rows (selection checkbox + cells + row striping/selected
//! tint + status-pill cells via `cell_tones` + row-actions cell), expanded rows,
//! empty state, and the pagination Footer (summary + prev/next controls).
//!
//! Preview-loop / accepted limits:
//! - Sort, select, column-visibility, export, filter edits, row actions, and
//!   pagination clicks are wired in the preview event loop, not the component.
//!   Toolbar/filter/pager render the chrome; interaction is host-owned (matches
//!   the GPUI build and the contract's host-ownership note).
//! - No ARIA channel (`<table>` semantics / `aria-sort` / `aria-selected`).
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CheckboxSpec, CheckState, DataTableSpec, StatusTone, TableSortDirection};

use crate::checkbox::js_checkbox;
use crate::presentation::{
    control_space_x_rem, data_table_actions_width_rem, data_table_selection_width_rem,
    panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::skeleton::js_skeleton;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius, tint};

pub fn js_data_table(spec: &DataTableSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let header_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let body_font = rem_to_px(size_font_rem(effective_size));
    let label_size = resolve_px(theme, "typography.label.size");
    let cell_gap = rem_to_px(0.5);
    let cell_px = rem_to_px(control_space_x_rem(spec.density));
    let header_py = rem_to_px(panel_space_y_rem(spec.density));
    let row_py = rem_to_px(panel_space_y_rem(spec.density) - 0.125);
    let gap_sm = resolve_px(theme, "space.inline.sm");
    let gap_md = resolve_px(theme, "space.inline.md");
    let icon_sm = resolve_px(theme, "size.icon.sm");
    // Contract §11 column widths (size-table selection, fixed 3.5rem actions).
    let selection_width = rem_to_px(data_table_selection_width_rem(effective_size));
    let actions_width = rem_to_px(data_table_actions_width_rem());

    let fill = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let border_default = resolve_color(theme, "color.border.default");
    let header_fill = resolve_color(theme, spec.header_fill_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let surface = resolve_color(theme, "color.background.surface");
    let success = resolve_color(theme, "color.status.success");
    let warning = resolve_color(theme, "color.status.warning");
    let danger = resolve_color(theme, "color.status.danger");
    let radius = resolve_radius(theme, "radius.surface");
    let radius_control = resolve_radius(theme, "radius.control");
    let radius_pill = resolve_radius(theme, "radius.pill");
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

    // ── Toolbar (export + column-visibility) ──────────────────────
    // Contract §9: rendered above the table when showExport or
    // showColumnVisibility is set. Buttons render chrome; the export/popover
    // behaviour is host-owned (preview loop).
    if spec.show_export || spec.show_column_visibility {
        let toolbar_btn = |icon_name: &str, text: &str| -> JsEl {
            ui_element::div()
                .flex_row().items_center().gap(gap_sm)
                .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
                .border(1.0).border_color(border_default)
                .rounded(radius_control)
                .cursor_pointer().focusable()
                .child(
                    ui_element::icon(icon_name)
                        .w(icon_sm).h(icon_sm)
                        .text_color(text_secondary),
                )
                .child(
                    ui_element::label(text)
                        .text_color(text_secondary)
                        .text_size(label_size),
                )
        };

        let mut toolbar = ui_element::div()
            .bg(header_fill)
            .flex_row().items_center().justify_end().gap(gap_md)
            .pl(cell_px).pr(cell_px).pt(header_py).pb(header_py)
            .border_b_1().border_color(border);

        if spec.show_export {
            toolbar = toolbar.child(toolbar_btn("download", "Export"));
        }
        if spec.show_column_visibility {
            toolbar = toolbar.child(toolbar_btn("columns-3", "Columns"));
        }
        el = el.child(toolbar);
    }

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
        header = header.child(
            ui_element::div().w(selection_width).flex_row().items_center()
                .child(js_checkbox(&cb_spec, theme)),
        );
    }

    for col in spec.visible_columns() {
        let is_sorted = spec.sort_column_id.as_deref() == Some(&*col.id);

        let mut col_cell = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.25))
            .grow();
        if col.align_end {
            col_cell = col_cell.justify_end();
        }

        col_cell = col_cell.child(
            ui_element::label(&col.label)
                .text_color(text_secondary)
                .text_size(header_font)
                .text_weight(600)
        );

        // Active sort column shows an arrow icon (size from size.icon.sm).
        // Svelte renders NO indicator on unsorted columns — match that.
        if col.is_sortable {
            if is_sorted {
                let icon_name = match spec.sort_direction {
                    TableSortDirection::Asc => "arrow-up",
                    TableSortDirection::Desc => "arrow-down",
                };
                col_cell = col_cell.child(
                    ui_element::icon(icon_name)
                        .w(icon_sm)
                        .h(icon_sm)
                        .text_color(accent)
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

    // Actions column header (fixed 3.5rem) when row actions are shown.
    if spec.show_row_actions {
        header = header.child(
            ui_element::div().w(actions_width).flex_row().justify_end()
                .child(
                    ui_element::label("Actions")
                        .text_color(text_secondary)
                        .text_size(header_font)
                        .text_weight(600),
                ),
        );
    }

    el = el.child(header);

    // ── Filter chip row ───────────────────────────────────────────
    // Contract §3 Filter Row. Active filters render as accent chips; the
    // per-column filter inputs are host-owned (preview loop).
    if spec.has_filters() {
        let mut filter_row = ui_element::div()
            .bg(header_fill)
            .flex_row().flex_wrap().gap(gap_sm)
            .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
            .border_b_1().border_color(border);
        for filter in &spec.filters {
            filter_row = filter_row.child(
                ui_element::label(&format!("{}: {}", filter.column_id, filter.value))
                    .text_color(accent)
                    .text_size(label_size)
                    .bg(tint(accent, 0.12))
                    .rounded(radius_pill)
                    .pl(gap_sm).pr(gap_sm).pt(rem_to_px(0.1875)).pb(rem_to_px(0.1875)),
            );
        }
        el = el.child(filter_row);
    }

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
                .flex_row().items_center().gap(cell_gap)
                .pl(cell_px).pr(cell_px).pt(row_py).pb(row_py)
                .border(1.0).border_color(border)
                .bg(row_bg);

            // Row-selection checkbox
            if spec.selectable {
                let cb_spec = CheckboxSpec::new().with_checked(is_selected);
                row_el = row_el.child(
                    ui_element::div().w(selection_width).flex_row().items_center()
                        .child(js_checkbox(&cb_spec, theme)),
                );
            }

            for col in &visible_cols {
                let value = row
                    .cells
                    .iter()
                    .find(|(k, _)| k == &col.id)
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("—");

                // Status-pill cell (Svelte custom-cell pattern via cell_tones).
                if let Some(tone) = row.cell_tone_for(&col.id) {
                    let (fg, base) = match tone {
                        StatusTone::Success => (success, success),
                        StatusTone::Warning => (warning, warning),
                        StatusTone::Danger => (danger, danger),
                        StatusTone::Info | StatusTone::Pending => (accent, accent),
                        StatusTone::Neutral => (text_secondary, text_secondary),
                    };
                    let pill_bg = color_mix(base, surface, 0.14);
                    let mut cell = ui_element::div().grow().flex_row();
                    if col.align_end {
                        cell = cell.justify_end();
                    }
                    row_el = row_el.child(
                        cell.child(
                            ui_element::label(value)
                                .text_color(fg)
                                .text_size(label_size)
                                .text_weight(600)
                                .bg(pill_bg)
                                .rounded(radius_pill)
                                .pl(gap_sm).pr(gap_sm).pt(rem_to_px(0.125)).pb(rem_to_px(0.125)),
                        ),
                    );
                } else {
                    let mut cell = ui_element::label(value)
                        .text_color(text_primary)
                        .text_size(body_font)
                        .grow();
                    if col.align_end {
                        cell = cell.text_right();
                    }
                    row_el = row_el.child(cell);
                }
            }

            // Row-actions cell (fixed 3.5rem) — legacy single action label.
            if spec.show_row_actions {
                row_el = row_el.child(
                    ui_element::div().w(actions_width).flex_row().justify_end()
                        .child(
                            ui_element::label(&spec.row_action_label)
                                .text_color(accent)
                                .text_size(label_size)
                                .cursor_pointer()
                                .focusable(),
                        ),
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

    // ── Pagination footer ─────────────────────────────────────────
    // Contract §3 Footer: summary + (limit selector host-owned) + prev/next
    // controls. Pager clicks are wired in the preview loop.
    if let Some(pagination) = spec.pagination {
        let summary = format!(
            "{}\u{2013}{} of {}",
            pagination.first_item(),
            pagination.last_item(),
            pagination.total
        );
        let total_pages = pagination.total_pages();
        let page_label = format!("Page {} of {}", pagination.page, total_pages);

        let pager_btn = |text: &str, enabled: bool| -> JsEl {
            let color = if enabled { accent } else { tint(text_secondary, 0.4) };
            let mut b = ui_element::label(text)
                .text_color(color)
                .text_size(label_size)
                .border(1.0).border_color(border_default)
                .rounded(radius_control)
                .pl(gap_sm).pr(gap_sm).pt(rem_to_px(0.25)).pb(rem_to_px(0.25));
            if enabled {
                b = b.cursor_pointer().focusable();
            }
            b
        };

        let controls = ui_element::div()
            .flex_row().items_center().gap(gap_md)
            .child(pager_btn("Prev", pagination.page > 1))
            .child(
                ui_element::label(&page_label)
                    .text_color(text_secondary)
                    .text_size(label_size),
            )
            .child(pager_btn("Next", pagination.page < total_pages));

        el = el.child(
            ui_element::div()
                .bg(header_fill)
                .flex_row().items_center().justify_between()
                .pl(cell_px).pr(cell_px).pt(header_py).pb(header_py)
                .border_t_1().border_color(border)
                .child(
                    ui_element::label(&summary)
                        .text_color(text_secondary)
                        .text_size(label_size),
                )
                .child(controls),
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{TableColumnSpec, TableFilter, TablePagination, TableRowSpec};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn columns() -> Vec<TableColumnSpec> {
        vec![
            TableColumnSpec::new("name", "Name").with_sortable(true),
            TableColumnSpec::new("owner", "Owner").with_sortable(true),
            TableColumnSpec::new("status", "Status"),
        ]
    }

    fn rows() -> Vec<TableRowSpec> {
        vec![
            TableRowSpec::new(
                "r1",
                vec![
                    ("name".into(), "Alpha".into()),
                    ("owner".into(), "Ada".into()),
                    ("status".into(), "Active".into()),
                ],
            ),
            TableRowSpec::new(
                "r2",
                vec![
                    ("name".into(), "Beta".into()),
                    ("owner".into(), "Linus".into()),
                    ("status".into(), "Paused".into()),
                ],
            ),
        ]
    }

    #[test]
    fn renders_header_labels_and_body_cells() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows());
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        for label in ["Name", "Owner", "Status"] {
            assert!(tree.has_text(label), "header label {label} missing: {:?}", tree.texts());
        }
        for value in ["Alpha", "Ada", "Active", "Beta", "Linus", "Paused"] {
            assert!(tree.has_text(value), "cell {value} missing: {:?}", tree.texts());
        }
    }

    #[test]
    fn active_sort_column_renders_an_arrow_icon() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows())
            .with_sort("name", TableSortDirection::Desc);
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        // Active sort column renders the arrow icon; unsorted ones do not.
        assert!(tree.has_text("arrow-down"), "active sort icon missing: {:?}", tree.texts());
        assert_eq!(tree.count_kind("Icon"), 1, "exactly one sort icon expected");
        // No neutral ⇅ glyph (removed to match Svelte).
        assert!(!tree.has_text("\u{21c5}"), "neutral sort glyph should be gone");
    }

    #[test]
    fn selectable_renders_select_all_and_row_checkboxes() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows())
            .with_selectable(true)
            .with_selected_row_ids(vec!["r1".into()]);
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        // Header select-all + one checkbox per row = 3 checkbox panels minimum.
        // Checkboxes compose js_checkbox; assert the table laid out with selection.
        assert!(!tree.is_empty());
        // Selected-row accent tint is present somewhere in the tree.
        let selected = tint(resolve_color(&th, "color.accent.base"), 0.08);
        let expect = crate::render_probe::ProbeColor {
            r: selected.x, g: selected.y, b: selected.z, a: selected.w,
        };
        assert!(tree.has_background(expect, 0.02), "selected-row tint missing");
    }

    #[test]
    fn toolbar_renders_export_and_columns_buttons() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows())
            .with_show_export(true)
            .with_show_column_visibility(true);
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        assert!(tree.has_text("Export"), "export button missing: {:?}", tree.texts());
        assert!(tree.has_text("Columns"), "columns button missing: {:?}", tree.texts());
    }

    #[test]
    fn filter_chips_render_active_filters() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows())
            .with_filters(vec![TableFilter::new("status", "Active")]);
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        assert!(
            tree.has_text("status: Active"),
            "filter chip missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn pagination_footer_renders_summary_and_controls() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows())
            .with_pagination(TablePagination::new(2, 10, 35));
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        assert!(tree.has_text("11\u{2013}20 of 35"), "summary missing: {:?}", tree.texts());
        assert!(tree.has_text("Page 2 of 4"), "page label missing: {:?}", tree.texts());
        assert!(tree.has_text("Prev"), "prev control missing");
        assert!(tree.has_text("Next"), "next control missing");
    }

    #[test]
    fn row_actions_column_renders_action_label() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), rows())
            .with_show_row_actions(true)
            .with_row_action_label("Open");
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        assert!(tree.has_text("Actions"), "actions header missing: {:?}", tree.texts());
        // One "Open" label per row.
        let opens = tree.texts().iter().filter(|t| **t == "Open").count();
        assert_eq!(opens, 2, "expected one action label per row");
    }

    #[test]
    fn empty_state_renders_message() {
        let th = theme();
        let spec = DataTableSpec::new(columns(), vec![])
            .with_empty_message("No team members match the current filters.");
        let tree = probe(&js_data_table(&spec, &th), 700.0, 400.0);

        assert!(
            tree.has_text("No team members match the current filters."),
            "empty message missing: {:?}",
            tree.texts()
        );
    }
}
