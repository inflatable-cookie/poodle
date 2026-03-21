//! Callout — Jetstream callout/alert component backed by CallOutSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::CallOutSpec;

use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_callout(spec: &CallOutSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let tone_color = resolve_color(theme, spec.fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    // Tinted background: tone color at ~12% opacity over surface
    let fill = tint(tone_color, 0.12);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .pl(12.0).pr(12.0)
        .pt(12.0).pb(12.0)
        .flex_col()
        .gap(4.0);

    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(13.0)
                .text_weight(600)
        );
    }

    if let Some(ref content) = spec.content {
        el = el.child(
            ui_element::label(content)
                .text_color(text_primary)
                .text_size(13.0)
        );
    }

    el
}
