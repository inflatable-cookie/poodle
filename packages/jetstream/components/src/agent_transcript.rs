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

use std::sync::Arc;

use crate::agent_message::AgentMessage;
use crate::agent_question_record::AgentQuestionRecord;
use crate::changed_files::ChangedFiles;
use crate::element::{Handler, IntoJsEl};
use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;
use crate::tool_call_group::ToolCallGroup;

/// AgentTranscript — the output surface of an agent conversation.
///
/// The transcript owns the events its blocks raise, because the host holds the
/// expansion state and the transcript is the only place that knows which block
/// a click came from. Handlers are forwarded down to the block that raises
/// them, not re-implemented here.
pub struct AgentTranscript {
    spec: AgentTranscriptSpec,
    theme: JetstreamThemeProvider,
    on_tool_run_toggle: Option<Handler>,
    on_tool_call_toggle: Option<Handler>,
    on_changed_files_toggle: Option<Handler>,
    on_file_select: Option<Handler>,
}

impl AgentTranscript {
    pub fn from_spec(spec: AgentTranscriptSpec, theme: &JetstreamThemeProvider) -> Self {
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
    pub fn on_tool_run_toggle(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_tool_run_toggle = Some(Arc::new(handler));
        self
    }

    /// Fires with the call id when one call's output is opened or closed.
    pub fn on_tool_call_toggle(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_tool_call_toggle = Some(Arc::new(handler));
        self
    }

    /// Fires with the changed-files id when that card is opened or closed.
    pub fn on_changed_files_toggle(
        mut self,
        handler: impl Fn(&str) + Send + Sync + 'static,
    ) -> Self {
        self.on_changed_files_toggle = Some(Arc::new(handler));
        self
    }

    /// Fires with a file's path when one is chosen in a changed-files card.
    pub fn on_file_select(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_file_select = Some(Arc::new(handler));
        self
    }
}

impl IntoJsEl for AgentTranscript {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, Handlers {
            tool_run_toggle: self.on_tool_run_toggle,
            tool_call_toggle: self.on_tool_call_toggle,
            changed_files_toggle: self.on_changed_files_toggle,
            file_select: self.on_file_select,
        })
    }
}

#[derive(Default)]
struct Handlers {
    tool_run_toggle: Option<Handler>,
    tool_call_toggle: Option<Handler>,
    changed_files_toggle: Option<Handler>,
    file_select: Option<Handler>,
}

pub fn js_agent_transcript(spec: &AgentTranscriptSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, Handlers::default())
}

fn build(
    spec: &AgentTranscriptSpec,
    theme: &JetstreamThemeProvider,
    handlers: Handlers,
) -> JsEl {
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
                root = root.child(AgentMessage::from_spec(message_spec, theme).into_js_el());
            }
            TranscriptBlock::ToolRun(run) => {
                let group = ToolCallGroupSpec::new(run.id.clone(), run.calls.clone())
                    .with_expanded(spec.expanded_tool_runs.contains(&run.id))
                    .with_expanded_calls(spec.expanded_tool_calls.clone())
                    .with_size(spec.size)
                    .with_density(spec.density);

                let mut element = ToolCallGroup::from_spec(group, theme);
                if let Some(handler) = &handlers.tool_run_toggle {
                    let handler = Arc::clone(handler);
                    element = element.on_toggle(move |id| handler(id));
                }
                if let Some(handler) = &handlers.tool_call_toggle {
                    let handler = Arc::clone(handler);
                    element = element.on_call_toggle(move |id| handler(id));
                }
                root = root.child(element.into_js_el());
            }
            TranscriptBlock::ChangedFiles(changed) => {
                let card = ChangedFilesSpec::new(changed.id.clone(), changed.files.clone())
                    .with_expanded(spec.expanded_changed_files.contains(&changed.id))
                    .with_size(spec.size)
                    .with_density(spec.density);

                let mut element = ChangedFiles::from_spec(card, theme);
                if let Some(handler) = &handlers.changed_files_toggle {
                    let handler = Arc::clone(handler);
                    element = element.on_toggle(move |id| handler(id));
                }
                if let Some(handler) = &handlers.file_select {
                    let handler = Arc::clone(handler);
                    element = element.on_file_select(move |path| handler(path));
                }
                root = root.child(element.into_js_el());
            }
            TranscriptBlock::AnsweredQuestion(record) => {
                if let Some(answer) = record.answer.clone() {
                    let card = AgentQuestionRecordSpec::new(record.question.clone(), answer)
                        .with_size(spec.size)
                        .with_density(spec.density);
                    root = root.child(AgentQuestionRecord::from_spec(card, theme).into_js_el());
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

    /// A click three levels down — transcript, run, row — reaches the host with
    /// the right payload, and the run's own event does not fire alongside it.
    ///
    /// The transcript is where the host actually attaches handlers, so
    /// forwarding is the thing that has to be true; the leaf components being
    /// individually clickable is not enough on its own.
    #[test]
    fn a_click_on_a_nested_row_reaches_the_transcript_handler() {
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let calls = Arc::clone(&seen);
        let runs = Arc::clone(&seen);

        let mut lead = TranscriptToolCall {
            id: "b".to_string(),
            label: "Ran command".to_string(),
            detail: Some("bun test".to_string()),
            status: ToolCallStatus::Success,
            ..Default::default()
        };
        lead.output = Some("272 pass".to_string());

        let spec = AgentTranscriptSpec::new(vec![
            call("a", "cargo check", ToolCallStatus::Success),
            TranscriptItem::ToolCall(lead),
        ]);

        let el = AgentTranscript::from_spec(spec, &theme())
            .on_tool_call_toggle(move |id| calls.lock().unwrap().push(format!("call:{id}")))
            .on_tool_run_toggle(move |id| runs.lock().unwrap().push(format!("run:{id}")))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 256.0, "bun test");

        assert_eq!(seen.lock().unwrap().as_slice(), ["call:b"]);
    }

    fn one_changed_file() -> AgentTranscriptSpec {
        use poodle_headless::agent_transcript::{ChangedFile, TranscriptChangedFiles};

        AgentTranscriptSpec::new(vec![TranscriptItem::ChangedFiles(TranscriptChangedFiles {
            id: "changed".to_string(),
            files: vec![ChangedFile {
                path: "pkg/src/main.rs".to_string(),
                additions: 3,
                deletions: 1,
                status: None,
            }],
        })])
    }

    /// Changed-files events forward too, and a file reports its path rather
    /// than the leaf the chip displays.
    #[test]
    fn a_file_chip_reaches_the_transcript_handler() {
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let paths = Arc::clone(&seen);

        let el = AgentTranscript::from_spec(one_changed_file(), &theme())
            .on_file_select(move |path| paths.lock().unwrap().push(path.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 256.0, "main.rs");

        assert_eq!(seen.lock().unwrap().as_slice(), ["pkg/src/main.rs"]);
    }

    /// The card's own id, not a file path: expanding the card and choosing a
    /// file inside it are separate events, and a host holds separate state for
    /// each.
    #[test]
    fn the_changed_files_header_reaches_the_transcript_handler() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = AgentTranscript::from_spec(one_changed_file(), &theme())
            .on_changed_files_toggle(move |id| {
                assert_eq!(id, "changed");
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 720.0, 256.0, "1 changed files");

        assert_eq!(hits.load(Ordering::SeqCst), 1, "on_changed_files_toggle fired exactly once");
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
