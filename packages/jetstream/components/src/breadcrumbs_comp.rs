//! Breadcrumbs — Jetstream breadcrumb navigation backed by BreadcrumbsSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::BreadcrumbsSpec;

use crate::theme_ext::resolve_color;

pub fn js_breadcrumbs(spec: &BreadcrumbsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.secondary");
    let current_color = resolve_color(theme, "semantic.color.text.primary");
    let sep_color = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div().flex_row().items_center().gap(4.0);
    let last_idx = spec.items.len().saturating_sub(1);

    for (i, item) in spec.items.iter().enumerate() {
        let is_current = i == last_idx;
        let color = if is_current { current_color } else { text_color };

        el = el.child(
            ui_element::label(&item.label)
                .text_color(color)
                .text_size(13.0)
                .text_weight(if is_current { 600 } else { 400 })
        );

        if !is_current {
            el = el.child(ui_element::label("/").text_color(sep_color).text_size(13.0));
        }
    }

    el
}
