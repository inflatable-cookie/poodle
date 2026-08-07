use crate::node_compat::{Eyebrow, MetricTile};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;
use poodle_specs::{ControlDensity, MetricTileSpec, MetricTrend};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Basic tiles (contract §12) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic tiles"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .flex_wrap()
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Components", "85"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Coverage", "94%"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Open issues", "12"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Build time", "1.8s"),
                            theme,
                        )),
                ),
        )
        // --- With trend indicators (contract §12: up / down / flat / up) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With trend indicators"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .flex_wrap()
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Active users", "2,847")
                                .with_trend(MetricTrend::Up)
                                .with_trend_label("+12.3%"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Error rate", "0.04%")
                                .with_trend(MetricTrend::Down)
                                .with_trend_label("-8%"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Latency", "42ms")
                                .with_trend(MetricTrend::Flat)
                                .with_trend_label("No change"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Revenue", "$14.2k")
                                .with_trend(MetricTrend::Up)
                                .with_trend_label("+3.1%"),
                            theme,
                        )),
                ),
        )
        // --- With sparklines (contract §12: incl. no-trend "Memory") ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With sparklines"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .flex_wrap()
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Requests/min", "1,204")
                                .with_trend(MetricTrend::Up)
                                .with_trend_label("+5%")
                                .with_sparkline(vec![
                                    800.0, 920.0, 850.0, 1100.0, 980.0, 1050.0, 1204.0,
                                ]),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("CPU usage", "62%")
                                .with_trend(MetricTrend::Down)
                                .with_trend_label("-4%")
                                .with_sparkline(vec![75.0, 72.0, 68.0, 70.0, 65.0, 63.0, 62.0]),
                            theme,
                        ))
                        // No-trend sparkline (contract §12 "Memory")
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Memory", "4.2 GB")
                                .with_sparkline(vec![3.8, 3.9, 4.0, 4.1, 4.0, 4.1, 4.2]),
                            theme,
                        )),
                ),
        )
        // --- Density (contract §3/§8: compact / default / comfortable) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Density"),
                    theme,
                ))
                .child(
                    div().flex().gap(px(12.0)).flex_wrap().children(
                        [
                            ("Compact", ControlDensity::Compact),
                            ("Default", ControlDensity::Default),
                            ("Comfortable", ControlDensity::Comfortable),
                        ]
                        .into_iter()
                        .map(|(_, density)| {
                            MetricTile::from_spec(
                                MetricTileSpec::new("Requests/min", "1,204")
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
                ),
        )
}
