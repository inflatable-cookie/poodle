//! AgentQuestionRecord — the read-only record an answered question leaves.
//!
//! Contract: `docs/contracts/components/agent-question-record.md`.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::AgentQuestionRecordSpec;

use crate::presentation::rem_to_px;
use crate::primitives::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct AgentQuestionRecord {
    spec: AgentQuestionRecordSpec,
    theme: GpuiThemeProvider,
}

impl AgentQuestionRecord {
    pub fn from_spec(spec: AgentQuestionRecordSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for AgentQuestionRecord {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let surface = resolve_color(theme, spec.surface_token());
        let border = resolve_color(theme, spec.border_token());
        let prompt_color = resolve_color(theme, spec.prompt_token());
        let chosen_color = resolve_color(theme, spec.chosen_token());
        let unchosen_color = resolve_color(theme, spec.unchosen_token());
        let mark_color = resolve_color(theme, spec.mark_token());
        let radius = resolve_radius(theme, spec.radius_token());

        let font_size = px(rem_to_px(spec.font_size_rem()));
        let gap = px(rem_to_px(spec.gap_rem()));
        let inset = px(rem_to_px(spec.padding_inset_rem()));

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(gap)
            .p(inset)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .bg(surface)
            .text_size(font_size);

        if let Some(header) = &spec.question.header {
            root = root.child(
                div()
                    .text_size(px(rem_to_px(spec.font_size_rem() * 0.85)))
                    .text_color(unchosen_color)
                    .child(header.clone()),
            );
        }

        root = root.child(div().text_color(prompt_color).child(spec.question.prompt.clone()));

        if spec.shows_options() {
            // Every option survives: why the agent did something is usually
            // answered by what it did not do.
            for option in &spec.question.options {
                let chosen = spec.is_chosen(&option.value);
                root = root.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(rem_to_px(0.375)))
                        .child(if chosen {
                            Icon::new("check", theme)
                                .with_px_size(rem_to_px(spec.font_size_rem()))
                                .with_color(mark_color)
                                .into_any_element()
                        } else {
                            div()
                                .w(px(rem_to_px(spec.font_size_rem())))
                                .h(px(rem_to_px(spec.font_size_rem())))
                                .into_any_element()
                        })
                        .child(
                            div()
                                .text_color(if chosen { chosen_color } else { unchosen_color })
                                .child(option.label.clone()),
                        ),
                );
            }
        } else {
            root = root.child(div().text_color(chosen_color).child(spec.summary()));
        }

        root.into_any_element()
    }
}
