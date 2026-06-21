use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, RangeSlider};
use poodle_specs::{EyebrowSpec, RangeSliderSpec};

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
                    EyebrowSpec::new().with_content("Default"),
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
                            .on_change("range-slider-default", cx.listener(
                                |this, pair: &(f64, f64), _w, cx| {
                                    this.state.specimens.text.insert(
                                        "range-slider-default-lo".to_string(),
                                        format!("{:.0}", pair.0),
                                    );
                                    this.state.specimens.text.insert(
                                        "range-slider-default-hi".to_string(),
                                        format!("{:.0}", pair.1),
                                    );
                                    cx.notify();
                                },
                            )),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!(
                                    "${:.0} \u{2013} ${:.0}",
                                    default_lo, default_hi
                                )),
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
                    EyebrowSpec::new().with_content("With step"),
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
                            .on_change("range-slider-step", cx.listener(
                                |this, pair: &(f64, f64), _w, cx| {
                                    this.state.specimens.text.insert(
                                        "range-slider-step-lo".to_string(),
                                        format!("{:.0}", pair.0),
                                    );
                                    this.state.specimens.text.insert(
                                        "range-slider-step-hi".to_string(),
                                        format!("{:.0}", pair.1),
                                    );
                                    cx.notify();
                                },
                            )),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!(
                                    "Ages {:.0} \u{2013} {:.0}",
                                    step_lo, step_hi
                                )),
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
        // --- Custom min / max + step ---
        // Bounds 0..500, [100, 350], step 50; fill from 20% to 70%.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom min / max + step (0–500, step 50)"),
                    theme,
                ))
                .child(RangeSlider::from_spec(
                    RangeSliderSpec::new(100.0, 350.0)
                        .with_bounds(0.0, 500.0)
                        .with_step(50.0)
                        .with_aria_label("Budget range"),
                    theme,
                )),
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
        |size, theme: &GpuiThemeProvider| {
            RangeSlider::from_spec(
                RangeSliderSpec::new(20.0, 80.0)
                    .with_bounds(0.0, 100.0)
                    .with_aria_label("Range"),
                theme,
            )
            .size(size)
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
