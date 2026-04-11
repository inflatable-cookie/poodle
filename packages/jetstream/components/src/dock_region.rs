//! DockRegion — Jetstream dock region backed by DockRegionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DockRegionSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::resolve_color;

pub fn js_dock_region(spec: &DockRegionSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tab_font = rem_to_px(size_font_rem(effective_size) - 0.0625);
    let tab_px = rem_to_px(control_space_x_rem(spec.density));
    let tab_py = rem_to_px(panel_space_y_rem(spec.density) - 0.5);
    let strip_px = rem_to_px(control_space_x_rem(spec.density));
    let strip_py = rem_to_px(panel_space_y_rem(spec.density) - 0.5);
    let tab_gap = rem_to_px(0.25);

    let fill = resolve_color(theme, "color.background.panel");
    let border = resolve_color(theme, "color.border.subtle");
    let muted = resolve_color(theme, "color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .flex_col().grow();

    // Tab bar
    let mut tabs = ui_element::div().flex_row().gap(tab_gap).pl(strip_px).pr(strip_px).pt(strip_py);
    for tab in &spec.items {
        tabs = tabs.child(
            ui_element::button(&tab.label)
                .text_color(muted).text_size(tab_font)
                .pl(tab_px).pr(tab_px).pt(tab_py).pb(tab_py)
                .focusable()
        );
    }
    el = el.child(tabs);

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
