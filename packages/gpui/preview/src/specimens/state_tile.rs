use crate::node_compat::{Eyebrow, StateTile};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, StateTileSpec};

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0)).child(group(
        "States",
        theme,
        div()
            .flex()
            .gap(px(12.0))
            .flex_wrap()
            .child(StateTile::from_spec(
                StateTileSpec::new("Queued jobs", "14"),
                theme,
            ))
            .child(StateTile::from_spec(
                StateTileSpec::new("Success rate", "99.8%")
                    .with_trend("up")
                    .with_trend_label("Up 1.2%"),
                theme,
            ))
            .child(StateTile::from_spec(
                StateTileSpec::new("Errors", "3")
                    .with_trend("down")
                    .with_trend_label("Down 4"),
                theme,
            ))
            .child(StateTile::from_spec(
                StateTileSpec::new("Capacity", "72%")
                    .with_trend("steady")
                    .with_trend_label("Stable over seven days")
                    .with_sparkline(true),
                theme,
            )),
    ))
}
