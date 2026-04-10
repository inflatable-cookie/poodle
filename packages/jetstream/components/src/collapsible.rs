//! Collapsible — Jetstream collapsible container backed by CollapsibleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::CollapsibleSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_collapsible(spec: &CollapsibleSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let gap = resolve_px(theme, spec.content_gap_token());
    let is_open = spec.current_open();
    let text_color = resolve_color(theme, "color.text.primary");
    let font_size = rem_to_px(size_font_rem(effective_size));
    // Icon size: one step smaller than control font for chrome feel
    let icon_size = rem_to_px(0.875); // 14px equivalent

    let mut el = ui_element::div().flex_col().gap(gap);

    // Header with toggle indicator (SVG icon)
    let chevron_icon = if is_open { "chevron-down" } else { "chevron-right" };
    let header_gap = rem_to_px(0.25);
    let mut header = ui_element::div().flex_row().items_center().gap(header_gap).cursor_pointer();
    header = header.child(ui_element::icon(chevron_icon).w(icon_size).h(icon_size).text_color(text_color));
    if let Some(ref title) = spec.title {
        header = header.child(ui_element::label(title).text_color(text_color).text_size(font_size).text_weight(600));
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
