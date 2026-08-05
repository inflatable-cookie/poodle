//! DataTable specimen — full contract state coverage with the real
//! `js_data_table` builder: header + rows, sortable headers (sort indicator),
//! selectable (checkbox column), row striping, toolbar (column-visibility +
//! export), filter chip row, pagination footer, custom status-pill cells +
//! expanded rows, row-action column, loading skeleton, empty state, and the
//! size + density axes. All visuals resolve from tokens.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::{js_data_table, js_data_table_loading};

use poodle_specs::{
    ControlDensity, ControlSize, DataTableSpec, StatusTone, TableColumnSpec, TableFilter,
    TablePagination, TableRowSpec, TableSortDirection,
};

fn columns() -> Vec<TableColumnSpec> {
    vec![
        TableColumnSpec::new("name", "Name").with_sortable(true).with_hideable(false),
        TableColumnSpec::new("email", "Email").with_sortable(true),
        TableColumnSpec::new("role", "Role").with_sortable(true),
        TableColumnSpec::new("status", "Status"),
    ]
}

fn rows() -> Vec<TableRowSpec> {
    vec![
        TableRowSpec::new("1", vec![
            ("name".into(), "Alice Chen".into()),
            ("email".into(), "alice@example.com".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
        ]).with_summary("Senior frontend engineer"),
        TableRowSpec::new("2", vec![
            ("name".into(), "Bob Martinez".into()),
            ("email".into(), "bob@example.com".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
        ]).with_summary("UX lead"),
        TableRowSpec::new("3", vec![
            ("name".into(), "Carol Patel".into()),
            ("email".into(), "carol@example.com".into()),
            ("role".into(), "PM".into()),
            ("status".into(), "On leave".into()),
        ]).with_summary("Product manager"),
        TableRowSpec::new("4", vec![
            ("name".into(), "Dan Okoro".into()),
            ("email".into(), "dan@example.com".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
        ]).with_summary("Backend engineer"),
        TableRowSpec::new("5", vec![
            ("name".into(), "Eve Nakamura".into()),
            ("email".into(), "eve@example.com".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
        ]).with_summary("Visual designer"),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let sizes: &[(&str, ControlSize)] = &[
        ("xs", ControlSize::Xs),
        ("sm", ControlSize::Sm),
        ("md", ControlSize::Md),
        ("lg", ControlSize::Lg),
        ("xl", ControlSize::Xl),
    ];

    let densities: &[(&str, ControlDensity)] = &[
        ("compact", ControlDensity::Compact),
        ("default", ControlDensity::Default),
        ("comfortable", ControlDensity::Comfortable),
    ];

    // Incident table exercising the custom status-pill cell pattern (cell_tones)
    // and host-owned expanded row summaries.
    let incident_columns = vec![
        TableColumnSpec::new("status", "Status").with_width_rem(7.0),
        TableColumnSpec::new("endpoint", "Endpoint"),
        TableColumnSpec::new("owner", "Owner").with_width_rem(10.0),
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

    let mut sizes_col = div().flex_col().gap(16.0);
    for &(key, size) in sizes {
        sizes_col = sizes_col
            .child(label(&format!("size = {}", key)).text_color(secondary).text_size(11.0))
            .child(js_data_table(
                &DataTableSpec::new(columns(), rows())
                    .with_aria_label(format!("Data table at {}", key))
                    .with_size(size),
                theme,
            ));
    }

    let mut density_col = div().flex_col().gap(16.0);
    for &(key, density) in densities {
        density_col = density_col
            .child(label(&format!("density = {}", key)).text_color(secondary).text_size(11.0))
            .child(js_data_table(
                &DataTableSpec::new(columns(), rows())
                    .with_aria_label(format!("Data table at {}", key))
                    .with_density(density),
                theme,
            ));
    }

    div().flex_col().gap(24.0)
        // Sorting + column visibility + export + selection (contract §12)
        .child(group("With sorting, column visibility, and export", secondary,
            js_data_table(
                &DataTableSpec::new(columns(), rows())
                    .with_selectable(true)
                    .with_selected_row_ids(vec!["1".into(), "3".into()])
                    .with_sort("name", TableSortDirection::Asc)
                    .with_show_column_visibility(true)
                    .with_show_export(true)
                    .with_aria_label("Team members"),
                theme,
            )
        ))
        // Sorted descending (sort indicator on active header)
        .child(group("Sorted by Role ↓", secondary,
            js_data_table(
                &DataTableSpec::new(columns(), rows())
                    .with_sort("role", TableSortDirection::Desc)
                    .with_aria_label("Sorted directory"),
                theme,
            )
        ))
        // Filters + pagination footer (compact + striped)
        .child(group("With filters and pagination", secondary,
            js_data_table(
                &DataTableSpec::new(columns(), rows())
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
        ))
        // Custom status-pill cells + expanded row
        .child(group("With custom cells and expanded rows", secondary,
            js_data_table(
                &DataTableSpec::new(incident_columns, incident_rows)
                    .with_expanded_row_ids(vec!["incident-1".into()])
                    .with_show_row_actions(false)
                    .with_aria_label("Active incidents"),
                theme,
            )
        ))
        // Loading skeleton posture (5 skeleton rows, selection column included)
        .child(group("Loading", secondary,
            js_data_table_loading(
                &DataTableSpec::new(columns(), vec![]).with_selectable(true),
                theme,
                5,
            )
        ))
        // Empty state
        .child(group("Empty state", secondary,
            js_data_table(
                &DataTableSpec::new(columns(), vec![])
                    .with_aria_label("Empty data table")
                    .with_empty_message("No team members match the current filters."),
                theme,
            )
        ))
        // Sizes
        .child(group("Sizes", secondary, sizes_col))
        // Densities
        .child(group("Densities", secondary, density_col))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
