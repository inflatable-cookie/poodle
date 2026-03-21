//! Slider — Jetstream slider backed by SliderSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::SliderSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, tint};

pub fn js_slider(spec: &SliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let accent = resolve_color(theme, spec.range_fill_token());
    let track_bg = resolve_color(theme, "semantic.color.background.surface");
    let track_tinted = tint(track_bg, 0.8);

    let range = (spec.max - spec.min).max(0.001);
    let fraction = ((spec.value - spec.min) / range).clamp(0.0, 1.0) as f32;

    let mut el = ui_element::div()
        .h(20.0).grow()
        .flex_row().items_center();

    // Track
    let track = ui_element::div()
        .h(4.0).grow().rounded(2.0).bg(track_tinted)
        .flex_row();

    // Filled portion (accent)
    let filled_width = (fraction * 100.0).max(0.0);
    let filled = ui_element::div()
        .h(4.0)
        .bg(accent)
        .rounded(2.0);

    el = el.child(track.child(filled));

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
