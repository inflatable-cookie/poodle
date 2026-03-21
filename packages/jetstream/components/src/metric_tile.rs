//! MetricTile — Jetstream metric tile backed by MetricTileSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_composites::MetricTileSpec;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_metric_tile(spec: &MetricTileSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "semantic.color.border.subtle");
    let radius = resolve_radius(theme, spec.radius_token());
    let label_color = resolve_color(theme, spec.label_color_token());
    let value_color = resolve_color(theme, spec.value_color_token());

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(16.0).pr(16.0).pt(12.0).pb(12.0)
        .flex_col().gap(4.0);

    el = el.child(ui_element::label(&spec.label).text_color(label_color).text_size(12.0));
    el = el.child(ui_element::label(&spec.value).text_color(value_color).text_size(24.0).text_weight(700));

    el
}
