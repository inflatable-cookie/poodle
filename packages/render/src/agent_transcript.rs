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

use poodle_headless::agent_transcript::TranscriptBlock;
use poodle_node::{LayoutDirection, LayoutSizing, Node, NodeRole};
use poodle_specs::{
    AgentMessageSpec, AgentPlanRecordSpec, AgentQuestionRecordSpec, AgentTranscriptSpec, ButtonSpec,
    ChangedFilesSpec, TextSpec, TextTone, ToolCallGroupSpec,
};

use crate::agent_message::agent_message;
use crate::agent_question_record::agent_question_record;
use crate::changed_files::{changed_files, ChangedFilesHandlers};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::tool_call_group::{tool_call_group, ToolCallGroupHandlers};

/// Build the renderer-owned jump-to-latest control.
///
/// Runtime adapters decide when to mount it and what "latest" means. The
/// component recipe stays shared so native runtimes do not invent host chrome.
pub fn agent_transcript_jump(
    spec: &AgentTranscriptSpec,
    ctx: &RenderContext<'_>,
    on_activate: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let button_spec = ButtonSpec::new()
        .with_label(spec.jump_label.clone())
        .with_leading_icon("arrow-down")
        .with_size(base_size)
        .with_density(density);
    let mut jump = crate::button::button(&button_spec, ctx, on_activate);
    let fill = ctx.theme().resolve_color(spec.jump_fill_token());
    let border = ctx.theme().resolve_color(spec.jump_border_token());
    let text = ctx.theme().resolve_color(spec.jump_text_token());
    let radius = ctx.theme().resolve_radius(spec.jump_radius_token());
    {
        let style = &mut jump.style;
        style.descriptor.layout.height = LayoutSizing::Fit;
        style.min_width = None;
        style.descriptor.layout.spacing.padding.top = rem_to_px(0.3125);
        style.descriptor.layout.spacing.padding.bottom = rem_to_px(0.3125);
        style.descriptor.layout.spacing.padding.left = rem_to_px(0.75);
        style.descriptor.layout.spacing.padding.right = rem_to_px(0.75);
        style.descriptor.layout.spacing.gap = rem_to_px(0.375);
        style.descriptor.background = Some(fill);
        style.descriptor.border.width = 1.0;
        style.descriptor.border.color = border;
        style.descriptor.text_color = Some(text);
        style.text_size = Some(rem_to_px(spec.font_size_rem(base_size)));
        style.descriptor.corner_radii.top_left = radius;
        style.descriptor.corner_radii.top_right = radius;
        style.descriptor.corner_radii.bottom_right = radius;
        style.descriptor.corner_radii.bottom_left = radius;
        style.hover = None;
        style.active = None;
    }
    for child in &mut jump.children {
        child.style.descriptor.text_color = Some(text);
    }
    jump
}

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
    /// Stable native instance scope for the transcript, its blocks, and every
    /// nested interactive record.
    pub instance_id: Option<String>,
}

pub fn agent_transcript_root_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("agent-transcript:{scope}"),
        None => "agent-transcript".to_owned(),
    }
}

pub fn agent_transcript_block_id(
    instance_id: Option<&str>,
    kind: &str,
    item_id: &str,
) -> String {
    format!(
        "{}:block:{kind}:{item_id}",
        agent_transcript_root_id(instance_id)
    )
}

fn posture(spec: &AgentTranscriptSpec) -> &'static str {
    use poodle_headless::agent_transcript::{ToolCallStatus, TranscriptItem};

    if spec.items.is_empty() {
        "empty"
    } else if spec.items.iter().any(|item| {
        matches!(
            item,
            TranscriptItem::ToolCall(call) if call.status == ToolCallStatus::Error
        )
    }) {
        "error"
    } else if spec.items.iter().any(|item| match item {
        TranscriptItem::Activity(activity) => activity.spinning.unwrap_or(true),
        TranscriptItem::Message(message) => message.is_streaming,
        TranscriptItem::ToolCall(call) => call.status == ToolCallStatus::Running,
        _ => false,
    }) {
        "loading"
    } else {
        "content"
    }
}

pub fn agent_transcript(
    spec: &AgentTranscriptSpec,
    ctx: &RenderContext<'_>,
    handlers: AgentTranscriptHandlers,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let activity_color = ctx.theme().resolve_color(spec.activity_token());
    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let inset = rem_to_px(spec.padding_inset_rem(density));
    let block_gap = rem_to_px(spec.block_gap_rem(density));

    let mut root = Node::container();
    root.runtime_id = Some(agent_transcript_root_id(handlers.instance_id.as_deref()));
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
    root.roles.insert("posture".to_owned(), posture(spec).to_owned());
    root.roles
        .insert("empty".to_owned(), spec.is_empty().to_string());
    root.roles.insert(
        "virtualized".to_owned(),
        spec.is_virtualized.to_string(),
    );
    root.roles.insert(
        "size".to_owned(),
        format!("{base_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );

    let activity_text = |content: String| -> Node {
        let mut t = crate::text::text(&TextSpec::new(content).with_tone(TextTone::Secondary), ctx);
        t.style.text_size = Some(font_size);
        t.style.descriptor.text_color = Some(activity_color);
        t
    };

    if spec.is_empty() {
        return root.child(activity_text(spec.empty_label.clone()));
    }

    for block in spec.rendered_blocks() {
        let block_kind = block.kind();
        let block_id = block.id().to_owned();
        match block {
            TranscriptBlock::Message(message) => {
                let mut message_spec = AgentMessageSpec::new(message.markdown.clone())
                    .with_streaming(message.is_streaming)
                    .with_size(base_size)
                    .with_density(density);
                if let Some(role) = message.role {
                    message_spec = message_spec.with_role(role);
                }
                let role = message_spec.role.as_str().to_owned();
                let mut child = agent_message(&message_spec, ctx);
                child.runtime_id = Some(agent_transcript_block_id(
                    handlers.instance_id.as_deref(),
                    block_kind,
                    &block_id,
                ));
                child.roles.insert("kind".to_owned(), block_kind.to_owned());
                child.roles.insert("role".to_owned(), role);
                child.roles.insert(
                    "status".to_owned(),
                    if message.is_streaming { "streaming" } else { "complete" }.to_owned(),
                );
                root = root.child(child);
            }
            TranscriptBlock::ToolRun(run) => {
                let group = ToolCallGroupSpec::new(run.id.clone(), run.calls.clone())
                    .with_expanded(spec.expanded_tool_runs.contains(&run.id))
                    .with_expanded_calls(spec.expanded_tool_calls.clone())
                    .with_size(base_size)
                    .with_density(density);

                let group_handlers = ToolCallGroupHandlers {
                    on_toggle: handlers.on_tool_run_toggle.as_ref().map(Arc::clone),
                    on_call_toggle: handlers.on_tool_call_toggle.as_ref().map(Arc::clone),
                    instance_id: handlers.instance_id.as_deref().map(|scope| {
                        format!("{scope}:transcript-run:{}", run.id)
                    }),
                };
                let status = group.status().as_str().to_owned();
                let mut child = tool_call_group(&group, ctx, group_handlers);
                child.runtime_id = Some(agent_transcript_block_id(
                    handlers.instance_id.as_deref(),
                    block_kind,
                    &block_id,
                ));
                child.roles.insert("kind".to_owned(), block_kind.to_owned());
                child.roles.insert("status".to_owned(), status);
                root = root.child(child);
            }
            TranscriptBlock::ChangedFiles(changed) => {
                let card = ChangedFilesSpec::new(changed.id.clone(), changed.files.clone())
                    .with_expanded(spec.expanded_changed_files.contains(&changed.id))
                    .with_size(base_size)
                    .with_density(density);

                let card_handlers = ChangedFilesHandlers {
                    on_toggle: handlers.on_changed_files_toggle.as_ref().map(Arc::clone),
                    on_file_select: handlers.on_file_select.as_ref().map(Arc::clone),
                    instance_id: handlers.instance_id.as_deref().map(|scope| {
                        format!("{scope}:transcript-files:{}", changed.id)
                    }),
                };
                let mut child = changed_files(&card, ctx, card_handlers);
                child.runtime_id = Some(agent_transcript_block_id(
                    handlers.instance_id.as_deref(),
                    block_kind,
                    &block_id,
                ));
                child.roles.insert("kind".to_owned(), block_kind.to_owned());
                root = root.child(child);
            }
            TranscriptBlock::AnsweredQuestion(record) => {
                if let Some(answer) = record.answer.clone() {
                    let outcome = answer.outcome.as_str().to_owned();
                    let card = AgentQuestionRecordSpec::new(record.question.clone(), answer)
                        .with_size(base_size)
                        .with_density(density);
                    let mut child = agent_question_record(&card, ctx);
                    child.runtime_id = Some(agent_transcript_block_id(
                        handlers.instance_id.as_deref(),
                        block_kind,
                        &block_id,
                    ));
                    child.roles.insert("kind".to_owned(), block_kind.to_owned());
                    child.roles.insert("status".to_owned(), outcome);
                    root = root.child(child);
                }
            }
            // A provider-owned child's work renders live in the transcript —
            // observation-only, with the disclosure and click-through handled
            // by the group itself.
            TranscriptBlock::SubagentGroup(group) => {
                let card = poodle_specs::AgentSubagentSpec::new(group.subagent.clone())
                    .with_expanded(spec.expanded_subagent_groups.contains(&group.id))
                    .with_detail_lines(group.detail_lines.clone().unwrap_or_default())
                    .with_size(base_size)
                    .with_density(density);

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
                    instance_id: handlers.instance_id.as_deref().map(|scope| {
                        format!("{scope}:transcript-subagent:{}", group.id)
                    }),
                };
                let mut child = crate::agent_subagent::agent_subagent(
                    &card,
                    ctx,
                    group_handlers,
                );
                child.runtime_id = Some(agent_transcript_block_id(
                    handlers.instance_id.as_deref(),
                    block_kind,
                    &block_id,
                ));
                child.roles.insert("kind".to_owned(), block_kind.to_owned());
                root = root.child(child);
            }
            TranscriptBlock::DecidedPlan(record) => {
                let mut card = AgentPlanRecordSpec::new(record.plan, record.status);
                if let Some(decided_at) = record.decided_at {
                    card = card.with_decided_at(decided_at);
                }
                let mut child = crate::agent_plan_record::agent_plan_record(
                    &card,
                    ctx,
                    crate::agent_plan_record::AgentPlanRecordHandlers {
                        instance_id: handlers.instance_id.as_deref().map(|scope| {
                            format!("{scope}:transcript-plan:{block_id}")
                        }),
                        ..crate::agent_plan_record::AgentPlanRecordHandlers::default()
                    },
                );
                child.runtime_id = Some(agent_transcript_block_id(
                    handlers.instance_id.as_deref(),
                    block_kind,
                    &block_id,
                ));
                child.roles.insert("kind".to_owned(), block_kind.to_owned());
                child
                    .roles
                    .insert("status".to_owned(), record.status.as_str().to_owned());
                root = root.child(child);
            }
            TranscriptBlock::Activity(_) => {}
        }
    }

    // The activity footer sits outside the block flow so it stays under the
    // transcript rather than scrolling as a block of its own.
    if let Some(label) = spec.activity_label() {
        let mut activity = activity_text(label.to_string());
        activity.runtime_id = Some(format!(
            "{}:activity",
            agent_transcript_root_id(handlers.instance_id.as_deref())
        ));
        activity.roles.insert("kind".to_owned(), "activity".to_owned());
        activity.roles.insert(
            "status".to_owned(),
            if posture(spec) == "loading" {
                "loading"
            } else {
                "terminal"
            }
            .to_owned(),
        );
        root = root.child(activity);
    }

    root
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use poodle_adapter::ThemeProvider;

    #[test]
    fn jump_control_uses_the_transcript_recipe_and_real_activation() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let spec = AgentTranscriptSpec::default();
        let activations = Arc::new(AtomicUsize::new(0));
        let sink = Arc::clone(&activations);
        let jump = agent_transcript_jump(
            &spec,
            &ctx,
            Some(Arc::new(move || {
                sink.fetch_add(1, Ordering::Relaxed);
            })),
        );

        assert_eq!(jump.a11y.role, Some(NodeRole::Button));
        // The leading icon rides in the contract §8 icon-md wrapper box; look
        // through it for the glyph node.
        let leading = jump
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "arrow-down"),
            )
            .expect("jump leading icon exists inside its wrapper");
        match &leading.kind {
            poodle_node::NodeKind::Icon { name, .. } => assert_eq!(name, "arrow-down"),
            _ => unreachable!(),
        }
        assert_eq!(
            jump.style.descriptor.background,
            Some(theme.resolve_color(spec.jump_fill_token())),
        );
        assert_eq!(
            jump.style.descriptor.border.color,
            theme.resolve_color(spec.jump_border_token()),
        );
        assert_eq!(
            jump.style.descriptor.corner_radii.top_left,
            theme.resolve_radius(spec.jump_radius_token()),
        );

        jump.interaction.on_activate.as_ref().unwrap()();
        assert_eq!(activations.load(Ordering::Relaxed), 1);
    }
}
