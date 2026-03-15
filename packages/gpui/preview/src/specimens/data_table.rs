use gpui::*;
use pug_gpui_composites::{DataTableSpec, TableColumnSpec, TableRowSpec, TableSortDirection};
use pug_gpui_components::PugDataTable;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let selected = state.specimens.selected("table-row");

    let columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
        TableColumnSpec::new("actions", "Actions"),
    ];

    let rows = vec![
        TableRowSpec::new("0", vec![
            ("name".into(), "Project Alpha".into()),
            ("status".into(), "Active".into()),
            ("actions".into(), "Edit".into()),
        ]),
        TableRowSpec::new("1", vec![
            ("name".into(), "Project Beta".into()),
            ("status".into(), "Draft".into()),
            ("actions".into(), "Edit".into()),
        ]),
        TableRowSpec::new("2", vec![
            ("name".into(), "Project Gamma".into()),
            ("status".into(), "Archived".into()),
            ("actions".into(), "Edit".into()),
        ]),
    ];

    let selected_ids = if selected < rows.len() {
        vec![format!("{}", selected)]
    } else {
        vec![]
    };

    let spec = DataTableSpec::new(columns, rows)
        .with_selected_row_ids(selected_ids)
        .with_sort("name", TableSortDirection::Asc)
        .with_row_action_label("Edit");

    div().child(PugDataTable::new(spec, theme))
}
