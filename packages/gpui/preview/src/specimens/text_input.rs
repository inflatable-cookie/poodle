use gpui::*;
use poodle_primitives::{ControlDensity, ControlSize, FieldSpec, TextInputSpec, ValidationState, EyebrowSpec};
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
    let workspace_value = state.specimens.text.get("text-input-workspace").cloned()
        .unwrap_or_else(|| "acme-admin".to_string());

    div().flex().flex_col().gap(px(24.0)).max_w(px(384.0)) // 24rem = Svelte specimen max-width
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0)) // 0.5rem
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
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With validation"), theme))
                .child(
                    Field::from_spec({
                            let mut field = FieldSpec::new("email-field", "Email")
                                .with_description("A valid email address is required.")
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
        // --- Async validation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Async validation"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("workspace-field", "Workspace")
                            .with_description("Check whether the workspace handle is available.")
                            .with_validation_state(ValidationState::Pending)
                            .with_pending_message("Checking availability..."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("workspace-field")
                                .with_value(&workspace_value)
                                .with_validation_state(ValidationState::Pending),
                            theme,
                        )
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("text-input-workspace".to_string(), val.to_string());
                            cx.notify();
                        }))
                    )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Extra small"),
                                theme,
                            ).size(ControlSize::Xs)
                        )
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Small"),
                                theme,
                            ).size(ControlSize::Sm)
                        )
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Medium"),
                                theme,
                            ).size(ControlSize::Md)
                        )
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Large"),
                                theme,
                            ).size(ControlSize::Lg)
                        )
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Extra large"),
                                theme,
                            ).size(ControlSize::Xl)
                        )
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Compact"),
                                theme,
                            ).density(ControlDensity::Compact)
                        )
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Default"),
                                theme,
                            ).density(ControlDensity::Default)
                        )
                        .child(
                            TextInput::from_spec(
                                TextInputSpec::new().with_placeholder("Comfortable"),
                                theme,
                            ).density(ControlDensity::Comfortable)
                        )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
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
