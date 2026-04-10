//! JsProgress — progress bar backed by ProgressSpec.
//!
//! Contract: `docs/contracts/foundation/progress.md`
//! Reference: `packages/svelte/primitives/src/Progress.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::ProgressSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, tint};

/// Build a progress bar element from a ProgressSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .progress] — <div role="progressbar">
///   └── [Indicator .progress__indicator] — <div>
/// ```
///
/// Contract dimensions:
/// - Track min-height: 0.5rem (8px)
/// - Track border-radius: 999px (pill)
/// - Track background: color-mix(surface 80%, elevated)
/// - Indicator background: linear-gradient(90deg, color-mix(accent 88%, white), accent)
/// - Determinate: scaleX(percentage)
/// - Indeterminate: width 40%, animation translateX(-100% to 250%)
pub fn js_progress(spec: &ProgressSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Resolve effective size for presentation consistency (size_role wiring).
    // Track height is currently fixed per contract but the plumbing is ready.
    let _effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let accent = resolve_color(theme, spec.indicator_fill_token());
    let surface = resolve_color(theme, "color.background.surface");

    // Contract: track bg = color-mix(surface 80%, elevated)
    let track_bg = tint(surface, 0.80);

    // Contract: track min-height 0.5rem = 8px
    let track_height = rem_to_px(0.5);

    // Contract: min-height 0.5rem = 8px, border-radius 999px
    let mut track = ui_element::div()
        .min_h(track_height)
        .self_stretch()
        .rounded(999.0) // pill
        .bg(track_bg);

    // Contract: indicator fills from left based on progress
    let progress = spec.normalized_progress();

    if let Some(_frac) = progress {
        // Determinate: indicator fills proportionally
        // Contract: scaleX(percentage) with transform-origin: left
        // In the current JsEl model, we render a child with accent fill.
        // Proper percentage-based width requires Widget::ProgressBar support.
        track = track.child(
            ui_element::div()
                .min_h(track_height)  // match track height
                .rounded(999.0)
                .bg(accent)
                .self_stretch()
        );
    } else {
        // Indeterminate: contract specifies width 40% with animation
        // Animation is a runtime capability not yet in JsEl.
        track = track.child(
            ui_element::div()
                .min_h(track_height)
                .rounded(999.0)
                .bg(accent)
        );
    }

    track
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn track_min_height_is_8px() {
        let el = js_progress(&ProgressSpec::new().with_value(0.5), &theme());
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(8.0));
    }

    #[test]
    fn determinate_has_indicator_child() {
        let el = js_progress(&ProgressSpec::new().with_value(0.5), &theme());
        assert_eq!(el.children.len(), 1);
    }
}
