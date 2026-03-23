//! Eyebrow — Jetstream small label component backed by EyebrowSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::EyebrowSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_eyebrow(spec: &EyebrowSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());
    let font_size = resolve_px(theme, spec.font_size_token());

    let text = spec.content.as_deref().unwrap_or("");

    ui_element::label(text)
        .text_color(text_color)
        .text_size(font_size)
        .text_weight(600) // eyebrow is typically semibold
}
