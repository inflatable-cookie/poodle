//! Popover — Jetstream popover container backed by PopoverSpec.
//!
//! Contract: `docs/contracts/components/popover.md`
//! Uses overlay() to escape parent clip rects and render on top.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::PopoverSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_popover(spec: &PopoverSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let fill = resolve_color(theme, spec.surface_fill_token());
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");

    // Contract: panel padding 0.75rem 0.5rem
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .shadow_md()
        .overlay(); // Render on top of all normal content

    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    el
}
