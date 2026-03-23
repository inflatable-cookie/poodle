//! TabStrip — Jetstream standalone tab bar backed by TabStripSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::TabStripSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_tab_strip(spec: &TabStripSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let gap = resolve_px(theme, spec.item_gap_token());
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let text_muted = resolve_color(theme, "semantic.color.text.secondary");
    let border = resolve_color(theme, "semantic.color.border.subtle");

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());

    let mut el = ui_element::div()
        .flex_row()
        .gap(gap)
        .border(1.0)
        .border_color(border);

    for item in &spec.items {
        let is_active = selected == Some(item.value.as_str());
        el = el.child(
            ui_element::button(&item.label)
                .text_size(13.0)
                .text_weight(if is_active { 600 } else { 400 })
                .text_color(if is_active { text_color } else { text_muted })
                .pt(4.0).pb(4.0).pl(8.0).pr(8.0)
                .focusable()
        );
    }

    el
}
