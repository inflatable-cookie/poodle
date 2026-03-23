//! HoverCard — Jetstream hover card backed by HoverCardSpec.
//!
//! Contract: `docs/contracts/foundation/hover-card.md`
//! Uses overlay() and on_pointer_enter/leave for trigger.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::HoverCardSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_hover_card(spec: &HoverCardSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0)
        .shadow_md()
        .overlay();

    if let Some(c) = content {
        el = el.child(c);
    }

    el
}
