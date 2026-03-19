use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{DataTableSpec, TableColumnSpec, TableRowSpec, TableSortDirection};
use pug_gpui_components::PugDataTable;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
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

    let selected_idx = state.specimens.selected("table-row");
    let selected_ids = if selected_idx < rows.len() {
        vec![format!("{}", selected_idx + 1)]
    } else {
        vec![]
    };

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
            PugDataTable::new(
                DataTableSpec::new(columns, rows)
                    .with_selected_row_ids(selected_ids)
                    .with_sort("name", TableSortDirection::Asc)
                    .with_row_action_label("Open"),
                theme,
            )
        )
        // --- Empty state ---
        .child(section_label("EMPTY STATE", text_secondary))
        .child(
            PugDataTable::new(
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
