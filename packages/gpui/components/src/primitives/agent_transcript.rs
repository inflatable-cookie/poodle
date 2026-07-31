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

type BlockHandler = std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>;

pub struct AgentTranscript {
    spec: AgentTranscriptSpec,
    theme: GpuiThemeProvider,
    on_tool_run_toggle: Option<BlockHandler>,
    on_tool_call_toggle: Option<BlockHandler>,
    on_changed_files_toggle: Option<BlockHandler>,
    on_file_select: Option<BlockHandler>,
}

impl AgentTranscript {
    pub fn from_spec(spec: AgentTranscriptSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_tool_run_toggle: None,
            on_tool_call_toggle: None,
            on_changed_files_toggle: None,
            on_file_select: None,
        }
    }

    /// Fires with the run id when a run is expanded or collapsed.
    ///
    /// The transcript forwards into whichever block raises the event, matching
    /// the Jetstream target: it is the only level that sees every block, and
    /// the host holds all the expansion state, so this is where a host
    /// attaches rather than at each block.
    pub fn on_tool_run_toggle(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tool_run_toggle = Some(std::rc::Rc::new(handler));
        self
    }

    /// Fires with the call id when one call's output is opened or closed.
    pub fn on_tool_call_toggle(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tool_call_toggle = Some(std::rc::Rc::new(handler));
        self
    }

    /// Fires with the changed-files id when that card is opened or closed.
    pub fn on_changed_files_toggle(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_changed_files_toggle = Some(std::rc::Rc::new(handler));
        self
    }

    /// Fires with a file's path when one is chosen in a changed-files card.
    pub fn on_file_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_file_select = Some(std::rc::Rc::new(handler));
        self
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
                    let mut element = ToolCallGroup::from_spec(group, theme);
                    if let Some(handler) = &self.on_tool_run_toggle {
                        let handler = handler.clone();
                        element = element.on_toggle(move |id, window, cx| handler(id, window, cx));
                    }
                    if let Some(handler) = &self.on_tool_call_toggle {
                        let handler = handler.clone();
                        element =
                            element.on_call_toggle(move |id, window, cx| handler(id, window, cx));
                    }
                    root = root.child(element);
                }
                TranscriptBlock::ChangedFiles(changed) => {
                    let card = ChangedFilesSpec::new(changed.id.clone(), changed.files.clone())
                        .with_expanded(spec.expanded_changed_files.contains(&changed.id))
                        .with_size(spec.size)
                        .with_density(spec.density);
                    let mut element = ChangedFiles::from_spec(card, theme);
                    if let Some(handler) = &self.on_changed_files_toggle {
                        let handler = handler.clone();
                        element = element.on_toggle(move |id, window, cx| handler(id, window, cx));
                    }
                    if let Some(handler) = &self.on_file_select {
                        let handler = handler.clone();
                        element =
                            element.on_file_select(move |path, window, cx| handler(path, window, cx));
                    }
                    root = root.child(element);
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
