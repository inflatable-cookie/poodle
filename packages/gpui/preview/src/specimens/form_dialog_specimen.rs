use gpui::*;
use poodle_gpui_components::{Button, Eyebrow, Field, FormDialog, TextInput};
use poodle_components::{ButtonSpec, ButtonTone, ButtonVariant, EyebrowSpec, TextInputSpec};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // -- Basic --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Add new user")
                        .description("Invite a user to this workspace.")
                        .submit_label("Add user")
                        .cancel_label("Cancel")
                        .with_child(
                            Field::new("fd-name", "Full name", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_placeholder("Enter name"),
                                        theme,
                                    ).with_id("fd-name")
                                )
                        )
                        .with_child(
                            Field::new("fd-role", "Role", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_placeholder("Select role"),
                                        theme,
                                    ).with_id("fd-role")
                                )
                        )
                )
        )

        // -- With error --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With error"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Create account")
                        .submit_label("Create")
                        .error_message("A user with this email already exists.")
                        .with_child(
                            Field::new("fd-email", "Email", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_value("existing@example.com"),
                                        theme,
                                    ).with_id("fd-email")
                                )
                        )
                )
        )

        // -- Submitting state --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Submitting state"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Add new user")
                        .submit_label("Add user")
                        .submitting(true)
                        .with_child(
                            Field::new("fd-name-sub", "Full name", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_value("Clay Tercek"),
                                        theme,
                                    ).with_id("fd-name-sub")
                                )
                        )
                )
        )

        // -- Shell mode with custom actions --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Shell mode with custom actions"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Edit workspace settings")
                        .subtitle("Update shared defaults for this workspace.")
                        .show_default_actions(false)
                        .with_child(
                            Field::new("fd-ws-name", "Workspace name", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_value("Northstar"),
                                        theme,
                                    ).with_id("fd-ws-name")
                                )
                        )
                        .with_child(
                            Field::new("fd-ws-role", "Default role", theme)
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_value("editor"),
                                        theme,
                                    ).with_id("fd-ws-role")
                                )
                        )
                        .with_actions(
                            div().flex().gap(px(8.0)).justify_end()
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_label("Cancel"),
                                        theme,
                                    )
                                    .with_id("fd-shell-cancel")
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Secondary)
                                            .with_tone(ButtonTone::Danger)
                                            .with_label("Reset defaults"),
                                        theme,
                                    )
                                    .with_id("fd-shell-reset")
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Primary)
                                            .with_label("Save changes"),
                                        theme,
                                    )
                                    .with_id("fd-shell-save")
                                )
                        )
                )
        )

        // -- Bare mode (no FormLayout wrapper) --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Bare mode (no FormLayout wrapper)"), theme))
                .child(
                    FormDialog::new(theme)
                        .title("Quick search")
                        .subtitle("Type to search across the workspace.")
                        .bare(true)
                        .show_default_actions(false)
                        .with_child(
                            TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_placeholder("Search components, pages, docs\u{2026}")
                                    .with_input_type("search"),
                                theme,
                            )
                            .with_id("fd-bare-search")
                        )
                )
        )
}
