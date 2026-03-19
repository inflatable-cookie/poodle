use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::RangeSliderSpec;
use pug_gpui_components::PugRangeSlider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            PugRangeSlider::new(
                RangeSliderSpec::new(20.0, 80.0)
                    .with_bounds(0.0, 100.0)
                    .with_aria_label("Price range"),
                theme,
            )
        )
        // --- With Step ---
        .child(section_label("WITH STEP", text_secondary))
        .child(
            PugRangeSlider::new(
                RangeSliderSpec::new(25.0, 45.0)
                    .with_bounds(18.0, 65.0)
                    .with_step(5.0)
                    .with_aria_label("Age range"),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            PugRangeSlider::new(
                RangeSliderSpec::new(30.0, 70.0)
                    .with_bounds(0.0, 100.0)
                    .with_disabled(true)
                    .with_aria_label("Disabled range"),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
