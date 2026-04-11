//! JsStatusIndicator — colored dot with optional label, backed by StatusIndicatorSpec.
//!
//! Contract: `docs/contracts/components/status-indicator.md`
//! Reference: `packages/svelte/primitives/src/StatusIndicator.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::StatusIndicatorSpec;

use crate::presentation::rem_to_px;
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
    let text_primary = resolve_color(theme, "color.text.primary");

    // Contract: dot dimensions (via spec helper rem values → px)
    let dot_size = rem_to_px(spec.dot_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let label_size = rem_to_px(spec.label_font_size_rem());

    // Contract: dot with box-shadow glow (18% of status color)
    let dot = ui_element::div()
        .w(dot_size)
        .h(dot_size)
        .rounded(999.0) // circle
        .bg(status_color);
    // Note: box-shadow (0 0 0 0.125rem with 18% opacity) requires JsEl shadow support

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
            // Note: font-weight 600 and line-height 1.3 require runtime text support
        );
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::StatusTone;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn dot_is_9px() {
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
        // Neutral status should resolve to text-secondary, not accent-base
        let dot = &el.children[0];
        let expected: jetstream_runtime::game_ui::Color = resolve_color(&theme, "color.text.secondary").into();
        assert_eq!(dot.style.background, Some(expected));
    }
}
