//! RangeSlider — Jetstream range slider backed by RangeSliderSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::RangeSliderSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_range_slider(spec: &RangeSliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let accent = resolve_color(theme, spec.range_fill_token());
    let track_bg = resolve_color(theme, spec.track_fill_token());

    let mut el = ui_element::div()
        .h(20.0).grow()
        .flex_row().items_center();

    let track = ui_element::div()
        .h(4.0).grow().rounded(2.0).bg(track_bg);

    el = el.child(track);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
