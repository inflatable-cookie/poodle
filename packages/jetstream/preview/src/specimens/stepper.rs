//! Stepper specimen — contract §13.
//!
//! The Failed group is the one that matters: a failed step sitting *behind* the
//! current one is exactly what position-derived state cannot express, and it is
//! why `status` is a property of the step.

use crate::app_state::AppState;
use crate::compat::js_stepper;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ControlDensity, ControlSize, Orientation, StepStatus, StepperSpec, StepperStep,
};

pub fn render(state: &AppState, theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // The Soundcheck arrangement this design came from.
    let wizard = vec![
        StepperStep::new("state", "Current state").with_status(StepStatus::Complete),
        StepperStep::new("recovery", "Recovery").with_status(StepStatus::Complete),
        StepperStep::new("categories", "Categories"),
        StepperStep::new("apply", "Apply and verify").with_disabled(true),
    ];

    let working = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("extract", "Extract tokens").with_status(StepStatus::Running),
        StepperStep::new("map", "Map to theme"),
    ];

    let failed = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("gate", "Quality gate").with_status(StepStatus::Failed),
        StepperStep::new("apply", "Apply changes"),
    ];

    let completed = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("extract", "Extract tokens").with_status(StepStatus::Complete),
    ];

    let lane = vec![
        StepperStep::new("scan", "Scan the tree").with_status(StepStatus::Complete),
        StepperStep::new("plan", "Draft the lane plan").with_status(StepStatus::Complete),
        StepperStep::new("review", "Review with the gate").with_status(StepStatus::Complete),
        StepperStep::new("apply", "Apply the changes").with_status(StepStatus::Complete),
        StepperStep::new("record", "Record architecture verdict and next lane")
            .with_status(StepStatus::Complete),
    ];

    // All four statuses on one rail — the only arrangement where the collapsed
    // form's colour coding is legible at a glance.
    let mixed = vec![
        StepperStep::new("read", "Read source").with_status(StepStatus::Complete),
        StepperStep::new("gate", "Quality gate").with_status(StepStatus::Failed),
        StepperStep::new("extract", "Extract tokens").with_status(StepStatus::Running),
        StepperStep::new("apply", "Apply changes"),
    ];

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Default",
            secondary,
            js_stepper(
                &StepperSpec::new(wizard.clone())
                    .with_value("categories")
                    .with_aria_label("DAW sync steps"),
                theme,
            ),
        ))
        .child(group("Vertical", secondary, {
            let mut el = js_stepper(
                &StepperSpec::new(wizard.clone())
                    .with_orientation(Orientation::Vertical)
                    .with_value("categories")
                    .with_aria_label("DAW sync steps, vertical"),
                theme,
            );
            el = el.w(320.0);
            el
        }))
        // Live: the shell routes a click on the summary's node id back into
        // `AppState::toggle_stepper_collapsed`, so this really folds.
        .child(group("Collapsed", secondary, {
            js_stepper(
                &StepperSpec::new(lane)
                    .with_orientation(Orientation::Vertical)
                    .with_collapsible(true)
                    .with_collapsed(state.stepper_collapsed)
                    .with_value("record")
                    .with_aria_label("Lane progress"),
                theme,
            )
            .w(480.0)
        }))
        .child(group("Collapsed statuses", secondary, {
            js_stepper(
                &StepperSpec::new(mixed)
                    .with_orientation(Orientation::Vertical)
                    .with_collapsible(true)
                    .with_collapsed(true)
                    .with_value("extract")
                    .with_aria_label("Pipeline progress"),
                theme,
            )
            .w(480.0)
        }))
        .child(group(
            "Working",
            secondary,
            js_stepper(
                &StepperSpec::new(working)
                    .with_value("extract")
                    .with_aria_label("Import progress"),
                theme,
            ),
        ))
        .child(group(
            "Failed",
            secondary,
            js_stepper(
                &StepperSpec::new(failed)
                    .with_value("gate")
                    .with_aria_label("Pipeline steps"),
                theme,
            ),
        ))
        .child(group(
            "Re-run",
            secondary,
            js_stepper(
                &StepperSpec::new(completed)
                    .with_value("read")
                    .with_show_rerun(true)
                    .with_aria_label("Completed pipeline"),
                theme,
            ),
        ))
        .child(group("Sizes", secondary, {
            let mut column = div().flex_col().gap(8.0);
            for size in [
                ControlSize::Xs,
                ControlSize::Sm,
                ControlSize::Md,
                ControlSize::Lg,
                ControlSize::Xl,
            ] {
                column = column.child(js_stepper(
                    &StepperSpec::new(wizard.clone())
                        .with_value("categories")
                        .with_size(size)
                        .with_aria_label("Size ladder"),
                    theme,
                ));
            }
            column
        }))
        .child(group("Densities", secondary, {
            let mut column = div().flex_col().gap(8.0);
            for density in [
                ControlDensity::Compact,
                ControlDensity::Default,
                ControlDensity::Comfortable,
            ] {
                column = column.child(js_stepper(
                    &StepperSpec::new(wizard.clone())
                        .with_value("categories")
                        .with_density(density)
                        .with_aria_label("Density ladder"),
                    theme,
                ));
            }
            column
        }))
        .child(group(
            "Disabled",
            secondary,
            js_stepper(
                &StepperSpec::new(wizard)
                    .with_value("categories")
                    .with_disabled(true)
                    .with_aria_label("Disabled steps"),
                theme,
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
