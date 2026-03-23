//! DockRegion — Jetstream dock region backed by DockRegionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::DockRegionSpec;
use crate::theme_ext::resolve_color;

pub fn js_dock_region(spec: &DockRegionSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.panel");
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .flex_col().grow();

    // Tab bar
    let mut tabs = ui_element::div().flex_row().gap(4.0).pl(8.0).pr(8.0).pt(4.0);
    for tab in &spec.items {
        tabs = tabs.child(
            ui_element::button(&tab.label)
                .text_color(muted).text_size(12.0)
                .pl(8.0).pr(8.0).pt(4.0).pb(4.0)
                .focusable()
        );
    }
    el = el.child(tabs);

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
