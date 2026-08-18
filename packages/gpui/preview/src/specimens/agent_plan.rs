use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{AgentChatInput, AgentPlan, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_plan::AgentPlanStatus;
use poodle_specs::{AgentChatInputSpec, AgentChatStatus, AgentPlanSpec, EyebrowSpec};
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

fn set_status(state: &AppState, status: AgentPlanStatus) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "agent-plan-status".to_string(),
            value: status.as_str().to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let status = AgentPlanStatus::from_str_or_default(
        state
            .specimens
            .text
            .get("agent-plan-status")
            .map(String::as_str)
            .unwrap_or("pending"),
    );
    let live = AgentPlan::from_spec(AgentPlanSpec::new(PLAN).with_status(status), theme)
        .with_instance_id("live")
        .on_accept(set_status(state, AgentPlanStatus::Accepted))
        .on_revise(set_status(state, AgentPlanStatus::Revised))
        .on_dismiss(set_status(state, AgentPlanStatus::Dismissed));

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Hosted by the composer",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    AgentChatInput::from_spec(
                        AgentChatInputSpec::new()
                            .with_placeholder("Ask for follow-up changes or attach images")
                            .with_status(AgentChatStatus::ReviewingPlan),
                        theme,
                    )
                    .plan_child(live),
                )
                .child(div().child(if status == AgentPlanStatus::Pending {
                    "no decision yet".to_string()
                } else {
                    format!("decided: {}", status.as_str())
                })),
        ))
        .child(group(
            "Settled",
            theme,
            AgentPlan::from_spec(
                AgentPlanSpec::new(PLAN).with_status(AgentPlanStatus::Accepted),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-plan",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                AgentPlan::from_spec(
                    AgentPlanSpec::new(PLAN)
                        .with_status(AgentPlanStatus::Accepted)
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                AgentPlan::from_spec(
                    AgentPlanSpec::new(PLAN)
                        .with_status(AgentPlanStatus::Accepted)
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
