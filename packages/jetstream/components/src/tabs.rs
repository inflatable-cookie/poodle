//! Tabs — Jetstream tabs backed by TabsSpec.
//!
//! Contract: `docs/contracts/components/tabs.md`
//! Reference: `packages/svelte/primitives/src/Tabs.svelte`
//!
//! Uses per-side border colors for the active tab indicator (bottom border).

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::TabsSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_tabs(spec: &TabsSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let tab_px = rem_to_px(control_space_x_rem(spec.density));
    let tab_py = rem_to_px(panel_space_y_rem(spec.density));
    let gap = rem_to_px(0.25);

    let indicator = resolve_color(theme, spec.indicator_token());
    let border = resolve_color(theme, spec.list_border_token());
    let text_color = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");

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
            .text_size(font_size)
            .text_weight(if is_active { 600 } else { 400 })
            .text_color(if is_active { text_color } else { text_muted })
            .pt(tab_py).pb(tab_py).pl(tab_px).pr(tab_px)
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
