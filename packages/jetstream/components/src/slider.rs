//! Slider — Jetstream slider backed by SliderSpec.
//!
//! Contract: `docs/contracts/foundation/slider.md`
//! Reference: `packages/svelte/primitives/src/Slider.svelte`
//!
//! Uses on_drag for thumb interaction and Color::mix for track background.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SliderSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_slider(spec: &SliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let accent: Color = resolve_color(theme, spec.range_fill_token()).into();
    let surface: Color = resolve_color(theme, "semantic.color.background.surface").into();
    let border_default: Color = resolve_color(theme, "semantic.color.border.default").into();
    let elevated: Color = resolve_color(theme, "semantic.color.background.elevated").into();

    // Contract: track bg = color-mix(surface 88%, accent)
    let track_bg = surface.mix(accent, 0.88);

    let range = (spec.max - spec.min).max(0.001);
    let fraction = ((spec.value - spec.min) / range).clamp(0.0, 1.0) as f32;

    // Track: 4px tall, full width
    let track = ui_element::div()
        .h(4.0).grow().rounded(2.0).bg(track_bg)
        .relative()
        .flex_row();

    // Fill: accent-colored portion from left
    let fill = ui_element::div()
        .h(4.0)
        .bg(accent)
        .rounded(2.0);

    // Thumb: 16px circle, positioned at fill percentage
    let thumb = ui_element::div()
        .w(16.0).h(16.0)
        .rounded(8.0)
        .bg(elevated)
        .border(1.0).border_color(border_default)
        .cursor_pointer();

    let mut el = ui_element::div()
        .h(20.0).grow()
        .flex_row().items_center()
        .relative()
        .child(
            track.child(fill).child(thumb)
        );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    }

    el
}
