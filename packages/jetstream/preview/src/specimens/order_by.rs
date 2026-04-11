//! OrderBy specimen — sort control with active/inactive fields.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::order_by::js_order_by;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{ActiveSort, OrderBySpec, SortDirection, SortField};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let fields = vec![
        SortField::new("name", "Name"),
        SortField::new("date", "Date"),
        SortField::new("status", "Status"),
    ];

    div().flex_col().gap(24.0)
        .child(group("No active sort", secondary,
            js_order_by(
                &OrderBySpec::new().with_fields(fields.clone()),
                theme,
            )
        ))
        .child(group("Sort by name (asc)", secondary,
            js_order_by(
                &OrderBySpec::new()
                    .with_fields(fields.clone())
                    .with_active_sort(ActiveSort::new("name", SortDirection::Asc)),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
