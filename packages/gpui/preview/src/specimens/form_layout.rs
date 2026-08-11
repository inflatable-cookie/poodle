use crate::app_state::AppState;
use crate::node_compat::{Button, Checkbox, CompatRow, Eyebrow, Field, FormLayout, TextInput};
use crate::PreviewRoot;
use gpui::*;
use poodle_specs::{
    ButtonSpec, ButtonVariant, CheckboxSpec, EyebrowSpec, TextInputSpec, ValidationState,
};

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(520.0))
        // -- Two-column layout --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Two-column layout (span 3 = half)"),
                    theme,
                ))
                .child(
                    FormLayout::new(theme)
                        .description("Fill in the details below to create a new user account.")
                        .columns(2)
                        .with_child(
                            Field::new("fl-first", "First name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Jane"),
                                    theme,
                                )
                                .with_id("fl-first"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-last", "Last name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Doe"),
                                    theme,
                                )
                                .with_id("fl-last"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-email", "Email", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("jane@example.com"),
                                    theme,
                                )
                                .with_id("fl-email"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-phone", "Phone", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Phone number"),
                                    theme,
                                )
                                .with_id("fl-phone"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-notes", "Notes", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new()
                                        .with_placeholder("Any additional notes...")
                                        .with_rows(3),
                                    theme,
                                )
                                .with_id("fl-notes"),
                            ),
                        )
                        .with_actions(
                            CompatRow::new()
                                .gap(theme.resolve_space_value("space.inline.sm"))
                                .justify_end()
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_label("Cancel"),
                                        theme,
                                    )
                                    .with_id("fl-cancel"),
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Primary)
                                            .with_label("Create user"),
                                        theme,
                                    )
                                    .with_id("fl-create"),
                                ),
                        ),
                ),
        )
        // -- Mixed 2-col and 3-col rows --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Mixed 2-col and 3-col rows"),
                    theme,
                ))
                .child(
                    FormLayout::new(theme)
                        .description("Three fields per row for compact entry forms.")
                        .columns(3)
                        .with_child(
                            Field::new("fl-mix-first", "First name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Jane"),
                                    theme,
                                )
                                .with_id("fl-mix-first"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-mix-middle", "Middle name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("M."),
                                    theme,
                                )
                                .with_id("fl-mix-middle"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-mix-last", "Last name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Doe"),
                                    theme,
                                )
                                .with_id("fl-mix-last"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-mix-email", "Email", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("jane@example.com"),
                                    theme,
                                )
                                .with_id("fl-mix-email"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-mix-phone", "Phone", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("+1 555 0100"),
                                    theme,
                                )
                                .with_id("fl-mix-phone"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-mix-dept", "Department", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Engineering"),
                                    theme,
                                )
                                .with_id("fl-mix-dept"),
                            ),
                        )
                        .with_actions(
                            CompatRow::new()
                                .gap(theme.resolve_space_value("space.inline.sm"))
                                .justify_end()
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_label("Cancel"),
                                        theme,
                                    )
                                    .with_id("fl-mix-cancel"),
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Primary)
                                            .with_label("Save"),
                                        theme,
                                    )
                                    .with_id("fl-mix-save"),
                                ),
                        ),
                ),
        )
        // -- Single column --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single column (columns=1)"),
                    theme,
                ))
                .child(
                    FormLayout::new(theme)
                        .columns(1)
                        .with_child(
                            Field::new("fl-display", "Display name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_placeholder("Enter a name"),
                                    theme,
                                )
                                .with_id("fl-display"),
                            ),
                        )
                        .with_child(
                            Field::new("fl-bio", "Bio", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new()
                                        .with_placeholder("Tell us about yourself...")
                                        .with_rows(3),
                                    theme,
                                )
                                .with_id("fl-bio"),
                            ),
                        )
                        .with_child(Checkbox::from_spec(
                            CheckboxSpec::new().with_label("I agree to the terms"),
                            theme,
                        ))
                        .with_actions(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save profile"),
                                theme,
                            )
                            .with_id("fl-save-profile"),
                        ),
                ),
        )
        // -- With error and field errors --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With error and field errors"),
                    theme,
                ))
                .child(
                    FormLayout::new(theme)
                        .error("Unable to save. Please fix the errors below.")
                        .with_field_error("Email", "This email is already in use")
                        .with_field_error("Role", "A role is required")
                        .columns(2)
                        .with_child(
                            Field::new("fl-err-email", "Email", theme)
                                .validation_state(ValidationState::Invalid)
                                .error("This email is already in use")
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new()
                                            .with_value("taken@example.com")
                                            .with_validation_state(ValidationState::Invalid),
                                        theme,
                                    )
                                    .with_id("fl-err-email"),
                                ),
                        )
                        .with_child(
                            Field::new("fl-err-role", "Role", theme)
                                .validation_state(ValidationState::Invalid)
                                .error("A role is required")
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new()
                                            .with_placeholder("Select a role...")
                                            .with_validation_state(ValidationState::Invalid),
                                        theme,
                                    )
                                    .with_id("fl-err-role"),
                                ),
                        )
                        .with_actions(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Retry"),
                                theme,
                            )
                            .with_id("fl-retry"),
                        ),
                ),
        )
        // -- With success message --
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With success message"),
                    theme,
                ))
                .child(
                    FormLayout::new(theme)
                        .success("Settings saved successfully.")
                        .columns(1)
                        .with_child(
                            Field::new("fl-site", "Site name", theme).with_control(
                                TextInput::from_spec(
                                    TextInputSpec::new().with_value("My Project"),
                                    theme,
                                )
                                .with_id("fl-site"),
                            ),
                        )
                        .with_actions(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save"),
                                theme,
                            )
                            .with_id("fl-save-2"),
                        ),
                ),
        )
}
