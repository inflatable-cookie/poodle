//! JsProgress — progress bar backed by ProgressSpec.
//!
//! Contract: `docs/contracts/components/progress.md`
//! Reference: `packages/svelte/components/src/Progress.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use glam::Vec4;
use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ProgressSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color};

/// Build a progress bar element from a ProgressSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .progress] — <div role="progressbar">
///   └── [Indicator .progress__indicator] — <div>
/// ```
///
/// Contract dimensions:
/// - Track min-height: size-driven (xs/sm 0.375rem, md 0.5rem, lg/xl 0.75rem)
/// - Track border-radius: 999px (pill)
/// - Track background: color-mix(surface 96%, text-primary)
/// - Indicator background: linear-gradient(90deg, color-mix(accent 88%, white), accent)
/// - Determinate: proportional fill (runtime ProgressBar widget)
/// - Indeterminate: width 40%, animation translateX(-100% to 250%) (animation is a runtime delta)
pub fn js_progress(spec: &ProgressSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Effective size resolves the explicit `size` against the `size_role`.
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let accent = resolve_color(theme, spec.indicator_fill_token());

    // Contract §8 Root: track bg = color-mix(surface 96%, text-primary).
    // Mix ratio + both endpoints are spec-owned (mirrors GPUI).
    let surface = resolve_color(theme, spec.track_fill_token());
    let track_mix = resolve_color(theme, spec.track_mix_token());
    let track_bg = color_mix(surface, track_mix, spec.track_mix_ratio());

    // Contract §8 Indicator gradient: leading stop = color-mix(accent 88%, white),
    // trailing stop = accent.
    let gradient_lead = color_mix(accent, Vec4::ONE, spec.indicator_gradient_accent_ratio());
    let gradient_stops: Vec<(Color, f32)> =
        vec![(gradient_lead.into(), 0.0), (accent.into(), 1.0)];

    // Contract §8 Size Variants — track min-height ladder owned by the spec.
    let track_height = rem_to_px(ProgressSpec::min_height_rem(effective_size));

    // Contract: indicator fills from left based on progress.
    match spec.normalized_progress() {
        Some(frac) => {
            // Determinate: the runtime ProgressBar widget fills `frac` of the
            // track proportionally (JsEl has no percentage child sizing). `bg`
            // sets the track; the accent fill is engine-drawn from the widget's
            // fill fraction. NOTE: the contract §8 indicator gradient is not
            // applied here — on the ProgressBar widget the `background_gradient`
            // channel and the fill-fraction share one GPU quad, so forcing a
            // gradient would recolor the whole track, not just the fill. The
            // accent fill is a solid runtime color (accepted JsEl delta); the
            // gradient is honored on the indeterminate bar below.
            ui_element::progress(frac as f32)
                .min_h(track_height)
                .self_stretch()
                .rounded(999.0) // pill
                .bg(track_bg)
        }
        None => {
            // Indeterminate: contract §8 static treatment is a 40%-width bar
            // (continuous slide animation is a runtime delta, §12). JsEl has no
            // percentage child sizing, so the 40/60 split is expressed via flex
            // grow factors (bar 0.4, trailing spacer 0.6) — exact regardless of
            // the parent-owned track width.
            let mut bar = ui_element::div()
                .min_h(track_height)
                .rounded(999.0)
                .bg_gradient_linear(90.0, gradient_stops);
            bar.layout.flex_grow = INDETERMINATE_BAR_WIDTH_FRAC;

            let mut spacer = ui_element::div().min_h(track_height);
            spacer.layout.flex_grow = 1.0 - INDETERMINATE_BAR_WIDTH_FRAC;

            ui_element::div()
                .min_h(track_height)
                .self_stretch()
                .rounded(999.0)
                .bg(track_bg)
                .flex_row()
                .child(bar)
                .child(spacer)
        }
    }
}

/// Indeterminate bar width as a fraction of the track. Contract §8
/// Indeterminate: `width: 40%`. Expressed as a flex-grow factor against a
/// trailing spacer since JsEl has no percentage child sizing.
const INDETERMINATE_BAR_WIDTH_FRAC: f32 = 0.4;

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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

    #[test]
    fn track_height_follows_size_ladder() {
        use poodle_specs::ControlSize;
        // Contract §8 Size Variants: xs/sm 0.375rem (6px), md 0.5rem (8px),
        // lg/xl 0.75rem (12px). Previously every size rendered at 8px.
        let cases = [
            (ControlSize::Xs, 6.0),
            (ControlSize::Sm, 6.0),
            (ControlSize::Md, 8.0),
            (ControlSize::Lg, 12.0),
            (ControlSize::Xl, 12.0),
        ];
        for (size, expected) in cases {
            let el = js_progress(
                &ProgressSpec::new().with_value(50.0).with_size(size),
                &theme(),
            );
            assert_eq!(
                el.layout.min_size.height,
                taffy::Dimension::length(expected),
                "size {size:?} should render {expected}px track"
            );
        }
    }

    #[test]
    fn track_background_is_spec_resolved_mix() {
        // Contract §8 Root: color-mix(surface 96%, text-primary). The track bg
        // must be the spec-owned mix, not a translucent surface tint.
        use crate::render_probe::{probe, ProbeColor};
        let th = theme();
        let spec = ProgressSpec::new().with_value(50.0);
        let surface = resolve_color(&th, spec.track_fill_token());
        let mix = resolve_color(&th, spec.track_mix_token());
        let expected = color_mix(surface, mix, spec.track_mix_ratio());

        let el = js_progress(&spec, &th);
        let tree = probe(&el, 200.0, 20.0);
        let want = ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        assert!(
            tree.has_background(want, 0.001),
            "track bg should be the spec-resolved surface/text-primary mix; \
             want {want:?}, tree: {}",
            tree.to_json()
        );
    }

    #[test]
    fn indeterminate_renders_partial_width_bar() {
        // Contract §8 Indeterminate: a 40%-width bar (distinguishable from a
        // complete determinate bar). Built as a flex row: an accent bar (grow
        // 0.4) plus a trailing spacer (grow 0.6), so the bar is a real child
        // and not a full-width fill.
        use crate::render_probe::probe;
        let el = js_progress(&ProgressSpec::new().with_indeterminate(true), &theme());
        let tree = probe(&el, 200.0, 20.0);

        // No ProgressBar widget on the indeterminate path.
        assert_eq!(
            tree.count_kind("ProgressBar"),
            0,
            "indeterminate should not use the ProgressBar widget"
        );
        // Two children of the 200px track: an accent bar at ~40% (≈80px) and a
        // trailing spacer at ~60% (≈120px). The 40/60 split proves the bar is a
        // partial-width affordance, not a full-width fill.
        let child_widths: Vec<f32> = tree.nodes[1..].iter().map(|n| n.w).collect();
        assert!(
            child_widths.iter().any(|w| (*w - 80.0).abs() < 1.0),
            "indeterminate bar should be ~40% (≈80px) of a 200px track; child widths: {child_widths:?}"
        );
        assert!(
            child_widths.iter().any(|w| (*w - 120.0).abs() < 1.0),
            "trailing spacer should be ~60% (≈120px); child widths: {child_widths:?}"
        );
    }
}
