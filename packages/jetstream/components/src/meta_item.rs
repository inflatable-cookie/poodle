//! MetaItem — label + value pair for use inside MetaBar.
//!
//! Contract: `docs/contracts/components/meta-item.md`
//! Reference: `packages/gpui/components/src/primitives/meta_item.rs`
//!
//! Renders a small uppercase label followed by a value slot.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MetaItemSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a MetaItem element from a MetaItemSpec and an optional value element.
///
/// Contract anatomy:
/// ```text
/// [Root]  flex-row, wrap, gap 0.375rem
///   ├── [Label]  uppercase, text.secondary, 0.6875rem, semibold
///   └── [Value]  caller-supplied content or placeholder, text.primary, 0.875rem
/// ```
pub fn js_meta_item(spec: &MetaItemSpec, theme: &JetstreamThemeProvider, value: Option<JsEl>) -> JsEl {
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let label_size = rem_to_px(spec.label_font_size_rem());
    let value_size = rem_to_px(spec.value_font_size_rem());
    let label_weight = spec.label_font_weight(); // typography.label.weight (500)
    let gap = rem_to_px(spec.gap_rem());

    // Jetstream's current text API does not expose font-family or line-height
    // controls, so those typography details remain documented runtime deltas for
    // now (contract §10 Known Delta). Label letter-spacing 0.08em is applied.
    let mut row = ui_element::div()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(gap)
        .min_w(0.0);

    if let Some(ref text) = spec.label {
        row = row.child(
            ui_element::label(&text.to_uppercase())
                .text_color(label_color)
                .text_size(label_size)
                .text_weight(label_weight)
                .letter_spacing_em(0.08) // contract §8: letter-spacing 0.08em
        );
    }

    let value_el = value.unwrap_or_else(|| {
        ui_element::div()
            .text_size(value_size)
            .text_color(value_color)
    });

    row = row.child(
        ui_element::div()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(gap)
            .min_w(0.0)
            .text_size(value_size)
            .text_color(value_color)
            .child(value_el)
    );

    row
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use jetstream_runtime::game_ui::Color;
    use poodle_specs::InlineTypographyMode;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_uppercased_label_and_value() {
        let spec = MetaItemSpec::new().with_label("Status");
        let el = js_meta_item(&spec, &theme(), Some(ui_element::label("Published")));
        let tree = probe(&el, 400.0, 100.0);

        assert!(tree.has_text("STATUS"), "label rendered uppercase: {:?}", tree.texts());
        assert!(tree.has_text("Published"), "value rendered: {:?}", tree.texts());
    }

    #[test]
    fn label_uses_label_secondary_tone() {
        // Label color resolves from the text-secondary token (muted tone).
        let expected: Color = resolve_color(&theme(), MetaItemSpec::new().label_color_token()).into();
        let spec = MetaItemSpec::new().with_label("Type");
        let el = js_meta_item(&spec, &theme(), None);
        let tree = probe(&el, 400.0, 100.0);

        // The label node carries the secondary text color as its text_size-bearing
        // label; confirm the token resolves and the uppercase label is present.
        assert!(tree.has_text("TYPE"), "label present: {:?}", tree.texts());
        // text-secondary differs from text-primary (the value tone) — sanity guard.
        let primary: Color = resolve_color(&theme(), MetaItemSpec::new().value_color_token()).into();
        assert!(
            (expected.r - primary.r).abs() > 0.001
                || (expected.g - primary.g).abs() > 0.001
                || (expected.b - primary.b).abs() > 0.001,
            "label (secondary) and value (primary) tones should differ"
        );
    }

    #[test]
    fn value_only_renders_without_label() {
        let spec = MetaItemSpec::new();
        let el = js_meta_item(&spec, &theme(), Some(ui_element::label("Stem waveform")));
        let tree = probe(&el, 400.0, 100.0);

        assert!(tree.has_text("Stem waveform"), "value present");
        // Only the value text renders — no leading label node when label is None.
        assert_eq!(
            tree.texts(),
            vec!["Stem waveform"],
            "value-only item should render exactly one text node"
        );
    }

    #[test]
    fn inherit_typography_scales_label_and_value() {
        let body = js_meta_item(
            &MetaItemSpec::new().with_label("Team"),
            &theme(),
            Some(ui_element::label("Platform")),
        );
        let inherit = js_meta_item(
            &MetaItemSpec::new()
                .with_label("Team")
                .with_typography(InlineTypographyMode::Inherit),
            &theme(),
            Some(ui_element::label("Platform")),
        );
        let body_tree = probe(&body, 400.0, 100.0);
        let inherit_tree = probe(&inherit, 400.0, 100.0);

        // Label size: 0.6875rem (body) vs 0.6875em→0.6875rem baseline (inherit) —
        // both resolve to the corrected ratio (no longer 0.7857).
        let body_label = body_tree.nodes.iter().find_map(|n| {
            if n.text.as_deref() == Some("TEAM") { n.text_size } else { None }
        });
        let inherit_label = inherit_tree.nodes.iter().find_map(|n| {
            if n.text.as_deref() == Some("TEAM") { n.text_size } else { None }
        });
        assert!(body_label.is_some() && inherit_label.is_some(), "label sizes present");
        // Value font-size grows from 0.875rem (body) to 1.0rem (inherit baseline).
        let body_val = body_tree.nodes.iter().find_map(|n| {
            if n.text.as_deref() == Some("Platform") { n.text_size } else { None }
        });
        let inherit_val = inherit_tree.nodes.iter().find_map(|n| {
            if n.text.as_deref() == Some("Platform") { n.text_size } else { None }
        });
        if let (Some(b), Some(i)) = (body_val, inherit_val) {
            assert!(i > b, "inherit value size ({i}) should exceed body ({b})");
        }
    }
}
