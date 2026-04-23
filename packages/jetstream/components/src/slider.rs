//! Slider — Jetstream slider backed by SliderSpec.
//!
//! Contract: `docs/contracts/components/slider.md`
//! Reference: `packages/svelte/components/src/Slider.svelte`
//!
//! Track layout: fixed-width flex-row with two segments (fill + remainder)
//! so the filled portion reflects the actual fraction. Thumb is absolutely
//! positioned at the junction of fill and remainder.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SliderSpec;

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Fixed track width — 10 rem, matching the GPUI reference basis.
fn track_w() -> f32 {
    rem_to_px(10.0)
}

pub fn js_slider(spec: &SliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let thumb_size = rem_to_px(control_height_rem(effective_size) * 0.44);
    let track_h = rem_to_px(0.25);
    let container_h = rem_to_px(control_height_rem(effective_size) * 0.56);

    let accent: Color = resolve_color(theme, spec.range_fill_token()).into();
    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();

    // Contract: track bg = color-mix(surface 88%, accent)
    let track_bg = surface.mix(accent, 0.88);

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
        .rounded_l(track_h * 0.5);

    // Remainder segment: takes the rest of the track.
    let remainder = ui_element::div()
        .w(rem_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_r(track_h * 0.5);

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
        .rounded(thumb_r)
        .bg(elevated)
        .border(1.0)
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
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
