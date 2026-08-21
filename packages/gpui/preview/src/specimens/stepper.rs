use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Stepper};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
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

/// Retained current step for the wizard. Both orientations read it, because
/// they are the same process drawn two ways.
const WIZARD_CURRENT: &str = "stepper-current";
/// Retained current step for the Re-run group, kept apart from the wizard's so
/// the receipt below can show that re-running left it alone.
const RERUN_CURRENT: &str = "stepper-rerun-current";
/// The last step the host was asked to re-run.
const RERUN_LAST: &str = "stepper-rerun-last";

fn retained<'a>(state: &'a AppState, key: &str, fallback: &'a str) -> &'a str {
    state
        .specimens
        .text
        .get(key)
        .map(String::as_str)
        .unwrap_or(fallback)
}

fn set_text(state: &AppState, key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

/// The visible step name for a value, so the receipts read as the wizard does
/// rather than as its identifiers.
fn label_of(steps: &[StepperStep], value: &str) -> String {
    steps
        .iter()
        .find(|step| step.value == value)
        .map(|step| step.label.clone())
        .unwrap_or_else(|| value.to_string())
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let receipt_color = color_to_hsla(theme.resolve_color("color.text.secondary"));
    let receipt = move |line: String| div().text_sm().text_color(receipt_color).child(line);

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
    let rerun_current = retained(state, RERUN_CURRENT, "read").to_string();

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

    // Live: clicking a step really selects it, on both orientations, because
    // they read the one retained value. The disabled step stays unreachable.
    let wizard_current = retained(state, WIZARD_CURRENT, "categories").to_string();
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Guided workflow",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(
                    Stepper::from_spec(
                        StepperSpec::new(wizard_steps())
                            .with_value(wizard_current.clone())
                            .with_aria_label("DAW sync steps"),
                        theme,
                    )
                    .on_change(set_text(state, WIZARD_CURRENT)),
                )
                .child(
                    div().max_w(px(320.0)).child(
                        Stepper::from_spec(
                            StepperSpec::new(wizard_steps())
                                .with_orientation(Orientation::Vertical)
                                .with_value(wizard_current.clone())
                                .with_aria_label("DAW sync steps, vertical"),
                            theme,
                        )
                        .on_change(set_text(state, WIZARD_CURRENT)),
                    ),
                )
                .child(receipt(format!(
                    "Current step: {}",
                    label_of(&wizard_steps(), &wizard_current)
                )))
                .into_any_element(),
        ))
        // Live: clicking the summary really folds and unfolds the track. The
        // key reads "expanded" rather than "collapsed" so the unset default
        // starts collapsed, which is the state worth showing first.
        .child(group(
            theme,
            "Collapsed progress",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
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
            "Running and failed states",
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(Stepper::from_spec(
                    StepperSpec::new(working)
                        .with_value("extract")
                        .with_aria_label("Import progress"),
                    theme,
                ))
                .child(Stepper::from_spec(
                    StepperSpec::new(failed)
                        .with_value("gate")
                        .with_aria_label("Pipeline steps"),
                    theme,
                ))
                .into_any_element(),
        ))
        // Live, and the point of the group: the trigger navigates, the re-run
        // control re-runs, and the receipt shows they never stand in for each
        // other — the current step does not move when a step is re-run.
        .child(group(
            theme,
            "Re-run",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    Stepper::from_spec(
                        StepperSpec::new(completed.clone())
                            .with_value(rerun_current.clone())
                            .with_show_rerun(true)
                            .with_aria_label("Completed pipeline"),
                        theme,
                    )
                    .on_change(set_text(state, RERUN_CURRENT))
                    .on_rerun(set_text(state, RERUN_LAST)),
                )
                .child(receipt(format!(
                    "Current step: {} — last re-run: {}",
                    label_of(&completed, &rerun_current),
                    match retained(state, RERUN_LAST, "") {
                        "" => "none yet".to_string(),
                        value => label_of(&completed, value),
                    }
                )))
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
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme: &GpuiThemeProvider| {
                Stepper::from_spec(
                    StepperSpec::new(wizard_steps())
                        .with_value("categories")
                        .with_size(size)
                        .with_aria_label("Size ladder"),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(move |density, theme: &GpuiThemeProvider| {
                Stepper::from_spec(
                    StepperSpec::new(wizard_steps())
                        .with_value("categories")
                        .with_density(density)
                        .with_aria_label("Density ladder"),
                    theme,
                )
                .into_any_element()
            }),
    )
}
