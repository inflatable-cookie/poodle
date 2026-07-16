//! Meter — Jetstream meter component backed by MeterSpec.
//!
//! Contract: `docs/contracts/components/meter.md`
//! Reference: `packages/svelte/components/src/Meter.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, MeterSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// Build a meter element from a MeterSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .meter]
///   ├── [Native <meter>] (semantics; not representable in JsEl)
///   └── [Track .meter__track]
///         └── [Fill .meter__fill]  (proportional to value)
/// ```
///
/// The proportional fill uses the runtime `ProgressBar` widget so the fill is
/// a true fraction of the parent-owned track width. The widget's fill color is the engine `status_success`,
/// matching contract §8 (`--poodle-color-status-success`). The track shell is
/// token-resolved here (`color-mix(surface 96%, text-primary)`).
pub fn js_meter(spec: &MeterSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Contract §8: track bg = color-mix(in srgb, surface 96%, text-primary).
    let surface = resolve_color(theme, spec.track_fill_token());
    let text_primary = resolve_color(theme, spec.track_mix_token());
    let track_bg = color_mix(surface, text_primary, spec.track_mix_ratio());

    // Contract §8: pill radius from the radius.pill token (not a 999 literal).
    let radius = resolve_radius(theme, "radius.pill");

    // Contract §8 Size Variants: track thickness resolves from the effective
    // size (size override → size_role against the inherited scale).
    let effective_size =
        resolve_semantic_size(spec.size.unwrap_or(ControlSize::Md), spec.size_role);
    let track_height = rem_to_px(spec.track_thickness_rem(effective_size));

    let fraction = spec.normalized_progress() as f32;

    // The ProgressBar widget draws a proportional fill of `fraction` over a
    // parent-owned (stretched) track. Width is owned by the parent (contract
    // §7: width 100%), not a hardcoded absolute.
    ui_element::progress(fraction)
        .w_full()
        .min_h(track_height)
        .self_stretch()
        .rounded(radius)
        .bg(track_bg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{ControlSize, SemanticControlSizeRole};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_progressbar_track_proportional_fill() {
        // The track must use the runtime ProgressBar widget so the fill is a
        // true fraction of the track (regression guard: the old impl used two
        // sibling divs over a fixed 10rem width).
        let el = js_meter(&MeterSpec::new().with_value(50.0), &theme());
        let tree = probe(&el, 300.0, 20.0);
        assert!(
            tree.nodes.iter().any(|n| n.kind == "ProgressBar"),
            "meter should render a ProgressBar widget; got {:?}",
            tree.nodes.iter().map(|n| n.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn track_background_is_surface_text_primary_mix() {
        let th = theme();
        let el = js_meter(&MeterSpec::new().with_value(50.0), &th);
        let tree = probe(&el, 300.0, 20.0);

        // Contract §8: track bg = color-mix(surface 96%, text-primary).
        let surface = resolve_color(&th, MeterSpec::new().track_fill_token());
        let text_primary = resolve_color(&th, MeterSpec::new().track_mix_token());
        let expected_vec = color_mix(surface, text_primary, 0.96);
        let expected = ProbeColor {
            r: expected_vec.x,
            g: expected_vec.y,
            b: expected_vec.z,
            a: expected_vec.w,
        };
        assert!(
            tree.has_background(expected, 0.01),
            "track bg should be surface↔text-primary 96% mix; nodes: {}",
            tree.to_json()
        );
    }

    #[test]
    fn track_thickness_scales_with_size() {
        // Contract §8 Size Variants: xs=0.25rem (4px), xl=0.75rem (12px).
        let xs = js_meter(
            &MeterSpec::new().with_value(50.0).with_size(ControlSize::Xs),
            &theme(),
        );
        assert_eq!(xs.layout.min_size.height, taffy::Dimension::length(4.0));

        let xl = js_meter(
            &MeterSpec::new().with_value(50.0).with_size(ControlSize::Xl),
            &theme(),
        );
        assert_eq!(xl.layout.min_size.height, taffy::Dimension::length(12.0));
    }

    #[test]
    fn md_track_thickness_is_8px() {
        // Default (md) thickness = 0.5rem = 8px (contract §8).
        let el = js_meter(&MeterSpec::new().with_value(50.0), &theme());
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(8.0));
    }

    #[test]
    fn size_role_resolves_against_scale() {
        // Prominent at md resolves one stop up → lg = 0.625rem = 10px.
        let el = js_meter(
            &MeterSpec::new()
                .with_value(50.0)
                .with_size(ControlSize::Md)
                .with_size_role(SemanticControlSizeRole::Prominent),
            &theme(),
        );
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(10.0));
    }

    #[test]
    fn fraction_clamps_and_normalizes_custom_range() {
        // Custom range 0–500, value 350 → 70%.
        let spec = MeterSpec::new().with_value(350.0).with_min(0.0).with_max(500.0);
        assert!((spec.normalized_progress() - 0.7).abs() < 1e-9);
    }
}
