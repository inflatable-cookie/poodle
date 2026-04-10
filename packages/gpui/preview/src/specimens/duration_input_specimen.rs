use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{DurationInputSpec, EyebrowSpec};
use poodle_gpui_components::{DurationInput, Eyebrow};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0))
        // --- Full (H:M:S) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Full (H:M:S)"), theme))
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
        // --- Hours and minutes only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Hours and minutes only"), theme))
                .child(
                    DurationInput::from_spec(
                        DurationInputSpec::new()
                            .with_value("02:45")
                            .with_show_seconds(false),
                        theme,
                    ).with_id("duration-hm")
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
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
