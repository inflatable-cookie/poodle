use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Slider};
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, SliderSpec};
use std::sync::Arc;

fn slider_change(state: &AppState, key: &'static str) -> Arc<dyn Fn(f64) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: format!("{value:.0}"),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let volume = state
        .specimens
        .text
        .get("slider-volume")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(65.0);
    let opacity = state
        .specimens
        .text
        .get("slider-opacity")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(100.0);

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
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(volume).with_bounds(0.0, 100.0);
                            spec.step = 1.0;
                            spec.aria_label = Some("Volume".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-volume")
                    .on_change(slider_change(state, "slider-volume")),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Volume: {:.0}%", volume)),
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
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(opacity).with_bounds(0.0, 100.0);
                            spec.step = 10.0;
                            spec.aria_label = Some("Opacity".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-opacity")
                    .on_change(slider_change(state, "slider-opacity")),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Opacity: {:.0}%", opacity)),
                ),
        )
        // --- Value (low / mid / high) ---
        // Static sliders showing proportional track fill + thumb resolved from
        // the spec value alone (all 0..100): 10% / 50% / 90%.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Value (low / mid / high)"),
                    theme,
                ))
                .child(
                    Slider::from_spec(SliderSpec::new(10.0).with_bounds(0.0, 100.0), theme)
                        .aria_label("Low value")
                        .with_id("slider-value-low"),
                )
                .child(
                    Slider::from_spec(SliderSpec::new(50.0).with_bounds(0.0, 100.0), theme)
                        .aria_label("Mid value")
                        .with_id("slider-value-mid"),
                )
                .child(
                    Slider::from_spec(SliderSpec::new(90.0).with_bounds(0.0, 100.0), theme)
                        .aria_label("High value")
                        .with_id("slider-value-high"),
                ),
        )
        // --- Custom min / max + step ---
        // Bounds 50..200, value 125, step 25; fill at (125-50)/(200-50) = 50%.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom min / max + step (50–200, step 25)"),
                    theme,
                ))
                .child(
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(125.0).with_bounds(50.0, 200.0);
                            spec.step = 25.0;
                            spec.aria_label = Some("Temperature".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-bounds"),
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
                .child(
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
                            spec.is_disabled = true;
                            spec.aria_label = Some("Disabled slider".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-disabled"),
                ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "slider",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Slider::from_spec(
                SliderSpec::new(60.0)
                    .with_bounds(0.0, 100.0)
                    .with_size(size),
                theme,
            )
            .with_id(format!("specimen-size-{:?}", size))
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Slider::from_spec(
                SliderSpec::new(60.0)
                    .with_bounds(0.0, 100.0)
                    .with_density(density),
                theme,
            )
            .with_id(format!("specimen-density-{:?}", density))
            .into_any_element()
        },
    )
}
