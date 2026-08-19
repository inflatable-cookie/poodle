use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::AgentChatInput;
use crate::node_compat::AgentQuestion;
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_specs::{AgentChatInputSpec, AgentChatStatus, AgentQuestionSpec, EyebrowSpec};

fn option(value: &str, label: &str, description: Option<&str>) -> AgentQuestionOption {
    AgentQuestionOption {
        value: value.to_string(),
        label: label.to_string(),
        description: description.map(str::to_string),
    }
}

fn placement() -> AgentQuestionItem {
    AgentQuestionItem {
        id: "placement".to_string(),
        header: Some("Placement".to_string()),
        prompt:
            "When the agent needs an answer mid-turn, where should the question surface appear?"
                .to_string(),
        options: vec![
            option(
                "inline",
                "Inline in the transcript",
                Some("A block in the conversation, in sequence with messages and tool runs."),
            ),
            option(
                "composer",
                "Anchored above the composer",
                Some("A card pinned over the input, always visible until answered."),
            ),
            option(
                "modal",
                "Modal dialog",
                Some("Blocks the app until answered."),
            ),
        ],
        allow_multiple: false,
    }
}

fn targets() -> AgentQuestionItem {
    AgentQuestionItem {
        id: "targets".to_string(),
        header: Some("Targets".to_string()),
        prompt: "Which targets should this ship to?".to_string(),
        options: vec![
            option("svelte", "Svelte", None),
            option("react", "React", None),
            option("gpui", "GPUI", None),
            option("jetstream", "Jetstream", None),
        ],
        allow_multiple: true,
    }
}

fn many() -> AgentQuestionItem {
    AgentQuestionItem {
        id: "many".to_string(),
        header: Some("Priority".to_string()),
        prompt: "Which remaining check should run first?".to_string(),
        options: (1..=12)
            .map(|index| option(&format!("step-{index}"), &format!("Step {index}"), None))
            .collect(),
        allow_multiple: false,
    }
}

fn batch() -> Vec<AgentQuestionItem> {
    let mut third = placement();
    third.id = "third".to_string();
    third.header = Some("Scale".to_string());
    let mut fourth = targets();
    fourth.id = "fourth".to_string();
    fourth.header = Some("Rollout".to_string());
    vec![placement(), targets(), third, fourth]
}

fn select_handler(state: &AppState, key: &str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.to_string();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.clone(),
            value: value.to_string(),
        });
    })
}

fn multi_select_handler(state: &AppState) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Toggle(format!(
                "agent-question-multi-{value}"
            )));
    })
}

fn selected_values(state: &AppState, prefix: &str, options: &[AgentQuestionOption]) -> Vec<String> {
    options
        .iter()
        .filter(|option| state.specimens.is_on(&format!("{prefix}{}", option.value)))
        .map(|option| option.value.clone())
        .collect()
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let hosted_key = "agent-question-hosted";
    let hosted_chosen = state
        .specimens
        .text
        .get(hosted_key)
        .cloned()
        .unwrap_or_default();
    let single_key = "agent-question-single";
    let chosen = state
        .specimens
        .text
        .get(single_key)
        .cloned()
        .unwrap_or_default();
    let multi_selected = selected_values(state, "agent-question-multi-", &targets().options);

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

    let hosted_question = poodle_render::agent_question(
        &AgentQuestionSpec::new(vec![placement()]).with_selections(if hosted_chosen.is_empty() {
            Vec::new()
        } else {
            vec![hosted_chosen.clone()]
        }),
        theme,
        poodle_render::AgentQuestionHandlers {
            on_select: Some(select_handler(state, hosted_key)),
            ..Default::default()
        },
    );
    // Native editors are render-only, so the override case is taught as the
    // already-typed result rather than a live keystroke.
    let hosted_override = poodle_render::agent_question(
        &AgentQuestionSpec::new(vec![placement()])
            .with_override("Neither — put it in the sidebar."),
        theme,
        poodle_render::AgentQuestionHandlers::default(),
    );

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Hosted by the composer",
            stack([
                AgentChatInput::from_spec(
                    AgentChatInputSpec::new()
                        .with_status(AgentChatStatus::Questioning)
                        .with_question_can_submit(!hosted_chosen.is_empty()),
                    theme,
                )
                .question_child(hosted_question)
                .into_any_element(),
                AgentChatInput::from_spec(
                    AgentChatInputSpec::new()
                        .with_status(AgentChatStatus::Questioning)
                        .with_value("Neither — put it in the sidebar.")
                        .with_question_can_submit(true),
                    theme,
                )
                .question_child(hosted_override)
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Choice modes",
            stack([
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![placement()]).with_selections(
                        if chosen.is_empty() {
                            Vec::new()
                        } else {
                            vec![chosen.clone()]
                        },
                    ),
                    theme,
                )
                .on_select(select_handler(state, single_key))
                .into_any_element(),
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![targets()]).with_selections(multi_selected),
                    theme,
                )
                .on_select(multi_select_handler(state))
                .into_any_element(),
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![placement()])
                        .with_selections(vec!["composer".to_string()]),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Batch progress",
            stack([
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(batch()).with_active_index(1),
                    theme,
                )
                .into_any_element(),
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(batch()).with_active_index(3),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Dismissal",
            stack([
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![placement()]).with_dismissible(true),
                    theme,
                )
                .into_any_element(),
                AgentQuestion::from_spec(AgentQuestionSpec::new(vec![placement()]), theme)
                    .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Shortcut limits",
            stack([
                AgentQuestion::from_spec(AgentQuestionSpec::new(vec![many()]), theme)
                    .into_any_element(),
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![placement()]).with_show_shortcuts(false),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "agent-question",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![placement()]).with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                AgentQuestion::from_spec(
                    AgentQuestionSpec::new(vec![placement()]).with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
