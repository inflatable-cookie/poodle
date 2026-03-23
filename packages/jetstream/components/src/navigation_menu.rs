//! NavigationMenu — Jetstream nav menu backed by NavigationMenuSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::NavigationMenuSpec;

use crate::theme_ext::resolve_color;

pub fn js_navigation_menu(spec: &NavigationMenuSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div().flex_row().items_center().gap(4.0);

    for entry in &spec.items {
        el = el.child(
            ui_element::button(&entry.label)
                .text_color(text_color)
                .text_size(13.0)
                .pl(8.0).pr(8.0).pt(4.0).pb(4.0)
                .focusable()
        );
    }

    el
}
