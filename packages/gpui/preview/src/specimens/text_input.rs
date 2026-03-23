use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{FieldSpec, TextInputSpec, ValidationState};
use pug_gpui_components::{Field, TextInput};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // Track typed values
    let name_value = state.specimens.text.get("text-input-name").cloned()
        .unwrap_or_default();
    let email_value = state.specimens.text.get("text-input-email").cloned()
        .unwrap_or_default();

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            Field::from_spec(
                FieldSpec::new("name-field", "Name")
                    .with_description("Enter your full name."),
                theme,
            )
            .with_control(
                TextInput::from_spec(
                    TextInputSpec::new()
                        .with_id("name-field")
                        .with_placeholder("Jane Doe")
                        .with_value(&name_value),
                    theme,
                )
                .on_change(cx.listener(|this, val: &str, _w, cx| {
                    this.state.specimens.text.insert("text-input-name".to_string(), val.to_string());
                    cx.notify();
                }))
            )
        )
        // --- With validation ---
        .child(section_label("WITH VALIDATION", text_secondary))
        .child(
            Field::from_spec(
                FieldSpec::new("email-field", "Email")
                    .with_validation_state(ValidationState::Invalid)
                    .with_error("Please enter a valid email address."),
                theme,
            )
            .with_control(
                TextInput::from_spec(
                    TextInputSpec::new()
                        .with_id("email-field")
                        .with_placeholder("you@example.com")
                        .with_value(&email_value)
                        .with_validation_state(ValidationState::Invalid),
                    theme,
                )
                .on_change(cx.listener(|this, val: &str, _w, cx| {
                    this.state.specimens.text.insert("text-input-email".to_string(), val.to_string());
                    cx.notify();
                }))
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            Field::from_spec(
                FieldSpec::new("disabled-field", "API key"),
                theme,
            )
            .with_control(
                TextInput::from_spec(
                    TextInputSpec::new()
                        .with_id("disabled-field")
                        .with_value("sk-\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}1234")
                        .with_disabled(true),
                    theme,
                )
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
