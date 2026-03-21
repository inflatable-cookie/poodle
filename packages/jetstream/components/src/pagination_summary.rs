//! PaginationSummary — Jetstream pagination summary backed by PaginationSummarySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_composites::PaginationSummarySpec;
use crate::theme_ext::resolve_color;

pub fn js_pagination_summary(spec: &PaginationSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.secondary");

    let text = format!("Showing {} – {} of {}", spec.start_index(), spec.end_index(), spec.total_items);

    ui_element::label(&text).text_color(text_color).text_size(13.0)
}
