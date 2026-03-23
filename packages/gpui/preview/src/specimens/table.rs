use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{TableSpec, TableColumn, TableRow};
use pug_gpui_components::Table;
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let columns = vec![
        TableColumn::new("name", "Name"),
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
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            Table::from_spec(
                TableSpec::new()
                    .with_columns(columns.clone())
                    .with_rows(rows.clone())
                    .with_aria_label("Component registry"),
                theme,
            )
        )
        // --- With caption ---
        .child(section_label("WITH CAPTION", text_secondary))
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
        // --- Empty ---
        .child(section_label("EMPTY TABLE", text_secondary))
        .child(
            Table::from_spec(
                TableSpec::new()
                    .with_columns(columns)
                    .with_empty_message("No components found")
                    .with_aria_label("Empty table"),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
