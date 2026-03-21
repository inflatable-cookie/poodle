use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{FieldSpec, TextInputSpec, ValidationState};
use pug_gpui_components::{Field, TextInput};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Default with description ---
        .child(section_label("DEFAULT WITH DESCRIPTION", text_secondary))
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
        // --- Required ---
        .child(section_label("REQUIRED", text_secondary))
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
        // --- With error ---
        .child(section_label("WITH ERROR", text_secondary))
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
        // --- Valid ---
        .child(section_label("VALID", text_secondary))
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
        // --- Optional ---
        .child(section_label("OPTIONAL", text_secondary))
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
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
