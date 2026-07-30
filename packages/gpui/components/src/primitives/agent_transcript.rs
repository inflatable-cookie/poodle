//! AgentTranscript — the output surface of an agent conversation.
//!
//! Contract: `docs/contracts/components/agent-transcript.md`.
//!
//! Grouping comes from `poodle-headless`, shared with the web target through
//! JSON vectors, so a turn summarises identically on both. Rendering is
//! unwindowed: GPUI does not measure blocks during spec resolution, and the
//! contract records that as an accepted delta.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::TranscriptBlock;
use poodle_specs::{
    AgentMessageSpec, AgentQuestionRecordSpec, AgentTranscriptSpec, ChangedFilesSpec,
    ToolCallGroupSpec,
};

use crate::presentation::rem_to_px;
use crate::primitives::agent_message::AgentMessage;
use crate::primitives::agent_question_record::AgentQuestionRecord;
use crate::primitives::changed_files::ChangedFiles;
use crate::primitives::tool_call_group::ToolCallGroup;
use crate::theme_ext::resolve_color;

pub struct AgentTranscript {
    spec: AgentTranscriptSpec,
    theme: GpuiThemeProvider,
}

impl AgentTranscript {
    pub fn from_spec(spec: AgentTranscriptSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for AgentTranscript {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let activity_color = resolve_color(theme, spec.activity_token());
        let font_size = px(rem_to_px(spec.font_size_rem()));
        let inset = px(rem_to_px(spec.padding_inset_rem()));
        let block_gap = px(rem_to_px(spec.block_gap_rem()));

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(block_gap)
            .p(inset)
            .overflow_hidden();

        if spec.is_empty() {
            return root
                .child(
                    div()
                        .text_size(font_size)
                        .text_color(activity_color)
                        .child(spec.empty_label.clone()),
                )
                .into_any_element();
        }

        for block in spec.rendered_blocks() {
            match block {
                TranscriptBlock::Message(message) => {
                    let mut message_spec = AgentMessageSpec::new(message.markdown.clone())
                        .with_streaming(message.is_streaming)
                        .with_size(spec.size)
                        .with_density(spec.density);
                    if let Some(role) = message.role {
                        message_spec = message_spec.with_role(role);
                    }
                    root = root.child(AgentMessage::from_spec(message_spec, theme));
                }
                TranscriptBlock::ToolRun(run) => {
                    let group = ToolCallGroupSpec::new(run.id.clone(), run.calls.clone())
                        .with_expanded(spec.expanded_tool_runs.contains(&run.id))
                        .with_expanded_calls(spec.expanded_tool_calls.clone())
                        .with_size(spec.size)
                        .with_density(spec.density);
                    root = root.child(ToolCallGroup::from_spec(group, theme));
                }
                TranscriptBlock::ChangedFiles(changed) => {
                    let card = ChangedFilesSpec::new(changed.id.clone(), changed.files.clone())
                        .with_expanded(spec.expanded_changed_files.contains(&changed.id))
                        .with_size(spec.size)
                        .with_density(spec.density);
                    root = root.child(ChangedFiles::from_spec(card, theme));
                }
                TranscriptBlock::AnsweredQuestion(record) => {
                    if let Some(answer) = record.answer.clone() {
                        let card = AgentQuestionRecordSpec::new(record.question.clone(), answer)
                            .with_size(spec.size)
                            .with_density(spec.density);
                        root = root.child(AgentQuestionRecord::from_spec(card, theme));
                    }
                }
                TranscriptBlock::Activity(_) => {}
            }
        }

        // The activity footer sits outside the block flow so it stays under the
        // transcript rather than scrolling as a block of its own.
        if let Some(label) = spec.activity_label() {
            root = root.child(
                div()
                    .text_size(font_size)
                    .text_color(activity_color)
                    .child(label.to_string()),
            );
        }

        root.into_any_element()
    }
}
