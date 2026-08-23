//! AgentPlan — the plan an agent proposes at the end of a plan-mode turn.
//!
//! Contract: `docs/contracts/components/agent-plan.md`
//!
//! The plan body renders through `agent_message`, so the plan reads exactly
//! like the turn's prose did. The controls render only while the plan is
//! pending; a settled spec shows the badge, covering the moment between the
//! decision and the host swapping in the record.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, StylePatch};
use poodle_specs::{AgentMessageSpec, AgentPlanSpec};

use crate::color::TRANSPARENT;
use crate::context::RenderContext;
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
    /// Stable native instance scope. Two pending plans would otherwise share
    /// one backend focus handle per action.
    pub instance_id: Option<String>,
}

/// The backend-state id of one plan action (`accept` / `revise` / `dismiss`).
pub fn agent_plan_action_focus_id(instance_id: Option<&str>, action: &str) -> String {
    match instance_id {
        Some(scope) => format!("agent-plan:{scope}:{action}"),
        None => format!("agent-plan-{action}"),
    }
}

fn scoped(instance_id: Option<&str>, action: &str) -> Option<String> {
    instance_id.map(|scope| format!("agent-plan:{scope}:{action}"))
}

pub fn agent_plan(
    spec: &AgentPlanSpec,
    ctx: &RenderContext<'_>,
    handlers: AgentPlanHandlers,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let accent = ctx.theme().resolve_color(spec.accent_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let action_color = ctx.theme().resolve_color(spec.action_token());
    let badge_color = ctx.theme().resolve_color(spec.badge_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let gap = rem_to_px(spec.gap_rem(density));
    let action_gap = rem_to_px(spec.action_gap_rem(density));
    let hairline = rem_to_px(0.0625);

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.style.descriptor.layout.spacing.gap = gap;

    // The plan is markdown, rendered by the same path as the turn's prose.
    let body = crate::agent_message::agent_message(
        &AgentMessageSpec::new(spec.plan.clone())
            .with_size(base_size)
            .with_density(density),
        ctx,
    );
    let mut root = root.child(body);

    if spec.can_decide() {
        let mut actions = Node::container();
        actions.style.descriptor.layout.direction = LayoutDirection::Row;
        actions.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        actions.style.descriptor.layout.spacing.gap = action_gap;

        let instance = handlers.instance_id.clone();
        let action = |kind: &str,
                      label: String,
                      primary: bool,
                      handler: Option<Arc<dyn Fn() + Send + Sync>>| {
            let mut button = Node::button("");
            button.id = Some(format!("agent-plan-{kind}"));
            button.runtime_id = scoped(instance.as_deref(), kind);
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
                border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
                text_color: None,
                opacity: None,
            });

            let mut text = Node::text(label);
            text.style.text_size = Some(font_size);
            text.style.descriptor.text_color = Some(if primary {
                ctx.theme().resolve_color(spec.primary_action_token())
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

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::agent_plan::AgentPlanStatus;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = AgentPlanSpec::new("1. Inspect.").with_status(AgentPlanStatus::Pending);
        let scoped = |scope: &str| AgentPlanHandlers {
            instance_id: Some(scope.to_string()),
            ..AgentPlanHandlers::default()
        };
        let first = agent_plan(&spec, &ctx, scoped("first"));
        let second = agent_plan(&spec, &ctx, scoped("second"));
        let accept = agent_plan_action_focus_id(Some("first"), "accept");
        assert!(first
            .find(&|n| n.runtime_id.as_deref() == Some(accept.as_str()))
            .is_some());
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(agent_plan_action_focus_id(Some("second"), "accept").as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some("agent-plan-accept"))
            .is_some());
        assert!(second
            .find(&|n| n.runtime_id.as_deref()
                == Some(agent_plan_action_focus_id(Some("second"), "revise").as_str()))
            .is_some());
    }
}
