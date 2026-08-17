//! AgentPlan — the plan an agent proposes at the end of a plan-mode turn.
//!
//! Contract: `docs/contracts/components/agent-plan.md`
//!
//! The plan body renders through `agent_message`, so the plan reads exactly
//! like the turn's prose did. The controls render only while the plan is
//! pending; a settled spec shows the badge, covering the moment between the
//! decision and the host swapping in the record.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{AgentMessageSpec, AgentPlanSpec};

use crate::color::TRANSPARENT;
use crate::presentation::rem_to_px;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct AgentPlanHandlers {
    /// Fires when the plan is accepted as proposed.
    pub on_accept: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Fires when the operator wants to revise. The component owns no text
    /// input: the host focuses its composer, where the feedback is typed as an
    /// ordinary message.
    pub on_revise: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Fires when the plan is dismissed.
    pub on_dismiss: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub fn agent_plan(
    spec: &AgentPlanSpec,
    theme: &dyn ThemeProvider,
    handlers: AgentPlanHandlers,
) -> Node {
    let accent = theme.resolve_color(spec.accent_token());
    let border = theme.resolve_color(spec.border_token());
    let action_color = theme.resolve_color(spec.action_token());
    let badge_color = theme.resolve_color(spec.badge_token());
    let radius = theme.resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let action_gap = rem_to_px(spec.action_gap_rem());
    let hairline = rem_to_px(0.0625);

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.style.descriptor.layout.spacing.gap = gap;

    // The plan is markdown, rendered by the same path as the turn's prose.
    let body = crate::agent_message::agent_message(
        &AgentMessageSpec::new(spec.plan.clone())
            .with_size(spec.size)
            .with_density(spec.density),
        theme,
    );
    let mut root = root.child(body);

    if spec.can_decide() {
        let mut actions = Node::container();
        actions.style.descriptor.layout.direction = LayoutDirection::Row;
        actions.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        actions.style.descriptor.layout.spacing.gap = action_gap;

        let action = |kind: &str,
                      label: String,
                      primary: bool,
                      handler: Option<Arc<dyn Fn() + Send + Sync>>| {
            let mut button = Node::button("");
            button.id = Some(format!("agent-plan-{kind}"));
            button.a11y.label = Some(label.clone());
            button.a11y.role = Some(NodeRole::Button);
            {
                let s = &mut button.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.top = rem_to_px(0.25);
                pad.bottom = rem_to_px(0.25);
                pad.left = rem_to_px(0.625);
                pad.right = rem_to_px(0.625);
                s.descriptor.border.width = hairline;
                s.descriptor.border.color = if primary { TRANSPARENT } else { border };
                s.descriptor.background = Some(if primary { accent } else { TRANSPARENT });
                let c = &mut s.descriptor.corner_radii;
                c.top_left = radius;
                c.top_right = radius;
                c.bottom_right = radius;
                c.bottom_left = radius;
            }
            button.interaction.focusable = true;
            button.style.focus = Some(StylePatch {
                background: None,
                border_color: Some(theme.resolve_color("color.accent.focusRing")),
                text_color: None,
                opacity: None,
            });

            let mut text = Node::text(label);
            text.style.text_size = Some(font_size);
            text.style.descriptor.text_color = Some(if primary {
                theme.resolve_color(spec.primary_action_token())
            } else {
                action_color
            });
            let mut button = button.child(text);

            if let Some(handler) = handler {
                button.style.descriptor.cursor = CursorHint::Pointer;
                button.interaction.on_activate = Some(Arc::new(move || handler()));
            }

            button
        };

        actions = actions.child(action(
            "accept",
            spec.accept_label.clone(),
            true,
            handlers.on_accept,
        ));
        actions = actions.child(action(
            "revise",
            spec.revise_label.clone(),
            false,
            handlers.on_revise,
        ));
        if spec.is_dismissible {
            actions = actions.child(action(
                "dismiss",
                spec.dismiss_label.clone(),
                false,
                handlers.on_dismiss,
            ));
        }

        root = root.child(actions);
    } else {
        // A settled plan is a fact, not a prompt: the badge replaces the
        // controls until the host swaps in the record.
        let mut badge = Node::text(spec.status_label().to_string());
        badge.style.text_size = Some(font_size);
        badge.style.descriptor.text_color = Some(badge_color);
        root = root.child(badge);
    }

    root
}
