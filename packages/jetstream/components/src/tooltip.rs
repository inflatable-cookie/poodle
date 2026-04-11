//! Tooltip — Jetstream tooltip backed by TooltipSpec.
//!
//! Contract: `docs/contracts/components/tooltip.md`
//! Uses overlay() for the tooltip panel. Triggered by on_pointer_enter/leave.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::TooltipSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_tooltip(spec: &TooltipSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let text_color = resolve_color(theme, "color.text.primary");
    let radius = resolve_radius(theme, "radius.surface");

    let content = spec.content.as_deref().unwrap_or("");

    // Contract: padding 0.375rem 0.5rem, font-size 0.6875rem (11px), max-width 16rem
    let pad_x = rem_to_px(spec.padding_x_rem());
    let pad_y = rem_to_px(spec.padding_y_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let max_w = rem_to_px(spec.max_width_rem());

    // Tooltip panel: overlay, small padding, elevated bg
    ui_element::div()
        .bg(fill)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .max_w(max_w)
        .shadow_sm()
        .overlay()
        .child(
            ui_element::label(content)
                .text_color(text_color)
                .text_size(font_size)
        )
}
