//! Eyebrow — Jetstream small label component backed by EyebrowSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::EyebrowSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_eyebrow(spec: &EyebrowSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());
    // Contract: font-size 0.6875rem (11px) — specific eyebrow typography
    let font_size = rem_to_px(spec.font_size_rem());

    // Contract/Svelte: `text-transform: uppercase`. JsEl has no CSS transform, so
    // uppercase the content here (matches GPUI's Eyebrow).
    let text = spec.content.as_deref().unwrap_or("").to_uppercase();

    ui_element::label(text)
        .text_color(text_color)
        .text_size(font_size)
        .text_weight(600) // eyebrow is typically semibold
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    #[test]
    fn uppercases_content() {
        let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK);
        let el = js_eyebrow(&EyebrowSpec::new().with_content("File"), &theme);
        let tree = probe(&el, 120.0, 24.0);
        assert!(tree.has_text("FILE"), "eyebrow should uppercase: {:?}", tree.texts());
    }
}
