//! RangeSlider — Jetstream range slider backed by RangeSliderSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::RangeSliderSpec;

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_range_slider(spec: &RangeSliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let container_h = rem_to_px(control_height_rem(effective_size) * 0.56);
    let track_h = rem_to_px(0.25);

    let accent = resolve_color(theme, spec.range_fill_token());
    let track_bg = resolve_color(theme, spec.track_fill_token());

    let mut el = ui_element::div()
        .h(container_h).grow()
        .flex_row().items_center();

    let track = ui_element::div()
        .h(track_h).grow().rounded(track_h * 0.5).bg(track_bg);

    el = el.child(track);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}
