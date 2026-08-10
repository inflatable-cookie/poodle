//! AgentSubagent — an inline group for a provider-owned child agent's work in
//! the transcript.
//!
//! Contract: `docs/contracts/components/agent-subagent.md`
//!
//! Identity and status in the header, a one-line activity while the child
//! runs, an expandable detail, and a click-through to the child's work.
//! Observation-only: the only handlers are the disclosure and the
//! click-through — there is no stop, cancel or steer affordance, because
//! controlling a provider-owned child is not the transcript's job.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::agent_subagent::{is_terminal_subagent_status, subagent_status_spins};
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::{AgentSubagentSpec, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::color::TRANSPARENT;
use crate::presentation::rem_to_px;
use crate::spinner::spinner;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct AgentSubagentHandlers {
    /// Fires with the next expanded state when the disclosure is used.
    pub on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Fires when the reader clicks through to the child's work.
    pub on_open_child: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn agent_subagent(
    spec: &AgentSubagentSpec,
    theme: &dyn ThemeProvider,
    handlers: AgentSubagentHandlers,
) -> Node {
    let surface = theme.resolve_color(spec.surface_token());
    let border = theme.resolve_color(spec.border_token());
    let label_color = theme.resolve_color(spec.label_token());
    let activity_color = theme.resolve_color(spec.activity_token());
    let meta_color = theme.resolve_color(spec.meta_token());
    let badge_color = theme.resolve_color(spec.badge_token());
    let radius = theme.resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let inset = rem_to_px(spec.inset_rem());
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

    // ── Header: identity + status at a glance ──────────────────
    let mut header = Node::container();
    header.style.descriptor.layout.direction = LayoutDirection::Row;
    header.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    header.style.descriptor.layout.spacing.gap = rem_to_px(0.5);

    let mut label = Node::text(spec.item.label.clone());
    label.style.text_size = Some(font_size);
    label.style.text_weight = Some(600);
    label.style.descriptor.text_color = Some(label_color);
    let mut header = header.child(label);

    let mut badge = Node::text(spec.status_label().to_string());
    badge.style.text_size = Some(font_size);
    badge.style.text_weight = Some(600);
    badge.style.descriptor.text_color = Some(badge_color);
    header = header.child(badge);

    let mut root = root.child(header);

    // ── Body: the live line, or the settled summary ─────────────
    if is_terminal_subagent_status(spec.item.status) {
        if let Some(summary) = &spec.item.summary {
            let mut text = Node::text(summary.clone());
            text.style.text_size = Some(font_size);
            text.style.descriptor.text_color = Some(activity_color);
            root = root.child(text);
        }
    } else {
        let mut activity = Node::container();
        activity.style.descriptor.layout.direction = LayoutDirection::Row;
        activity.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        activity.style.descriptor.layout.spacing.gap = rem_to_px(0.375);

        // Only `running` spins: a pending or waiting child is not actively
        // working, and a terminal status never signals ongoing work.
        if subagent_status_spins(spec.item.status) {
            let dots = spinner(
                &SpinnerSpec::new()
                    .with_variant(SpinnerVariant::Dots)
                    .with_tone(SpinnerTone::Muted),
                theme,
            );
            activity = activity.child(dots);
        }

        if let Some(line) = &spec.item.activity_line {
            let mut text = Node::text(line.clone());
            text.style.text_size = Some(font_size);
            text.style.descriptor.text_color = Some(activity_color);
            activity = activity.child(text);
        }

        if !activity.children.is_empty() {
            root = root.child(activity);
        }
    }

    // ── Expanded detail: recent activity lines ─────────────────
    if spec.is_expanded && !spec.detail_lines.is_empty() {
        let mut detail = Node::container();
        detail.style.descriptor.layout.direction = LayoutDirection::Column;
        detail.style.descriptor.layout.spacing.gap = rem_to_px(0.25);

        for line in &spec.detail_lines {
            let mut text = Node::text(line.clone());
            text.style.text_size = Some(font_size);
            text.style.descriptor.text_color = Some(activity_color);
            detail = detail.child(text);
        }

        root = root.child(detail);
    }

    // ── Actions: disclosure + click-through ────────────────────
    let mut actions = Node::container();
    actions.style.descriptor.layout.direction = LayoutDirection::Row;
    actions.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    actions.style.descriptor.layout.spacing.gap = rem_to_px(0.75);

    let action =
        |label: String, handler: Option<Arc<dyn Fn() + Send + Sync>>| {
            let mut button = Node::button("");
            button.a11y.label = Some(label.clone());
            button.a11y.role = Some(NodeRole::Button);
            button.style.descriptor.layout.direction = LayoutDirection::Row;
            button.style.descriptor.background = Some(TRANSPARENT);
            button.interaction.focusable = true;

            let mut text = Node::text(label);
            text.style.text_size = Some(font_size);
            text.style.descriptor.text_color = Some(meta_color);
            let mut button = button.child(text);

            if let Some(handler) = handler {
                button.interaction.on_activate = Some(Arc::new(move || handler()));
            }

            button
        };

    // The disclosure is pointless without anything to reveal.
    if spec.shows_toggle() {
        let next = !spec.is_expanded;
        let toggle_label = if spec.is_expanded {
            spec.collapse_label.clone()
        } else {
            spec.expand_label.clone()
        };
        let toggle = action(toggle_label, handlers.on_toggle.as_ref().map(|handler| {
            let handler = Arc::clone(handler);
            let next = next;
            Arc::new(move || handler(next)) as Arc<dyn Fn() + Send + Sync>
        }));
        actions = actions.child(toggle);
    }

    let open = action(spec.open_child_label.clone(), handlers.on_open_child);
    actions = actions.child(open);

    root.child(actions)
}
