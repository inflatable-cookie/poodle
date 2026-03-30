//! PasswordRequirements specimen — met/unmet rules, all met, loading.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::password_requirements::js_password_requirements;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{PasswordRequirementsPolicy, PasswordRequirementsSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let policy = PasswordRequirementsPolicy::new(8)
        .with_require_mixed_case(true)
        .with_require_digit(true)
        .with_require_special(true);

    div().flex_col().gap(24.0)
        // Partial — some rules met
        .child(group("Partially met", secondary,
            div().w(320.0)
                .child(js_password_requirements(
                    &PasswordRequirementsSpec::new()
                        .with_password("Hello1")
                        .with_requirements(policy.clone()),
                    theme,
                ))
        ))
        // All met
        .child(group("All met", secondary,
            div().w(320.0)
                .child(js_password_requirements(
                    &PasswordRequirementsSpec::new()
                        .with_password("Hello1World!")
                        .with_requirements(policy.clone()),
                    theme,
                ))
        ))
        // Loading
        .child(group("Loading", secondary,
            div().w(320.0)
                .child(js_password_requirements(
                    &PasswordRequirementsSpec::new()
                        .with_loading(true),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
