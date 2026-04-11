use gpui::*;
use poodle_specs::{TableSpec, TableColumn, TableRow, ColumnAlign, EyebrowSpec};
use poodle_gpui_components::{Table, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let columns = vec![
        TableColumn::new("name", "Name").with_row_header(true),
        TableColumn::new("type", "Type"),
        TableColumn::new("status", "Status"),
        TableColumn::new("updated", "Updated"),
    ];

    let rows = vec![
        TableRow::new("1", vec![
            ("name".to_string(), "Button".to_string()),
            ("type".to_string(), "Primitive".to_string()),
            ("status".to_string(), "Stable".to_string()),
            ("updated".to_string(), "2 days ago".to_string()),
        ]),
        TableRow::new("2", vec![
            ("name".to_string(), "DataTable".to_string()),
            ("type".to_string(), "Composite".to_string()),
            ("status".to_string(), "Beta".to_string()),
            ("updated".to_string(), "1 week ago".to_string()),
        ]),
        TableRow::new("3", vec![
            ("name".to_string(), "Dialog".to_string()),
            ("type".to_string(), "Primitive".to_string()),
            ("status".to_string(), "Stable".to_string()),
            ("updated".to_string(), "3 days ago".to_string()),
        ]),
        TableRow::new("4", vec![
            ("name".to_string(), "Drawer".to_string()),
            ("type".to_string(), "Primitive".to_string()),
            ("status".to_string(), "Draft".to_string()),
            ("updated".to_string(), "Today".to_string()),
        ]),
        TableRow::new("5", vec![
            ("name".to_string(), "Select".to_string()),
            ("type".to_string(), "Primitive".to_string()),
            ("status".to_string(), "Stable".to_string()),
            ("updated".to_string(), "5 days ago".to_string()),
        ]),
    ];

    // Numeric table with right-aligned columns
    let num_columns = vec![
        TableColumn::new("endpoint", "Endpoint").with_row_header(true),
        TableColumn::new("requests", "Requests").with_align(ColumnAlign::End),
        TableColumn::new("avg_ms", "Avg (ms)").with_align(ColumnAlign::End),
        TableColumn::new("p99_ms", "P99 (ms)").with_align(ColumnAlign::End),
    ];

    let num_rows = vec![
        TableRow::new("1", vec![
            ("endpoint".to_string(), "/api/users".to_string()),
            ("requests".to_string(), "14,230".to_string()),
            ("avg_ms".to_string(), "42".to_string()),
            ("p99_ms".to_string(), "187".to_string()),
        ]),
        TableRow::new("2", vec![
            ("endpoint".to_string(), "/api/projects".to_string()),
            ("requests".to_string(), "8,912".to_string()),
            ("avg_ms".to_string(), "68".to_string()),
            ("p99_ms".to_string(), "312".to_string()),
        ]),
        TableRow::new("3", vec![
            ("endpoint".to_string(), "/api/auth".to_string()),
            ("requests".to_string(), "3,456".to_string()),
            ("avg_ms".to_string(), "23".to_string()),
            ("p99_ms".to_string(), "95".to_string()),
        ]),
        TableRow::new("4", vec![
            ("endpoint".to_string(), "/api/search".to_string()),
            ("requests".to_string(), "1,087".to_string()),
            ("avg_ms".to_string(), "156".to_string()),
            ("p99_ms".to_string(), "890".to_string()),
        ]),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Standard table ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Standard table"), theme))
                .child(
                    Table::from_spec(
                        TableSpec::new()
                            .with_columns(columns.clone())
                            .with_rows(rows.clone())
                            .with_aria_label("Component registry"),
                        theme,
                    )
                )
        )
        // --- With caption ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With caption"), theme))
                .child(
                    Table::from_spec(
                        TableSpec::new()
                            .with_columns(columns.clone())
                            .with_rows(rows)
                            .with_caption("Components in the design system")
                            .with_aria_label("Components table"),
                        theme,
                    )
                )
        )
        // --- Right-aligned numeric data ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Minimal key-value"), theme))
                .child(
                    Table::from_spec(
                        TableSpec::new()
                            .with_columns(num_columns)
                            .with_rows(num_rows)
                            .with_caption("API endpoint performance metrics")
                            .with_aria_label("Performance metrics"),
                        theme,
                    )
                )
        )
        // --- Minimal key-value ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty state"), theme))
                .child({
                    let kv_columns = vec![
                        TableColumn::new("key", "Property").with_row_header(true),
                        TableColumn::new("value", "Value"),
                    ];
                    let kv_rows = vec![
                        TableRow::new("1", vec![
                            ("key".to_string(), "Version".to_string()),
                            ("value".to_string(), "2.4.1".to_string()),
                        ]),
                        TableRow::new("2", vec![
                            ("key".to_string(), "License".to_string()),
                            ("value".to_string(), "MIT".to_string()),
                        ]),
                        TableRow::new("3", vec![
                            ("key".to_string(), "Bundle size".to_string()),
                            ("value".to_string(), "12.3 kB".to_string()),
                        ]),
                    ];
                    Table::from_spec(
                        TableSpec::new()
                            .with_columns(kv_columns)
                            .with_rows(kv_rows)
                            .with_aria_label("Package info"),
                        theme,
                    )
                })
        )
        // --- Empty ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty table"), theme))
                .child(
                    Table::from_spec(
                        TableSpec::new()
                            .with_columns(columns)
                            .with_empty_message("No components found")
                            .with_aria_label("Empty table"),
                        theme,
                    )
                )
        )
}
