//! EmptyState — Jetstream empty state backed by EmptyStateSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::EmptyStateSpec;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_empty_state(spec: &EmptyStateSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let gap = resolve_px(theme, spec.layout_gap_token());

    let mut el = ui_element::div()
        .flex_col().items_center().justify_center().gap(gap)
        .pt(32.0).pb(32.0);

    el = el.child(ui_element::label(&spec.title).text_color(text_primary).text_size(16.0).text_weight(600));

    if let Some(ref desc) = spec.message {
        el = el.child(ui_element::label(desc).text_color(text_secondary).text_size(13.0));
    }

    el
}
