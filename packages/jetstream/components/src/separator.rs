//! JsSeparator — horizontal or vertical divider backed by SeparatorSpec.
//!
//! Contract: `docs/contracts/components/separator.md`
//! Reference: `packages/svelte/components/src/Separator.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{SeparatorOrientation, SeparatorSpec};

use crate::theme_ext::{resolve_color, resolve_px, tint};

/// Build a separator element from a SeparatorSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .separator] — <div>
/// ```
///
/// Contract dimensions:
/// - Horizontal: width 100%, min-height 0.0625rem (1px)
/// - Vertical: width 0.0625rem (1px), align-self stretch, min-height 100%
/// - flex: 0 0 auto (no grow/shrink)
/// - Subtle tone: color-mix(border-subtle 72%, transparent)
/// - Default tone: border-default full color
pub fn js_separator(spec: &SeparatorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Contract: color depends on tone
    // - subtle: color-mix(border-subtle 72%, transparent)
    // - default: border-default full color
    // The subtle mix ratio is sourced from the spec (single source of truth),
    // not a magic float; Default tone resolves to ratio 1.0 (full color).
    let base = resolve_color(theme, spec.resolved_color());
    let color = tint(base, spec.subtle_mix_ratio());

    // Stroke width from the border-width-default token (0.0625rem = 1px),
    // resolved through the theme — not a hardcoded rem literal.
    let stroke = resolve_px(theme, spec.resolved_stroke_width());

    // `decorative` has no visual effect (contract §4) and no AX channel exists
    // in Jetstream; read it so the prop is not dead. Semantic (non-decorative)
    // separators are the structural-division case for any future AX hook.
    let _is_semantic = !spec.decorative;

    // Contract: flex 0 0 auto — separator doesn't grow or shrink
    match spec.orientation {
        SeparatorOrientation::Horizontal => {
            ui_element::div()
                .min_h(stroke)
                .self_stretch() // width 100%
                .bg(color)
                .flex_none() // flex: 0 0 auto
        }
        SeparatorOrientation::Vertical => {
            ui_element::div()
                .w(stroke)
                .h_full()   // min-height 100%
                .bg(color)
                .flex_none() // flex: 0 0 auto
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::RuleTone;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn horizontal_separator_has_bg() {
        let el = js_separator(&SeparatorSpec::new(), &theme());
        assert!(el.style.background.is_some());
    }

    /// Thickness comes from the border-width-default token (0.0625rem = 1px),
    /// resolved through the theme — horizontal min-height.
    #[test]
    fn horizontal_separator_thickness_from_token() {
        let stroke = resolve_px(&theme(), SeparatorSpec::new().resolved_stroke_width());
        assert_eq!(stroke, 1.0, "border-width-default did not resolve to 1px");
        let el = js_separator(&SeparatorSpec::new(), &theme());
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(stroke));
    }

    /// Vertical orientation: width is the token-resolved 1px, stretches in height.
    #[test]
    fn vertical_separator_thickness_from_token() {
        let stroke = resolve_px(&theme(), SeparatorSpec::new().resolved_stroke_width());
        let el = js_separator(
            &SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
            &theme(),
        );
        assert_eq!(el.layout.size.width, taffy::Dimension::length(stroke));
    }

    /// Subtle tone applies the 72% mix over border-subtle; the probed background
    /// alpha is below the full border-subtle alpha.
    #[test]
    fn subtle_tone_dims_alpha() {
        let full = resolve_color(&theme(), "color.border.subtle");
        let el = js_separator(&SeparatorSpec::new().with_tone(RuleTone::Subtle), &theme());
        let tree = probe(&el, 200.0, 20.0);
        let bg = tree.nodes[0].bg.expect("separator has no background");
        assert!(
            (bg.a - full.w * 0.72).abs() < 0.02,
            "subtle alpha not 72% of border-subtle: got {} vs {}",
            bg.a,
            full.w * 0.72
        );
    }

    /// Default tone uses the full border-default color at full alpha (ratio 1.0).
    #[test]
    fn default_tone_uses_full_border_default() {
        let bd = resolve_color(&theme(), "color.border.default");
        let expected = ProbeColor { r: bd.x, g: bd.y, b: bd.z, a: bd.w };
        let el = js_separator(&SeparatorSpec::new().with_tone(RuleTone::Default), &theme());
        let tree = probe(&el, 200.0, 20.0);
        assert!(
            tree.has_background(expected, 0.02),
            "default tone did not render full border-default color"
        );
    }
}
