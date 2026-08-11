use crate::app_state::AppState;
use crate::node_compat::{AgentChatInput, Eyebrow, IntoCompatNode, ModelPicker};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_plan::AgentPlanStatus;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_node::Node;
use poodle_specs::{
    AgentChatAttachment, AgentChatInputSpec, AgentChatStatus, AgentPlanSpec, AgentQuestionSpec,
    ControlSize, EyebrowSpec, ModelAxisOption, ModelAxisValue, ModelCapabilityAxis, ModelOption,
    ModelPickerEmphasis, ModelPickerSpec, ModelSelection,
};

fn demo_picker(theme: &GpuiThemeProvider, size: ControlSize) -> Node {
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
    ModelPicker::from_spec(spec, theme).into_compat_node()
}

fn toolbar_text(theme: &GpuiThemeProvider, value: &'static str) -> Node {
    let mut node = Node::text(value);
    node.style.text_size = Some(theme.resolve_space("typography.caption.size"));
    node.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    node
}

fn demo_spec() -> AgentChatInputSpec {
    AgentChatInputSpec::new().with_placeholder("Ask for follow-up changes or attach images")
}

fn question_node(theme: &GpuiThemeProvider) -> Node {
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
    poodle_render::agent_question(
        &AgentQuestionSpec::new(vec![item]).with_selections(vec!["composer".to_string()]),
        theme,
        poodle_render::AgentQuestionHandlers::default(),
    )
}

fn plan_node(theme: &GpuiThemeProvider) -> Node {
    poodle_render::agent_plan(
        &AgentPlanSpec::new("1. Inspect the contract.\n2. Apply the bounded change.")
            .with_status(AgentPlanStatus::Pending),
        theme,
        poodle_render::AgentPlanHandlers::default(),
    )
}

fn section(title: &str, theme: &GpuiThemeProvider, content: AnyElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(section(
            "Composer with model picker + context ring",
            theme,
            // Three controls, so the hairline dividers between them render.
            AgentChatInput::from_spec(demo_spec().with_context(64_000.0, 200_000.0), theme)
                .toolbar_child(demo_picker(theme, ControlSize::Md))
                .toolbar_child(toolbar_text(theme, "Full access"))
                .toolbar_child(toolbar_text(theme, "Build"))
                .into_any_element(),
        ))
        .child(section(
            "Questioning",
            theme,
            AgentChatInput::from_spec(
                demo_spec()
                    .with_status(AgentChatStatus::Questioning)
                    .with_question_can_submit(true),
                theme,
            )
            .question_child(question_node(theme))
            .into_any_element(),
        ))
        .child(section(
            "Reviewing plan",
            theme,
            AgentChatInput::from_spec(
                demo_spec().with_status(AgentChatStatus::ReviewingPlan),
                theme,
            )
            .plan_child(plan_node(theme))
            .into_any_element(),
        ))
        .child(section(
            "Composing (submit enabled)",
            theme,
            AgentChatInput::from_spec(
                demo_spec()
                    .with_value("Summarise the release notes and open a PR")
                    .with_context(64_000.0, 200_000.0),
                theme,
            )
            .toolbar_child(demo_picker(theme, ControlSize::Md))
            .into_any_element(),
        ))
        .child(section(
            "Busy (stop state, context above the warn threshold)",
            theme,
            AgentChatInput::from_spec(
                demo_spec()
                    .with_value("Summarise the release notes and open a PR")
                    .with_status(AgentChatStatus::Busy)
                    .with_context(172_000.0, 200_000.0),
                theme,
            )
            .toolbar_child(demo_picker(theme, ControlSize::Md))
            .into_any_element(),
        ))
        .child(section(
            "Attachments (image tile + file chip) + footer bar",
            theme,
            AgentChatInput::from_spec(
                demo_spec()
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
            )
            .toolbar_child(demo_picker(theme, ControlSize::Md))
            .footer_child(toolbar_text(theme, "Current checkout"))
            .footer_child(toolbar_text(theme, "main"))
            .into_any_element(),
        ))
        .child(section(
            "Grown editor (at the maxRows ceiling)",
            theme,
            AgentChatInput::from_spec(
                demo_spec()
                    .with_value("Line one\nLine two\nLine three\nLine four\nLine five\nLine six")
                    .with_rows(2, 4),
                theme,
            )
            .into_any_element(),
        ))
        .child(section(
            "Disabled",
            theme,
            AgentChatInput::from_spec(
                demo_spec()
                    .with_value("Composer unavailable")
                    .with_disabled(true)
                    .with_context(10_000.0, 200_000.0),
                theme,
            )
            .into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-chat-input",
        examples,
        |size, theme: &GpuiThemeProvider| {
            AgentChatInput::from_spec(demo_spec().with_context(40_000.0, 200_000.0), theme)
                .size(size)
                .toolbar_child(demo_picker(theme, size))
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            AgentChatInput::from_spec(demo_spec().with_context(40_000.0, 200_000.0), theme)
                .with_density(density)
                .toolbar_child(demo_picker(theme, ControlSize::Md))
                .into_any_element()
        },
    )
}
