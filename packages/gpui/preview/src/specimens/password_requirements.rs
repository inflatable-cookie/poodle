use crate::app_state::AppState;
use crate::node_compat::PasswordRequirements;
use crate::node_compat::{Eyebrow, TextInput};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    EyebrowSpec, PasswordRequirementsPolicy, PasswordRequirementsSpec, TextInputSpec,
};

fn policy() -> PasswordRequirementsPolicy {
    PasswordRequirementsPolicy::new(12)
        .with_require_mixed_case(true)
        .with_require_digit(true)
        .with_require_special(true)
        .with_min_strength_score(3)
        .with_description("Use a long password with varied character types.")
}

fn group(theme: &GpuiThemeProvider, label: &'static str, body: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(body)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let root = div().flex().flex_col().gap(px(24.0)).max_w(px(448.0));

    // Default — live field plus checklist, fully met.
    let default_group = group(
        theme,
        "Default",
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
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
                    .with_requirements(policy()),
                theme,
            )),
    );

    // All met — every rule satisfied (check icon + met tone on every row).
    let all_met = group(
        theme,
        "All met",
        PasswordRequirements::from_spec(
            PasswordRequirementsSpec::new()
                .with_password("ClayRules123!")
                .with_requirements(policy()),
            theme,
        ),
    );

    // None met — empty password, every rule fails (cross icon + unmet tone).
    let none_met = group(
        theme,
        "None met",
        PasswordRequirements::from_spec(
            PasswordRequirementsSpec::new()
                .with_password("")
                .with_requirements(policy()),
            theme,
        ),
    );

    // Mixed — some rules met, some unmet (short, lowercase + digit only).
    let mixed = group(
        theme,
        "Mixed",
        PasswordRequirements::from_spec(
            PasswordRequirementsSpec::new()
                .with_password("clay123")
                .with_requirements(policy()),
            theme,
        ),
    );

    // Loading — loading paragraph, no checklist.
    let loading = group(
        theme,
        "Loading",
        PasswordRequirements::from_spec(PasswordRequirementsSpec::new().with_loading(true), theme),
    );

    // Error — inline error message, no checklist.
    let error = group(
        theme,
        "Error",
        PasswordRequirements::from_spec(
            PasswordRequirementsSpec::new().with_error("Could not load password requirements."),
            theme,
        ),
    );

    // Empty — no requirements, no loading, no error: renders nothing inside.
    let empty = group(
        theme,
        "Empty (no policy)",
        PasswordRequirements::from_spec(PasswordRequirementsSpec::new(), theme),
    );
    let examples = root
        .child(default_group)
        .child(all_met)
        .child(none_met)
        .child(mixed)
        .child(loading)
        .child(error)
        .child(empty)
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "password-requirements",
        examples,
        SpecimenAxes::examples_only().with_sizes(|size, theme: &GpuiThemeProvider| {
            PasswordRequirements::from_spec(
                PasswordRequirementsSpec::new()
                    .with_password("Example123!")
                    .with_requirements(policy())
                    .with_size(size),
                theme,
            )
            .into_any_element()
        }),
    )
}
