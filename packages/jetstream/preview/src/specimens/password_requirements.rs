//! PasswordRequirements specimen — met/unmet rules, all met, none met,
//! mixed, loading, error, empty, and the size ladder.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_password_requirements;

use poodle_specs::{ControlSize, PasswordRequirementsPolicy, PasswordRequirementsSpec};

fn policy() -> PasswordRequirementsPolicy {
    PasswordRequirementsPolicy::new(12)
        .with_require_mixed_case(true)
        .with_require_digit(true)
        .with_require_special(true)
        .with_min_strength_score(3)
        .with_description("Use a long password with varied character types.")
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let sizes: [(ControlSize, &str); 5] = [
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ];
    let mut size_ladder = div().flex_col().gap(16.0);
    for (size, name) in sizes {
        size_ladder = size_ladder.child(group(
            name,
            secondary,
            div().w(320.0).child(js_password_requirements(
                &PasswordRequirementsSpec::new()
                    .with_password("Example123!")
                    .with_requirements(policy())
                    .with_size(size),
                theme,
            )),
        ));
    }

    div()
        .flex_col()
        .gap(24.0)
        // All met — every rule satisfied (check icon + met tone).
        .child(group(
            "All met",
            secondary,
            div().w(320.0).child(js_password_requirements(
                &PasswordRequirementsSpec::new()
                    .with_password("ClayRules123!")
                    .with_requirements(policy()),
                theme,
            )),
        ))
        // None met — empty password, every rule fails (cross icon + unmet tone).
        .child(group(
            "None met",
            secondary,
            div().w(320.0).child(js_password_requirements(
                &PasswordRequirementsSpec::new()
                    .with_password("")
                    .with_requirements(policy()),
                theme,
            )),
        ))
        // Mixed — short, lowercase + digit only: some met, some unmet.
        .child(group(
            "Mixed",
            secondary,
            div().w(320.0).child(js_password_requirements(
                &PasswordRequirementsSpec::new()
                    .with_password("clay123")
                    .with_requirements(policy()),
                theme,
            )),
        ))
        // Loading — loading paragraph, no checklist.
        .child(group(
            "Loading",
            secondary,
            div().w(320.0).child(js_password_requirements(
                &PasswordRequirementsSpec::new().with_loading(true),
                theme,
            )),
        ))
        // Error — inline error message, no checklist.
        .child(group(
            "Error",
            secondary,
            div().w(320.0).child(js_password_requirements(
                &PasswordRequirementsSpec::new()
                    .with_error("Could not load password requirements."),
                theme,
            )),
        ))
        // Empty — no policy, no loading, no error: renders nothing inside.
        .child(group(
            "Empty (no policy)",
            secondary,
            div()
                .w(320.0)
                .child(js_password_requirements(&PasswordRequirementsSpec::new(), theme)),
        ))
        // Size ladder — xs → xl.
        .child(group("Sizes", secondary, size_ladder))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
