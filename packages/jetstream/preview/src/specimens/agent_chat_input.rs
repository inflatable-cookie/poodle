//! AgentChatInput specimen — agent composer with a model picker in the toolbar,
//! a context ring and the submit/stop action.

use crate::compat::{js_agent_chat_input, js_agent_plan, js_agent_question, js_model_picker};
use crate::nel::*;
use poodle_headless::agent_plan::AgentPlanStatus;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    AgentChatAttachment, AgentChatInputSpec, AgentChatStatus, AgentPlanSpec, AgentQuestionSpec,
    ControlDensity, ControlSize, ModelAxisOption, ModelAxisValue, ModelCapabilityAxis, ModelOption,
    ModelPickerEmphasis, ModelPickerSpec, ModelSelection,
};

fn picker(theme: &JetstreamThemeProvider, size: ControlSize) -> El {
    let spec = ModelPickerSpec::new()
        .with_models(vec![
            ModelOption::new("atlas-pro", "Atlas Pro")
                .with_description("Deepest reasoning")
                .with_badge("1M")
                .with_icon("sparkles"),
            ModelOption::new("atlas", "Atlas").with_description("Balanced"),
        ])
        .with_axes(vec![
            ModelCapabilityAxis::select(
                "effort",
                "Effort",
                vec![
                    ModelAxisOption::new("low", "Low"),
                    ModelAxisOption::new("high", "High"),
                ],
            ),
            ModelCapabilityAxis::toggle("fast", "Fast mode").with_labels("Fast", "Normal"),
        ])
        .with_value(
            ModelSelection::new("atlas-pro")
                .with_axis("effort", ModelAxisValue::Text("high".into()))
                .with_axis("fast", ModelAxisValue::Flag(false)),
        )
        .with_size(size)
        // Subdued inside the composer: the editor should hold the eye.
        .with_emphasis(ModelPickerEmphasis::Subdued);
    js_model_picker(&spec, theme)
}

fn base() -> AgentChatInputSpec {
    AgentChatInputSpec::new().with_placeholder("Ask for follow-up changes or attach images")
}

fn question(theme: &JetstreamThemeProvider) -> El {
    let item = AgentQuestionItem {
        id: "placement".to_string(),
        header: Some("Placement".to_string()),
        prompt: "Where should the question surface appear?".to_string(),
        options: vec![
            AgentQuestionOption {
                value: "composer".to_string(),
                label: "Anchored above the composer".to_string(),
                description: Some("Pinned over the input.".to_string()),
            },
            AgentQuestionOption {
                value: "inline".to_string(),
                label: "Inline in the transcript".to_string(),
                description: Some("A block in the conversation.".to_string()),
            },
        ],
        allow_multiple: false,
    };
    js_agent_question(
        &AgentQuestionSpec::new(vec![item]).with_selections(vec!["composer".to_string()]),
        theme,
    )
}

fn plan(theme: &JetstreamThemeProvider) -> El {
    js_agent_plan(
        &AgentPlanSpec::new("1. Inspect the contract.\n2. Apply the bounded change.")
            .with_status(AgentPlanStatus::Pending),
        theme,
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Composer with model picker + context ring",
            secondary,
            js_agent_chat_input(
                &base().with_context(64_000.0, 200_000.0),
                theme,
                Vec::new(),
                Vec::new(),
                // Three controls, so the hairline dividers between them render.
                vec![
                    picker(theme, ControlSize::Md),
                    label("Full access").text_color(secondary).text_size(13.0),
                    label("Build").text_color(secondary).text_size(13.0),
                ],
                Vec::new(),
            ),
        ))
        .child(group(
            "Questioning",
            secondary,
            js_agent_chat_input(
                &base()
                    .with_status(AgentChatStatus::Questioning)
                    .with_question_can_submit(true),
                theme,
                vec![question(theme)],
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ))
        .child(group(
            "Reviewing plan",
            secondary,
            js_agent_chat_input(
                &base().with_status(AgentChatStatus::ReviewingPlan),
                theme,
                Vec::new(),
                vec![plan(theme)],
                Vec::new(),
                Vec::new(),
            ),
        ))
        .child(group(
            "Composing (submit enabled)",
            secondary,
            js_agent_chat_input(
                &base()
                    .with_value("Summarise the release notes and open a PR")
                    .with_context(64_000.0, 200_000.0),
                theme,
                Vec::new(),
                Vec::new(),
                vec![picker(theme, ControlSize::Md)],
                Vec::new(),
            ),
        ))
        .child(group(
            "Busy (stop state, context above the warn threshold)",
            secondary,
            js_agent_chat_input(
                &base()
                    .with_value("Summarise the release notes and open a PR")
                    .with_status(AgentChatStatus::Busy)
                    .with_context(172_000.0, 200_000.0),
                theme,
                Vec::new(),
                Vec::new(),
                vec![picker(theme, ControlSize::Md)],
                Vec::new(),
            ),
        ))
        .child(group(
            "Attachments (image tile + file chip) + footer bar",
            secondary,
            js_agent_chat_input(
                &base()
                    .with_value("Fix the failing parity gate")
                    .with_attachments(vec![
                        AgentChatAttachment::new("a1", "architecture.png")
                            .with_kind("image")
                            // Images render as tiles, not chips.
                            .with_thumbnail("assets/thumbs/architecture.png"),
                        AgentChatAttachment::new("a2", "release-notes.md")
                            .with_kind("document")
                            .with_icon("file-text"),
                    ])
                    .with_context(22_000.0, 200_000.0),
                theme,
                Vec::new(),
                Vec::new(),
                vec![picker(theme, ControlSize::Md)],
                vec![
                    label("Current checkout")
                        .text_color(secondary)
                        .text_size(13.0),
                    label("main").text_color(secondary).text_size(13.0),
                ],
            ),
        ))
        .child(group(
            "Grown editor (at the maxRows ceiling)",
            secondary,
            js_agent_chat_input(
                &base()
                    .with_value("Line one\nLine two\nLine three\nLine four\nLine five\nLine six")
                    .with_rows(2, 4),
                theme,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_agent_chat_input(
                &base()
                    .with_value("Composer unavailable")
                    .with_disabled(true)
                    .with_context(10_000.0, 200_000.0),
                theme,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
        ))
        .child(group(
            "Sizes",
            secondary,
            div().flex_col().gap(12.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| {
                    js_agent_chat_input(
                        &base().with_size(size).with_context(40_000.0, 200_000.0),
                        theme,
                        Vec::new(),
                        Vec::new(),
                        vec![picker(theme, size)],
                        Vec::new(),
                    )
                }),
            ),
        ))
        .child(group(
            "Densities",
            secondary,
            div().flex_col().gap(12.0).children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| {
                    js_agent_chat_input(
                        &base()
                            .with_density(density)
                            .with_context(40_000.0, 200_000.0),
                        theme,
                        Vec::new(),
                        Vec::new(),
                        vec![picker(theme, ControlSize::Md)],
                        Vec::new(),
                    )
                }),
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
