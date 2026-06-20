//! Slider — Jetstream slider backed by SliderSpec.
//!
//! Contract: `docs/contracts/components/slider.md`
//! Reference: `packages/svelte/components/src/Slider.svelte`
//!
//! Track layout: fixed-width flex-row with two segments (fill + remainder)
//! so the filled portion reflects the actual fraction. Thumb is absolutely
//! positioned at the junction of fill and remainder.
//!
//! All dimensions resolve from the contract §8 size table — no invented ratios.
//! Drag/keyboard interaction is preview-event-loop bound; this renders the
//! track + fill + thumb at the spec's current value only.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, SliderSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// Fixed track length — 10 rem, matching the GPUI reference basis.
fn track_w() -> f32 {
    rem_to_px(10.0)
}

/// Thumb diameter in rem per the contract §8 size table.
fn thumb_diameter_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Root min-height in rem per the contract §8 size table.
/// (lg/xl inherit the md base of 1.5rem.)
fn min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.375,
        ControlSize::Md | ControlSize::Lg | ControlSize::Xl => 1.5,
    }
}

pub fn js_slider(spec: &SliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8: thumb diameter + min-height from the size table,
    // track thickness fixed at 0.375rem.
    let thumb_size = rem_to_px(thumb_diameter_rem(effective_size));
    let track_h = rem_to_px(0.375);
    let container_h = rem_to_px(min_height_rem(effective_size));

    // Pill radius (contract: 999px full-pill) and thumb border (0.0625rem).
    let pill = resolve_radius(theme, "radius.pill");
    let border_w = rem_to_px(0.0625);

    let accent: Color = resolve_color(theme, spec.range_fill_token()).into();
    let surface = resolve_color(theme, "color.background.surface");
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();

    // Contract §8 track bg = color-mix(surface 88%, transparent):
    // mix surface toward transparency (alpha 0.88), NOT toward accent.
    let track_bg: Color = tint(surface, 0.88).into();

    let range = (spec.max - spec.min).max(0.001);
    let fraction = ((spec.value - spec.min) / range).clamp(0.0, 1.0) as f32;

    let tw = track_w();
    let fill_w = fraction * tw;
    // Remaining track width — never negative.
    let rem_w = (tw - fill_w).max(0.0);

    let thumb_r = thumb_size * 0.5;

    // Fill segment: left portion in accent color.
    let fill = ui_element::div()
        .w(fill_w)
        .h(track_h)
        .bg(accent)
        .rounded_l(pill);

    // Remainder segment: takes the rest of the track.
    let remainder = ui_element::div()
        .w(rem_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_r(pill);

    // Thumb: absolutely positioned at the fill/remainder junction.
    // top offsets the thumb vertically to center on the track.
    let thumb_top = -(thumb_r - track_h * 0.5);
    let thumb_left = fill_w - thumb_r;
    let thumb = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left(thumb_left)
        .w(thumb_size)
        .h(thumb_size)
        .rounded(pill)
        .bg(elevated)
        .border(border_w)
        .border_color(border_default)
        .cursor_pointer();

    // Track row: relative container holding fill, remainder, and thumb.
    let track = ui_element::div()
        .w(tw)
        .h(thumb_size)
        .relative()
        .flex_row()
        .items_center()
        .child(fill)
        .child(remainder)
        .child(thumb);

    let mut el = ui_element::div()
        .h(container_h)
        .grow()
        .flex_row()
        .items_center()
        .child(track);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn vec4_to_probe(c: glam::Vec4) -> ProbeColor {
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    #[test]
    fn unfilled_track_mixes_toward_transparent_not_accent() {
        let th = theme();
        // Mid value so both fill and remainder segments are present.
        let el = js_slider(&SliderSpec::new(50.0), &th);
        let tree = probe(&el, 200.0, 40.0);

        let accent = resolve_color(&th, "color.accent.base");
        let surface = resolve_color(&th, "color.background.surface");

        // Contract: unfilled track = color-mix(surface 88%, transparent).
        let expected_track = vec4_to_probe(tint(surface, 0.88));
        let accent_probe = vec4_to_probe(accent);

        // The unfilled track color must be present...
        assert!(
            tree.has_background(expected_track, 0.01),
            "unfilled track should be surface mixed toward transparent (alpha 0.88): {}",
            tree.to_json()
        );
        // ...and it must NOT equal the accent (the original bug mixed toward accent).
        assert!(
            !expected_track.approx(accent_probe, 0.02),
            "unfilled track must not be the accent color"
        );
        // The track's alpha is the transparency mix, not opaque.
        assert!(
            (expected_track.a - 0.88).abs() < 0.001,
            "unfilled track alpha should be 0.88 (surface mixed toward transparent)"
        );

        // The filled portion must be the opaque accent.
        assert!(
            tree.has_background(accent_probe, 0.01),
            "filled track segment should be the accent color: {}",
            tree.to_json()
        );
    }

    #[test]
    fn thumb_and_track_sizes_match_contract_table() {
        let th = theme();
        // md thumb = 1rem = 16px, track thickness = 0.375rem = 6px,
        // min-height = 1.5rem = 24px.
        let el = js_slider(&SliderSpec::new(50.0).with_size(ControlSize::Md), &th);
        // Root container height = min-height (size role Control = identity for md).
        assert_eq!(
            el.layout.size.height,
            taffy::Dimension::length(rem_to_px(1.5))
        );

        // xs thumb = 0.75rem = 12px from the size table (not an invented ratio).
        let el_xs = js_slider(
            &SliderSpec::new(50.0)
                .with_size(ControlSize::Xs)
                .with_size_role(poodle_specs::SemanticControlSizeRole::Control),
            &th,
        );
        let tree = probe(&el_xs, 200.0, 40.0);
        // The thumb is the only square node of width == height == 12px.
        let has_xs_thumb = tree
            .nodes
            .iter()
            .any(|n| (n.w - 12.0).abs() < 0.5 && (n.h - 12.0).abs() < 0.5);
        assert!(
            has_xs_thumb,
            "xs thumb should be 0.75rem (12px) per the size table: {}",
            tree.to_json()
        );
    }
}
