//! Banner — Jetstream status banner backed by BannerSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::BannerSpec;

use crate::theme_ext::{resolve_color, tint};

pub fn js_banner(spec: &BannerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let tone_color = resolve_color(theme, spec.fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let text_primary = resolve_color(theme, "semantic.color.text.primary");

    let fill = tint(tone_color, 0.12);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .pl(12.0).pr(12.0)
        .pt(8.0).pb(8.0)
        .flex_row()
        .items_center()
        .gap(8.0);

    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(13.0)
                .text_weight(600)
        );
    }

    if let Some(ref message) = spec.message {
        el = el.child(
            ui_element::label(message)
                .text_color(text_primary)
                .text_size(13.0)
        );
    }

    el
}
