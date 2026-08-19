use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, Meter};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, MeterShape, MeterSpec, MeterTone};

fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

fn note(text_secondary: poodle_tokens::typed::ColorValue, copy: &str) -> Div {
    div()
        .text_sm()
        .text_color(color_to_hsla(text_secondary))
        .child(copy.to_string())
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0))
        .child(group(
            theme,
            "Default usage",
            Meter::from_spec(
                MeterSpec::new()
                    .with_value(50.0)
                    .with_max(100.0)
                    .with_aria_label("Storage usage"),
                theme,
            ),
        ))
        .child(group(
            theme,
            "Threshold states",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
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
                .child(note(text_secondary, "82% \u{2014} above high threshold"))
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
                .child(note(text_secondary, "30% \u{2014} within normal range")),
        ))
        .child(group(
            theme,
            "Custom range",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_value(350.0)
                        .with_min(0.0)
                        .with_max(500.0)
                        .with_aria_label("API calls"),
                    theme,
                ))
                .child(note(text_secondary, "350 / 500 API calls used")),
        ))
        .child(group(
            theme,
            "Ring shape and readout",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .child(Meter::from_spec(
                            MeterSpec::new()
                                .with_shape(MeterShape::Ring)
                                .with_value(38.0)
                                .with_aria_label("Context used"),
                            theme,
                        ))
                        .child(Meter::from_spec(
                            MeterSpec::new()
                                .with_shape(MeterShape::Ring)
                                .with_value(86.0)
                                .with_high(80.0)
                                .with_aria_label("Context used, above warn threshold"),
                            theme,
                        ))
                        .child(Meter::from_spec(
                            MeterSpec::new()
                                .with_shape(MeterShape::Ring)
                                .with_value(64.0)
                                .with_show_value(true)
                                .with_size(poodle_specs::ControlSize::Xl)
                                .with_aria_label("Context used"),
                            theme,
                        )),
                )
                .child(note(
                    text_secondary,
                    "38% \u{00b7} 86% (above high) \u{00b7} 64% with readout",
                )),
        ))
        .child(group(
            theme,
            "Ring tones",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_shape(MeterShape::Ring)
                        .with_value(60.0)
                        .with_tone(MeterTone::Success)
                        .with_aria_label("Success tone"),
                    theme,
                ))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_shape(MeterShape::Ring)
                        .with_value(60.0)
                        .with_tone(MeterTone::Accent)
                        .with_aria_label("Accent tone"),
                    theme,
                ))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_shape(MeterShape::Ring)
                        .with_value(60.0)
                        .with_tone(MeterTone::Warning)
                        .with_aria_label("Warning tone"),
                    theme,
                ))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_shape(MeterShape::Ring)
                        .with_value(60.0)
                        .with_tone(MeterTone::Danger)
                        .with_aria_label("Danger tone"),
                    theme,
                ))
                .child(Meter::from_spec(
                    MeterSpec::new()
                        .with_shape(MeterShape::Ring)
                        .with_value(60.0)
                        .with_tone(MeterTone::Neutral)
                        .with_aria_label("Neutral tone"),
                    theme,
                )),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "meter",
        examples,
        SpecimenAxes::examples_only().with_sizes(|size, theme: &GpuiThemeProvider| {
            Meter::from_spec(
                MeterSpec::new()
                    .with_shape(MeterShape::Ring)
                    .with_value(60.0)
                    .with_aria_label("Context used")
                    .with_size(size),
                theme,
            )
            .into_any_element()
        }),
    )
}
