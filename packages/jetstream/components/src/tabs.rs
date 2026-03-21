//! Tabs — Jetstream tabs backed by TabsSpec.
//!
//! Contract: `docs/contracts/foundation/tabs.md`
//! Reference: `packages/svelte/primitives/src/Tabs.svelte`
//!
//! Uses per-side border colors for the active tab indicator (bottom border).

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::TabsSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_tabs(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let gap = resolve_px(theme, spec.list_gap_token());
    let indicator = resolve_color(theme, spec.indicator_token());
    let border = resolve_color(theme, spec.list_border_token());
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let text_muted = resolve_color(theme, "semantic.color.text.secondary");
    let label_size = resolve_px(theme, "semantic.typography.label.size");

    let selected = spec.value.as_deref().or(spec.default_value.as_deref());

    // Tab list: horizontal row with bottom border only (underline variant)
    let mut tab_bar = ui_element::div()
        .flex_row()
        .gap(gap)
        .border_b_1()
        .border_color(border);

    for tab in &spec.tabs {
        let is_active = selected == Some(tab.value.as_str());
        let mut tab_el = ui_element::button(&tab.label)
            .text_size(label_size)
            .text_weight(if is_active { 600 } else { 400 })
            .text_color(if is_active { text_color } else { text_muted })
            .pt(8.0).pb(8.0).pl(12.0).pr(12.0)
            .focusable()
            .cursor_pointer();

        // Active tab: 2px colored bottom border (indicator)
        if is_active {
            tab_el = tab_el.border_b_2().border_color_bottom(indicator);
        }

        tab_bar = tab_bar.child(tab_el);
    }

    tab_bar
}
