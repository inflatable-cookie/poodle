//! AgentPlanRecord — the read-only record a decided plan leaves.
//!
//! Contract: `docs/contracts/components/agent-plan-record.md`
//!
//! The only handler is the disclosure: a decision the agent has already acted
//! on cannot be changed from the transcript, so there is nothing else to
//! click.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{AgentMessageSpec, AgentPlanRecordSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct AgentPlanRecordHandlers {
    /// Fires with the next expanded state when the disclosure is used.
    pub on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Stable native instance scope. Records share status and often have no
    /// `decided_at`; two of them would otherwise share one backend focus
    /// handle. Identity never includes `is_expanded`.
    pub instance_id: Option<String>,
}

pub const AGENT_PLAN_RECORD_TOGGLE_ID: &str = "agent-plan-record-toggle";

/// The backend-state id of the disclosure control.
pub fn agent_plan_record_toggle_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("agent-plan-record:{scope}:toggle"),
        None => AGENT_PLAN_RECORD_TOGGLE_ID.to_string(),
    }
}

fn scoped(instance_id: Option<&str>, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("agent-plan-record:{scope}:{part}"))
}

pub fn agent_plan_record(
    spec: &AgentPlanRecordSpec,
    ctx: &RenderContext<'_>,
    handlers: AgentPlanRecordHandlers,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let surface = ctx.theme().resolve_color(spec.surface_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let badge_color = ctx.theme().resolve_color(spec.badge_token());
    let summary_color = ctx.theme().resolve_color(spec.summary_token());
    let meta_color = ctx.theme().resolve_color(spec.meta_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let gap = rem_to_px(spec.gap_rem(density));
    let inset = rem_to_px(spec.padding_inset_rem(density));
    let hairline = rem_to_px(0.0625);

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.fill_width = true;
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = inset;
        pad.right = inset;
        pad.top = inset;
        pad.bottom = inset;
        s.descriptor.border.width = hairline;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(surface);
    }

    // The header row is the decision at a glance: the badge, and when it
    // happened when the host tracks it.
    let mut header = Node::container();
    header.style.descriptor.layout.direction = LayoutDirection::Row;
    header.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    header.style.descriptor.layout.spacing.gap = rem_to_px(0.5);

    let mut badge = Node::text(spec.badge_label());
    badge.style.text_size = Some(font_size);
    badge.style.text_weight = Some(600);
    badge.style.descriptor.text_color = Some(badge_color);
    let mut header = header.child(badge);

    if let Some(decided_at) = &spec.decided_at {
        let mut meta = Node::text(decided_at.clone());
        meta.style.text_size = Some(font_size);
        meta.style.descriptor.text_color = Some(meta_color);
        header = header.child(meta);
    }

    let mut root = root.child(header);

    if spec.shows_summary() {
        let mut summary = Node::text(spec.summary());
        summary.style.text_size = Some(font_size);
        summary.style.descriptor.text_color = Some(summary_color);
        root = root.child(summary);
    } else {
        let body = crate::agent_message::agent_message(
            &AgentMessageSpec::new(spec.plan.clone())
                .with_size(base_size)
                .with_density(density),
            ctx,
        );
        root = root.child(body);
    }

    // The disclosure is the one control: the summary is a stand-in for exactly
    // the content it hides.
    let toggle_label = if spec.is_expanded {
        spec.collapse_label.clone()
    } else {
        spec.expand_label.clone()
    };
    let mut toggle = Node::button("");
    toggle.id = Some(AGENT_PLAN_RECORD_TOGGLE_ID.to_string());
    toggle.runtime_id = scoped(handlers.instance_id.as_deref(), "toggle");
    toggle.a11y.label = Some(toggle_label.clone());
    toggle.a11y.role = Some(NodeRole::Button);
    toggle.a11y.expanded = Some(spec.is_expanded);
    toggle.style.descriptor.layout.direction = LayoutDirection::Row;
    toggle.style.descriptor.background = Some(crate::color::TRANSPARENT);
    toggle.interaction.focusable = true;
    toggle.style.focus = Some(StylePatch {
        background: None,
        border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
        text_color: None,
        opacity: None,
    });

    let mut label = Node::text(toggle_label);
    label.style.text_size = Some(font_size);
    label.style.descriptor.text_color = Some(meta_color);
    let mut toggle = toggle.child(label);

    if let Some(handler) = handlers.on_toggle {
        let next = !spec.is_expanded;
        toggle.style.descriptor.cursor = CursorHint::Pointer;
        toggle.interaction.on_activate = Some(Arc::new(move || handler(next)));
    }

    root.child(toggle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_plan::AgentPlanStatus;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec(expanded: bool) -> AgentPlanRecordSpec {
        AgentPlanRecordSpec::new("## Plan", AgentPlanStatus::Accepted).with_expanded(expanded)
    }

    #[test]
    fn toggle_identity_does_not_include_expanded_state() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let shut = agent_plan_record(&spec(false), &ctx, AgentPlanRecordHandlers::default());
        let open = agent_plan_record(&spec(true), &ctx, AgentPlanRecordHandlers::default());
        let shut_toggle = shut
            .find(&|n| n.id.as_deref() == Some(AGENT_PLAN_RECORD_TOGGLE_ID))
            .expect("shut toggle");
        let open_toggle = open
            .find(&|n| n.id.as_deref() == Some(AGENT_PLAN_RECORD_TOGGLE_ID))
            .expect("open toggle");
        assert_eq!(shut_toggle.id, open_toggle.id);
        assert_eq!(shut_toggle.runtime_id, open_toggle.runtime_id);
        assert!(shut_toggle.runtime_id.is_none());
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let first = agent_plan_record(
            &spec(false),
            &ctx,
            AgentPlanRecordHandlers {
                instance_id: Some("first".to_string()),
                ..AgentPlanRecordHandlers::default()
            },
        );
        let second = agent_plan_record(
            &spec(false),
            &ctx,
            AgentPlanRecordHandlers {
                instance_id: Some("second".to_string()),
                ..AgentPlanRecordHandlers::default()
            },
        );
        let expected = agent_plan_record_toggle_focus_id(Some("first"));
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(expected.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(agent_plan_record_toggle_focus_id(Some("second")).as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some(AGENT_PLAN_RECORD_TOGGLE_ID))
            .is_some());
        let open_first = agent_plan_record(
            &spec(true),
            &ctx,
            AgentPlanRecordHandlers {
                instance_id: Some("first".to_string()),
                ..AgentPlanRecordHandlers::default()
            },
        );
        assert!(open_first
            .find(&|n| n.runtime_id.as_deref() == Some(expected.as_str()))
            .is_some());
        assert!(second
            .find(&|n| n.runtime_id.as_deref()
                == Some(agent_plan_record_toggle_focus_id(Some("second")).as_str()))
            .is_some());
    }
}
