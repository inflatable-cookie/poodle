//! Tooltip — Jetstream tooltip backed by TooltipSpec.
//!
//! Contract: `docs/contracts/foundation/tooltip.md`
//! Uses overlay() for the tooltip panel. Triggered by on_pointer_enter/leave.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::TooltipSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_tooltip(spec: &TooltipSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let radius = resolve_radius(theme, "semantic.radius.surface");

    let content = spec.content.as_deref().unwrap_or("");

    // Tooltip panel: overlay, small padding, elevated bg
    ui_element::div()
        .bg(fill)
        .rounded(radius)
        .pl(8.0).pr(8.0).pt(4.0).pb(4.0)
        .shadow_sm()
        .overlay()
        .child(
            ui_element::label(content)
                .text_color(text_color)
                .text_size(12.0)
        )
}
