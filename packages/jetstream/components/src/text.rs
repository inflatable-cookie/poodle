//! JsText — text primitive backed by TextSpec.
//!
//! Contract: `docs/contracts/components/text.md`
//! Reference: `packages/svelte/components/src/Text.svelte`, GPUI `primitives/text.rs`
//!
//! ALL dimensions resolve from the spec/tokens. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{TextSpec, TextWeight};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a text element from a `TextSpec`.
///
/// Anatomy: a single Label node carrying the resolved tone color, size, weight,
/// and line height. The `element` prop (`p`/`span`/`div`) has no semantic effect
/// in Jetstream (there is no DOM), so every variant renders the same Label —
/// matching GPUI, which also collapses `as` to one node.
pub fn js_text(spec: &TextSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let color = resolve_color(theme, spec.color_token());
    let weight: u16 = match spec.weight {
        TextWeight::Normal => 400,
        TextWeight::Medium => 500,
        TextWeight::Semibold => 600,
        TextWeight::Bold => 700,
    };

    let mut el = ui_element::label(&spec.content)
        .text_color(color)
        .text_size(rem_to_px(spec.font_size_rem()))
        .text_weight(weight)
        .line_height(spec.line_height());

    // `clamp` limits visible lines. JsEl has no line-clamp, so (like GPUI) this
    // degrades to clipping overflow — the N-line cap is a runtime gap.
    if spec.clamp.is_some() {
        el = el.overflow_hidden();
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{TextSize, TextTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_content_as_label() {
        let el = js_text(&TextSpec::new("Hello"), &theme());
        let tree = probe(&el, 200.0, 40.0);
        assert!(tree.has_text("Hello"), "text content missing: {:?}", tree.texts());
    }

    #[test]
    fn danger_tone_resolves_status_danger() {
        // Danger tone must resolve a real (non-default) color from the theme.
        let th = theme();
        let danger = resolve_color(&th, "color.status.danger");
        let el = js_text(&TextSpec::new("x").with_tone(TextTone::Danger), &th);
        assert_eq!(el.style.text_color, Some(danger.into()));
    }

    #[test]
    fn size_maps_to_font_px() {
        let el = js_text(&TextSpec::new("x").with_size(TextSize::Xs), &theme());
        assert_eq!(el.style.text_size, Some(rem_to_px(0.75)));
    }
}
