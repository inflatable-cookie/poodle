use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, PasswordRequirements, TextInput};
use poodle_specs::{
    EyebrowSpec, PasswordRequirementsPolicy, PasswordRequirementsSpec, TextInputSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let policy = PasswordRequirementsPolicy::new(12)
        .with_require_mixed_case(true)
        .with_require_digit(true)
        .with_require_special(true)
        .with_min_strength_score(3)
        .with_description("Use a long password with varied character types.");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(448.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(TextInput::from_spec(
                    TextInputSpec::new()
                        .with_id("password-preview")
                        .with_input_type("password")
                        .with_placeholder("Enter password"),
                    theme,
                ))
                .child(PasswordRequirements::from_spec(
                    PasswordRequirementsSpec::new()
                        .with_password("ClayRules123!")
                        .with_requirements(policy.clone()),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading"),
                    theme,
                ))
                .child(PasswordRequirements::from_spec(
                    PasswordRequirementsSpec::new().with_loading(true),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Error"),
                    theme,
                ))
                .child(PasswordRequirements::from_spec(
                    PasswordRequirementsSpec::new()
                        .with_error("Could not load password requirements."),
                    theme,
                )),
        )
}
