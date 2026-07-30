//! AgentQuestion — the question an agent asks mid-turn.
//!
//! Contract: `docs/contracts/components/agent-question.md`.
//!
//! Renders a question and its selection state. It does not drive selection —
//! the render-only posture every native component here shares — so a question
//! shown in the preview cannot be answered there. See the contract's §14.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::AgentQuestionSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

pub fn js_agent_question(spec: &AgentQuestionSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let Some(question) = spec.active_question() else {
        return ui_element::div();
    };

    let prompt_color: Color = resolve_color(theme, spec.prompt_token()).into();
    let label_color: Color = resolve_color(theme, spec.option_label_token()).into();
    let description_color: Color = resolve_color(theme, spec.option_description_token()).into();
    let option_fill_raw = resolve_color(theme, spec.option_fill_token());
    let accent_raw = resolve_color(theme, spec.accent_token());
    let option_fill: Color = option_fill_raw.into();
    let accent: Color = accent_raw.into();
    // Contract §10: a selected option carries the accent at 10% over its own
    // fill, not the border alone.
    let selected_fill: Color = color_mix(accent_raw, option_fill_raw, 0.1).into();
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let shortcut_color: Color = resolve_color(theme, spec.shortcut_token()).into();
    let progress_color: Color = resolve_color(theme, spec.progress_token()).into();
    let dismiss_color: Color = resolve_color(theme, spec.dismiss_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let prompt_size = rem_to_px(spec.prompt_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let option_gap = rem_to_px(spec.option_gap_rem());
    let pad_block = rem_to_px(spec.option_padding_block_rem());
    let pad_inline = rem_to_px(spec.option_padding_inline_rem());
    let hairline = rem_to_px(0.0625);

    let mut root = ui_element::div().flex_col().w_full().gap(gap);

    if spec.shows_progress() {
        let progress = spec.progress();
        let mut dots = ui_element::div().flex_row().items_center().gap(rem_to_px(0.25));

        for state in &progress.states {
            let answered = !matches!(
                state,
                poodle_headless::agent_question::QuestionProgressState::Pending
            );
            let current = matches!(
                state,
                poodle_headless::agent_question::QuestionProgressState::Current
            );
            dots = dots.child(
                ui_element::div()
                    // The live one reads as the focus of the row without a
                    // second colour.
                    .w(if current { rem_to_px(0.875) } else { rem_to_px(0.375) })
                    .h(rem_to_px(0.375))
                    .rounded(999.0)
                    .bg(if answered { accent } else { border }),
            );
        }

        root = root.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.5))
                // The dots are a picture of the label; the label carries the
                // fact for anyone who cannot see them.
                .child(dots)
                .child(
                    ui_element::label(spec.resolved_progress_label())
                        .text_size(font_size)
                        .text_color(progress_color),
                ),
        );
    }

    if let Some(header) = &question.header {
        root = root.child(
            ui_element::label(header.clone())
                .text_size(font_size * 0.85)
                .text_color(progress_color),
        );
    }

    root = root.child(
        ui_element::label(question.prompt.clone())
            .text_size(prompt_size)
            .text_color(prompt_color),
    );

    // The question and its answers are separate units, so the step between them
    // is larger than the gap stacking progress, header and prompt.
    let mut options = ui_element::div()
        .flex_col()
        .w_full()
        .gap(option_gap)
        .pt(rem_to_px(spec.prompt_gap_rem()) - gap)
        .aria_role(if spec.is_multi_select() {
            jetstream_ui::accesskit::Role::Group
        } else {
            jetstream_ui::accesskit::Role::RadioGroup
        })
        .aria_label(question.prompt.clone());

    for (index, option) in question.options.iter().enumerate() {
        let selected = spec.is_selected(&option.value);

        let mut body = ui_element::div()
            .flex_col()
            .grow()
            .min_w_0()
            .gap(rem_to_px(0.125))
            .child(
                ui_element::label(option.label.clone())
                    .text_size(font_size)
                    .text_weight(600)
                    .text_color(label_color),
            );

        if let Some(description) = &option.description {
            body = body.child(
                ui_element::label(description.clone())
                    .text_size(font_size)
                    .text_color(description_color),
            );
        }

        let mut row = ui_element::button("")
            .aria_label(option.label.clone())
            .aria_role(if spec.is_multi_select() {
                jetstream_ui::accesskit::Role::CheckBox
            } else {
                jetstream_ui::accesskit::Role::RadioButton
            })
            .flex_row()
            .w_full()
            .items_start()
            .gap(rem_to_px(0.5))
            .pt(pad_block)
            .pb(pad_block)
            .pl(pad_inline)
            .pr(pad_inline)
            .border(hairline)
            .border_color(if selected { accent } else { Color::TRANSPARENT })
            .rounded(radius)
            .bg(if selected { selected_fill } else { option_fill })
            .focusable();

        // Only multi-select shows a check, so the mode is visible before the
        // first click rather than inferred after it.
        if spec.is_multi_select() {
            row = row.child(
                ui_element::icon(if selected { "check" } else { "square" })
                    .w(font_size)
                    .h(font_size)
                    .text_color(if selected { accent } else { description_color }),
            );
        }

        row = row.child(body);

        if let Some(shortcut) = spec.shortcut_for(index) {
            row = row.child(
                ui_element::label(format!("{shortcut}"))
                    .text_size(font_size * 0.9)
                    .text_color(shortcut_color),
            );
        }

        options = options.child(row);
    }

    root = root.child(options);

    if spec.is_dismissible {
        root = root.child(
            ui_element::button("")
                .aria_label(spec.dismiss_label.clone())
                .aria_role(jetstream_ui::accesskit::Role::Button)
                .bg(Color::TRANSPARENT)
                .focusable()
                .child(
                    ui_element::label(spec.dismiss_label.clone())
                        .text_size(font_size)
                        .text_color(dismiss_color),
                ),
        );
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn question(allow_multiple: bool) -> AgentQuestionItem {
        AgentQuestionItem {
            id: "placement".into(),
            header: Some("Placement".into()),
            prompt: "Where should it appear?".into(),
            options: vec![
                AgentQuestionOption { value: "inline".into(), label: "Inline".into(), description: Some("In the transcript".into()) },
                AgentQuestionOption { value: "composer".into(), label: "Composer".into(), description: None },
            ],
            allow_multiple,
        }
    }

    #[test]
    fn the_prompt_and_every_option_reach_the_render() {
        let spec = AgentQuestionSpec::new(vec![question(false)]);
        let tree = crate::render_probe::probe(&js_agent_question(&spec, &theme()), 720.0, 320.0);

        assert!(tree.has_text("Where should it appear?"), "{:?}", tree.texts());
        assert!(tree.has_text("Inline"), "{:?}", tree.texts());
        assert!(tree.has_text("Composer"), "{:?}", tree.texts());
        assert!(tree.has_text("In the transcript"), "{:?}", tree.texts());
    }

    #[test]
    fn a_lone_question_renders_no_progress_label() {
        let spec = AgentQuestionSpec::new(vec![question(false)]);
        let tree = crate::render_probe::probe(&js_agent_question(&spec, &theme()), 720.0, 320.0);
        assert!(!tree.has_text("1 of 1"), "{:?}", tree.texts());
    }

    #[test]
    fn a_batch_reports_its_position() {
        let spec = AgentQuestionSpec::new(vec![question(false), question(true), question(false)])
            .with_active_index(1);
        let tree = crate::render_probe::probe(&js_agent_question(&spec, &theme()), 720.0, 320.0);
        assert!(tree.has_text("2 of 3"), "{:?}", tree.texts());
    }

    #[test]
    fn shortcuts_stop_after_nine() {
        let mut many = question(false);
        many.options = (0..11)
            .map(|i| AgentQuestionOption {
                value: format!("v{i}"),
                label: format!("Option {i}"),
                description: None,
            })
            .collect();

        let spec = AgentQuestionSpec::new(vec![many]);
        let tree = crate::render_probe::probe(&js_agent_question(&spec, &theme()), 720.0, 900.0);

        assert!(tree.has_text("9"), "{:?}", tree.texts());
        assert!(!tree.has_text("10"), "a tenth shortcut was rendered: {:?}", tree.texts());
    }
}
