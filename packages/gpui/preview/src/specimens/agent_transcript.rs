use crate::app_state::AppState;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{AgentTranscript, Eyebrow};
use poodle_specs::{AgentTranscriptSpec, EyebrowSpec};
use poodle_headless::agent_transcript::{
    ChangedFile, ToolCallStatus, TranscriptActivity, TranscriptChangedFiles, TranscriptItem,
    TranscriptMessage, TranscriptToolCall,
};

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

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
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
                ChangedFile { path: "cp-api/crates/latex/src/lexer.rs".into(), additions: 271, deletions: 10, status: None },
                ChangedFile { path: "cp-api/tools/export_fixture.rs".into(), additions: 89, deletions: 1, status: None },
                ChangedFile { path: "cp-docs/book-port.md".into(), additions: 15, deletions: 5, status: None },
            ],
        }),
        call("t6", "jq -r .body_html /tmp/g0216.json", ToolCallStatus::Success),
        TranscriptItem::Activity(TranscriptActivity {
            id: "act".to_string(),
            label: "Working for 1h 1m".to_string(),
        }),
    ];

    // A failure that is not the newest call: collapsed, the run must still
    // advertise it.
    let failing = vec![
        message("f0", "Running the gate."),
        call("f1", "cargo check", ToolCallStatus::Success),
        call("f2", "effigy check:gpui", ToolCallStatus::Error),
        call("f3", "bun test", ToolCallStatus::Success),
    ];

    let markdown = vec![message(
        "md",
        "Supported subset:\n\n- `inline code` and **strong**\n- nested\n  - items\n\n```rust\nfn main() {}\n```\n\n> a quoted line",
    )];

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "A worked turn",
            AgentTranscript::from_spec(AgentTranscriptSpec::new(turn), theme).into_any_element(),
        ))
        .child(group(
            theme,
            "A run containing a failure",
            AgentTranscript::from_spec(AgentTranscriptSpec::new(failing.clone()), theme)
                .into_any_element(),
        ))
        .child(group(
            theme,
            "Expanded run",
            AgentTranscript::from_spec(
                AgentTranscriptSpec::new(failing)
                    .with_expanded_tool_runs(vec!["f1".to_string()]),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Markdown subset",
            AgentTranscript::from_spec(AgentTranscriptSpec::new(markdown), theme).into_any_element(),
        ))
        .child(group(
            theme,
            "Empty",
            AgentTranscript::from_spec(AgentTranscriptSpec::new(Vec::new()), theme)
                .into_any_element(),
        ))
}
