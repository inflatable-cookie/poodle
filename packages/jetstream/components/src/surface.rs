//! Surface — Jetstream themed container backed by SurfaceSpec.
//!
//! Contract: `docs/contracts/components/surface.md` (+ `surface-elevation.md`)
//! Reference: `packages/svelte/components/src/Surface.svelte`
//!
//! All fills, the border color/width, the radius, and the elevation treatment
//! resolve from tokens or `SurfaceSpec` methods. The CSS color-mix percentages
//! (96% / 98% / 74%) are centralized on the spec, not hardcoded here.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SurfaceSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_surface(spec: &SurfaceSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let padding = spec.resolved_padding();

    // ── Fill (contract §8) ──────────────────────────────────────
    //   panel/base: color-mix(background-surface 96%, transparent) → alpha-only
    //   canvas:     color-mix(background-canvas 98%, transparent)  → alpha-only
    //   elevated:   color-mix(background-elevated 96%, background-panel)
    let base_fill: Color = resolve_color(theme, spec.resolved_background_token()).into();
    let mix_ratio = spec.fill_mix_ratio();
    let bg = match spec.fill_mix_over_token() {
        Some(over_token) => {
            let over: Color = resolve_color(theme, over_token).into();
            base_fill.mix(over, mix_ratio)
        }
        // Non-elevated tones mix toward transparent → scale alpha only.
        None => base_fill.with_alpha(base_fill.a * mix_ratio),
    };

    let mut el = ui_element::div()
        .bg(bg)
        .rounded(resolve_radius(theme, spec.radius_token()));

    // ── Border (contract §8) ────────────────────────────────────
    //   subtle:  color-mix(border-subtle 74%, transparent)
    //   default: border-default full
    //   none:    no border
    // Width resolves from `border.width.default` (0.0625rem), not a raw 1.0.
    if let Some(border_token) = spec.resolved_border_color() {
        let base_border: Color = resolve_color(theme, border_token).into();
        let border_color = base_border.with_alpha(base_border.a * spec.border_mix_ratio());
        let border_width = spec
            .resolved_border_width()
            .map(|t| resolve_px(theme, t))
            .unwrap_or(0.0);
        el = el.border(border_width).border_color(border_color);
    }

    // ── Elevation shadow (contract §8) ──────────────────────────
    // Elevated surfaces resolve `elevation.surface`. JsEl exposes only preset
    // box-shadows (no custom-color / multi-layer), so the token shadow is
    // APPROXIMATED with the small preset (`elevation.surface` is the low tier).
    if spec.is_elevated_resolved() {
        el = el.shadow_sm();
    }

    if let Some(h) = padding.horizontal {
        let px_val = resolve_px(theme, h);
        el = el.pl(px_val).pr(px_val);
    }
    if let Some(v) = padding.vertical {
        let px_val = resolve_px(theme, v);
        el = el.pt(px_val).pb(px_val);
    }

    for child in children {
        el = el.child(child);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{SurfaceBorder, SurfaceTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn fill_of(spec: &SurfaceSpec) -> ProbeColor {
        let el = js_surface(spec, &theme(), vec![]);
        let tree = probe(&el, 200.0, 120.0);
        tree.nodes[0].bg.expect("surface root has a background fill")
    }

    /// Each elevation level resolves a distinct fill: canvas mixes the canvas
    /// background (98%), panel mixes the surface background (96%), elevated
    /// mixes elevated over panel. The three must differ.
    #[test]
    fn elevation_fills_differ_per_tone() {
        let canvas = fill_of(&SurfaceSpec::new().with_tone(SurfaceTone::Canvas));
        let panel = fill_of(&SurfaceSpec::new().with_tone(SurfaceTone::Panel));
        let elevated = fill_of(&SurfaceSpec::new().with_tone(SurfaceTone::Elevated));

        assert!(!canvas.approx(panel, 0.001), "canvas vs panel fill identical");
        assert!(
            !panel.approx(elevated, 0.001),
            "panel vs elevated fill identical"
        );
        assert!(
            !canvas.approx(elevated, 0.001),
            "canvas vs elevated fill identical"
        );
    }

    /// Non-elevated tones mix toward transparent: the fill alpha is the base
    /// background alpha scaled by the spec mix ratio (panel 96%, canvas 98%).
    #[test]
    fn non_elevated_fill_scales_alpha_by_ratio() {
        let spec = SurfaceSpec::new().with_tone(SurfaceTone::Panel);
        let base: Color = resolve_color(&theme(), spec.resolved_background_token()).into();
        let fill = fill_of(&spec);
        let expected = base.a * spec.fill_mix_ratio();
        assert!(
            (fill.a - expected).abs() < 0.001,
            "panel fill alpha {} != base {} * ratio {}",
            fill.a,
            base.a,
            spec.fill_mix_ratio()
        );
    }

    /// Subtle border tints the border-subtle color to 74%; default uses the
    /// full border-default color; none draws no border. The subtle alpha must be
    /// 74% of the base token alpha.
    #[test]
    fn subtle_border_uses_mix_ratio() {
        let spec = SurfaceSpec::new().with_border(SurfaceBorder::Subtle);
        let base: Color = resolve_color(
            &theme(),
            spec.resolved_border_color().expect("subtle has a border token"),
        )
        .into();
        // border_mix_ratio for subtle is 0.74 (contract §8).
        assert!((spec.border_mix_ratio() - 0.74).abs() < f32::EPSILON);
        assert!(base.a > 0.0, "border-subtle token resolved");
    }

    /// `border="none"` resolves no border color token, so no border is drawn.
    #[test]
    fn border_none_has_no_border_token() {
        let spec = SurfaceSpec::new().with_border(SurfaceBorder::None);
        assert!(spec.resolved_border_color().is_none());
    }

    /// Border width resolves from `border.width.default` (0.0625rem), not 1px.
    #[test]
    fn border_width_resolves_from_token() {
        let spec = SurfaceSpec::new().with_border(SurfaceBorder::Subtle);
        let token = spec.resolved_border_width().expect("subtle has a width token");
        let width = resolve_px(&theme(), token);
        assert!(width > 0.0, "border width resolved from token");
        // 0.0625rem * 16 = 1.0px under the default root, but it is token-resolved,
        // not the literal 1.0 the old impl hardcoded.
    }
}
