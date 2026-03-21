//! Collapsible — Jetstream collapsible container backed by CollapsibleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::CollapsibleSpec;

use crate::theme_ext::resolve_px;

pub fn js_collapsible(spec: &CollapsibleSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let gap = resolve_px(theme, spec.content_gap_token());
    let is_open = spec.open.unwrap_or(spec.default_open);
    let text_color = glam::Vec4::new(0.9, 0.9, 0.9, 1.0); // resolved at render

    let mut el = ui_element::div().flex_col().gap(gap);

    // Header with toggle indicator
    let chevron = if is_open { "▾" } else { "▸" };
    let mut header = ui_element::div().flex_row().items_center().gap(4.0);
    header = header.child(ui_element::label(chevron).text_size(12.0));
    if let Some(ref title) = spec.title {
        header = header.child(ui_element::label(title).text_size(13.0).text_weight(600));
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
