use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, RangeSlider};
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, Orientation, RangeSliderSpec, SliderPolarity};
use std::sync::Arc;

fn range_change(state: &AppState, key: &'static str) -> Arc<dyn Fn(f64, f64) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |low, high| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetText {
            key: format!("{key}-lo"),
            value: format!("{low:.0}"),
        });
        events.push(NodeSpecimenEvent::SetText {
            key: format!("{key}-hi"),
            value: format!("{high:.0}"),
        });
    })
}

fn range_fraction_change(
    state: &AppState,
    key: impl Into<String>,
) -> Arc<dyn Fn(f64, f64) + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move |low, high| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetText {
            key: format!("{key}-lo"),
            value: format!("{low:.2}"),
        });
        events.push(NodeSpecimenEvent::SetText {
            key: format!("{key}-hi"),
            value: format!("{high:.2}"),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let default_lo = state
        .specimens
        .text
        .get("range-slider-default-lo")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(20.0);
    let default_hi = state
        .specimens
        .text
        .get("range-slider-default-hi")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(80.0);

    let step_lo = state
        .specimens
        .text
        .get("range-slider-step-lo")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(25.0);
    let step_hi = state
        .specimens
        .text
        .get("range-slider-step-hi")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(45.0);
    let embedded_unipolar_lo = state
        .specimens
        .text
        .get("range-slider-embedded-unipolar-lo")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.2);
    let embedded_unipolar_hi = state
        .specimens
        .text
        .get("range-slider-embedded-unipolar-hi")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.75);
    let embedded_bipolar_lo = state
        .specimens
        .text
        .get("range-slider-embedded-bipolar-lo")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(-0.6);
    let embedded_bipolar_hi = state
        .specimens
        .text
        .get("range-slider-embedded-bipolar-hi")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.35);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0))
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("A lower and upper bound the reader drags"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            RangeSlider::from_spec(
                                RangeSliderSpec::new(default_lo, default_hi)
                                    .with_bounds(0.0, 100.0)
                                    .with_aria_label("Price range"),
                                theme,
                            )
                            .on_change(
                                "range-slider-default",
                                range_change(state, "range-slider-default"),
                            ),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("${:.0} \u{2013} ${:.0}", default_lo, default_hi)),
                        ),
                ),
        )
        // --- With step ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Stepped — the thumbs land on whole increments"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            RangeSlider::from_spec(
                                RangeSliderSpec::new(step_lo, step_hi)
                                    .with_bounds(18.0, 65.0)
                                    .with_step(5.0)
                                    .with_aria_label("Age range"),
                                theme,
                            )
                            .on_change(
                                "range-slider-step",
                                range_change(state, "range-slider-step"),
                            ),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Ages {:.0} \u{2013} {:.0}", step_lo, step_hi)),
                        ),
                ),
        )
        // --- Positions (narrow / full / low / high) ---
        // Static dual-thumb sliders: both thumbs + the between-fill window
        // resolved from the spec low/high alone (all 0..100).
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Positions (narrow / full / low / high)"),
                    theme,
                ))
                .child(RangeSlider::from_spec(
                    RangeSliderSpec::new(45.0, 55.0)
                        .with_bounds(0.0, 100.0)
                        .with_aria_label("Narrow range"),
                    theme,
                ))
                .child(RangeSlider::from_spec(
                    RangeSliderSpec::new(0.0, 100.0)
                        .with_bounds(0.0, 100.0)
                        .with_aria_label("Full range"),
                    theme,
                ))
                .child(RangeSlider::from_spec(
                    RangeSliderSpec::new(0.0, 25.0)
                        .with_bounds(0.0, 100.0)
                        .with_aria_label("Low range"),
                    theme,
                ))
                .child(RangeSlider::from_spec(
                    RangeSliderSpec::new(75.0, 100.0)
                        .with_bounds(0.0, 100.0)
                        .with_aria_label("High range"),
                    theme,
                )),
        )
        // --- Embedded variant ---
        // Both polarities in one section: they are the same variant answering
        // the same question, and the web pages now show them together too.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content(
                        "Embedded variant — unipolar fills from the floor, bipolar from centre",
                    ),
                    theme,
                ))
                .child(
                    RangeSlider::from_spec(
                        RangeSliderSpec::new(embedded_unipolar_lo, embedded_unipolar_hi)
                            .with_bounds(0.0, 1.0)
                            .with_step(0.01)
                            .with_embedded_control(SliderPolarity::Unipolar)
                            .with_aria_label("Unipolar modulation range"),
                        theme,
                    )
                    .on_change(
                        "range-slider-embedded-unipolar",
                        range_fraction_change(state, "range-slider-embedded-unipolar"),
                    ),
                )
                .child(
                    RangeSlider::from_spec(
                        RangeSliderSpec::new(embedded_bipolar_lo, embedded_bipolar_hi)
                            .with_bounds(-1.0, 1.0)
                            .with_step(0.01)
                            .with_embedded_control(SliderPolarity::Bipolar)
                            .with_aria_label("Bipolar modulation range"),
                        theme,
                    )
                    .on_change(
                        "range-slider-embedded-bipolar",
                        range_fraction_change(state, "range-slider-embedded-bipolar"),
                    ),
                ),
        )
        // --- Vertical ---
        // The contract covers vertical orientation with its own sizing rules
        // and no page taught it. Added by the g15.011 pilot.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Vertical — the same control on the other axis"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(32.0))
                        .h(px(192.0))
                        .child(RangeSlider::from_spec(
                            RangeSliderSpec::new(30.0, 70.0)
                                .with_bounds(0.0, 100.0)
                                .with_orientation(Orientation::Vertical)
                                .with_aria_label("Vertical range"),
                            theme,
                        ))
                        .child(RangeSlider::from_spec(
                            RangeSliderSpec::new(-0.4, 0.6)
                                .with_bounds(-1.0, 1.0)
                                .with_step(0.01)
                                .with_embedded_control(SliderPolarity::Bipolar)
                                .with_orientation(Orientation::Vertical)
                                .with_aria_label("Vertical embedded range"),
                            theme,
                        )),
                ),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(RangeSlider::from_spec(
                    RangeSliderSpec::new(30.0, 70.0)
                        .with_bounds(0.0, 100.0)
                        .with_disabled(true)
                        .with_aria_label("Disabled range"),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "range-slider",
        examples,
        // Sizes pane: one control per step. Three per step was an exhaustive
        // matrix inside the tab that exists to keep matrices out of Examples.
        |size, theme: &GpuiThemeProvider| {
            let key = format!("range-slider-size-{size:?}");
            let lo = state
                .specimens
                .text
                .get(&format!("{key}-lo"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(0.2);
            let hi = state
                .specimens
                .text
                .get(&format!("{key}-hi"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(0.75);
            RangeSlider::from_spec(
                RangeSliderSpec::new(lo, hi)
                    .with_bounds(0.0, 1.0)
                    .with_step(0.01)
                    .with_aria_label("Range"),
                theme,
            )
            .size(size)
            .on_change(key.clone(), range_fraction_change(state, key))
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            RangeSlider::from_spec(
                RangeSliderSpec::new(20.0, 80.0)
                    .with_bounds(0.0, 100.0)
                    .with_aria_label("Range"),
                theme,
            )
            .density(density)
            .into_any_element()
        },
    )
}
