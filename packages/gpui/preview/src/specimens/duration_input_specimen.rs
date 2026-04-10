use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, DurationInputSpec, EyebrowSpec};
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
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-size-xs").size(ControlSize::Xs))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-size-sm").size(ControlSize::Sm))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-size-md").size(ControlSize::Md))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-size-lg").size(ControlSize::Lg))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-size-xl").size(ControlSize::Xl))
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-density-compact").with_density(ControlDensity::Compact))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-density-default").with_density(ControlDensity::Default))
                        .child(DurationInput::from_spec(DurationInputSpec::new().with_value("01:00"), theme).with_id("duration-density-comfortable").with_density(ControlDensity::Comfortable))
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
