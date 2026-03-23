use gpui::*;
use flint_primitives::{FieldSpec, TextInputSpec, ValidationState, EyebrowSpec};
use flint_gpui_components::{Field, TextInput, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().flex_col().gap(px(24.0))
        // --- Default with description ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default with description"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-name", "Display name")
                            .with_description("This is how your name appears to other users."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-name")
                                .with_placeholder("Enter your name")
                                .with_aria_label("Display name"),
                            theme,
                        )
                    )
                )
        )
        // --- Required ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Required"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-email", "Email address")
                            .with_required(true),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-email")
                                .with_placeholder("you@example.com")
                                .with_aria_label("Email address"),
                            theme,
                        )
                    )
                )
        )
        // --- With error ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With error"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-user", "Username")
                            .with_error("This username is already taken.")
                            .with_validation_state(ValidationState::Invalid),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-user")
                                .with_value("admin")
                                .with_validation_state(ValidationState::Invalid)
                                .with_aria_label("Username"),
                            theme,
                        )
                    )
                )
        )
        // --- Valid ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Valid"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-pass", "Password")
                            .with_validation_state(ValidationState::Valid)
                            .with_description("Must be at least 8 characters."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-pass")
                                .with_value("\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}")
                                .with_validation_state(ValidationState::Valid)
                                .with_aria_label("Password"),
                            theme,
                        )
                    )
                )
        )
        // --- Optional ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Optional"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-phone", "Phone number")
                            .with_optional_label("optional"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-phone")
                                .with_placeholder("+1 (555) 000-0000")
                                .with_aria_label("Phone number"),
                            theme,
                        )
                    )
                )
        )
        // --- With hint (progressive disclosure) ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With hint (progressive disclosure)"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-slug", "URL Slug")
                            .with_description("A URL-friendly identifier used in page addresses. Lowercase letters, numbers, and hyphens only."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-slug")
                                .with_placeholder("my-page-slug")
                                .with_aria_label("URL Slug"),
                            theme,
                        )
                    )
                )
        )
        // --- Hint + description + required ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Hint + description + required"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-api", "API Key")
                            .with_description("Your personal API key for authentication.")
                            .with_required(true),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-api")
                                .with_placeholder("sk_live_...")
                                .with_aria_label("API Key"),
                            theme,
                        )
                    )
                )
        )
}
