use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{EyebrowSpec, RangeSliderSpec};
use poodle_gpui_components::{Eyebrow, RangeSlider};
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            RangeSlider::from_spec(
                                RangeSliderSpec::new(20.0, 80.0)
                                    .with_bounds(0.0, 100.0)
                                    .with_aria_label("Price range"),
                                theme,
                            )
                        )
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("$20 \u{2013} $80".to_string())
                        )
                )
        )
        // --- With step ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With step"), theme))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            RangeSlider::from_spec(
                                RangeSliderSpec::new(25.0, 45.0)
                                    .with_bounds(18.0, 65.0)
                                    .with_step(5.0)
                                    .with_aria_label("Age range"),
                                theme,
                            )
                        )
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Ages 25 \u{2013} 45".to_string())
                        )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    RangeSlider::from_spec(
                        RangeSliderSpec::new(30.0, 70.0)
                            .with_bounds(0.0, 100.0)
                            .with_disabled(true)
                            .with_aria_label("Disabled range"),
                        theme,
                    )
                )
        )
}
