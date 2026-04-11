//! PaginationSummary — Jetstream pagination summary backed by PaginationSummarySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::PaginationSummarySpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_pagination_summary(spec: &PaginationSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "color.text.secondary");

    let text = format!("Showing {} – {} of {}", spec.start_index(), spec.end_index(), spec.total_items);

    ui_element::label(&text).text_color(text_color).text_size(rem_to_px(0.8125))
}
