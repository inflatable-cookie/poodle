//! Pagination specimen — page navigation in different states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::pagination_comp::js_pagination;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::PaginationSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // First page selected
        .child(group("First page", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(1)
                    .with_total_pages(10),
                theme,
            )
        ))
        // Middle page selected
        .child(group("Middle page", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(5)
                    .with_total_pages(10),
                theme,
            )
        ))
        // Last page selected
        .child(group("Last page", secondary,
            js_pagination(
                &PaginationSpec::new()
                    .with_current_page(10)
                    .with_total_pages(10),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
