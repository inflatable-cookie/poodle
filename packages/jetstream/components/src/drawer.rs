//! Drawer — Jetstream slide-out panel backed by DrawerSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::DrawerSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_drawer(spec: &DrawerSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let title_color = resolve_color(theme, "semantic.color.text.primary");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .flex_col().gap(12.0)
        .pl(16.0).pr(16.0).pt(16.0).pb(16.0)
        .h(480.0).w(360.0);

    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title).text_color(title_color).text_size(16.0).text_weight(600)
        );
    }

    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    el
}
