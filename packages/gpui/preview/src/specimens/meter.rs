use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::MeterSpec;
use pug_gpui_components::Meter;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0))
        // --- Default (50%) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DEFAULT (50%)", text_secondary))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(50.0)
                        .with_max(100.0)
                        .with_aria_label("Storage usage"),
                    theme,
                ))
        )
        // --- With thresholds ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("WITH THRESHOLDS", text_secondary))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(82.0)
                        .with_max(100.0)
                        .with_low(25.0)
                        .with_high(75.0)
                        .with_optimum(50.0)
                        .with_aria_label("CPU usage"),
                    theme,
                ))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("82% \u{2014} above high threshold".to_string())
                )
        )
        // --- Low value (optimal range) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("LOW VALUE (OPTIMAL RANGE)", text_secondary))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(30.0)
                        .with_max(100.0)
                        .with_low(25.0)
                        .with_high(75.0)
                        .with_optimum(50.0)
                        .with_aria_label("Memory usage"),
                    theme,
                ))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("30% \u{2014} within normal range".to_string())
                )
        )
        // --- Custom range (0-500) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("CUSTOM RANGE (0\u{2013}500)", text_secondary))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(350.0)
                        .with_min(0.0)
                        .with_max(500.0)
                        .with_aria_label("API calls"),
                    theme,
                ))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("350 / 500 API calls used".to_string())
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
