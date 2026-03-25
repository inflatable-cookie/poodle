use gpui::*;
use poodle_gpui_components::{FormDialog, Field, TextInput, Eyebrow};
use poodle_primitives::{TextInputSpec, EyebrowSpec};
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
}
