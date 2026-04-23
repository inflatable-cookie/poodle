//! Meter — Jetstream meter component backed by MeterSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::MeterSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_meter(spec: &MeterSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let track = resolve_color(theme, spec.track_fill_token());

    // Contract: track height 0.5rem (8px), border-radius 999px (pill)
    let track_height = rem_to_px(spec.track_height_rem());
    // Fixed track width: 10rem — meter needs a known width to compute fill fraction
    let track_w = rem_to_px(10.0);

    let fraction = spec.normalized_progress() as f32;
    let fill_w = fraction * track_w;
    let remainder_w = (1.0 - fraction) * track_w;

    let fill_el = ui_element::div()
        .w(fill_w).h(track_height).bg(fill).rounded(999.0);

    let remainder_el = ui_element::div()
        .w(remainder_w).h(track_height);

    let track_el = ui_element::div()
        .w(track_w).h(track_height).rounded(999.0).bg(track)
        .flex_row()
        .child(fill_el)
        .child(remainder_el);

    ui_element::div()
        .grow().flex_row().items_center()
        .child(track_el)
}
