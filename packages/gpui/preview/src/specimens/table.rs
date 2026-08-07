use crate::node_compat::{Eyebrow, Table};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ColumnAlign, ControlDensity, ControlSize, EyebrowSpec, TableColumn, TableRow, TableSpec,
};

fn team_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("name", "Name").with_row_header(true),
        TableColumn::new("role", "Role"),
        TableColumn::new("status", "Status"),
        TableColumn::new("hours", "Hours").with_align(ColumnAlign::End),
    ]
}

fn team_rows() -> Vec<TableRow> {
    vec![
        TableRow::new(
            "1",
            vec![
                ("name".to_string(), "Alice Chen".to_string()),
                ("role".to_string(), "Engineer".to_string()),
                ("status".to_string(), "Active".to_string()),
                ("hours".to_string(), "40".to_string()),
            ],
        ),
        TableRow::new(
            "2",
            vec![
                ("name".to_string(), "Bob Martinez".to_string()),
                ("role".to_string(), "Designer".to_string()),
                ("status".to_string(), "Active".to_string()),
                ("hours".to_string(), "36".to_string()),
            ],
        ),
        TableRow::new(
            "3",
            vec![
                ("name".to_string(), "Carol Patel".to_string()),
                ("role".to_string(), "PM".to_string()),
                ("status".to_string(), "On leave".to_string()),
                ("hours".to_string(), "0".to_string()),
            ],
        ),
        TableRow::new(
            "4",
            vec![
                ("name".to_string(), "Dan Okoro".to_string()),
                ("role".to_string(), "Engineer".to_string()),
                ("status".to_string(), "Active".to_string()),
                ("hours".to_string(), "42".to_string()),
            ],
        ),
    ]
}

fn minimal_columns() -> Vec<TableColumn> {
    vec![
        TableColumn::new("key", "Property").with_row_header(true),
        TableColumn::new("value", "Value"),
    ]
}

fn minimal_rows() -> Vec<TableRow> {
    vec![
        TableRow::new(
            "1",
            vec![
                ("key".to_string(), "Version".to_string()),
                ("value".to_string(), "2.4.1".to_string()),
            ],
        ),
        TableRow::new(
            "2",
            vec![
                ("key".to_string(), "License".to_string()),
                ("value".to_string(), "MIT".to_string()),
            ],
        ),
        TableRow::new(
            "3",
            vec![
                ("key".to_string(), "Bundle size".to_string()),
                ("value".to_string(), "12.3 kB".to_string()),
            ],
        ),
    ]
}

fn group(title: &str, theme: &GpuiThemeProvider, content: AnyElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Standard table (Name row-header bold, Hours end-aligned) ---
        .child(group(
            "Standard table",
            theme,
            Table::from_spec(
                TableSpec::new()
                    .with_columns(team_columns())
                    .with_rows(team_rows())
                    .with_aria_label("Team members"),
                theme,
            )
            .into_any_element(),
        ))
        // --- With caption ---
        .child(group(
            "With caption",
            theme,
            Table::from_spec(
                TableSpec::new()
                    .with_columns(team_columns())
                    .with_rows(team_rows())
                    .with_caption("Q1 team allocation")
                    .with_aria_label("Team allocation table"),
                theme,
            )
            .into_any_element(),
        ))
        // --- Minimal key-value ---
        .child(group(
            "Minimal key-value",
            theme,
            Table::from_spec(
                TableSpec::new()
                    .with_columns(minimal_columns())
                    .with_rows(minimal_rows())
                    .with_aria_label("Package info"),
                theme,
            )
            .into_any_element(),
        ))
        // --- Empty state ---
        .child(group(
            "Empty state",
            theme,
            Table::from_spec(
                TableSpec::new()
                    .with_columns(team_columns())
                    .with_empty_message("No team members found.")
                    .with_aria_label("Empty table"),
                theme,
            )
            .into_any_element(),
        ))
        // --- Sizes ---
        .child(group("Sizes", theme, {
            let sizes: &[(&str, ControlSize)] = &[
                ("xs", ControlSize::Xs),
                ("sm", ControlSize::Sm),
                ("md", ControlSize::Md),
                ("lg", ControlSize::Lg),
                ("xl", ControlSize::Xl),
            ];
            let mut col = div().flex().flex_col().gap(px(16.0));
            for &(key, size) in sizes {
                col = col.child(Table::from_spec(
                    TableSpec::new()
                        .with_columns(minimal_columns())
                        .with_rows(minimal_rows())
                        .with_aria_label(format!("Table at {}", key))
                        .with_size(size),
                    theme,
                ));
            }
            col.into_any_element()
        }))
        // --- Densities ---
        .child(group("Densities", theme, {
            let densities: &[(&str, ControlDensity)] = &[
                ("compact", ControlDensity::Compact),
                ("default", ControlDensity::Default),
                ("comfortable", ControlDensity::Comfortable),
            ];
            let mut col = div().flex().flex_col().gap(px(16.0));
            for &(key, density) in densities {
                col = col.child(Table::from_spec(
                    TableSpec::new()
                        .with_columns(minimal_columns())
                        .with_rows(minimal_rows())
                        .with_aria_label(format!("Table at {}", key))
                        .with_density(density),
                    theme,
                ));
            }
            col.into_any_element()
        }))
}
