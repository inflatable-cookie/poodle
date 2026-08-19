use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{AgentTranscript, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::{
    ChangedFile, ToolCallStatus, TranscriptActivity, TranscriptChangedFiles, TranscriptItem,
    TranscriptMessage, TranscriptToolCall,
};
use poodle_specs::{AgentTranscriptSpec, EyebrowSpec};

fn call(id: &str, detail: &str, status: ToolCallStatus) -> TranscriptItem {
    TranscriptItem::ToolCall(TranscriptToolCall {
        id: id.to_string(),
        label: "Ran command".to_string(),
        detail: Some(detail.to_string()),
        status,
        ..Default::default()
    })
}

fn message(id: &str, markdown: &str) -> TranscriptItem {
    TranscriptItem::Message(TranscriptMessage {
        id: id.to_string(),
        markdown: markdown.to_string(),
        ..Default::default()
    })
}

fn streaming_message(id: &str, markdown: &str) -> TranscriptItem {
    TranscriptItem::Message(TranscriptMessage {
        id: id.to_string(),
        markdown: markdown.to_string(),
        is_streaming: true,
        ..Default::default()
    })
}

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

fn stack(children: impl IntoIterator<Item = AnyElement>) -> Div {
    let mut col = div().flex().flex_col().gap(px(12.0));
    for child in children {
        col = col.child(child);
    }
    col
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // The worked turn: the changed-files card splits the commands either side
    // of it into two runs rather than being absorbed into one.
    let turn = vec![
        message(
            "m1",
            "The latest fixes hold: 41 parser tests pass. AO415 now matches in text and structure.",
        ),
        call("t1", "effigy cp-api/test:latex", ToolCallStatus::Success),
        call("t2", "nl -ba src/lexer.rs", ToolCallStatus::Success),
        call("t3", "sed -n '145,175p' lexer.rs", ToolCallStatus::Success),
        message("m2", "AO415 and RO418 now both reach full semantic parity."),
        TranscriptItem::ChangedFiles(TranscriptChangedFiles {
            id: "diff".to_string(),
            files: vec![
                ChangedFile {
                    path: "cp-api/crates/latex/src/lexer.rs".into(),
                    additions: 271,
                    deletions: 10,
                    status: None,
                },
                ChangedFile {
                    path: "cp-api/tools/export_fixture.rs".into(),
                    additions: 89,
                    deletions: 1,
                    status: None,
                },
                ChangedFile {
                    path: "cp-docs/book-port.md".into(),
                    additions: 15,
                    deletions: 5,
                    status: None,
                },
            ],
        }),
        call(
            "t6",
            "jq -r .body_html /tmp/g0216.json",
            ToolCallStatus::Success,
        ),
        TranscriptItem::Activity(TranscriptActivity {
            id: "act".to_string(),
            label: "Working for 1h 1m".to_string(),
            spinning: None,
        }),
    ];

    let simple = vec![
        message("s0", "Running the gate."),
        call("s1", "effigy check:gpui", ToolCallStatus::Success),
    ];

    let mut thirty = vec![message("t30m", "Running the remaining checks.")];
    thirty.extend((1..=30).map(|index| {
        call(
            &format!("t30-{index}"),
            &format!("check {index}"),
            ToolCallStatus::Success,
        )
    }));

    let failing = vec![
        message("f0", "Running the gate."),
        call("f1", "cargo check", ToolCallStatus::Success),
        call("f2", "effigy check:gpui", ToolCallStatus::Error),
        call("f3", "bun test", ToolCallStatus::Success),
    ];

    let streaming = vec![
        message("st1", "Reading the parser now"),
        streaming_message("st2", "The corpus-wide patterns were genuine legacy"),
    ];

    let long: Vec<TranscriptItem> = (0..40)
        .map(|index| {
            if index % 3 == 0 {
                message(
                    &format!("lm{index}"),
                    &format!("Block {index}. Mixed-height content for windowing."),
                )
            } else {
                call(
                    &format!("lc{index}"),
                    &format!("step {index} of a long session"),
                    ToolCallStatus::Success,
                )
            }
        })
        .collect();

    let expanded_for = |prefix: &str, ids: &[&str]| -> Vec<String> {
        ids.iter()
            .filter(|id| state.specimens.is_on(&format!("transcript.{prefix}.{id}")))
            .map(|id| id.to_string())
            .collect()
    };
    let toggle = |prefix: &'static str| -> Arc<dyn Fn(&str) + Send + Sync> {
        let events = state.node_events.clone();
        Arc::new(move |id| {
            events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::Toggle(format!(
                    "transcript.{prefix}.{id}"
                )));
        })
    };

    let detached_spec = AgentTranscriptSpec::new(long.clone());
    let detached_content = poodle_render::agent_transcript(
        &detached_spec,
        theme,
        poodle_render::AgentTranscriptHandlers::default(),
    );
    let mut jump_control = poodle_render::agent_transcript::agent_transcript_jump(
        &detached_spec,
        theme,
        Some(state.agent_transcript_scroll.jump_handler()),
    );
    jump_control.id = Some("agent-transcript-jump-control".to_string());

    let worked = AgentTranscript::from_spec(
        AgentTranscriptSpec::new(turn)
            .with_expanded_tool_runs(expanded_for("run", &["t1", "t6"]))
            .with_expanded_tool_calls(expanded_for("call", &["t1", "t2", "t3", "t6"]))
            .with_expanded_changed_files(expanded_for("diff", &["diff"])),
        theme,
    )
    .on_tool_run_toggle(toggle("run"))
    .on_tool_call_toggle(toggle("call"))
    .on_changed_files_toggle(toggle("diff"))
    .into_any_element();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(theme, "A worked turn", worked))
        .child(group(
            theme,
            "Tool run states",
            stack([
                AgentTranscript::from_spec(AgentTranscriptSpec::new(simple), theme)
                    .into_any_element(),
                AgentTranscript::from_spec(AgentTranscriptSpec::new(thirty.clone()), theme)
                    .into_any_element(),
                AgentTranscript::from_spec(
                    AgentTranscriptSpec::new(thirty)
                        .with_expanded_tool_runs(vec!["t30-1".to_string()]),
                    theme,
                )
                .into_any_element(),
                AgentTranscript::from_spec(AgentTranscriptSpec::new(failing), theme)
                    .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Streaming and detached scroll",
            stack([
                AgentTranscript::from_spec(AgentTranscriptSpec::new(streaming), theme)
                    .into_any_element(),
                div()
                    .h(px(256.0))
                    .child(poodle_gpui_node_backend::tracked_vertical_scroll(
                        &detached_content,
                        &jump_control,
                        &state.agent_transcript_scroll,
                        poodle_gpui_node_backend::TrackedScrollOptions {
                            viewport_id: "agent-transcript-detached-viewport",
                            jump_id: "agent-transcript-jump",
                            pin_threshold: detached_spec.pin_threshold,
                            auto_follow: detached_spec.is_auto_scroll,
                            is_empty: detached_spec.is_empty(),
                        },
                    ))
                    .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Long transcript rendering",
            stack([
                AgentTranscript::from_spec(
                    AgentTranscriptSpec::new(long.clone()).with_virtualized(true),
                    theme,
                )
                .into_any_element(),
                AgentTranscript::from_spec(
                    AgentTranscriptSpec::new(long).with_virtualized(false),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Empty",
            AgentTranscript::from_spec(AgentTranscriptSpec::new(Vec::new()), theme)
                .into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-transcript",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                AgentTranscript::from_spec(
                    AgentTranscriptSpec::new(vec![
                        message("axis-m1", "A short worked turn."),
                        call("axis-t1", "effigy check:gpui", ToolCallStatus::Success),
                    ])
                    .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                AgentTranscript::from_spec(
                    AgentTranscriptSpec::new(vec![
                        message("axis-m1", "A short worked turn."),
                        call("axis-t1", "effigy check:gpui", ToolCallStatus::Success),
                    ])
                    .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
