//! PaginationSummary specimen — contract §13 groups: Default, Single page,
//! Large dataset. Read-only "Showing X-Y of Z" range text; no interaction.
//!
//! `PaginationSummarySpec::new(page, page_size, total_items)`.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_pagination_summary;

use poodle_specs::PaginationSummarySpec;

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Contract §13 Default — "Showing 1-20 of 156".
        .child(group("Default", secondary,
            js_pagination_summary(&PaginationSummarySpec::new(1, 20, 156), theme)
        ))
        // Contract §13 Single page — "Showing 1-12 of 12".
        .child(group("Single page", secondary,
            js_pagination_summary(&PaginationSummarySpec::new(1, 20, 12), theme)
        ))
        // Contract §13 Large dataset — mid-range "Showing 81-100 of 1000".
        .child(group("Large dataset", secondary,
            js_pagination_summary(&PaginationSummarySpec::new(5, 20, 1000), theme)
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
