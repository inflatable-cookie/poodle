//! Table specimen — contract §13 state coverage: standard table (row-header
//! bold + end-aligned column), caption, minimal key-value, empty state, plus
//! the size and density variant axes. All visuals resolve from `js_table`/tokens.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_table;

use poodle_specs::{ColumnAlign, ControlDensity, ControlSize, TableColumn, TableRow, TableSpec};

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
        TableRow::new("1", vec![
            ("name".into(), "Alice Chen".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
            ("hours".into(), "40".into()),
        ]),
        TableRow::new("2", vec![
            ("name".into(), "Bob Martinez".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
            ("hours".into(), "36".into()),
        ]),
        TableRow::new("3", vec![
            ("name".into(), "Carol Patel".into()),
            ("role".into(), "PM".into()),
            ("status".into(), "On leave".into()),
            ("hours".into(), "0".into()),
        ]),
        TableRow::new("4", vec![
            ("name".into(), "Dan Okoro".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
            ("hours".into(), "42".into()),
        ]),
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
        TableRow::new("1", vec![
            ("key".into(), "Version".into()),
            ("value".into(), "2.4.1".into()),
        ]),
        TableRow::new("2", vec![
            ("key".into(), "License".into()),
            ("value".into(), "MIT".into()),
        ]),
        TableRow::new("3", vec![
            ("key".into(), "Bundle size".into()),
            ("value".into(), "12.3 kB".into()),
        ]),
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

    let mut sizes_col = div().flex_col().gap(16.0);
    for &(key, size) in sizes {
        sizes_col = sizes_col
            .child(label(&format!("size = {}", key)).text_color(secondary).text_size(11.0))
            .child(js_table(
                &TableSpec::new()
                    .with_columns(minimal_columns())
                    .with_rows(minimal_rows())
                    .with_aria_label(format!("Table at {}", key))
                    .with_size(size),
                theme,
            ));
    }

    let mut density_col = div().flex_col().gap(16.0);
    for &(key, density) in densities {
        density_col = density_col
            .child(label(&format!("density = {}", key)).text_color(secondary).text_size(11.0))
            .child(js_table(
                &TableSpec::new()
                    .with_columns(minimal_columns())
                    .with_rows(minimal_rows())
                    .with_aria_label(format!("Table at {}", key))
                    .with_density(density),
                theme,
            ));
    }

    div().flex_col().gap(24.0)
        // Standard table — Name row-header (bold), Hours end-aligned
        .child(group("Standard table", secondary,
            js_table(
                &TableSpec::new()
                    .with_columns(team_columns())
                    .with_rows(team_rows())
                    .with_aria_label("Team members"),
                theme,
            )
        ))
        // With caption
        .child(group("With caption", secondary,
            js_table(
                &TableSpec::new()
                    .with_columns(team_columns())
                    .with_rows(team_rows())
                    .with_caption("Q1 team allocation")
                    .with_aria_label("Team allocation table"),
                theme,
            )
        ))
        // Minimal key-value
        .child(group("Minimal key-value", secondary,
            js_table(
                &TableSpec::new()
                    .with_columns(minimal_columns())
                    .with_rows(minimal_rows())
                    .with_aria_label("Package info"),
                theme,
            )
        ))
        // Empty state
        .child(group("Empty state", secondary,
            js_table(
                &TableSpec::new()
                    .with_columns(team_columns())
                    .with_aria_label("Empty table")
                    .with_empty_message("No team members found."),
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
