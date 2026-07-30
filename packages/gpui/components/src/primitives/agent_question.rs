//! AgentQuestion — the question an agent asks mid-turn.
//!
//! Contract: `docs/contracts/components/agent-question.md`.
//!
//! Renders a question and its selection state. It does not drive selection —
//! the render-only posture every native component here shares.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_question::QuestionProgressState;
use poodle_specs::AgentQuestionSpec;

use crate::presentation::rem_to_px;
use crate::primitives::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

pub struct AgentQuestion {
    spec: AgentQuestionSpec,
    theme: GpuiThemeProvider,
    on_select: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_dismiss: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl AgentQuestion {
    pub fn from_spec(spec: AgentQuestionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
            on_dismiss: None,
        }
    }

    /// An option was chosen. In single-select this also resolves the question;
    /// the host applies `toggle_question_selection` and decides, because
    /// `submits_on_select` is shared logic rather than a rendering concern.
    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(std::rc::Rc::new(handler));
        self
    }

    /// The question was declined.
    pub fn on_dismiss(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for AgentQuestion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let Some(question) = spec.active_question() else {
            return div().into_any_element();
        };

        let prompt_color = resolve_color(theme, spec.prompt_token());
        let label_color = resolve_color(theme, spec.option_label_token());
        let description_color = resolve_color(theme, spec.option_description_token());
        let option_fill = resolve_color(theme, spec.option_fill_token());
        let accent = resolve_color(theme, spec.accent_token());
        let border = resolve_color(theme, spec.border_token());
        let shortcut_color = resolve_color(theme, spec.shortcut_token());
        let progress_color = resolve_color(theme, spec.progress_token());
        let dismiss_color = resolve_color(theme, spec.dismiss_token());
        let radius = resolve_radius(theme, spec.radius_token());
        // Contract §10: a selected option carries the accent at 10% over its
        // own fill, not the border alone.
        let selected_fill = color_mix(accent, option_fill, 0.1);

        let font_size = px(rem_to_px(spec.font_size_rem()));
        let prompt_size = px(rem_to_px(spec.prompt_size_rem()));
        let gap = px(rem_to_px(spec.gap_rem()));
        let option_gap = px(rem_to_px(spec.option_gap_rem()));
        let pad_block = px(rem_to_px(spec.option_padding_block_rem()));
        let pad_inline = px(rem_to_px(spec.option_padding_inline_rem()));

        let mut root = div().flex().flex_col().w_full().gap(gap);

        if spec.shows_progress() {
            let mut dots = div().flex().items_center().gap(px(rem_to_px(0.25)));

            for state in &spec.progress().states {
                let filled = !matches!(state, QuestionProgressState::Pending);
                let current = matches!(state, QuestionProgressState::Current);
                dots = dots.child(
                    div()
                        // The live one reads as the focus of the row without a
                        // second colour.
                        .w(px(rem_to_px(if current { 0.875 } else { 0.375 })))
                        .h(px(rem_to_px(0.375)))
                        .rounded_full()
                        .bg(if filled { accent } else { border }),
                );
            }

            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(rem_to_px(0.5)))
                    .child(dots)
                    .child(
                        div()
                            .text_size(font_size)
                            .text_color(progress_color)
                            .child(spec.resolved_progress_label()),
                    ),
            );
        }

        if let Some(header) = &question.header {
            root = root.child(
                div()
                    .text_size(px(rem_to_px(spec.font_size_rem() * 0.85)))
                    .text_color(progress_color)
                    .child(header.clone()),
            );
        }

        root = root.child(
            div()
                .text_size(prompt_size)
                .text_color(prompt_color)
                .child(question.prompt.clone()),
        );

        // The question and its answers are separate units, so the step between
        // them is larger than the gap stacking progress, header and prompt.
        let mut options = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(option_gap)
            .pt(px(rem_to_px(spec.prompt_gap_rem())) - gap);

        for (index, option) in question.options.iter().enumerate() {
            let selected = spec.is_selected(&option.value);

            let mut body = div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .gap(px(rem_to_px(0.125)))
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(label_color)
                        .child(option.label.clone()),
                );

            if let Some(description) = &option.description {
                body = body.child(div().text_color(description_color).child(description.clone()));
            }

            let mut row = div()
                .id(SharedString::from(format!(
                    "poodle-agent-question-option-{}-{}",
                    question.id, option.value
                )))
                .cursor_pointer()
                .flex()
                .w_full()
                .items_start()
                .gap(px(rem_to_px(0.5)))
                .py(pad_block)
                .px(pad_inline)
                .border_1()
                .border_color(if selected { accent } else { gpui::transparent_black() })
                .rounded(radius)
                .bg(if selected { selected_fill } else { option_fill })
                .text_size(font_size);

            // Only multi-select shows a check, so the mode is visible before the
            // first click rather than inferred after it.
            if spec.is_multi_select() {
                row = row.child(
                    Icon::new(if selected { "check" } else { "square" }, theme)
                        .with_px_size(rem_to_px(spec.font_size_rem()))
                        .with_color(if selected { accent } else { description_color })
                        .into_any_element(),
                );
            }

            row = row.child(body);

            if let Some(shortcut) = spec.shortcut_for(index) {
                row = row.child(
                    div()
                        .text_size(px(rem_to_px(spec.font_size_rem() * 0.9)))
                        .text_color(shortcut_color)
                        .child(format!("{shortcut}")),
                );
            }

            if let Some(handler) = &self.on_select {
                let handler = handler.clone();
                let value = option.value.clone();
                row = row.on_click(move |_event, window, cx| handler(&value, window, cx));
            }

            options = options.child(row);
        }

        root = root.child(options);

        if spec.is_dismissible {
            let mut dismiss = div()
                .id(SharedString::from(format!("poodle-agent-question-dismiss-{}", question.id)))
                .cursor_pointer()
                .text_size(font_size)
                .text_color(dismiss_color)
                .child(spec.dismiss_label.clone());

            if let Some(handler) = &self.on_dismiss {
                let handler = handler.clone();
                let id = question.id.clone();
                dismiss = dismiss.on_click(move |_event, window, cx| handler(&id, window, cx));
            }

            root = root.child(dismiss);
        }

        root.into_any_element()
    }
}
