use gpui::*;
use poodle_composites::MetricTileSpec;
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{MetricTile, Eyebrow};
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
}
