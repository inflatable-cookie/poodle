//! Meter — Jetstream meter component backed by MeterSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::MeterSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

pub fn js_meter(spec: &MeterSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let track = resolve_color(theme, spec.track_fill_token());

    // Contract: track height 0.5rem (8px), border-radius 999px (pill)
    let track_height = rem_to_px(spec.track_height_rem());

    let track_el = ui_element::div()
        .h(track_height).grow().rounded(999.0).bg(track)
        .flex_row();

    let fill_el = ui_element::div()
        .h(track_height).bg(fill).rounded(999.0);

    ui_element::div()
        .grow().flex_row().items_center()
        .child(track_el.child(fill_el))
}
