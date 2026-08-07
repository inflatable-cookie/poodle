use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::Eyebrow;
use crate::node_compat::AgentQuestion;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_specs::{AgentQuestionSpec, EyebrowSpec};

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
        prompt: "Where should the question surface appear?".to_string(),
        options: vec![
            option(
                "inline",
                "Inline in the transcript",
                Some("A block in the conversation."),
            ),
            option(
                "composer",
                "Anchored above the composer",
                Some("Pinned over the input."),
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
        header: None,
        prompt: "Which targets should this ship to?".to_string(),
        options: vec![
            option("svelte", "Svelte", None),
            option("react", "React", None),
            option("gpui", "GPUI", None),
        ],
        allow_multiple: true,
    }
}

fn single_select_handler(state: &AppState) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: "agent-question-single".to_string(),
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
            .push(NodeSpecimenEvent::Toggle(format!("agent-question-multi-{value}")));
    })
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // Single-select stores the chosen value; multi-select stores a flag per
    // option. The preview owns this state, exactly as a host would.
    let single_key = "agent-question-single";
    let chosen = state
        .specimens
        .text
        .get(single_key)
        .cloned()
        .unwrap_or_default();

    let multi_selected: Vec<String> = targets()
        .options
        .iter()
        .filter(|option| {
            state
                .specimens
                .is_on(&format!("agent-question-multi-{}", option.value))
        })
        .map(|option| option.value.clone())
        .collect();

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

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // Live: clicking an option actually selects it.
        .child(group(
            theme,
            "Single select",
            AgentQuestion::from_spec(
                AgentQuestionSpec::new(vec![placement()]).with_selections(if chosen.is_empty() {
                    Vec::new()
                } else {
                    vec![chosen.clone()]
                }),
                theme,
            )
            .on_select(single_select_handler(state))
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Multi select",
            AgentQuestion::from_spec(
                AgentQuestionSpec::new(vec![targets()]).with_selections(multi_selected),
                theme,
            )
            .on_select(multi_select_handler(state))
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Batch",
            AgentQuestion::from_spec(
                AgentQuestionSpec::new(vec![placement(), targets(), placement()])
                    .with_active_index(1),
                theme,
            )
            .into_any_element(),
        ))
}
