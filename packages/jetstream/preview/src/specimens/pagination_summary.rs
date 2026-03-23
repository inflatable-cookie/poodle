//! PaginationSummary specimen — page range display text.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::pagination_summary::js_pagination_summary;
use flint_jetstream_components::theme_ext::*;
use flint_composites::PaginationSummarySpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Page 1", secondary,
            js_pagination_summary(&PaginationSummarySpec::new(1, 25, 67), theme)
        ))
        .child(group("Page 3", secondary,
            js_pagination_summary(&PaginationSummarySpec::new(3, 25, 67), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
