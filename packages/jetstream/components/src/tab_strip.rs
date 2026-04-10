//! TabStrip — Jetstream standalone tab bar backed by TabStripSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::TabStripSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_tab_strip(spec: &TabStripSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let gap = resolve_px(theme, spec.item_gap_token());
    let text_color = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.subtle");

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());

    // Contract: font-size 0.8125rem (13px), tab padding 0.25rem 0.5rem
    let font_size = rem_to_px(0.8125);
    let tab_py = rem_to_px(0.25);
    let tab_px = rem_to_px(0.5);

    let mut el = ui_element::div()
        .flex_row()
        .gap(gap)
        .border_b_1()
        .border_color(border);

    for item in &spec.items {
        let is_active = selected == Some(item.value.as_str());
        el = el.child(
            ui_element::button(&item.label)
                .text_size(font_size)
                .text_weight(if is_active { 600 } else { 400 })
                .text_color(if is_active { text_color } else { text_muted })
                .pt(tab_py).pb(tab_py).pl(tab_px).pr(tab_px)
                .focusable()
        );
    }

    el
}
