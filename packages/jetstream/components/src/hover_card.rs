//! HoverCard — Jetstream hover card backed by HoverCardSpec.
//!
//! Jetstream has no hover detection. Renders as static card.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::HoverCardSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_hover_card(_spec: &HoverCardSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, "semantic.color.background.elevated");
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0);

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
