use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, Stepper};
use poodle_specs::{EyebrowSpec, StepStatus, StepperSpec, StepperStep};

/// The Soundcheck arrangement this design came from.
fn wizard_steps() -> Vec<StepperStep> {
    vec![
        StepperStep::new("state", "Current state").with_status(StepStatus::Complete),
        StepperStep::new("recovery", "Recovery").with_status(StepStatus::Complete),
        StepperStep::new("categories", "Categories"),
        StepperStep::new("apply", "Apply and verify").with_disabled(true),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    fn group(theme: &GpuiThemeProvider, label: &str, content: AnyElement) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(label),
                theme,
            ))
            .child(content)
    }

    let working = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("extract", "Extract tokens").with_status(StepStatus::Running),
        StepperStep::new("map", "Map to theme"),
    ];

    // The case position-derived state cannot express: a failed step *behind*
    // the current one, which `index < current` would render as complete.
    let failed = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("gate", "Quality gate").with_status(StepStatus::Failed),
        StepperStep::new("apply", "Apply changes"),
    ];

    let completed = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("extract", "Extract tokens").with_status(StepStatus::Complete),
    ];

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Default",
            Stepper::from_spec(
                StepperSpec::new(wizard_steps())
                    .with_value("categories")
                    .with_aria_label("DAW sync steps"),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Working",
            Stepper::from_spec(
                StepperSpec::new(working)
                    .with_value("extract")
                    .with_aria_label("Import progress"),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Failed",
            Stepper::from_spec(
                StepperSpec::new(failed)
                    .with_value("gate")
                    .with_aria_label("Pipeline steps"),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Re-run",
            Stepper::from_spec(
                StepperSpec::new(completed)
                    .with_value("read")
                    .with_show_rerun(true)
                    .with_aria_label("Completed pipeline"),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Disabled",
            Stepper::from_spec(
                StepperSpec::new(wizard_steps())
                    .with_value("categories")
                    .with_disabled(true)
                    .with_aria_label("Disabled steps"),
                theme,
            )
            .into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "stepper",
        examples,
        move |size, theme: &GpuiThemeProvider| {
            Stepper::from_spec(
                StepperSpec::new(wizard_steps())
                    .with_value("categories")
                    .with_size(size)
                    .with_aria_label("Size ladder"),
                theme,
            )
            .into_any_element()
        },
        move |density, theme: &GpuiThemeProvider| {
            Stepper::from_spec(
                StepperSpec::new(wizard_steps())
                    .with_value("categories")
                    .with_density(density)
                    .with_aria_label("Density ladder"),
                theme,
            )
            .into_any_element()
        },
    )
}
