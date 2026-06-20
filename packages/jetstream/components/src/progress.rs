//! JsProgress — progress bar backed by ProgressSpec.
//!
//! Contract: `docs/contracts/components/progress.md`
//! Reference: `packages/svelte/components/src/Progress.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ProgressSpec;

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

    // Contract: indicator fills from left based on progress.
    match spec.normalized_progress() {
        Some(frac) => {
            // Determinate: the runtime ProgressBar widget fills `frac` of the
            // track proportionally (JsEl has no percentage sizing, so a
            // hand-built child div cannot do this — it would always fill 100%).
            // Track bg is token-resolved; the proportional fill is engine-drawn.
            ui_element::progress(frac as f32)
                .min_h(track_height)
                .self_stretch()
                .rounded(999.0) // pill
                .bg(track_bg)
        }
        None => {
            // Indeterminate: a 40%-width animated bar is a runtime capability
            // not yet exposed to JsEl (accepted delta); render a full accent bar.
            ui_element::div()
                .min_h(track_height)
                .self_stretch()
                .rounded(999.0)
                .bg(track_bg)
                .child(
                    ui_element::div()
                        .min_h(track_height)
                        .rounded(999.0)
                        .bg(accent),
                )
        }
    }
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
    fn determinate_renders_progressbar_widget() {
        // The determinate bar must use the runtime ProgressBar widget so the
        // fill is proportional to the value (regression: it used to render a
        // stretched child, so every value looked full).
        use crate::render_probe::probe;
        let el = js_progress(&ProgressSpec::new().with_value(0.5), &theme());
        let tree = probe(&el, 200.0, 20.0);
        assert!(
            tree.nodes.iter().any(|n| n.kind == "ProgressBar"),
            "determinate progress should render a ProgressBar widget; got {:?}",
            tree.nodes.iter().map(|n| n.kind).collect::<Vec<_>>()
        );
    }
}
