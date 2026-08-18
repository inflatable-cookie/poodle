use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, NumberInput};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, NumberInputSpec, ValidationState};
use std::sync::{Arc, Mutex};

fn step_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
    current: f64,
    step: f64,
    bound: f64,
    increment: bool,
    precision: Option<usize>,
) -> Arc<dyn Fn() + Send + Sync> {
    let next = if increment {
        (current + step).min(bound)
    } else {
        (current - step).max(bound)
    };
    let value = precision
        .map(|digits| format!("{next:.digits$}"))
        .unwrap_or_else(|| format!("{next}"));
    let events = Arc::clone(events);
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.clone(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // Track quantity value (stored as string, parsed to f64)
    let quantity_str = state
        .specimens
        .text
        .get("number-input-quantity")
        .cloned()
        .unwrap_or_else(|| "1".to_string());
    let quantity: f64 = quantity_str.parse().unwrap_or(1.0);

    let price_str = state
        .specimens
        .text
        .get("number-input-price")
        .cloned()
        .unwrap_or_else(|| "29.99".to_string());
    let price: f64 = price_str.parse().unwrap_or(29.99);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(224.0)) // 14rem
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
                    NumberInput::from_spec(
                        NumberInputSpec::new(quantity)
                            .with_min(0.0)
                            .with_max(100.0)
                            .with_aria_label("Quantity"),
                        theme,
                    )
                    .on_increment(step_handler(
                        &state.node_events,
                        "number-input-quantity",
                        quantity,
                        1.0,
                        100.0,
                        true,
                        None,
                    ))
                    .on_decrement(step_handler(
                        &state.node_events,
                        "number-input-quantity",
                        quantity,
                        1.0,
                        0.0,
                        false,
                        None,
                    )),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Quantity: {}", quantity)),
                ),
        )
        // --- With steppers ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With steppers"),
                    theme,
                ))
                .child(
                    NumberInput::from_spec(
                        NumberInputSpec::new(price)
                            .with_min(0.0)
                            .with_step(0.01)
                            .with_steppers(true)
                            .with_aria_label("Price"),
                        theme,
                    )
                    .on_increment(step_handler(
                        &state.node_events,
                        "number-input-price",
                        price,
                        0.01,
                        f64::INFINITY,
                        true,
                        Some(2),
                    ))
                    .on_decrement(step_handler(
                        &state.node_events,
                        "number-input-price",
                        price,
                        0.01,
                        0.0,
                        false,
                        Some(2),
                    )),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Price: ${:.2}", price)),
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
                .child(NumberInput::from_spec(
                    NumberInputSpec::new(42.0).with_disabled(true),
                    theme,
                )),
        )
        // --- Invalid ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Invalid"),
                    theme,
                ))
                .child(NumberInput::from_spec(
                    NumberInputSpec::new(-5.0)
                        .with_min(0.0)
                        .with_validation_state(ValidationState::Invalid),
                    theme,
                )),
        )
        // --- Prefix (currency) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Prefix (currency)"),
                    theme,
                ))
                .child(NumberInput::from_spec(
                    NumberInputSpec::new(29.99)
                        .with_min(0.0)
                        .with_step(0.01)
                        .with_prefix("$")
                        .with_precision(2),
                    theme,
                )),
        )
        // --- Suffix (unit) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Suffix (unit)"),
                    theme,
                ))
                .child(NumberInput::from_spec(
                    NumberInputSpec::new(72.0).with_min(0.0).with_suffix("kg"),
                    theme,
                )),
        )
        // --- Precision (3 decimal places) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Precision (3 decimal places)"),
                    theme,
                ))
                .child(NumberInput::from_spec(
                    NumberInputSpec::new(std::f64::consts::PI)
                        .with_step(0.001)
                        .with_precision(3),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "number-input",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                NumberInput::from_spec(NumberInputSpec::new(1.0), theme)
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                NumberInput::from_spec(NumberInputSpec::new(1.0), theme)
                    .density(density)
                    .into_any_element()
            }),
    )
}
