//! AgentTranscript — the output surface of an agent conversation.
//!
//! Contract: `docs/contracts/components/agent-transcript.md`.
//!
//! Grouping comes from `poodle-headless`, shared with the web target through
//! JSON vectors, so a turn summarises identically on both. Rendering is
//! unwindowed: `jetstream-ui` materializes every child of a scroll container,
//! and the contract records that as an accepted delta.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_headless::agent_transcript::TranscriptBlock;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    AgentMessageSpec, AgentQuestionRecordSpec, AgentTranscriptSpec, ChangedFilesSpec,
    ToolCallGroupSpec,
};

use crate::agent_message::js_agent_message;
use crate::agent_question_record::js_agent_question_record;
use crate::changed_files::js_changed_files;
use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;
use crate::tool_call_group::js_tool_call_group;

pub fn js_agent_transcript(spec: &AgentTranscriptSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let activity_color: jetstream_ui::Color = resolve_color(theme, spec.activity_token()).into();
    let font_size = rem_to_px(spec.font_size_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
    let block_gap = rem_to_px(spec.block_gap_rem());

    let mut root = ui_element::div()
        .flex_col()
        .w_full()
        .gap(block_gap)
        .p(inset)
        // `Log` is the role for append-only output.
        .aria_role(jetstream_ui::accesskit::Role::Log)
        .aria_label(spec.aria_label.clone());

    if spec.is_empty() {
        return root.child(
            ui_element::label(spec.empty_label.clone())
                .text_size(font_size)
                .text_color(activity_color),
        );
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
                root = root.child(js_agent_message(&message_spec, theme));
            }
            TranscriptBlock::ToolRun(run) => {
                let group = ToolCallGroupSpec::new(run.id.clone(), run.calls.clone())
                    .with_expanded(spec.expanded_tool_runs.contains(&run.id))
                    .with_expanded_calls(spec.expanded_tool_calls.clone())
                    .with_size(spec.size)
                    .with_density(spec.density);
                root = root.child(js_tool_call_group(&group, theme));
            }
            TranscriptBlock::ChangedFiles(changed) => {
                let card = ChangedFilesSpec::new(changed.id.clone(), changed.files.clone())
                    .with_expanded(spec.expanded_changed_files.contains(&changed.id))
                    .with_size(spec.size)
                    .with_density(spec.density);
                root = root.child(js_changed_files(&card, theme));
            }
            TranscriptBlock::AnsweredQuestion(record) => {
                if let Some(answer) = record.answer.clone() {
                    let card = AgentQuestionRecordSpec::new(record.question.clone(), answer)
                        .with_size(spec.size)
                        .with_density(spec.density);
                    root = root.child(js_agent_question_record(&card, theme));
                }
            }
            TranscriptBlock::Activity(_) => {}
        }
    }

    // The activity footer sits outside the block flow so it stays under the
    // transcript rather than scrolling as a block of its own.
    if let Some(label) = spec.activity_label() {
        root = root.child(
            ui_element::label(label.to_string())
                .text_size(font_size)
                .text_color(activity_color),
        );
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_transcript::{
        ToolCallStatus, TranscriptActivity, TranscriptItem, TranscriptMessage, TranscriptToolCall,
    };

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn call(id: &str, detail: &str, status: ToolCallStatus) -> TranscriptItem {
        TranscriptItem::ToolCall(TranscriptToolCall {
            id: id.to_string(),
            label: "Ran command".to_string(),
            detail: Some(detail.to_string()),
            status,
            ..Default::default()
        })
    }

    /// The defect grouping exists to prevent: a run collapsing to its newest
    /// call while a buried failure goes unmentioned.
    #[test]
    fn a_buried_failure_still_reaches_the_collapsed_run() {
        let spec = AgentTranscriptSpec::new(vec![
            call("a", "cargo check", ToolCallStatus::Success),
            call("b", "effigy check:gpui", ToolCallStatus::Error),
            call("c", "bun test", ToolCallStatus::Success),
        ]);
        let tree = crate::render_probe::probe(&js_agent_transcript(&spec, &theme()), 720.0, 200.0);

        // The visible row is the passing one...
        assert!(tree.has_text("bun test"), "{:?}", tree.texts());
        assert!(!tree.has_text("effigy check:gpui"), "{:?}", tree.texts());
        // ...so the toggle is the only thing that can carry the failure.
        assert!(tree.has_text("+2 previous tool calls"), "{:?}", tree.texts());
    }

    #[test]
    fn a_message_splits_two_runs() {
        let spec = AgentTranscriptSpec::new(vec![
            call("a", "one", ToolCallStatus::Success),
            TranscriptItem::Message(TranscriptMessage {
                id: "m".to_string(),
                markdown: "between".to_string(),
                ..Default::default()
            }),
            call("b", "two", ToolCallStatus::Success),
        ]);
        let tree = crate::render_probe::probe(&js_agent_transcript(&spec, &theme()), 720.0, 256.0);

        // Two single-call runs, so neither has a toggle and both rows show.
        assert!(tree.has_text("one"), "{:?}", tree.texts());
        assert!(tree.has_text("between"), "{:?}", tree.texts());
        assert!(tree.has_text("two"), "{:?}", tree.texts());
    }

    #[test]
    fn the_activity_footer_renders_outside_the_blocks() {
        let spec = AgentTranscriptSpec::new(vec![
            call("a", "one", ToolCallStatus::Success),
            TranscriptItem::Activity(TranscriptActivity {
                id: "act".to_string(),
                label: "Working for 1h 1m".to_string(),
            }),
        ]);
        let tree = crate::render_probe::probe(&js_agent_transcript(&spec, &theme()), 720.0, 200.0);

        assert!(tree.has_text("Working for 1h 1m"), "{:?}", tree.texts());
    }

    #[test]
    fn an_empty_transcript_shows_its_empty_label() {
        let tree = crate::render_probe::probe(
            &js_agent_transcript(&AgentTranscriptSpec::new(Vec::new()), &theme()),
            720.0,
            120.0,
        );
        assert!(tree.has_text("No messages yet"), "{:?}", tree.texts());
    }
}
