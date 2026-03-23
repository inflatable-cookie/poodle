//! Collapsible — Jetstream collapsible container backed by CollapsibleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::CollapsibleSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_collapsible(spec: &CollapsibleSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.content_gap_token());
    let is_open = spec.open.unwrap_or(spec.default_open);
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div().flex_col().gap(gap);

    // Header with toggle indicator (SVG icon)
    let chevron_icon = if is_open { "chevron-down" } else { "chevron-right" };
    let mut header = ui_element::div().flex_row().items_center().gap(4.0).cursor_pointer();
    header = header.child(ui_element::icon(chevron_icon).w(14.0).h(14.0).text_color(text_color));
    if let Some(ref title) = spec.title {
        header = header.child(ui_element::label(title).text_color(text_color).text_size(13.0).text_weight(600));
    }
    el = el.child(header);

    // Content (only when open)
    if is_open {
        if let Some(content_el) = content {
            el = el.child(content_el);
        }
    }

    el
}
