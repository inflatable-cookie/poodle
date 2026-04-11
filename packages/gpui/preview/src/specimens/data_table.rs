use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_specs::{
    DataTableSpec, TableColumnSpec, TableFilter, TablePagination, TableRowSpec,
    TableSortDirection,
};
use poodle_specs::{ControlSize, EyebrowSpec, StatusTone};
use poodle_gpui_components::{DataTable, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

fn make_columns() -> Vec<TableColumnSpec> {
    vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("email", "Email").with_sortable(true),
        TableColumnSpec::new("role", "Role").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
    ]
}

fn make_rows() -> Vec<TableRowSpec> {
    vec![
        TableRowSpec::new("1", vec![
            ("name".into(), "Alice Chen".into()),
            ("email".into(), "alice@example.com".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("2", vec![
            ("name".into(), "Bob Martinez".into()),
            ("email".into(), "bob@example.com".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("3", vec![
            ("name".into(), "Carol Patel".into()),
            ("email".into(), "carol@example.com".into()),
            ("role".into(), "PM".into()),
            ("status".into(), "On leave".into()),
        ]),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("email", "Email").with_sortable(true),
        TableColumnSpec::new("role", "Role").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
    ];

    let rows = vec![
        TableRowSpec::new("1", vec![
            ("name".into(), "Alice Chen".into()),
            ("email".into(), "alice@example.com".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("2", vec![
            ("name".into(), "Bob Martinez".into()),
            ("email".into(), "bob@example.com".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("3", vec![
            ("name".into(), "Carol Patel".into()),
            ("email".into(), "carol@example.com".into()),
            ("role".into(), "PM".into()),
            ("status".into(), "On leave".into()),
        ]),
        TableRowSpec::new("4", vec![
            ("name".into(), "Dan Okoro".into()),
            ("email".into(), "dan@example.com".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("5", vec![
            ("name".into(), "Eve Nakamura".into()),
            ("email".into(), "eve@example.com".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
        ]),
    ];

    // Track sort state
    let sort_col = state.specimens.text.get("dt-sort-col")
        .cloned()
        .unwrap_or_else(|| "name".to_string());
    let sort_dir_str = state.specimens.text.get("dt-sort-dir")
        .cloned()
        .unwrap_or_else(|| "asc".to_string());
    let sort_dir = if sort_dir_str == "desc" {
        TableSortDirection::Desc
    } else {
        TableSortDirection::Asc
    };

    // Track last action
    let last_action = state.specimens.text.get("dt-last-action")
        .cloned()
        .unwrap_or_default();

    // Track selected rows
    let selected_ids: Vec<String> = (1..=5)
        .filter(|i| state.specimens.is_on(&format!("dt-row-{}", i)))
        .map(|i| format!("{}", i))
        .collect();
    let selected_count = selected_ids.len();

    let empty_columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("email", "Email").with_sortable(true),
        TableColumnSpec::new("role", "Role").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- With sorting, column visibility, and export ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With sorting, column visibility, and export"), theme))
                .child(
                    DataTable::from_spec(
                        DataTableSpec::new(columns, rows)
                            .with_selected_row_ids(selected_ids)
                            .with_sort(&sort_col, sort_dir)
                            .with_row_action_label("Open"),
                        theme,
                    )
                    .on_sort(cx.listener(|this, col_id: &str, _w, cx| {
                        let current_col = this.state.specimens.text.get("dt-sort-col")
                            .cloned()
                            .unwrap_or_else(|| "name".to_string());
                        let current_dir = this.state.specimens.text.get("dt-sort-dir")
                            .cloned()
                            .unwrap_or_else(|| "asc".to_string());

                        if col_id == current_col {
                            // Toggle direction
                            let new_dir = if current_dir == "asc" { "desc" } else { "asc" };
                            this.state.specimens.text.insert("dt-sort-dir".to_string(), new_dir.to_string());
                        } else {
                            this.state.specimens.text.insert("dt-sort-col".to_string(), col_id.to_string());
                            this.state.specimens.text.insert("dt-sort-dir".to_string(), "asc".to_string());
                        }
                        cx.notify();
                    }))
                    .on_row_click(cx.listener(|this, row_id: &str, _w, cx| {
                        this.state.specimens.toggle(&format!("dt-row-{}", row_id));
                        this.state.specimens.text.insert(
                            "dt-last-action".to_string(),
                            format!("Clicked row {}", row_id),
                        );
                        cx.notify();
                    }))
                )
        )
        // Status line
        .when(!last_action.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(last_action)
            )
        })
        .child(
            div().text_sm().text_color(color_to_hsla(text_secondary))
                .child(format!("{} of 5 selected", selected_count))
        )

        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let sizes: &[(&str, ControlSize)] = &[
                        ("xs", ControlSize::Xs),
                        ("sm", ControlSize::Sm),
                        ("md", ControlSize::Md),
                        ("lg", ControlSize::Lg),
                        ("xl", ControlSize::Xl),
                    ];
                    let mut col = div().flex().flex_col().gap(px(16.0));
                    for &(key, size) in sizes {
                        col = col
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("size = {}", key))
                            )
                            .child(
                                DataTable::from_spec(
                                    DataTableSpec::new(make_columns(), make_rows())
                                        .with_aria_label(format!("Data table at {}", key))
                                        .with_size(size),
                                    theme,
                                )
                            );
                    }
                    col
                })
        )
        // --- With filters and pagination (compact + striped + sticky header) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With filters and pagination"), theme))
                .child(
                    DataTable::from_spec(
                        DataTableSpec::new(make_columns(), make_rows())
                            .with_filters(vec![
                                TableFilter::new("name", "Alice"),
                                TableFilter::new("role", "Engineer"),
                            ])
                            .with_pagination(TablePagination::new(1, 10, 42))
                            .with_compact(true)
                            .with_striped(true)
                            .with_sticky_header(true)
                            .with_show_row_actions(false)
                            .with_aria_label("Directory table"),
                        theme,
                    )
                )
        )
        // --- With column visibility, export, and row selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Column visibility + export + row selection"), theme))
                .child(
                    DataTable::from_spec(
                        DataTableSpec::new(make_columns(), make_rows())
                            .with_selectable(true)
                            .with_selected_row_ids(vec!["1".to_string(), "3".to_string()])
                            .with_hidden_column_ids(vec!["email".to_string()])
                            .with_show_column_visibility(true)
                            .with_show_export(true)
                            .with_show_row_actions(false)
                            .with_aria_label("Team members"),
                        theme,
                    )
                )
        )
        // --- Custom cells and expanded rows ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom cells and expanded rows"), theme))
                .child({
                    let incident_columns = vec![
                        TableColumnSpec::new("status", "Status")
                            .with_width_rem(7.0),
                        TableColumnSpec::new("endpoint", "Endpoint"),
                        TableColumnSpec::new("owner", "Owner")
                            .with_width_rem(10.0),
                    ];

                    let incident_rows = vec![
                        TableRowSpec::new("incident-1", vec![
                            ("status".into(), "Open".into()),
                            ("endpoint".into(), "POST /api/orders".into()),
                            ("owner".into(), "Alice".into()),
                        ])
                        .with_summary("Active incident — last updated 2026-03-27 11:18 UTC.")
                        .with_cell_tone("status", StatusTone::Danger),
                        TableRowSpec::new("incident-2", vec![
                            ("status".into(), "Resolved".into()),
                            ("endpoint".into(), "GET /api/catalog".into()),
                            ("owner".into(), "Bob".into()),
                        ])
                        .with_summary("Resolved 2026-03-27 09:42 UTC — rollback completed.")
                        .with_cell_tone("status", StatusTone::Success),
                    ];

                    DataTable::from_spec(
                        DataTableSpec::new(incident_columns, incident_rows)
                            .with_expanded_row_ids(vec!["incident-1".to_string()])
                            .with_show_row_actions(false)
                            .with_aria_label("Active incidents"),
                        theme,
                    )
                })
        )
        // --- Empty state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty state"), theme))
                .child(
                    DataTable::from_spec(
                        DataTableSpec::new(empty_columns, vec![])
                            .with_empty_message("No team members match the current filters."),
                        theme,
                    )
                )
        )
}
