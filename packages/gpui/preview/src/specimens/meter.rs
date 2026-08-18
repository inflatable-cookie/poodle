use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, Meter};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, MeterSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0))
        // --- Default (50%) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default (50%)"),
                    theme,
                ))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(50.0)
                        .with_max(100.0)
                        .with_aria_label("Storage usage"),
                    theme,
                )),
        )
        // --- With thresholds ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With thresholds"),
                    theme,
                ))
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
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("82% \u{2014} above high threshold".to_string()),
                ),
        )
        // --- Low value (optimal range) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Low value (optimal range)"),
                    theme,
                ))
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
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("30% \u{2014} within normal range".to_string()),
                ),
        )
        // --- Custom range (0-500) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom range (0\u{2013}500)"),
                    theme,
                ))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(350.0)
                        .with_min(0.0)
                        .with_max(500.0)
                        .with_aria_label("API calls"),
                    theme,
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("350 / 500 API calls used".to_string()),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "meter",
        examples,
        SpecimenAxes::examples_only().with_sizes(|size, theme: &GpuiThemeProvider| {
            Meter::from_spec(
                MeterSpec::new()
                    .with_value(50.0)
                    .with_max(100.0)
                    .with_aria_label("Storage usage")
                    .with_size(size),
                theme,
            )
            .into_any_element()
        }),
    )
}
