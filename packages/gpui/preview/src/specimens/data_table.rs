use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_composites::{DataTableSpec, TableColumnSpec, TableRowSpec, TableSortDirection};
use pug_gpui_components::DataTable;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("email", "Email").with_sortable(true),
        TableColumnSpec::new("role", "Role").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
    ];

    let rows = vec![
        TableRowSpec::new("1", vec![
            ("name".into(), "Alice Johnson".into()),
            ("email".into(), "alice@example.com".into()),
            ("role".into(), "Admin".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("2", vec![
            ("name".into(), "Bob Smith".into()),
            ("email".into(), "bob@example.com".into()),
            ("role".into(), "Editor".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("3", vec![
            ("name".into(), "Carol Davis".into()),
            ("email".into(), "carol@example.com".into()),
            ("role".into(), "Viewer".into()),
            ("status".into(), "Invited".into()),
        ]),
        TableRowSpec::new("4", vec![
            ("name".into(), "Dan Lee".into()),
            ("email".into(), "dan@example.com".into()),
            ("role".into(), "Editor".into()),
            ("status".into(), "Active".into()),
        ]),
        TableRowSpec::new("5", vec![
            ("name".into(), "Eve Martinez".into()),
            ("email".into(), "eve@example.com".into()),
            ("role".into(), "Admin".into()),
            ("status".into(), "Inactive".into()),
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

    div().flex().flex_col().gap(px(16.0))
        // --- With sorting, column visibility, and export ---
        .child(section_label("WITH SORTING, COLUMN VISIBILITY, AND EXPORT", text_secondary))
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

        // --- Empty state ---
        .child(section_label("EMPTY STATE", text_secondary))
        .child(
            DataTable::from_spec(
                DataTableSpec::new(empty_columns, vec![])
                    .with_empty_message("No team members match the current filters."),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
