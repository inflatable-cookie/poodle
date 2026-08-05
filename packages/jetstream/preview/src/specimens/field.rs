//! Field specimen — label/control, required, optional, description (info icon),
//! error, pending, valid, plus sizes and densities.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_field;
use crate::compat::js_text_input;

use poodle_specs::{ControlDensity, ControlSize, FieldSpec, TextInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Label + control (plain)
        .child(group("Label + control", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("workspace", "Workspace name"),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_placeholder("Acme Inc."), theme)),
                )
            )
        ))
        // With label and description (info icon)
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
        // Required
        .child(group("Required", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("email-req", "Email address").with_required(true),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_placeholder("you@example.com"), theme)),
                )
            )
        ))
        // Optional
        .child(group("Optional", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("phone", "Phone number").with_optional_label("optional"),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_placeholder("+1 (555) 000-0000"), theme)),
                )
            )
        ))
        // Optional with description (info icon + optional marker)
        .child(group("Optional with description", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("fax", "Fax number")
                        .with_optional_label("optional")
                        .with_description("Include country code."),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_placeholder("+1 (555) 000-0000"), theme)),
                )
            )
        ))
        // With error (invalid)
        .child(group("With error", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("email", "Email")
                        .with_error("Please enter a valid email address")
                        .with_validation_state(ValidationState::Invalid),
                    theme,
                    Some(js_text_input(
                        &TextInputSpec::new()
                            .with_value("not-an-email")
                            .with_validation_state(ValidationState::Invalid),
                        theme,
                    )),
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
                    Some(js_text_input(
                        &TextInputSpec::new()
                            .with_value("newuser42")
                            .with_validation_state(ValidationState::Pending),
                        theme,
                    )),
                )
            )
        ))
        // Valid state
        .child(group("Valid state", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("score", "Score")
                        .with_validation_state(ValidationState::Valid)
                        .with_description("Must be at least 8 characters."),
                    theme,
                    Some(js_text_input(
                        &TextInputSpec::new()
                            .with_value("95")
                            .with_validation_state(ValidationState::Valid),
                        theme,
                    )),
                )
            )
        ))
        // With info icon (description + required)
        .child(group("With info icon", secondary,
            div().w(300.0).child(
                js_field(
                    &FieldSpec::new("api", "API Key")
                        .with_description("Your personal API key for authentication.")
                        .with_required(true),
                    theme,
                    Some(js_text_input(&TextInputSpec::new().with_placeholder("sk_live_..."), theme)),
                )
            )
        ))
        // Sizes (xs -> xl)
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(size_row(theme, ControlSize::Xs, "field-size-xs", "XS"))
                .child(size_row(theme, ControlSize::Sm, "field-size-sm", "SM"))
                .child(size_row(theme, ControlSize::Md, "field-size-md", "MD"))
                .child(size_row(theme, ControlSize::Lg, "field-size-lg", "LG"))
                .child(size_row(theme, ControlSize::Xl, "field-size-xl", "XL"))
        ))
        // Densities (compact / default / comfortable)
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(density_row(theme, ControlDensity::Compact, "field-density-compact", "COMPACT"))
                .child(density_row(theme, ControlDensity::Default, "field-density-default", "DEFAULT"))
                .child(density_row(theme, ControlDensity::Comfortable, "field-density-comfortable", "COMFORTABLE"))
        ))
}

fn size_row(theme: &JetstreamThemeProvider, size: ControlSize, id: &str, ph: &str) -> El {
    div().w(300.0).child(js_field(
        &FieldSpec::new(id, "Display name")
            .with_description("This is how your name appears to other users.")
            .with_size(size),
        theme,
        Some(js_text_input(
            &TextInputSpec::new().with_placeholder(ph).with_size(size),
            theme,
        )),
    ))
}

fn density_row(theme: &JetstreamThemeProvider, density: ControlDensity, id: &str, ph: &str) -> El {
    div().w(300.0).child(js_field(
        &FieldSpec::new(id, "Display name")
            .with_description("This is how your name appears to other users.")
            .with_density(density),
        theme,
        Some(js_text_input(
            &TextInputSpec::new().with_placeholder(ph).with_density(density),
            theme,
        )),
    ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
