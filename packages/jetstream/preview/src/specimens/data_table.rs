//! DataTable specimen — tabular data display.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::data_table::js_data_table;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{DataTableSpec, TableColumnSpec, TableRowSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("owner", "Owner"),
        TableColumnSpec::new("status", "Status"),
    ];

    let rows = vec![
        TableRowSpec::new("1", vec![
            ("name".into(), "Button".into()),
            ("owner".into(), "Core".into()),
            ("status".into(), "Stable".into()),
        ]),
        TableRowSpec::new("2", vec![
            ("name".into(), "Dialog".into()),
            ("owner".into(), "Overlay".into()),
            ("status".into(), "Beta".into()),
        ]),
        TableRowSpec::new("3", vec![
            ("name".into(), "DataTable".into()),
            ("owner".into(), "Composites".into()),
            ("status".into(), "Alpha".into()),
        ]),
    ];

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            js_data_table(&DataTableSpec::new(columns, rows), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
