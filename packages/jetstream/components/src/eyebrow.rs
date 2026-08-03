//! Eyebrow — Jetstream small label component backed by EyebrowSpec.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::EyebrowSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_eyebrow(spec: &EyebrowSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, spec.text_color_token());
    // Contract §8: size-dependent font-size (0.6875rem xs/sm, 0.85rem md).
    let font_size = rem_to_px(spec.font_size_rem());

    // Contract/Svelte: `text-transform: uppercase`. JsEl has no CSS transform, so
    // uppercase the content here (matches GPUI's Eyebrow).
    let text = spec.content.as_deref().unwrap_or("").to_uppercase();

    // Contract §8: weight 600 (literal — heavier than the 500 label-weight token).
    // Contract §8 letter-spacing: 0.12em (xs/sm) / 0.04em (md) via the spec.
    // line-height/font-family have no JsEl channel (runtime gap).
    let mut el = ui_element::label(text)
        .text_color(text_color)
        .text_size(font_size)
        .text_weight(spec.font_weight())
        .letter_spacing_em(spec.letter_spacing_em());

    // Contract §8 spacing="bottom": bottom margin (0.5rem, 0.35rem for xs).
    let mb = spec.margin_bottom_rem();
    if mb > 0.0 {
        el = el.mb(rem_to_px(mb));
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{EyebrowSize, EyebrowSpacing};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Contract §8 + Svelte `text-transform: uppercase`: mixed-case input is
    /// uppercased by the component (JsEl has no CSS transform).
    #[test]
    fn uppercases_content() {
        let el = js_eyebrow(&EyebrowSpec::new().with_content("File"), &theme());
        let tree = probe(&el, 120.0, 24.0);
        assert!(
            tree.has_text("FILE"),
            "eyebrow should uppercase: {:?}",
            tree.texts()
        );
    }

    /// Contract §8 size variants: `sm` (default) renders at 0.6875rem; `md`
    /// renders larger at 0.85rem.
    #[test]
    fn md_size_has_larger_font_than_sm() {
        let th = theme();
        let sm = probe(
            &js_eyebrow(&EyebrowSpec::new().with_content("section"), &th),
            240.0,
            48.0,
        );
        let md = probe(
            &js_eyebrow(
                &EyebrowSpec::new()
                    .with_content("section")
                    .with_size(EyebrowSize::Md),
                &th,
            ),
            240.0,
            48.0,
        );
        let sm_size = sm
            .nodes
            .iter()
            .find_map(|n| n.text_size)
            .expect("sm font size");
        let md_size = md
            .nodes
            .iter()
            .find_map(|n| n.text_size)
            .expect("md font size");
        assert!(
            md_size > sm_size,
            "md font-size ({md_size}) should exceed sm ({sm_size})"
        );
    }

    /// Contract §8 spacing="bottom": the labelled element still renders; the
    /// bottom-margin pushes following content down (geometry-level effect).
    #[test]
    fn spacing_bottom_still_renders_label() {
        let el = js_eyebrow(
            &EyebrowSpec::new()
                .with_content("overview")
                .with_element(poodle_specs::EyebrowElement::H3)
                .with_size(EyebrowSize::Md)
                .with_spacing(EyebrowSpacing::Bottom),
            &theme(),
        );
        let tree = probe(&el, 240.0, 48.0);
        assert!(tree.has_text("OVERVIEW"), "texts: {:?}", tree.texts());
    }
}
