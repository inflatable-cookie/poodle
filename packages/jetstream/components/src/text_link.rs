//! JsTextLink — inline text link backed by TextLinkSpec.
//!
//! Contract: `docs/contracts/components/text-link.md`
//! Reference: `packages/svelte/components/src/TextLink.svelte`, GPUI `primitives/text_link.rs`
//!
//! Render-only: the click/navigation is handled by the preview event loop via
//! the node id, matching the Jetstream interaction model. ZERO hardcoded values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::TextLinkSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

/// Build a text-link element from a `TextLinkSpec`.
///
/// The link renders as a tone-colored Label. Font size is inherited (not set),
/// matching GPUI which lets the link take the surrounding text size. Disabled
/// links dim via the disabled-opacity token. Underline + focus ring are CSS
/// affordances with no JsEl equivalent (runtime gap, noted in the parity doc).
pub fn js_text_link(spec: &TextLinkSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let color = resolve_color(theme, spec.color_token());

    let mut el = ui_element::label(&spec.label).text_color(color);

    if spec.disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::TextLinkTone;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_label() {
        let el = js_text_link(&TextLinkSpec::new("Open docs"), &theme());
        let tree = probe(&el, 200.0, 40.0);
        assert!(tree.has_text("Open docs"), "link label missing: {:?}", tree.texts());
    }

    #[test]
    fn accent_tone_is_default_color() {
        let th = theme();
        let accent = resolve_color(&th, "color.accent.base");
        let el = js_text_link(&TextLinkSpec::new("x"), &th);
        assert_eq!(el.style.text_color, Some(accent.into()));
    }

    #[test]
    fn secondary_tone_resolves_secondary() {
        let th = theme();
        let secondary = resolve_color(&th, "color.text.secondary");
        let el = js_text_link(&TextLinkSpec::new("x").with_tone(TextLinkTone::Secondary), &th);
        assert_eq!(el.style.text_color, Some(secondary.into()));
    }

    #[test]
    fn disabled_dims_opacity() {
        let el = js_text_link(&TextLinkSpec::new("x").with_disabled(true), &theme());
        assert!(el.style.opacity < 1.0, "disabled link should be dimmed");
    }
}
