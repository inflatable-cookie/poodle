//! DataTable specimen — default, sorted, selectable, and empty states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::data_table::js_data_table;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{DataTableSpec, TableColumnSpec, TableRowSpec, TableSortDirection};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let columns = vec![
        TableColumnSpec::new("name", "Name").with_sortable(true),
        TableColumnSpec::new("owner", "Owner").with_sortable(true),
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
        // Default — no sort, no selection
        .child(group("Default", secondary,
            js_data_table(&DataTableSpec::new(columns.clone(), rows.clone()), theme)
        ))

        // Sorted ascending by Name
        .child(group("Sorted by Name ↑", secondary,
            js_data_table(
                &DataTableSpec::new(columns.clone(), rows.clone())
                    .with_sort("name", TableSortDirection::Asc),
                theme,
            )
        ))

        // Sorted descending by Owner
        .child(group("Sorted by Owner ↓", secondary,
            js_data_table(
                &DataTableSpec::new(columns.clone(), rows.clone())
                    .with_sort("owner", TableSortDirection::Desc),
                theme,
            )
        ))

        // Selectable rows
        .child(group("Selectable rows", secondary,
            js_data_table(
                &DataTableSpec::new(columns.clone(), rows.clone())
                    .with_selectable(true),
                theme,
            )
        ))

        // Empty state
        .child(group("Empty state", secondary,
            js_data_table(
                &DataTableSpec::new(columns.clone(), vec![]),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
