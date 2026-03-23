use gpui::*;
use flint_adapter::ThemeProvider;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{TimeAgoSpec, DurationInputSpec, EyebrowSpec};
use flint_gpui_components::{TimeAgo, DurationInput, Eyebrow};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // -- TimeAgo --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Time ago: recent timestamps"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(time_ago_row("2 minutes ago", "2026-03-23T10:58:00Z", theme, text_secondary))
                        .child(time_ago_row("3 hours ago", "2026-03-23T08:00:00Z", theme, text_secondary))
                        .child(time_ago_row("2 days ago", "2026-03-21T11:00:00Z", theme, text_secondary))
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Time ago: future"), theme))
                .child(
                    time_ago_row("in 5 minutes", "2026-03-23T11:05:00Z", theme, text_secondary)
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Time ago: static (no live update)"), theme))
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

        // -- DurationInput --
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Duration input: full (H:M:S)"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            DurationInput::from_spec(
                                DurationInputSpec::new()
                                    .with_value("01:30:00")
                                    .with_show_seconds(true),
                                theme,
                            ).with_id("duration-full")
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("Value: 01:30:00 (5400 seconds)")
                        )
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Duration input: hours and minutes only"), theme))
                .child(
                    DurationInput::from_spec(
                        DurationInputSpec::new()
                            .with_value("02:45")
                            .with_show_seconds(false),
                        theme,
                    ).with_id("duration-hm")
                )
        )
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Duration input: disabled"), theme))
                .child(
                    DurationInput::from_spec(
                        DurationInputSpec::new()
                            .with_value("00:30:00")
                            .with_disabled(true),
                        theme,
                    ).with_id("duration-disabled")
                )
        )
}

fn time_ago_row(
    label: &str,
    timestamp: &str,
    theme: &GpuiThemeProvider,
    text_secondary: flint_tokens::typed::ColorValue,
) -> Div {
    div().flex().items_center().gap(px(8.0))
        .child(
            TimeAgo::from_spec(
                TimeAgoSpec::new().with_timestamp(timestamp),
                theme,
            )
        )
        .child(
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child(format!("({})", label))
        )
}
