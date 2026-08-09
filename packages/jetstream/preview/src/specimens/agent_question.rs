//! AgentQuestion specimen — contract §16.
//!
//! Rendering only: selection state is shown, never driven. Answering a question
//! is host-event-loop work, as with every native control here.

use crate::compat::js_agent_question;
use crate::nel::*;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::AgentQuestionSpec;

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

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Single select",
            secondary,
            js_agent_question(&AgentQuestionSpec::new(vec![placement()]), theme),
        ))
        .child(group(
            "Selected",
            secondary,
            js_agent_question(
                &AgentQuestionSpec::new(vec![placement()])
                    .with_selections(vec!["composer".to_string()]),
                theme,
            ),
        ))
        .child(group(
            "Multi select",
            secondary,
            js_agent_question(
                &AgentQuestionSpec::new(vec![targets()])
                    .with_selections(vec!["svelte".to_string(), "gpui".to_string()]),
                theme,
            ),
        ))
        .child(group(
            "Batch",
            secondary,
            js_agent_question(
                &AgentQuestionSpec::new(vec![placement(), targets(), placement()])
                    .with_active_index(1),
                theme,
            ),
        ))
        .child(group(
            "Dismissible",
            secondary,
            js_agent_question(
                &AgentQuestionSpec::new(vec![placement()]).with_dismissible(true),
                theme,
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
