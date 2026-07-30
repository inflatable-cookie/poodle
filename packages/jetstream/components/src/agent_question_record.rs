//! AgentQuestionRecord — the read-only record an answered question leaves.
//!
//! Contract: `docs/contracts/components/agent-question-record.md`.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::AgentQuestionRecordSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_agent_question_record(
    spec: &AgentQuestionRecordSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let surface: Color = resolve_color(theme, spec.surface_token()).into();
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let prompt_color: Color = resolve_color(theme, spec.prompt_token()).into();
    let chosen_color: Color = resolve_color(theme, spec.chosen_token()).into();
    let unchosen_color: Color = resolve_color(theme, spec.unchosen_token()).into();
    let mark_color: Color = resolve_color(theme, spec.mark_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
    let hairline = rem_to_px(0.0625);

    let mut root = ui_element::div()
        .flex_col()
        .w_full()
        .gap(gap)
        .p(inset)
        .border(hairline)
        .border_color(border)
        .rounded(radius)
        .bg(surface);

    if let Some(header) = &spec.question.header {
        root = root.child(
            ui_element::label(header.clone())
                .text_size(font_size * 0.85)
                .text_color(unchosen_color),
        );
    }

    root = root.child(
        ui_element::label(spec.question.prompt.clone())
            .text_size(font_size)
            .text_color(prompt_color),
    );

    if spec.shows_options() {
        // Every option survives: why the agent did something is usually
        // answered by what it did not do.
        for option in &spec.question.options {
            let chosen = spec.is_chosen(&option.value);
            root = root.child(
                ui_element::div()
                    .flex_row()
                    .w_full()
                    .items_center()
                    .gap(rem_to_px(0.375))
                    // The tick alone is not the signal.
                    .aria_label(spec.option_accessible_name(&option.value, &option.label))
                    .child(if chosen {
                        ui_element::icon("check")
                            .w(font_size)
                            .h(font_size)
                            .text_color(mark_color)
                    } else {
                        ui_element::div().w(font_size).h(font_size)
                    })
                    .child(
                        ui_element::label(option.label.clone())
                            .text_size(font_size)
                            .text_color(if chosen { chosen_color } else { unchosen_color }),
                    ),
            );
        }
    } else {
        root = root.child(
            ui_element::label(spec.summary())
                .text_size(font_size)
                .text_color(chosen_color),
        );
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_question::{
        AgentQuestionAnswer, AgentQuestionItem, AgentQuestionOption, AgentQuestionOutcome,
    };

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn question() -> AgentQuestionItem {
        AgentQuestionItem {
            id: "placement".into(),
            prompt: "Where should it appear?".into(),
            options: vec![
                AgentQuestionOption { value: "inline".into(), label: "Inline".into(), description: None },
                AgentQuestionOption { value: "composer".into(), label: "Composer".into(), description: None },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn a_selected_answer_keeps_every_option_on_screen() {
        let spec = AgentQuestionRecordSpec::new(
            question(),
            AgentQuestionAnswer {
                question_id: "placement".into(),
                outcome: AgentQuestionOutcome::Selected,
                values: vec!["composer".into()],
                text: String::new(),
            },
        );
        let tree = crate::render_probe::probe(&js_agent_question_record(&spec, &theme()), 720.0, 200.0);

        assert!(tree.has_text("Composer"), "{:?}", tree.texts());
        assert!(tree.has_text("Inline"), "the unchosen option was dropped: {:?}", tree.texts());
    }

    #[test]
    fn an_override_shows_the_text_and_no_options() {
        let spec = AgentQuestionRecordSpec::new(
            question(),
            AgentQuestionAnswer {
                question_id: "placement".into(),
                outcome: AgentQuestionOutcome::Override,
                values: Vec::new(),
                text: "in the sidebar".into(),
            },
        );
        let tree = crate::render_probe::probe(&js_agent_question_record(&spec, &theme()), 720.0, 200.0);

        assert!(tree.has_text("in the sidebar"), "{:?}", tree.texts());
        assert!(!tree.has_text("Inline"), "{:?}", tree.texts());
    }
}
