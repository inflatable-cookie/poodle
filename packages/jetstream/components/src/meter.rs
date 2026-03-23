//! Meter — Jetstream meter component backed by MeterSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::MeterSpec;

use crate::theme_ext::resolve_color;

pub fn js_meter(spec: &MeterSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let track = resolve_color(theme, spec.track_fill_token());

    let range = (spec.max - spec.min).max(0.001);
    let fraction = ((spec.value - spec.min) / range).clamp(0.0, 1.0) as f32;

    let track_el = ui_element::div()
        .h(8.0).grow().rounded(4.0).bg(track)
        .flex_row();

    let fill_el = ui_element::div()
        .h(8.0).bg(fill).rounded(4.0);

    ui_element::div()
        .grow().flex_row().items_center()
        .child(track_el.child(fill_el))
}
