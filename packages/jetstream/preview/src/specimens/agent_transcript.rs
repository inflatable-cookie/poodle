//! AgentTranscript specimen — contract §14.
//!
//! The failing group is the one that matters: a failure that is *not* the run's
//! newest call is invisible in the visible row, so the collapsed toggle is the
//! only thing that can carry it.

use crate::compat::js_agent_transcript;
use crate::nel::*;
use poodle_headless::agent_question::{
    AgentQuestionAnswer, AgentQuestionItem, AgentQuestionOption, AgentQuestionOutcome,
};
use poodle_headless::agent_transcript::{
    ChangedFile, ToolCallStatus, TranscriptActivity, TranscriptAnsweredQuestion,
    TranscriptChangedFiles, TranscriptItem, TranscriptMessage, TranscriptToolCall,
};
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::AgentTranscriptSpec;

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

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // The changed-files card splits the commands either side of it into two
    // runs rather than being absorbed into one.
    let turn = vec![
        message("m1", "The latest fixes hold: 41 parser tests pass."),
        call("t1", "effigy cp-api/test:latex", ToolCallStatus::Success),
        call("t2", "nl -ba src/lexer.rs", ToolCallStatus::Success),
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

    let failing = vec![
        message("f0", "Running the gate."),
        call("f1", "cargo check", ToolCallStatus::Success),
        call("f2", "effigy check:gpui", ToolCallStatus::Error),
        call("f3", "bun test", ToolCallStatus::Success),
    ];

    // The record an answered question leaves: read-only, with every option kept
    // so the reader can see what was not chosen.
    let answered = vec![
        message("aq0", "I need a decision before continuing."),
        TranscriptItem::AnsweredQuestion(TranscriptAnsweredQuestion {
            id: "aq".to_string(),
            question: AgentQuestionItem {
                id: "placement".to_string(),
                header: Some("Placement".to_string()),
                prompt: "Where should the question surface appear?".to_string(),
                options: vec![
                    AgentQuestionOption {
                        value: "inline".into(),
                        label: "Inline in the transcript".into(),
                        description: None,
                    },
                    AgentQuestionOption {
                        value: "composer".into(),
                        label: "Anchored above the composer".into(),
                        description: None,
                    },
                    AgentQuestionOption {
                        value: "modal".into(),
                        label: "Modal dialog".into(),
                        description: None,
                    },
                ],
                allow_multiple: false,
            },
            answer: Some(AgentQuestionAnswer {
                question_id: "placement".to_string(),
                outcome: AgentQuestionOutcome::Selected,
                values: vec!["composer".to_string()],
                text: String::new(),
            }),
        }),
        message("aq1", "Taking the composer route."),
    ];

    let markdown = vec![message(
        "md",
        "Supported subset:\n\n- `inline code` and **strong**\n- nested\n  - items\n\n```rust\nfn main() {}\n```\n\n> a quoted line",
    )];

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "An answered question",
            secondary,
            js_agent_transcript(&AgentTranscriptSpec::new(answered), theme),
        ))
        .child(group(
            "A worked turn",
            secondary,
            js_agent_transcript(&AgentTranscriptSpec::new(turn), theme),
        ))
        .child(group(
            "A run containing a failure",
            secondary,
            js_agent_transcript(&AgentTranscriptSpec::new(failing.clone()), theme),
        ))
        .child(group(
            "Expanded run",
            secondary,
            js_agent_transcript(
                &AgentTranscriptSpec::new(failing).with_expanded_tool_runs(vec!["f1".to_string()]),
                theme,
            ),
        ))
        .child(group(
            "Expanded changed files",
            secondary,
            js_agent_transcript(
                &AgentTranscriptSpec::new(vec![TranscriptItem::ChangedFiles(
                    TranscriptChangedFiles {
                        id: "tree".to_string(),
                        files: vec![
                            ChangedFile {
                                path: "app/src/lib/editor/machine.ts".into(),
                                additions: 12,
                                deletions: 3,
                                status: None,
                            },
                            ChangedFile {
                                path: "app/src/lib/editor/view.ts".into(),
                                additions: 4,
                                deletions: 0,
                                status: None,
                            },
                            ChangedFile {
                                path: "docs/notes.md".into(),
                                additions: 1,
                                deletions: 1,
                                status: None,
                            },
                        ],
                    },
                )])
                .with_expanded_changed_files(vec!["tree".to_string()]),
                theme,
            ),
        ))
        .child(group(
            "Markdown subset",
            secondary,
            js_agent_transcript(&AgentTranscriptSpec::new(markdown), theme),
        ))
        .child(group(
            "Empty",
            secondary,
            js_agent_transcript(&AgentTranscriptSpec::new(Vec::new()), theme),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
