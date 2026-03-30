//! Table specimen — with columns and rows, empty state.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::table::js_table;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{ColumnAlign, TableColumn, TableRow, TableSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let columns = vec![
        TableColumn::new("name", "Name").with_row_header(true),
        TableColumn::new("role", "Role"),
        TableColumn::new("status", "Status"),
        TableColumn::new("hours", "Hours").with_align(ColumnAlign::End),
    ];

    let rows = vec![
        TableRow::new("1", vec![
            ("name".into(), "Alice Chen".into()),
            ("role".into(), "Engineer".into()),
            ("status".into(), "Active".into()),
            ("hours".into(), "40".into()),
        ]),
        TableRow::new("2", vec![
            ("name".into(), "Bob Smith".into()),
            ("role".into(), "Designer".into()),
            ("status".into(), "Active".into()),
            ("hours".into(), "38".into()),
        ]),
        TableRow::new("3", vec![
            ("name".into(), "Carol Davis".into()),
            ("role".into(), "Manager".into()),
            ("status".into(), "Away".into()),
            ("hours".into(), "32".into()),
        ]),
    ];

    div().flex_col().gap(24.0)
        // With data
        .child(group("With rows", secondary,
            js_table(
                &TableSpec::new()
                    .with_columns(columns.clone())
                    .with_rows(rows)
                    .with_caption("Team members"),
                theme,
            )
        ))
        // Empty
        .child(group("Empty table", secondary,
            js_table(
                &TableSpec::new()
                    .with_columns(columns)
                    .with_empty_message("No team members found."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
