use gpui::*;
use poodle_composites::{MetricTileSpec, MetricTrend};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{Eyebrow, MetricTile};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // --- Basic tiles ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic tiles"), theme))
                .child(
                    div().flex().gap(px(12.0)).flex_wrap()
                        .child(MetricTile::from_spec(MetricTileSpec::new("Total Users", "12,847"), theme))
                        .child(MetricTile::from_spec(MetricTileSpec::new("Active Sessions", "342"), theme))
                        .child(MetricTile::from_spec(MetricTileSpec::new("Conversion Rate", "3.2%"), theme))
                        .child(MetricTile::from_spec(MetricTileSpec::new("Revenue", "$48,290"), theme))
                )
        )
        // --- With trend ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With trend"), theme))
                .child(
                    div().flex().gap(px(12.0)).flex_wrap()
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Monthly Revenue", "$48,290")
                                .with_trend(MetricTrend::Up)
                                .with_trend_label("+12.4%"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Bounce Rate", "38%")
                                .with_trend(MetricTrend::Down)
                                .with_trend_label("\u{2212}3.1%"),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Avg Session", "2m 14s")
                                .with_trend(MetricTrend::Flat)
                                .with_trend_label("no change"),
                            theme,
                        ))
                )
        )
        // --- With sparkline ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With sparkline"), theme))
                .child(
                    div().flex().gap(px(12.0)).flex_wrap()
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("Weekly Signups", "1,284")
                                .with_trend(MetricTrend::Up)
                                .with_trend_label("+18%")
                                .with_sparkline(vec![
                                    32.0, 40.0, 36.0, 48.0, 52.0, 60.0, 58.0,
                                    66.0, 72.0, 78.0, 82.0, 88.0,
                                ]),
                            theme,
                        ))
                        .child(MetricTile::from_spec(
                            MetricTileSpec::new("API Latency", "184ms")
                                .with_trend(MetricTrend::Down)
                                .with_trend_label("\u{2212}22ms")
                                .with_sparkline(vec![
                                    220.0, 214.0, 230.0, 206.0, 198.0, 200.0,
                                    192.0, 186.0, 188.0, 184.0,
                                ]),
                            theme,
                        ))
                )
        )
}
