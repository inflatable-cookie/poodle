use gpui::*;
use poodle_primitives::{FieldSpec, TextInputSpec, ValidationState, EyebrowSpec};
use poodle_gpui_components::{Field, TextInput, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // Track typed values
    let name_value = state.specimens.text.get("text-input-name").cloned()
        .unwrap_or_default();
    let email_value = state.specimens.text.get("text-input-email").cloned()
        .unwrap_or_else(|| "invalid-email".to_string());
    let email_is_valid = email_value.contains('@');
    let validation_state = if email_is_valid {
        ValidationState::Valid
    } else {
        ValidationState::Invalid
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
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
        )
        // --- With validation ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With validation"), theme))
                .child(
                    Field::from_spec({
                            let mut field = FieldSpec::new("email-field", "Email")
                                .with_validation_state(validation_state);
                            if !email_is_valid {
                                field = field.with_error("Please enter a valid email address.");
                            }
                            field
                        },
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("email-field")
                                .with_placeholder("you@example.com")
                                .with_value(&email_value)
                                .with_validation_state(validation_state),
                            theme,
                        )
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("text-input-email".to_string(), val.to_string());
                            cx.notify();
                        }))
                    )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("disabled-field", "API key"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("disabled-field")
                                .with_value("sk-xxxx-xxxx-xxxx")
                                .with_disabled(true),
                            theme,
                        )
                    )
                )
        )
}
