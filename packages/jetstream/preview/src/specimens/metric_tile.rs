//! MetricTile specimen — metric tiles with trend indicators.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::metric_tile::js_metric_tile;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{MetricTileSpec, MetricTrend};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Plain metrics — no trend
        .child(group("Plain (no trend)", secondary,
            div().flex_row().gap(12.0).items_start()
                .child(js_metric_tile(&MetricTileSpec::new("Total Users", "1,284"), theme))
                .child(js_metric_tile(&MetricTileSpec::new("Active", "947"), theme))
                .child(js_metric_tile(&MetricTileSpec::new("Revenue", "$12.4k"), theme))
        ))

        // With trend: up
        .child(group("Trend: Up", secondary,
            div().flex_row().gap(12.0).items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Plays", "84,203")
                        .with_trend(MetricTrend::Up)
                        .with_trend_label("+12.4%"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Downloads", "2,108")
                        .with_trend(MetricTrend::Up)
                        .with_trend_label("+8.1%"),
                    theme,
                ))
        ))

        // With trend: down
        .child(group("Trend: Down", secondary,
            div().flex_row().gap(12.0).items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Churn", "4.2%")
                        .with_trend(MetricTrend::Down)
                        .with_trend_label("-0.8%"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Errors", "17")
                        .with_trend(MetricTrend::Down)
                        .with_trend_label("-41%"),
                    theme,
                ))
        ))

        // With trend: flat
        .child(group("Trend: Flat", secondary,
            div().flex_row().gap(12.0).items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Retention", "73%")
                        .with_trend(MetricTrend::Flat)
                        .with_trend_label("No change"),
                    theme,
                ))
        ))

        // Trend without label
        .child(group("Trend without label", secondary,
            div().flex_row().gap(12.0).items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Sessions", "12,400")
                        .with_trend(MetricTrend::Up),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Bounces", "38%")
                        .with_trend(MetricTrend::Down),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
