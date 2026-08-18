use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ToolCallGroup};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::{ToolCallStatus, TranscriptToolCall};
use poodle_specs::{EyebrowSpec, ToolCallGroupSpec};
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

fn call(id: &str, detail: &str, status: ToolCallStatus) -> TranscriptToolCall {
    TranscriptToolCall {
        id: id.to_string(),
        label: "Ran command".to_string(),
        detail: Some(detail.to_string()),
        status,
        icon: None,
        output: None,
    }
}

fn three() -> Vec<TranscriptToolCall> {
    vec![
        call("a", "nl -ba src/lexer.rs", ToolCallStatus::Success),
        TranscriptToolCall {
            id: "b".to_string(),
            label: "Ran command".to_string(),
            detail: Some("effigy cp-api/test:latex".to_string()),
            status: ToolCallStatus::Success,
            icon: None,
            output: Some("ok".to_string()),
        },
        call("c", "sed -n '430,560p' cp_html.rs", ToolCallStatus::Success),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let expanded = state.specimens.is_on("tool-call-group-three");
    let mut expanded_calls = Vec::new();
    if state.specimens.is_on("tool-call-group-call-b") {
        expanded_calls.push("b".to_string());
    }
    let toggle_events = state.node_events.clone();
    let call_events = state.node_events.clone();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Single call",
            theme,
            ToolCallGroup::from_spec(
                ToolCallGroupSpec::new(
                    "single",
                    vec![call("only", "bun test", ToolCallStatus::Success)],
                ),
                theme,
            ),
        ))
        .child(group(
            "Collapsed and expanded",
            theme,
            ToolCallGroup::from_spec(
                ToolCallGroupSpec::new("three", three())
                    .with_expanded(expanded)
                    .with_expanded_calls(expanded_calls),
                theme,
            )
            .on_toggle(Arc::new(move |id| {
                toggle_events
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::Toggle(format!("tool-call-group-{id}")));
            }))
            .with_instance_id("three")
            .on_call_toggle(Arc::new(move |id| {
                call_events
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::Toggle(format!(
                        "tool-call-group-call-{id}"
                    )));
            })),
        ))
        .child(group(
            "Buried failure",
            theme,
            ToolCallGroup::from_spec(
                ToolCallGroupSpec::new(
                    "buried",
                    vec![
                        call("f1", "cargo check", ToolCallStatus::Success),
                        call("f2", "effigy check:gpui", ToolCallStatus::Error),
                        call("f3", "bun test", ToolCallStatus::Success),
                    ],
                ),
                theme,
            ),
        ))
        .child(group(
            "Running",
            theme,
            ToolCallGroup::from_spec(
                ToolCallGroupSpec::new(
                    "running",
                    vec![
                        call("r1", "cargo build", ToolCallStatus::Success),
                        call("r2", "cargo test", ToolCallStatus::Running),
                    ],
                ),
                theme,
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "tool-call-group",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                ToolCallGroup::from_spec(
                    ToolCallGroupSpec::new(format!("sz-{size:?}"), three()).with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                ToolCallGroup::from_spec(
                    ToolCallGroupSpec::new(format!("dn-{density:?}"), three())
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
