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

fn value_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn(Option<f64>) + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move |value: Option<f64>| {
        let text = value
            .map(|v| format!("{v}"))
            .unwrap_or_default();
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: text,
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let quantity_str = state
        .specimens
        .text
        .get("number-input-quantity")
        .cloned()
        .unwrap_or_else(|| "1".to_string());
    let quantity: Option<f64> = if quantity_str.is_empty() {
        None
    } else {
        Some(quantity_str.parse().unwrap_or(1.0))
    };

    let price_str = state
        .specimens
        .text
        .get("number-input-price")
        .cloned()
        .unwrap_or_else(|| "29.99".to_string());
    let price: Option<f64> = if price_str.is_empty() {
        None
    } else {
        Some(price_str.parse().unwrap_or(29.99))
    };

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
                            .with_min(Some(0.0))
                            .with_max(Some(100.0))
                            .with_steppers(true)
                            .with_aria_label("Quantity"),
                        theme,
                    )
                    .on_value_change(value_handler(
                        &state.node_events,
                        "number-input-quantity",
                    )),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!(
                            "Quantity: {}",
                            quantity
                                .map(|v| v.to_string())
                                .unwrap_or_else(|| "empty".into())
                        )),
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
                            .with_min(Some(0.0))
                            .with_step(Some(0.01))
                            .with_steppers(true)
                            .with_aria_label("Price"),
                        theme,
                    )
                    .on_value_change(value_handler(&state.node_events, "number-input-price")),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(match price {
                            Some(v) => format!("Price: ${v:.2}"),
                            None => "Price: empty".into(),
                        }),
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
                    NumberInputSpec::new(Some(42.0)).with_disabled(true),
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
                    NumberInputSpec::new(Some(-5.0))
                        .with_min(Some(0.0))
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
                    NumberInputSpec::new(Some(29.99))
                        .with_min(Some(0.0))
                        .with_step(Some(0.01))
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
                    NumberInputSpec::new(Some(72.0))
                        .with_min(Some(0.0))
                        .with_suffix("kg"),
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
                    NumberInputSpec::new(Some(std::f64::consts::PI))
                        .with_step(Some(0.001))
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
                NumberInput::from_spec(NumberInputSpec::new(Some(1.0)), theme)
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                NumberInput::from_spec(NumberInputSpec::new(Some(1.0)), theme)
                    .density(density)
                    .into_any_element()
            }),
    )
}
