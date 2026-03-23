//! Popover — Jetstream popover container backed by PopoverSpec.
//!
//! Contract: `docs/contracts/foundation/popover.md`
//! Uses overlay() to escape parent clip rects and render on top.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::PopoverSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_popover(spec: &PopoverSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, spec.surface_fill_token());
    let border = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(12.0).pr(12.0).pt(8.0).pb(8.0)
        .shadow_md()
        .overlay(); // Render on top of all normal content

    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    el
}
