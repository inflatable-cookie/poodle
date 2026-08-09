//! FormLayout specimen — responsive form grid with status messages.
//!
//! Real components only (CLAUDE.md "no fakes"): every field is a real `js_field`
//! wrapping a real `js_text_input`, and actions are real `js_button`s. All visual
//! values resolve from tokens via the component specs.

use crate::compat::js_button;
use crate::compat::js_field;
use crate::compat::js_form_layout;
use crate::compat::js_text_input;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, FieldSpec, FormLayoutSpec, TextInputSpec, ValidationState,
};

/// Real Field (label + control) wrapping a real TextInput, all token-resolved.
fn field(id: &str, lbl: &str, placeholder: &str, theme: &JetstreamThemeProvider) -> El {
    let input = js_text_input(
        &TextInputSpec::new()
            .with_id(id)
            .with_placeholder(placeholder),
        theme,
    );
    js_field(&FieldSpec::new(id, lbl), theme, Some(input))
}

/// Real Field in an invalid validation state — drives the field error treatment.
fn field_invalid(
    id: &str,
    lbl: &str,
    value: &str,
    error: &str,
    theme: &JetstreamThemeProvider,
) -> El {
    let input = js_text_input(
        &TextInputSpec::new()
            .with_id(id)
            .with_value(value)
            .with_validation_state(ValidationState::Invalid),
        theme,
    );
    js_field(
        &FieldSpec::new(id, lbl)
            .with_validation_state(ValidationState::Invalid)
            .with_error(error),
        theme,
        Some(input),
    )
}

/// Real action button (Cancel = secondary, Save = primary).
fn action_button(lbl: &str, variant: ButtonVariant, theme: &JetstreamThemeProvider) -> El {
    js_button(
        &ButtonSpec::new().with_label(lbl).with_variant(variant),
        theme,
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let actions = || -> El {
        div()
            .flex_row()
            .justify_end()
            .gap(8.0)
            .child(action_button("Cancel", ButtonVariant::Secondary, theme))
            .child(action_button("Save", ButtonVariant::Primary, theme))
    };

    div()
        .flex_col()
        .gap(32.0)
        .child(group(
            "Single-column layout",
            secondary,
            js_form_layout(
                &FormLayoutSpec::new().with_columns(1),
                theme,
                vec![
                    field("fl-project", "Project name", "Untitled project", theme),
                    field("fl-desc", "Description", "What is this for?", theme),
                ],
                Some(actions()),
            ),
        ))
        .child(group(
            "Two-column layout",
            secondary,
            js_form_layout(
                &FormLayoutSpec::new().with_columns(2),
                theme,
                vec![
                    field("fl-first", "First name", "Jane", theme),
                    field("fl-last", "Last name", "Doe", theme),
                    field("fl-email", "Email", "jane@example.com", theme),
                    field("fl-phone", "Phone", "Phone number", theme),
                ],
                None,
            ),
        ))
        .child(group(
            "Mixed 2-col and 3-col rows",
            secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_description("Three fields per row for compact entry forms.")
                    .with_columns(3),
                theme,
                vec![
                    field("fl-mix-first", "First name", "Jane", theme),
                    field("fl-mix-middle", "Middle name", "M.", theme),
                    field("fl-mix-last", "Last name", "Doe", theme),
                    field("fl-mix-email", "Email", "jane@example.com", theme),
                    field("fl-mix-phone", "Phone", "+1 555 0100", theme),
                    field("fl-mix-dept", "Department", "Engineering", theme),
                ],
                None,
            ),
        ))
        .child(group(
            "With description",
            secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_description("Fill in the track metadata before publishing.")
                    .with_columns(1),
                theme,
                vec![
                    field("fl-track", "Track title", "Track title", theme),
                    field("fl-artist", "Artist", "Artist name", theme),
                ],
                None,
            ),
        ))
        .child(group(
            "With error and field errors",
            secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_error("Unable to save. Please fix the errors below.")
                    .with_field_error("Email", "This email is already in use")
                    .with_field_error("Role", "A role is required")
                    .with_columns(2),
                theme,
                vec![
                    field_invalid(
                        "fl-err-email",
                        "Email",
                        "taken@example.com",
                        "This email is already in use",
                        theme,
                    ),
                    field_invalid("fl-err-role", "Role", "", "A role is required", theme),
                ],
                Some(div().flex_row().justify_end().child(action_button(
                    "Retry",
                    ButtonVariant::Primary,
                    theme,
                ))),
            ),
        ))
        .child(group(
            "With success message",
            secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_success("Profile saved successfully.")
                    .with_columns(1),
                theme,
                vec![field("fl-display", "Display name", "Display name", theme)],
                None,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
