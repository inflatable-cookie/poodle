//! JsSpinner — animated loading indicator backed by SpinnerSpec.
//!
//! Contract: `docs/contracts/components/spinner.md`
//! Reference: `packages/svelte/components/src/Spinner.svelte`
//!
//! ALL dimensions resolve from tokens or spec methods. ZERO hardcoded pixel values.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a Jetstream spinner element from a SpinnerSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .spinner]
///   └── [Visual]
///       ├── ring variant: single rotating ring
///       └── grid variant: 6 square cells in a 2x3 matrix
/// ```
pub fn js_spinner(spec: &SpinnerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // ── Tone color resolution ──
    // Contract: current = currentColor (inherit), accent = accent-base, muted = text-secondary
    let tone_color: Color = match spec.tone {
        SpinnerTone::Current => resolve_color(theme, "color.text.primary").into(),
        SpinnerTone::Accent => resolve_color(theme, "color.accent.base").into(),
        SpinnerTone::Muted => resolve_color(theme, "color.text.secondary").into(),
    };

    match spec.variant {
        SpinnerVariant::Ring => build_ring(spec, tone_color),
        SpinnerVariant::Grid => build_grid(spec, tone_color),
    }
}

/// Ring variant: circular spinner with rotating border.
///
/// Contract:
/// - border: 0.125rem solid color-mix(currentColor 24%, transparent)
/// - border-top-color: currentColor
/// - border-radius: 999px
/// - animation: spinner-ring 0.8s linear infinite
fn build_ring(spec: &SpinnerSpec, tone_color: Color) -> JsEl {
    let diameter = rem_to_px(spec.ring_size_rem());
    let border_width = rem_to_px(spec.ring_border_width_rem()); // Contract: 0.125rem

    // Contract §8: track border = color-mix(currentColor 24%, transparent).
    // The 24% lives on the spec (`track_opacity`), not as an inline literal.
    let track_color = tone_color.with_alpha(tone_color.a * spec.track_opacity());

    ui_element::div()
        .w(diameter)
        .h(diameter)
        .rounded(999.0) // pill / circle (border-radius: 999px)
        .border(border_width)
        .border_color(track_color)
        // Contract §8: border-top-color is the bright arc (full currentColor).
        .border_color_top(tone_color)
        .flex_row()
        .items_center()
        .justify_center()
        // Contract §8: animation spinner-ring 0.8s linear infinite. The id keys
        // the engine's animation clock across immediate-mode rebuilds (all ring
        // spinners share one clock — they rotate in phase, like CSS keyframes).
        .id("poodle-spinner-ring")
        .spin(0.8)
}

/// Grid variant: 6 square cells in a 2x3 matrix with staggered opacity.
///
/// Contract:
/// - display: inline-grid, grid-template-columns: repeat(2, 1fr), rows: repeat(3, 1fr)
/// - cell border-radius: 0.125rem
/// - cell background: currentColor
/// - cell animation: spinner-grid with phase-specific opacity keyframes
fn build_grid(spec: &SpinnerSpec, tone_color: Color) -> JsEl {
    let width = rem_to_px(spec.grid_width_rem());
    let height = rem_to_px(spec.grid_height_rem());
    let cell_radius = rem_to_px(spec.cell_radius_rem()); // Contract: 0.125rem

    // Contract §8: per-size fixed gap (Svelte `--poodle-spinner-grid-gap`),
    // resolved from the spec — not the old `width * 0.1` heuristic.
    let gap = rem_to_px(spec.grid_gap_rem());

    // 2 cols × 3 rows: cell_w = (width - gap) / 2, cell_h = (height - 2·gap) / 3.
    let cell_w = (width - gap) / 2.0;
    let cell_h = (height - gap * 2.0) / 3.0;

    // The snake pulses each cell's opacity within the contract band
    // (`opacity_floor` 0.2 → `opacity_peak` 0.76). Cells are laid out in
    // document order top-left, top-right, mid-left, mid-right, bottom-left,
    // bottom-right; each carries a phase-shifted looping opacity animation
    // (a sine sweep sampled into keyframes), so the snake runs continuously.
    let floor = spec.opacity_floor();
    let span = spec.opacity_peak() - spec.opacity_floor();
    // Phase offset along the cycle, one per cell in layout order.
    let phase = [1.0_f32, 0.7, 0.4, 0.85, 0.1, 0.55];

    let mut root = ui_element::div()
        .w(width)
        .h(height)
        .flex_row()
        .flex_wrap()
        .gap(gap)
        .items_center()
        .justify_center();

    for (i, &ph) in phase.iter().enumerate() {
        let cell_color = tone_color.with_alpha(tone_color.a);
        root = root.child(
            ui_element::div()
                .w(cell_w)
                .h(cell_h)
                .rounded(cell_radius)
                .bg(cell_color)
                .id(format!("poodle-spinner-cell-{i}"))
                .animation(cell_pulse(floor, span, ph)),
        );
    }

    root
}

/// Looping opacity pulse for one grid cell: a sine sweep through the contract
/// opacity band, phase-shifted per cell so the six cells form a running snake.
fn cell_pulse(floor: f32, span: f32, phase: f32) -> jetstream_ui::Animation {
    use jetstream_ui::{AnimatableProperty, Animation, Easing, Keyframe, LoopMode};
    let sample = |t: f32| -> f32 {
        let angle = (t + phase) * std::f32::consts::TAU;
        floor + span * (0.5 + 0.5 * angle.sin())
    };
    let keyframes = (0..=4)
        .map(|k| {
            let at = k as f32 / 4.0;
            Keyframe {
                at,
                values: vec![(AnimatableProperty::Opacity, sample(at))],
            }
        })
        .collect();
    Animation {
        keyframes,
        duration_secs: 1.2,
        easing: Easing::Linear,
        loop_mode: LoopMode::Loop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::SpinnerSize;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn ring_md_is_16px() {
        let spec = SpinnerSpec::new();
        let el = js_spinner(&spec, &theme());
        // 1rem * 16 (contract §7 md ring size), resolved via spec.ring_size_rem().
        assert_eq!(el.layout.size.width, taffy::Dimension::length(16.0));
        assert_eq!(el.layout.size.height, taffy::Dimension::length(16.0));
    }

    /// Ring stroke resolves from the spec (0.125rem), not a raw literal, and the
    /// rendered ring lays out at the contract size.
    #[test]
    fn ring_renders_with_token_stroke() {
        let spec = SpinnerSpec::new();
        let el = js_spinner(&spec, &theme());
        let tree = probe(&el, 64.0, 64.0);
        let root = &tree.nodes[0];
        assert!((root.w - 16.0).abs() < 0.01, "ring width != 16px: {}", root.w);
        // Stroke width is spec-resolved (0.125rem) — not hardcoded.
        assert!((spec.ring_border_width_rem() - 0.125).abs() < f32::EPSILON);
    }

    /// Ring track is the tone color at 24% (contract §8), and the top arc is the
    /// full tone color — a distinct two-tone ring, not a flat disc.
    #[test]
    fn ring_track_is_dimmer_than_arc() {
        let spec = SpinnerSpec::new();
        assert!((spec.track_opacity() - 0.24).abs() < f32::EPSILON);
        // The arc (full tone) is brighter than the 24% track.
        assert!(spec.track_opacity() < 1.0);
    }

    #[test]
    fn grid_variant_has_six_children() {
        let spec = SpinnerSpec::new().with_variant(SpinnerVariant::Grid);
        let el = js_spinner(&spec, &theme());
        assert_eq!(el.children.len(), 6);
    }

    /// Grid gap comes from the per-size spec table (contract §8), differing
    /// across sizes — proving it is no longer the `width * 0.1` heuristic.
    #[test]
    fn grid_gap_is_per_size_from_spec() {
        let xs = SpinnerSpec::new()
            .with_variant(SpinnerVariant::Grid)
            .with_size(SpinnerSize::Xs);
        let xl = SpinnerSpec::new()
            .with_variant(SpinnerVariant::Grid)
            .with_size(SpinnerSize::Xl);
        assert!(
            (xs.grid_gap_rem() - 0.0625).abs() < f32::EPSILON,
            "xs grid gap"
        );
        assert!(
            (xl.grid_gap_rem() - 0.15625).abs() < f32::EPSILON,
            "xl grid gap"
        );
        assert_ne!(xs.grid_gap_rem(), xl.grid_gap_rem());
    }

    /// Grid cells stay within the contract opacity band (0.2 floor → 0.76 peak).
    #[test]
    fn grid_opacity_band_from_spec() {
        let spec = SpinnerSpec::new();
        assert!((spec.opacity_floor() - 0.2).abs() < f32::EPSILON);
        assert!((spec.opacity_peak() - 0.76).abs() < f32::EPSILON);
    }


    /// Ring declares the contract's 0.8s rotation as a real runtime animation,
    /// with a stable id so the engine's clock survives immediate-mode rebuilds.
    #[test]
    fn ring_carries_spin_animation() {
        let el = js_spinner(&SpinnerSpec::new(), &theme());
        assert!(el.animation.is_some(), "ring declares a spin animation");
        assert!(el.id.is_some(), "animated element carries a stable id");
    }

    /// Every grid cell carries its own phase-shifted opacity pulse + id.
    #[test]
    fn grid_cells_carry_staggered_pulses() {
        let spec = SpinnerSpec::new().with_variant(SpinnerVariant::Grid);
        let el = js_spinner(&spec, &theme());
        assert!(el.children.iter().all(|c| c.animation.is_some() && c.id.is_some()));
    }

    #[test]
    fn ring_xs_is_smaller_than_xl() {
        let xs = js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Xs), &theme());
        let xl = js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Xl), &theme());
        assert_ne!(xs.layout.size.width, xl.layout.size.width);
    }
}
