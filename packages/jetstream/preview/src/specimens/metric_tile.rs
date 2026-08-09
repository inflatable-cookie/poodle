//! MetricTile specimen — metric tiles with trend indicators and sparklines.

use crate::compat::js_metric_tile;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, MetricTileSpec, MetricTrend};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Basic tiles — label + value, no trend (contract §12)
        .child(group(
            "Basic tiles",
            secondary,
            div()
                .flex_row()
                .gap(12.0)
                .items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Components", "85"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Coverage", "94%"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Open issues", "12"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Build time", "1.8s"),
                    theme,
                )),
        ))
        // With trend indicators — up / down / flat tones (contract §12)
        .child(group(
            "With trend indicators",
            secondary,
            div()
                .flex_row()
                .gap(12.0)
                .items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Active users", "2,847")
                        .with_trend(MetricTrend::Up)
                        .with_trend_label("+12.3%"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Error rate", "0.04%")
                        .with_trend(MetricTrend::Down)
                        .with_trend_label("-8%"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Latency", "42ms")
                        .with_trend(MetricTrend::Flat)
                        .with_trend_label("No change"),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Revenue", "$14.2k")
                        .with_trend(MetricTrend::Up)
                        .with_trend_label("+3.1%"),
                    theme,
                )),
        ))
        // With sparklines — incl. no-trend "Memory" (contract §12)
        .child(group(
            "With sparklines",
            secondary,
            div()
                .flex_row()
                .gap(12.0)
                .items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Requests/min", "1,204")
                        .with_trend(MetricTrend::Up)
                        .with_trend_label("+5%")
                        .with_sparkline(vec![800.0, 920.0, 850.0, 1100.0, 980.0, 1050.0, 1204.0]),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("CPU usage", "62%")
                        .with_trend(MetricTrend::Down)
                        .with_trend_label("-4%")
                        .with_sparkline(vec![75.0, 72.0, 68.0, 70.0, 65.0, 63.0, 62.0]),
                    theme,
                ))
                // Sparkline without trend (contract §12 "Memory")
                .child(js_metric_tile(
                    &MetricTileSpec::new("Memory", "4.2 GB")
                        .with_sparkline(vec![3.8, 3.9, 4.0, 4.1, 4.0, 4.1, 4.2]),
                    theme,
                )),
        ))
        // Density — compact / default / comfortable (contract §3/§8)
        .child(group(
            "Density",
            secondary,
            div().flex_row().gap(12.0).items_start().children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| {
                    js_metric_tile(
                        &MetricTileSpec::new("Requests/min", "1,204")
                            .with_trend(MetricTrend::Up)
                            .with_trend_label("+5%")
                            .with_sparkline(vec![
                                800.0, 920.0, 850.0, 1100.0, 980.0, 1050.0, 1204.0,
                            ])
                            .with_density(density),
                        theme,
                    )
                }),
            ),
        ))
        // Trend without label — arrow tone only, no text (extra)
        .child(group(
            "Trend without label",
            secondary,
            div()
                .flex_row()
                .gap(12.0)
                .items_start()
                .child(js_metric_tile(
                    &MetricTileSpec::new("Sessions", "12,400").with_trend(MetricTrend::Up),
                    theme,
                ))
                .child(js_metric_tile(
                    &MetricTileSpec::new("Bounces", "38%").with_trend(MetricTrend::Down),
                    theme,
                )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
