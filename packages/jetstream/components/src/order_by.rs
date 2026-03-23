//! OrderBy — Jetstream sort control backed by OrderBySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::OrderBySpec;
use crate::theme_ext::resolve_color;

pub fn js_order_by(spec: &OrderBySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div().flex_row().items_center().gap(6.0);

    el = el.child(ui_element::label("Sort by:").text_color(muted).text_size(12.0));

    for field in &spec.fields {
        let is_active = spec.active_sort.as_ref().map(|s| s.field.as_str()) == Some(field.value.as_str());
        el = el.child(
            ui_element::button(&field.label)
                .text_color(if is_active { text_color } else { muted })
                .text_size(13.0)
                .text_weight(if is_active { 600 } else { 400 })
                .focusable()
        );
    }

    el
}
