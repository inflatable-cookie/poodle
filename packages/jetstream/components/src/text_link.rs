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
/// TextLink — inline navigation.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_click(handler)`.
pub struct TextLink {
    spec: TextLinkSpec,
    theme: JetstreamThemeProvider,
    on_click: Option<crate::element::ActionHandler>,
}

impl TextLink {
    pub fn from_spec(spec: TextLinkSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    /// Fires when the link is activated. The `href` is on the spec the caller
    /// already holds, so there is nothing for the payload to add.
    pub fn on_click(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_click = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for TextLink {
    fn into_js_el(self) -> JsEl {
        let el = js_text_link(&self.spec, &self.theme);

        match (self.spec.disabled, self.on_click) {
            (false, Some(handler)) => el.cursor_pointer().on_click(move |_event| handler()),
            _ => el,
        }
    }
}

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
        assert!(
            tree.has_text("Open docs"),
            "link label missing: {:?}",
            tree.texts()
        );
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
        let el = js_text_link(
            &TextLinkSpec::new("x").with_tone(TextLinkTone::Secondary),
            &th,
        );
        assert_eq!(el.style.text_color, Some(secondary.into()));
    }

    #[test]
    fn disabled_dims_opacity() {
        let el = js_text_link(&TextLinkSpec::new("x").with_disabled(true), &theme());
        assert!(el.style.opacity < 1.0, "disabled link should be dimmed");
    }

    #[test]
    fn a_click_reaches_the_handler() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = TextLink::from_spec(TextLinkSpec::new("Open docs"), &theme())
            .on_click(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 200.0, 40.0, "Open docs");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_click fired exactly once"
        );
    }

    #[test]
    fn a_disabled_link_ignores_clicks() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = TextLink::from_spec(TextLinkSpec::new("Open docs").with_disabled(true), &theme())
            .on_click(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 200.0, 40.0, "Open docs");

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a disabled link fired");
    }
}
