//! AgentTranscript — the output surface of an agent conversation.
//!
//! Contract: `docs/contracts/components/agent-transcript.md`
//! Ported from: `packages/jetstream/components/src/agent_transcript.rs`.
//!
//! Grouping comes from `poodle-headless`, shared with the web target through
//! JSON vectors, so a turn summarises identically on both. Rendering is
//! unwindowed: backends materialize every child of a scroll container, and the
//! contract records that as an accepted delta.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::agent_transcript::TranscriptBlock;
use poodle_node::{LayoutDirection, Node, NodeRole};
use poodle_specs::{
    AgentMessageSpec, AgentQuestionRecordSpec, AgentTranscriptSpec, ChangedFilesSpec,
    ToolCallGroupSpec,
};

use crate::agent_message::agent_message;
use crate::agent_question_record::agent_question_record;
use crate::changed_files::{changed_files, ChangedFilesHandlers};
use crate::presentation::rem_to_px;
use crate::tool_call_group::{tool_call_group, ToolCallGroupHandlers};

/// The transcript owns the events its blocks raise, because the host holds the
/// expansion state and the transcript is the only place that knows which block
/// a click came from. Handlers are forwarded down to the block that raises
/// them, not re-implemented here.
#[derive(Default)]
pub struct AgentTranscriptHandlers {
    /// Fires with the run id when a run is expanded or collapsed.
    pub on_tool_run_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the call id when one call's output is opened or closed.
    pub on_tool_call_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the changed-files id when that card is opened or closed.
    pub on_changed_files_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with a file's path when one is chosen in a changed-files card.
    pub on_file_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the group id when a subagent group is expanded or collapsed.
    pub on_subagent_toggle: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the group id when a subagent group's click-through is used.
    pub on_subagent_open: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn agent_transcript(
    spec: &AgentTranscriptSpec,
    theme: &dyn ThemeProvider,
    handlers: AgentTranscriptHandlers,
) -> Node {
    let activity_color = theme.resolve_color(spec.activity_token());
    let font_size = rem_to_px(spec.font_size_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
    let block_gap = rem_to_px(spec.block_gap_rem());

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.fill_width = true;
        s.descriptor.layout.spacing.gap = block_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = inset;
        pad.right = inset;
        pad.top = inset;
        pad.bottom = inset;
    }
    // `Log` is the role for append-only output.
    root.a11y.role = Some(NodeRole::Log);
    root.a11y.label = Some(spec.aria_label.clone());

    let activity_text = |content: String| -> Node {
        let mut t = Node::text(content);
        t.style.text_size = Some(font_size);
        t.style.descriptor.text_color = Some(activity_color);
        t
    };

    if spec.is_empty() {
        return root.child(activity_text(spec.empty_label.clone()));
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
                root = root.child(agent_message(&message_spec, theme));
            }
            TranscriptBlock::ToolRun(run) => {
                let group = ToolCallGroupSpec::new(run.id.clone(), run.calls.clone())
                    .with_expanded(spec.expanded_tool_runs.contains(&run.id))
                    .with_expanded_calls(spec.expanded_tool_calls.clone())
                    .with_size(spec.size)
                    .with_density(spec.density);

                let group_handlers = ToolCallGroupHandlers {
                    on_toggle: handlers.on_tool_run_toggle.as_ref().map(Arc::clone),
                    on_call_toggle: handlers.on_tool_call_toggle.as_ref().map(Arc::clone),
                };
                root = root.child(tool_call_group(&group, theme, group_handlers));
            }
            TranscriptBlock::ChangedFiles(changed) => {
                let card = ChangedFilesSpec::new(changed.id.clone(), changed.files.clone())
                    .with_expanded(spec.expanded_changed_files.contains(&changed.id))
                    .with_size(spec.size)
                    .with_density(spec.density);

                let card_handlers = ChangedFilesHandlers {
                    on_toggle: handlers.on_changed_files_toggle.as_ref().map(Arc::clone),
                    on_file_select: handlers.on_file_select.as_ref().map(Arc::clone),
                };
                root = root.child(changed_files(&card, theme, card_handlers));
            }
            TranscriptBlock::AnsweredQuestion(record) => {
                if let Some(answer) = record.answer.clone() {
                    let card = AgentQuestionRecordSpec::new(record.question.clone(), answer)
                        .with_size(spec.size)
                        .with_density(spec.density);
                    root = root.child(agent_question_record(&card, theme));
                }
            }
            // A provider-owned child's work renders live in the transcript —
            // observation-only, with the disclosure and click-through handled
            // by the group itself.
            TranscriptBlock::SubagentGroup(group) => {
                let card = poodle_specs::AgentSubagentSpec::new(group.subagent.clone())
                    .with_expanded(spec.expanded_subagent_groups.contains(&group.id))
                    .with_detail_lines(group.detail_lines.clone().unwrap_or_default())
                    .with_size(spec.size)
                    .with_density(spec.density);

                let group_id = group.id.clone();
                let group_handlers = crate::agent_subagent::AgentSubagentHandlers {
                    on_toggle: handlers.on_subagent_toggle.as_ref().map(|handler| {
                        let handler = Arc::clone(handler);
                        let id = group_id.clone();
                        Arc::new(move |_expanded| handler(&id)) as Arc<dyn Fn(bool) + Send + Sync>
                    }),
                    on_open_child: handlers.on_subagent_open.as_ref().map(|handler| {
                        let handler = Arc::clone(handler);
                        let id = group_id.clone();
                        Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>
                    }),
                };
                root = root.child(crate::agent_subagent::agent_subagent(
                    &card,
                    theme,
                    group_handlers,
                ));
            }
            // Decided plans are retained in the shared transcript contract;
            // this renderer has no plan-card primitive yet, so keep the
            // record in the grouping stream without adding a visual block.
            TranscriptBlock::DecidedPlan(_) => {}
            TranscriptBlock::Activity(_) => {}
        }
    }

    // The activity footer sits outside the block flow so it stays under the
    // transcript rather than scrolling as a block of its own.
    if let Some(label) = spec.activity_label() {
        root = root.child(activity_text(label.to_string()));
    }

    root
}
