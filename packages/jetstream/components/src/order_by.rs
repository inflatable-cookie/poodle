//! OrderBy — Jetstream sort control backed by OrderBySpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::OrderBySpec;

use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_order_by(spec: &OrderBySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let label_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let gap = resolve_px(theme, spec.gap_token());

    let text_color = resolve_color(theme, spec.field_text_token());
    let muted = resolve_color(theme, spec.label_color_token());

    let mut el = ui_element::div().flex_row().items_center().gap(gap);

    el = el.child(ui_element::label("Sort by:").text_color(muted).text_size(label_font));

    for field in &spec.fields {
        let is_active = spec.active_sort.as_ref().map(|s| s.field.as_str()) == Some(field.value.as_str());
        el = el.child(
            ui_element::button(&field.label)
                .text_color(if is_active { text_color } else { muted })
                .text_size(font_size)
                .text_weight(if is_active { 600 } else { 400 })
                .focusable()
        );
    }

    el
}
