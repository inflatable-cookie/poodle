//! MetricTile specimen — single metric display tile.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::metric_tile::js_metric_tile;
use pug_jetstream_components::theme_ext::*;
use pug_composites::MetricTileSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Metrics", secondary,
            div().flex_row().gap(12.0)
                .child(js_metric_tile(&MetricTileSpec::new("Total Users", "1,284"), theme))
                .child(js_metric_tile(&MetricTileSpec::new("Active", "947"), theme))
                .child(js_metric_tile(&MetricTileSpec::new("Revenue", "$12.4k"), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
