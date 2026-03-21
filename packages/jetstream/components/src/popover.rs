//! Popover — Jetstream popover container backed by PopoverSpec.
//!
//! Jetstream cannot do overlay positioning. Renders inline below trigger.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::PopoverSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_popover(_spec: &PopoverSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0);

    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    el
}
