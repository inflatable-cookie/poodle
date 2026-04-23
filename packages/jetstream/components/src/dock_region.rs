//! DockRegion — Jetstream dock region backed by DockRegionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DockRegionSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, tint};

pub fn js_dock_region(spec: &DockRegionSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tab_font = rem_to_px(size_font_rem(effective_size));
    let tab_px = rem_to_px(control_space_x_rem(spec.density));
    let tab_py = rem_to_px(panel_space_y_rem(spec.density) * 0.5);
    let strip_px = rem_to_px(control_space_x_rem(spec.density));
    let strip_py = rem_to_px(panel_space_y_rem(spec.density) * 0.25);
    let tab_gap = rem_to_px(0.25);

    let fill = resolve_color(theme, spec.strip_fill_token());
    let border = resolve_color(theme, "color.border.subtle");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let active_bg = tint(accent, 0.18);

    let active = spec.current_value().map(|s| s.to_string());

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .flex_col().grow();

    // Tab strip — underline style; active tab gets accent bottom border + tint bg.
    let mut tab_bar = ui_element::div()
        .flex_row()
        .gap(tab_gap)
        .pl(strip_px).pr(strip_px).pt(strip_py)
        .border_b_1().border_color(border);

    for tab in &spec.items {
        let is_active = active.as_deref() == Some(tab.value.as_str());
        let text_color = if is_active { text_primary } else { text_muted };

        let mut tab_btn = ui_element::button(&tab.label)
            .text_color(text_color)
            .text_size(tab_font)
            .text_weight(if is_active { 600 } else { 400 })
            .pl(tab_px).pr(tab_px).pt(tab_py).pb(tab_py)
            .focusable()
            .cursor_pointer();

        if is_active {
            tab_btn = tab_btn
                .bg(active_bg)
                .border_b_2()
                .border_color_bottom(accent);
        }

        tab_bar = tab_bar.child(tab_btn);
    }
    el = el.child(tab_bar);

    // Content area — hidden when collapsed.
    if !spec.is_collapsed {
        if let Some(c) = content {
            el = el.child(c);
        }
    }

    el
}
