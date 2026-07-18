//! JsText — text primitive backed by TextSpec.
//!
//! Contract: `docs/contracts/components/text.md`
//! Reference: `packages/svelte/components/src/Text.svelte`, GPUI `primitives/text.rs`
//!
//! ALL dimensions resolve from the spec/tokens. ZERO hardcoded values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{TextSpec, TextWeight};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

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
        .line_height(spec.line_height())
        // CSS-default soft wrap (`white-space: normal`): paragraphs reflow to
        // the container width. Only engages under a width constraint — auto-
        // sized text still lays out on one line, so chips/inline uses are safe.
        .text_wrap(true);

    // `clamp` limits visible lines. JsEl has no line-clamp, so (like GPUI) this
    // degrades to wrapped text clipped at the box — the exact N-line cap +
    // ellipsis is a runtime gap.
    if spec.clamp.is_some() {
        el = el.overflow_hidden();
    }

    // `spacing="compact"` renders a stacked grid with a `space.stack.sm` gap
    // between child paragraphs (contract §3). Wrap the label in a flex-column
    // carrying the resolved gap so multi-paragraph content stacks compactly.
    if let Some(token) = spec.spacing_gap_token() {
        let gap = resolve_px(theme, token);
        return ui_element::div().flex_col().gap(gap).child(el);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{TextSize, TextSpacing, TextTone};

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

    /// Paragraph text soft-wraps (CSS white-space: normal) — reflows under a
    /// width constraint instead of running off the box.
    #[test]
    fn text_soft_wraps() {
        let el = js_text(&TextSpec::new("hello"), &theme());
        assert!(el.style.text_wrap, "js_text enables soft wrap");
    }

    #[test]
    fn size_maps_to_font_px() {
        let el = js_text(&TextSpec::new("x").with_size(TextSize::Xs), &theme());
        assert_eq!(el.style.text_size, Some(rem_to_px(0.75)));
    }

    #[test]
    fn weight_maps_to_css_value() {
        use poodle_specs::TextWeight;
        let el = js_text(&TextSpec::new("x").with_weight(TextWeight::Bold), &theme());
        assert_eq!(el.style.text_weight, Some(700));
    }

    #[test]
    fn compact_spacing_wraps_in_gapped_column() {
        // spacing="compact" wraps the label in a flex-column carrying the
        // resolved space.stack.sm gap; default spacing renders a bare label.
        let th = theme();
        let gap = resolve_px(&th, poodle_tokens::semantic::SPACE_STACK_SM);
        assert!(gap > 0.0, "stack-sm token resolves > 0");

        let plain = js_text(&TextSpec::new("p"), &th);
        let plain_tree = probe(&plain, 200.0, 80.0);
        assert_eq!(plain_tree.nodes[0].kind, "Label", "default is a bare label");

        let compact = js_text(
            &TextSpec::new("p").with_spacing(TextSpacing::Compact),
            &th,
        );
        let tree = probe(&compact, 200.0, 80.0);
        assert_eq!(tree.nodes[0].kind, "Panel", "compact wraps in a container");
        assert!(tree.has_text("p"), "label survives the wrap: {:?}", tree.texts());
    }
}
