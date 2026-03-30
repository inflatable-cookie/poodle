//! MetricTile — Jetstream metric tile backed by MetricTileSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::MetricTileSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_metric_tile(spec: &MetricTileSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let radius = resolve_radius(theme, spec.radius_token());
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());
    let pad = resolve_px(theme, spec.padding_token());
    let gap = resolve_px(theme, spec.gap_token());

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad).pr(pad).pt(rem_to_px(0.75)).pb(rem_to_px(0.75))
        .flex_col().gap(gap);

    el = el.child(ui_element::label(&spec.label).text_color(label_color).text_size(rem_to_px(0.75)));
    el = el.child(ui_element::label(&spec.value).text_color(value_color).text_size(rem_to_px(1.5)).text_weight(700));

    el
}
