use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{NumberInputSpec, ValidationState, EyebrowSpec};
use poodle_gpui_components::{NumberInput, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // Track quantity value (stored as string, parsed to f64)
    let quantity_str = state.specimens.text.get("number-input-quantity").cloned()
        .unwrap_or_else(|| "1".to_string());
    let quantity: f64 = quantity_str.parse().unwrap_or(1.0);

    let price_str = state.specimens.text.get("number-input-price").cloned()
        .unwrap_or_else(|| "29.99".to_string());
    let price: f64 = price_str.parse().unwrap_or(29.99);

    div().flex().flex_col().gap(px(24.0)).max_w(px(224.0)) // 14rem
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    NumberInput::from_spec(
                        NumberInputSpec::new(quantity)
                            .with_min(0.0)
                            .with_max(100.0)
                            .with_aria_label("Quantity"),
                        theme,
                    )
                    .on_increment(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                        let cur: f64 = this.state.specimens.text.get("number-input-quantity")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(1.0);
                        let new_val = (cur + 1.0).min(100.0);
                        this.state.specimens.text.insert("number-input-quantity".to_string(), format!("{}", new_val));
                        cx.notify();
                    }))
                    .on_decrement(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                        let cur: f64 = this.state.specimens.text.get("number-input-quantity")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(1.0);
                        let new_val = (cur - 1.0).max(0.0);
                        this.state.specimens.text.insert("number-input-quantity".to_string(), format!("{}", new_val));
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Quantity: {}", quantity))
                )
        )
        // --- With steppers ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With steppers"), theme))
                .child(
                    NumberInput::from_spec(
                        NumberInputSpec::new(price)
                            .with_min(0.0)
                            .with_step(0.01)
                            .with_aria_label("Price"),
                        theme,
                    )
                    .on_increment(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                        let cur: f64 = this.state.specimens.text.get("number-input-price")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(29.99);
                        let new_val = ((cur + 0.01) * 100.0).round() / 100.0;
                        this.state.specimens.text.insert("number-input-price".to_string(), format!("{:.2}", new_val));
                        cx.notify();
                    }))
                    .on_decrement(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                        let cur: f64 = this.state.specimens.text.get("number-input-price")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(29.99);
                        let new_val = ((cur - 0.01) * 100.0).round() / 100.0;
                        this.state.specimens.text.insert("number-input-price".to_string(), format!("{:.2}", new_val.max(0.0)));
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Price: ${:.2}", price))
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    NumberInput::from_spec(
                        NumberInputSpec::new(42.0)
                            .with_disabled(true),
                        theme,
                    )
                )
        )
        // --- Invalid ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Invalid"), theme))
                .child(
                    NumberInput::from_spec(
                        NumberInputSpec::new(-5.0)
                            .with_min(0.0)
                            .with_validation_state(ValidationState::Invalid),
                        theme,
                    )
                )
        )
}
