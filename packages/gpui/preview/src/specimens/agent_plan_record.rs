use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{AgentPlanRecord, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_plan::AgentPlanStatus;
use poodle_specs::{AgentPlanRecordSpec, EyebrowSpec};
use std::sync::Arc;

const PLAN: &str = "## Proposed plan\n\n1. Add the `AgentPlan` surface to the composer\n2. Wire the decision callbacks through the host\n3. Append the settled record to the transcript\n\nThen run the gates.";

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let expanded = state.specimens.is_on("agent-plan-record-expanded");
    let events = state.node_events.clone();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Accepted",
            theme,
            AgentPlanRecord::from_spec(
                AgentPlanRecordSpec::new(PLAN, AgentPlanStatus::Accepted).with_expanded(expanded),
                theme,
            )
            .with_instance_id("accepted")
            .on_toggle(Arc::new(move |next| {
                events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                    key: "agent-plan-record-expanded".to_string(),
                    value: next,
                });
            })),
        ))
        .child(group(
            "Revised",
            theme,
            AgentPlanRecord::from_spec(
                AgentPlanRecordSpec::new(PLAN, AgentPlanStatus::Revised)
                    .with_decision_label("Revised with operator feedback"),
                theme,
            )
            .with_instance_id("revised"),
        ))
        .child(group(
            "Dismissed",
            theme,
            AgentPlanRecord::from_spec(
                AgentPlanRecordSpec::new(PLAN, AgentPlanStatus::Dismissed),
                theme,
            )
            .with_instance_id("dismissed"),
        ))
        .child(group(
            "With provenance",
            theme,
            AgentPlanRecord::from_spec(
                AgentPlanRecordSpec::new(PLAN, AgentPlanStatus::Accepted)
                    .with_decided_at("2026-08-07 10:00"),
                theme,
            )
            .with_instance_id("provenance"),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-plan-record",
        examples,
        |size, theme: &GpuiThemeProvider| {
            AgentPlanRecord::from_spec(
                AgentPlanRecordSpec::new(PLAN, AgentPlanStatus::Accepted).with_size(size),
                theme,
            )
            .with_instance_id(format!("size-{size:?}"))
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            AgentPlanRecord::from_spec(
                AgentPlanRecordSpec::new(PLAN, AgentPlanStatus::Accepted).with_density(density),
                theme,
            )
            .with_instance_id(format!("density-{density:?}"))
            .into_any_element()
        },
    )
}
