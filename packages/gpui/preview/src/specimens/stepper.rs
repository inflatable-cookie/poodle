use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Stepper};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, Orientation, StepStatus, StepperSpec, StepperStep};

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
            "Vertical",
            div()
                .max_w(px(320.0))
                .child(Stepper::from_spec(
                    StepperSpec::new(wizard_steps())
                        .with_orientation(Orientation::Vertical)
                        .with_value("categories")
                        .with_aria_label("DAW sync steps, vertical"),
                    theme,
                ))
                .into_any_element(),
        ))
        // Live: clicking the summary really folds and unfolds the track. The
        // key reads "expanded" rather than "collapsed" so the unset default
        // starts collapsed, which is the state worth showing first.
        .child(group(
            theme,
            "Collapsed",
            div()
                .max_w(px(480.0))
                .child(
                    Stepper::from_spec(
                        StepperSpec::new(lane)
                            .with_orientation(Orientation::Vertical)
                            .with_collapsible(true)
                            .with_collapsed(!state.specimens.is_on("stepper.expanded"))
                            .with_value("record")
                            .with_aria_label("Lane progress"),
                        theme,
                    )
                    .on_collapsed_change({
                        let events = state.node_events.clone();
                        Arc::new(move |_collapsed| {
                            events
                                .lock()
                                .unwrap()
                                .push(NodeSpecimenEvent::Toggle("stepper.expanded".to_string()));
                        })
                    }),
                )
                .into_any_element(),
        ))
        .child(group(
            theme,
            "Collapsed statuses",
            div()
                .max_w(px(480.0))
                .child(Stepper::from_spec(
                    StepperSpec::new(mixed)
                        .with_orientation(Orientation::Vertical)
                        .with_collapsible(true)
                        .with_collapsed(true)
                        .with_value("extract")
                        .with_aria_label("Pipeline progress"),
                    theme,
                ))
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
