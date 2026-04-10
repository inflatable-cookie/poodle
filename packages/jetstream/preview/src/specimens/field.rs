//! Field specimen — fields with label, description, error, and pending states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::field::js_field;
use poodle_jetstream_components::text_input::js_text_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{FieldSpec, TextInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // With label and description
        .child(group("With label + description", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("name", "Full Name")
                        .with_description("Enter your first and last name"),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_placeholder("Jane Doe"), theme)),
                )
            )
        ))
        // With error
        .child(group("With error", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("email", "Email")
                        .with_error("Please enter a valid email address")
                        .with_validation_state(ValidationState::Invalid),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_value("not-an-email"), theme)),
                )
            )
        ))
        // Pending
        .child(group("Pending validation", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("username", "Username")
                        .with_pending_message("Checking availability...")
                        .with_validation_state(ValidationState::Pending),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_value("newuser42"), theme)),
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
