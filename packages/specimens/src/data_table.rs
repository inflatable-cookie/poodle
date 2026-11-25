use std::sync::Arc;

use poodle_headless::selection::{
    selection_transition, SelectionEvent, SelectionMode, SelectionOptions,
};
use poodle_node::Node;
use poodle_render::{data_table, DataTableHandlers, RenderContext};
use poodle_specs::{
    DataTableSpec, StatusTone, TableColumnSpec, TableFilter, TablePagination, TableRowSpec,
    TableSortDirection,
};

use crate::{column, readout, section, Sink, View};

pub(crate) struct State {
    sort_column: String,
    sort_direction: TableSortDirection,
    selected: Vec<String>,
    last_action: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sort_column: "name".into(),
            sort_direction: TableSortDirection::Asc,
            selected: Vec::new(),
            last_action: None,
        }
    }
}

pub(crate) enum Event {
    Sort(String),
    RowAction(String),
}

pub(crate) fn reduce(mut state: State, event: Event) -> State {
    match event {
        Event::Sort(column) => {
            state.sort_direction =
                if column == state.sort_column && state.sort_direction == TableSortDirection::Asc {
                    TableSortDirection::Desc
                } else {
                    TableSortDirection::Asc
                };
            state.sort_column = column;
        }
        Event::RowAction(id) => {
            state.selected = selection_transition(
                &state.selected,
                SelectionEvent::Toggle {
                    item: &id,
                    disabled: false,
                },
                SelectionOptions {
                    mode: SelectionMode::Multiple,
                    max: None,
                },
                String::as_str,
            );
            state.last_action = Some(format!("Clicked row {id}"));
        }
    }
    state
}

fn columns() -> Vec<TableColumnSpec> {
    vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("email", "Email").with_sortable(true),
        TableColumnSpec::new("role", "Role").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
    ]
}

const PEOPLE: [[&str; 5]; 5] = [
    ["1", "Alice Chen", "alice@example.com", "Engineer", "Active"],
    ["2", "Bob Martinez", "bob@example.com", "Designer", "Active"],
    ["3", "Carol Patel", "carol@example.com", "PM", "On leave"],
    ["4", "Dan Okoro", "dan@example.com", "Engineer", "Active"],
    ["5", "Eve Nakamura", "eve@example.com", "Designer", "Active"],
];

fn rows(count: usize) -> Vec<TableRowSpec> {
    PEOPLE[..count]
        .iter()
        .map(|[id, name, email, role, status]| {
            TableRowSpec::new(
                *id,
                vec![
                    ("name".into(), (*name).into()),
                    ("email".into(), (*email).into()),
                    ("role".into(), (*role).into()),
                    ("status".into(), (*status).into()),
                ],
            )
        })
        .collect()
}

fn sorted_rows(state: &State) -> Vec<TableRowSpec> {
    let mut rows = rows(PEOPLE.len());
    let column = rows[0]
        .cells
        .iter()
        .position(|(id, _)| id == &state.sort_column)
        .expect("sort callbacks only emit fixture columns");
    rows.sort_by(|left, right| {
        let order = left.cells[column].1.cmp(&right.cells[column].1);
        match state.sort_direction {
            TableSortDirection::Asc => order,
            TableSortDirection::Desc => order.reverse(),
        }
    });
    rows
}

fn incidents() -> DataTableSpec {
    let columns = vec![
        TableColumnSpec::new("status", "Status").with_width_rem(7.0),
        TableColumnSpec::new("endpoint", "Endpoint"),
        TableColumnSpec::new("owner", "Owner").with_width_rem(10.0),
    ];
    let rows = vec![
        TableRowSpec::new(
            "incident-1",
            vec![
                ("status".into(), "Open".into()),
                ("endpoint".into(), "POST /api/orders".into()),
                ("owner".into(), "Alice".into()),
            ],
        )
        .with_summary("Active incident — last updated 2026-03-27 11:18 UTC.")
        .with_cell_tone("status", StatusTone::Danger),
        TableRowSpec::new(
            "incident-2",
            vec![
                ("status".into(), "Resolved".into()),
                ("endpoint".into(), "GET /api/catalog".into()),
                ("owner".into(), "Bob".into()),
            ],
        )
        .with_summary("Resolved 2026-03-27 09:42 UTC — rollback completed.")
        .with_cell_tone("status", StatusTone::Success),
    ];
    DataTableSpec::new(columns, rows)
        .with_expanded_row_ids(vec!["incident-1".into()])
        .with_show_row_actions(false)
        .with_aria_label("Active incidents")
}

pub(crate) fn examples(state: &State, sink: Sink<Event>, ctx: &RenderContext<'_>) -> Node {
    let on_sort = Arc::clone(&sink);
    let mut table = data_table(
        &DataTableSpec::new(columns(), sorted_rows(state))
            .with_selected_row_ids(state.selected.clone())
            .with_sort(&state.sort_column, state.sort_direction)
            .with_row_action_label("Open"),
        ctx,
        DataTableHandlers {
            on_sort: Some(Arc::new(move |column| on_sort(Event::Sort(column.into())))),
            on_row_click: Some(Arc::new(move |id| sink(Event::RowAction(id.into())))),
            ..Default::default()
        },
    );
    table.id = Some("specimen-data-table-live".into());

    let show = |spec| data_table(&spec, ctx, DataTableHandlers::default());
    let sorting = section("With sorting, column visibility, and export", [table], ctx);
    let filters = section(
        "With filters and pagination",
        [show(
            DataTableSpec::new(columns(), rows(3))
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
        )],
        ctx,
    );
    let visibility = section(
        "Column visibility + export + row selection",
        [show(
            DataTableSpec::new(columns(), rows(3))
                .with_selectable(true)
                .with_selected_row_ids(vec!["1".into(), "3".into()])
                .with_hidden_column_ids(vec!["email".into()])
                .with_show_column_visibility(true)
                .with_show_export(true)
                .with_show_row_actions(false)
                .with_aria_label("Team members"),
        )],
        ctx,
    );
    let custom = section("Custom cells and expanded rows", [show(incidents())], ctx);
    let empty = section(
        "Empty state",
        [show(
            DataTableSpec::new(columns(), vec![])
                .with_empty_message("No team members match the current filters."),
        )],
        ctx,
    );

    let mut children = vec![sorting];
    if let Some(action) = &state.last_action {
        children.push(readout(action, ctx));
    }
    children.push(readout(
        format!("{} of {} selected", state.selected.len(), PEOPLE.len()),
        ctx,
    ));
    children.extend([filters, visibility, custom, empty]);
    column(24.0, children)
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let spec = DataTableSpec::new(columns(), rows(3)).with_aria_label("Data table");
    let spec = match view {
        View::Size(size) => spec.with_size(size),
        View::Density(density) => spec.with_density(density),
        View::Examples => unreachable!("Examples uses the session"),
    };
    data_table(&spec, ctx, DataTableHandlers::default())
}
