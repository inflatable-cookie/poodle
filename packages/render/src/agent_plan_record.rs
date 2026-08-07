//! AgentPlanRecord — the read-only record a decided plan leaves.
//!
//! Contract: `docs/contracts/components/agent-plan-record.md`
//!
//! The only handler is the disclosure: a decision the agent has already acted
//! on cannot be changed from the transcript, so there is nothing else to
//! click.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole};
use poodle_specs::{AgentMessageSpec, AgentPlanRecordSpec};

use crate::presentation::rem_to_px;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct AgentPlanRecordHandlers {
    /// Fires with the next expanded state when the disclosure is used.
    pub on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub fn agent_plan_record(
    spec: &AgentPlanRecordSpec,
    theme: &dyn ThemeProvider,
    handlers: AgentPlanRecordHandlers,
) -> Node {
    let surface = theme.resolve_color(spec.surface_token());
    let border = theme.resolve_color(spec.border_token());
    let badge_color = theme.resolve_color(spec.badge_token());
    let summary_color = theme.resolve_color(spec.summary_token());
    let meta_color = theme.resolve_color(spec.meta_token());
    let radius = theme.resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
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
                .with_size(spec.size)
                .with_density(spec.density),
            theme,
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
    toggle.a11y.label = Some(toggle_label.clone());
    toggle.a11y.role = Some(NodeRole::Button);
    toggle.a11y.expanded = Some(spec.is_expanded);
    toggle.style.descriptor.layout.direction = LayoutDirection::Row;
    toggle.style.descriptor.background = Some(crate::color::TRANSPARENT);
    toggle.interaction.focusable = true;

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
