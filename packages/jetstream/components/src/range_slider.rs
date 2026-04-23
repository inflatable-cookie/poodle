//! RangeSlider — Jetstream range slider backed by RangeSliderSpec.
//!
//! Track layout: three fixed-width segments (lo-unfilled | filled | hi-unfilled)
//! side by side in a flex-row. Two thumbs are absolutely positioned at the
//! segment junctions so each thumb sits at the correct value position.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::RangeSliderSpec;

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Fixed track width — 10 rem, matching the single-thumb Slider basis.
fn track_w() -> f32 {
    rem_to_px(10.0)
}

pub fn js_range_slider(spec: &RangeSliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let thumb_size = rem_to_px(control_height_rem(effective_size) * 0.44);
    let track_h = rem_to_px(0.25);
    let container_h = rem_to_px(control_height_rem(effective_size) * 0.56);

    let accent: Color = resolve_color(theme, spec.range_fill_token()).into();
    let surface: Color = resolve_color(theme, spec.track_fill_token()).into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();

    // Contract: unfilled track bg = color-mix(surface 88%, accent)
    let track_bg = surface.mix(accent, 0.88);

    let range = (spec.max - spec.min).max(0.001);
    let lo = ((spec.low - spec.min) / range).clamp(0.0, 1.0) as f32;
    let hi = ((spec.high - spec.min) / range).clamp(0.0, 1.0) as f32;

    let tw = track_w();
    let thumb_r = thumb_size * 0.5;

    // Three track segments.
    let lo_w = lo * tw;
    let fill_w = ((hi - lo) * tw).max(0.0);
    let hi_w = ((1.0 - hi) * tw).max(0.0);

    let seg_lo = ui_element::div()
        .w(lo_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_l(track_h * 0.5);

    let seg_fill = ui_element::div()
        .w(fill_w)
        .h(track_h)
        .bg(accent);

    let seg_hi = ui_element::div()
        .w(hi_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_r(track_h * 0.5);

    // Thumbs absolutely positioned at segment junctions.
    // top: offset to center vertically on the track within the thumb_size-tall container.
    let thumb_top = -(thumb_r - track_h * 0.5);

    let thumb_lo = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left(lo_w - thumb_r)
        .w(thumb_size)
        .h(thumb_size)
        .rounded(thumb_r)
        .bg(elevated)
        .border(1.0)
        .border_color(border_default)
        .cursor_pointer();

    let thumb_hi = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left((lo_w + fill_w) - thumb_r)
        .w(thumb_size)
        .h(thumb_size)
        .rounded(thumb_r)
        .bg(elevated)
        .border(1.0)
        .border_color(border_default)
        .cursor_pointer();

    // Track row: relative container so thumbs are positioned within it.
    let track = ui_element::div()
        .w(tw)
        .h(thumb_size)
        .relative()
        .flex_row()
        .items_center()
        .child(seg_lo)
        .child(seg_fill)
        .child(seg_hi)
        .child(thumb_lo)
        .child(thumb_hi);

    let mut el = ui_element::div()
        .h(container_h)
        .grow()
        .flex_row()
        .items_center()
        .child(track);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
