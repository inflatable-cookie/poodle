use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{TimeAgoSpec, EyebrowSpec};
use poodle_gpui_components::{TimeAgo, Eyebrow};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Recent timestamps ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Recent timestamps"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(time_ago_row("2 minutes ago", "2026-03-23T10:58:00Z", theme, text_secondary))
                        .child(time_ago_row("3 hours ago", "2026-03-23T08:00:00Z", theme, text_secondary))
                        .child(time_ago_row("2 days ago", "2026-03-21T11:00:00Z", theme, text_secondary))
                )
        )
        // --- Future ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Future"), theme))
                .child(time_ago_row("in 5 minutes", "2026-03-23T11:05:00Z", theme, text_secondary))
        )
        // --- Static (no live update) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Static (no live update)"), theme))
                .child(
                    div().flex().items_center().gap(px(8.0))
                        .child(
                            TimeAgo::from_spec(
                                TimeAgoSpec::new()
                                    .with_timestamp("2026-03-23T09:00:00Z")
                                    .with_live(false),
                                theme,
                            )
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("(live updates disabled)")
                        )
                )
        )
}

fn time_ago_row(
    label: &str,
    timestamp: &str,
    theme: &GpuiThemeProvider,
    text_secondary: poodle_tokens::typed::ColorValue,
) -> Div {
    div().flex().items_center().gap(px(8.0))
        .child(TimeAgo::from_spec(TimeAgoSpec::new().with_timestamp(timestamp), theme))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child(format!("({})", label)))
}
