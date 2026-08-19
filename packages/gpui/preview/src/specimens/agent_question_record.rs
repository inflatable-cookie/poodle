use crate::app_state::AppState;
use crate::node_compat::{AgentQuestionRecord, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_question::{
    AgentQuestionAnswer, AgentQuestionItem, AgentQuestionOption, AgentQuestionOutcome,
};
use poodle_specs::{AgentQuestionRecordSpec, EyebrowSpec};

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

fn stack(children: impl IntoIterator<Item = AnyElement>) -> Div {
    let mut col = div().flex().flex_col().gap(px(12.0));
    for child in children {
        col = col.child(child);
    }
    col
}

fn option(value: &str, label: &str) -> AgentQuestionOption {
    AgentQuestionOption {
        value: value.to_string(),
        label: label.to_string(),
        description: None,
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
            option("inline", "Inline in the transcript"),
            option("composer", "Anchored above the composer"),
            option("modal", "Modal dialog"),
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
            option("svelte", "Svelte"),
            option("react", "React"),
            option("gpui", "GPUI"),
        ],
        allow_multiple: true,
    }
}

fn answer(
    question_id: &str,
    outcome: AgentQuestionOutcome,
    values: &[&str],
    text: &str,
) -> AgentQuestionAnswer {
    AgentQuestionAnswer {
        question_id: question_id.to_string(),
        outcome,
        values: values.iter().map(|v| (*v).to_string()).collect(),
        text: text.to_string(),
    }
}

fn selected() -> AgentQuestionAnswer {
    answer(
        "placement",
        AgentQuestionOutcome::Selected,
        &["composer"],
        "",
    )
}

fn several() -> AgentQuestionAnswer {
    answer(
        "targets",
        AgentQuestionOutcome::Selected,
        &["svelte", "gpui"],
        "",
    )
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Selected answers",
            theme,
            stack([
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(placement(), selected()),
                    theme,
                )
                .into_any_element(),
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(targets(), several()),
                    theme,
                )
                .into_any_element(),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Free-text override",
            theme,
            AgentQuestionRecord::from_spec(
                AgentQuestionRecordSpec::new(
                    placement(),
                    answer(
                        "placement",
                        AgentQuestionOutcome::Override,
                        &[],
                        "Neither — put it in the sidebar.",
                    ),
                ),
                theme,
            ),
        ))
        .child(group(
            "Declined",
            theme,
            AgentQuestionRecord::from_spec(
                AgentQuestionRecordSpec::new(
                    placement(),
                    answer("placement", AgentQuestionOutcome::Declined, &[], ""),
                ),
                theme,
            ),
        ))
        .child(group(
            "Presentation options",
            theme,
            stack([
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(placement(), selected()).with_show_options(false),
                    theme,
                )
                .into_any_element(),
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(placement(), selected()),
                    theme,
                )
                .into_any_element(),
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(targets(), several()),
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
        "agent-question-record",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(placement(), selected()).with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                AgentQuestionRecord::from_spec(
                    AgentQuestionRecordSpec::new(placement(), selected()).with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
