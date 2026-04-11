//! Breadcrumbs — Jetstream breadcrumb navigation backed by BreadcrumbsSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::BreadcrumbsSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::resolve_color;

pub fn js_breadcrumbs(spec: &BreadcrumbsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let gap = rem_to_px(0.25);

    let text_color = resolve_color(theme, "color.text.secondary");
    let current_color = resolve_color(theme, "color.text.primary");
    let sep_color = resolve_color(theme, "color.text.secondary");

    let mut el = ui_element::div().flex_row().items_center().gap(gap);
    let last_idx = spec.items.len().saturating_sub(1);

    for (i, item) in spec.items.iter().enumerate() {
        let is_current = i == last_idx;
        let color = if is_current { current_color } else { text_color };

        el = el.child(
            ui_element::label(&item.label)
                .text_color(color)
                .text_size(font_size)
                .text_weight(if is_current { 600 } else { 400 })
        );

        if !is_current {
            el = el.child(ui_element::label("/").text_color(sep_color).text_size(font_size));
        }
    }

    el
}
