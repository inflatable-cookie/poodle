use crate::app_state::AppState;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{AgentQuestion, Eyebrow};
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
            option("inline", "Inline in the transcript", Some("A block in the conversation.")),
            option("composer", "Anchored above the composer", Some("Pinned over the input.")),
            option("modal", "Modal dialog", Some("Blocks the app until answered.")),
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

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    fn group(theme: &GpuiThemeProvider, label: &str, content: AnyElement) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
            .child(content)
    }

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Single select",
            AgentQuestion::from_spec(AgentQuestionSpec::new(vec![placement()]), theme)
                .into_any_element(),
        ))
        .child(group(
            theme,
            "Selected",
            AgentQuestion::from_spec(
                AgentQuestionSpec::new(vec![placement()])
                    .with_selections(vec!["composer".to_string()]),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            theme,
            "Multi select",
            AgentQuestion::from_spec(
                AgentQuestionSpec::new(vec![targets()])
                    .with_selections(vec!["svelte".to_string(), "gpui".to_string()]),
                theme,
            )
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
