//! Slider — Jetstream slider backed by SliderSpec.
//!
//! Contract: `docs/contracts/components/slider.md`
//! Reference: `packages/svelte/primitives/src/Slider.svelte`
//!
//! Uses on_drag for thumb interaction and Color::mix for track background.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SliderSpec;

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

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

    // Track
    let track = ui_element::div()
        .h(track_h).grow().rounded(track_h * 0.5).bg(track_bg)
        .relative()
        .flex_row();

    // Fill: accent-colored portion from left
    let fill = ui_element::div()
        .h(track_h)
        .bg(accent)
        .rounded(track_h * 0.5);

    // Thumb
    let thumb = ui_element::div()
        .w(thumb_size).h(thumb_size)
        .rounded(thumb_size * 0.5)
        .bg(elevated)
        .border(1.0).border_color(border_default)
        .cursor_pointer();

    let mut el = ui_element::div()
        .h(container_h).grow()
        .flex_row().items_center()
        .relative()
        .child(
            track.child(fill).child(thumb)
        );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
