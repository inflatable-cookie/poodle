use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{AgentSubagent, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_subagent::{AgentSubagentItem, AgentSubagentStatus};
use poodle_specs::{AgentSubagentSpec, EyebrowSpec};
use std::sync::Arc;

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

fn item(
    id: &str,
    status: AgentSubagentStatus,
    activity: Option<&str>,
    summary: Option<&str>,
) -> AgentSubagentItem {
    AgentSubagentItem {
        id: id.to_string(),
        label: "Scout".to_string(),
        status,
        activity_line: activity.map(str::to_string),
        summary: summary.map(str::to_string),
    }
}

fn detail_lines() -> Vec<String> {
    vec![
        "Searching packages/contracts/headless/vectors for stale fixtures".to_string(),
        "Matched 41 of 44 vectors against the TS core".to_string(),
        "Diffing the three misses against the Rust mirror".to_string(),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let expanded = state.specimens.is_on("agent-subagent-expanded");
    let opened = state.specimens.is_on("agent-subagent-opened");
    let toggle_events = state.node_events.clone();
    let open_events = state.node_events.clone();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Running",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    AgentSubagent::from_spec(
                        AgentSubagentSpec::new(item(
                            "scout-running",
                            AgentSubagentStatus::Running,
                            Some("Searching the parser crate for the drift"),
                            Some("Found the drift: three vectors were stale"),
                        ))
                        .with_detail_lines(detail_lines())
                        .with_expanded(expanded),
                        theme,
                    )
                    .on_toggle(Arc::new(move |next| {
                        toggle_events
                            .lock()
                            .unwrap()
                            .push(NodeSpecimenEvent::SetToggle {
                                key: "agent-subagent-expanded".to_string(),
                                value: next,
                            });
                    }))
                    .on_open_child(Arc::new(move || {
                        open_events
                            .lock()
                            .unwrap()
                            .push(NodeSpecimenEvent::SetToggle {
                                key: "agent-subagent-opened".to_string(),
                                value: true,
                            });
                    })),
                )
                .child(div().child(if opened {
                    "opened child work"
                } else {
                    "child work closed"
                })),
        ))
        .child(group(
            "Waiting",
            theme,
            AgentSubagent::from_spec(
                AgentSubagentSpec::new(item(
                    "scout-waiting",
                    AgentSubagentStatus::Waiting,
                    Some("Waiting for the operator's decision"),
                    None,
                )),
                theme,
            ),
        ))
        .child(group(
            "Completed",
            theme,
            AgentSubagent::from_spec(
                AgentSubagentSpec::new(item(
                    "scout-completed",
                    AgentSubagentStatus::Completed,
                    None,
                    Some("Found the drift: three vectors were stale"),
                )),
                theme,
            ),
        ))
        .child(group(
            "Failed",
            theme,
            AgentSubagent::from_spec(
                AgentSubagentSpec::new(item(
                    "scout-failed",
                    AgentSubagentStatus::Failed,
                    None,
                    Some("The parser crate failed to build: 3 errors in lexer.rs"),
                )),
                theme,
            ),
        ))
        .child(group(
            "Unknown",
            theme,
            AgentSubagent::from_spec(
                AgentSubagentSpec::new(item(
                    "scout-unknown",
                    AgentSubagentStatus::Unknown,
                    None,
                    None,
                )),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-subagent",
        examples,
        |size, theme: &GpuiThemeProvider| {
            AgentSubagent::from_spec(
                AgentSubagentSpec::new(item(
                    "scout-size",
                    AgentSubagentStatus::Completed,
                    None,
                    Some("Found the drift: three vectors were stale"),
                ))
                .with_size(size),
                theme,
            )
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            AgentSubagent::from_spec(
                AgentSubagentSpec::new(item(
                    "scout-density",
                    AgentSubagentStatus::Running,
                    Some("Searching the parser crate for the drift"),
                    None,
                ))
                .with_density(density),
                theme,
            )
            .into_any_element()
        },
    )
}
