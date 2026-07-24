//! JsStatusIndicator — colored dot with optional label, backed by StatusIndicatorSpec.
//!
//! Contract: `docs/contracts/components/status-indicator.md`
//! Reference: `packages/svelte/components/src/StatusIndicator.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, StatusIndicatorSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::resolve_color;

/// Build a status indicator element from a StatusIndicatorSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .status-indicator] — <span>, inline-flex
///   ├── [Dot .status-indicator__dot] — <span>
///   └── [Label .status-indicator__label] — <span> (optional)
/// ```
///
/// Contract dimensions:
/// - Dot: 0.5625rem (9px) square, border-radius 999px
/// - Dot box-shadow: 0 0 0 0.125rem color-mix(status-color 18%, transparent)
/// - Gap: 0.4375rem (7px)
/// - Label font-size: 0.75rem (12px), font-weight 600, line-height 1.3
pub fn js_status_indicator(spec: &StatusIndicatorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let status_color = resolve_color(theme, spec.status_color_token());
    let text_primary = resolve_color(theme, spec.label_color_token());

    // Contract §8: dot/gap/label metrics resolve from the effective size
    // (size override → size_role against the inherited scale) and density.
    let effective_size =
        resolve_semantic_size(spec.size.unwrap_or(ControlSize::Md), spec.size_role);
    let effective_density = spec.density.unwrap_or(ControlDensity::Default);

    let dot_size = rem_to_px(spec.dot_size_rem_for(effective_size));
    let gap = rem_to_px(spec.gap_rem_for(effective_size, effective_density));
    let label_size = rem_to_px(spec.label_font_size_rem_for(effective_size));

    // Contract: dot with box-shadow glow (18% of status color)
    let dot = ui_element::div()
        .w(dot_size)
        .h(dot_size)
        .rounded(999.0) // circle
        .bg(status_color);
    // Note: box-shadow (0 0 0 0.125rem with 18% opacity) requires JsEl shadow support.
    // Label line-height and richer text metrics are also not exposed yet, so
    // those remain documented runtime deltas for Jetstream.

    // Root: inline-flex, gap, min-width 0
    let mut root = ui_element::div()
        .flex_row()
        .gap(gap)
        .items_center()
        .child(dot);

    // Contract: optional label
    if let Some(ref label_text) = spec.label {
        root = root.child(
            ui_element::label(label_text)
                .text_color(text_primary)
                .text_size(label_size)
                .text_weight(600)
            // Note: line-height 1.3 still requires richer runtime text support.
        );
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ControlSize, StatusTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn probe_color(theme: &JetstreamThemeProvider, token: &str) -> ProbeColor {
        let c = resolve_color(theme, token);
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    #[test]
    fn dot_is_9px_at_md() {
        let el = js_status_indicator(
            &StatusIndicatorSpec::new().with_status(StatusTone::Success),
            &theme(),
        );
        let dot = &el.children[0];
        assert_eq!(dot.layout.size.width, taffy::Dimension::length(9.0));
        assert_eq!(dot.layout.size.height, taffy::Dimension::length(9.0));
    }

    #[test]
    fn neutral_uses_text_secondary_color() {
        let theme = theme();
        let el = js_status_indicator(
            &StatusIndicatorSpec::new().with_status(StatusTone::Neutral),
            &theme,
        );
        // Neutral status resolves to text-secondary, not accent-base.
        let dot = &el.children[0];
        let expected: jetstream_ui::Color =
            resolve_color(&theme, "color.text.secondary").into();
        assert_eq!(dot.style.background, Some(expected));
    }

    #[test]
    fn tone_variants_resolve_distinct_status_colors() {
        // Contract §4: each tone maps to its status token.
        let th = theme();
        let cases = [
            (StatusTone::Info, "color.status.info"),
            (StatusTone::Success, "color.status.success"),
            (StatusTone::Warning, "color.status.warning"),
            (StatusTone::Danger, "color.status.danger"),
            (StatusTone::Pending, "color.accent.base"),
        ];
        for (tone, token) in cases {
            let el = js_status_indicator(&StatusIndicatorSpec::new().with_status(tone), &th);
            let tree = probe(&el, 120.0, 24.0);
            let expected = probe_color(&th, token);
            assert!(
                tree.has_background(expected, 0.01),
                "tone {tone:?} dot should resolve {token}; nodes: {}",
                tree.to_json()
            );
        }
    }

    #[test]
    fn label_text_renders() {
        let el = js_status_indicator(
            &StatusIndicatorSpec::new()
                .with_status(StatusTone::Success)
                .with_label("Online"),
            &theme(),
        );
        let tree = probe(&el, 160.0, 24.0);
        assert!(
            tree.has_text("Online"),
            "label text missing; texts: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn dot_size_scales_with_size() {
        // Contract §8: xs dot = 0.375rem (6px), xl dot = 0.8125rem (13px).
        let xs = js_status_indicator(
            &StatusIndicatorSpec::new()
                .with_status(StatusTone::Success)
                .with_size(ControlSize::Xs),
            &theme(),
        );
        assert_eq!(xs.children[0].layout.size.width, taffy::Dimension::length(6.0));

        let xl = js_status_indicator(
            &StatusIndicatorSpec::new()
                .with_status(StatusTone::Success)
                .with_size(ControlSize::Xl),
            &theme(),
        );
        assert_eq!(xl.children[0].layout.size.width, taffy::Dimension::length(13.0));
    }
}
