//! InlineRemediation — inline fix suggestion.
//!
//! Contract: `docs/contracts/components/inline-remediation.md` (standalone
//! authority). Ported from:
//! `packages/jetstream/components/src/inline_remediation.rs`.
//!
//! Anatomy (contract §2): Root `<aside>` → tone-colored **left border** →
//! Content [Title?, Message] → optional Action (Button). No leading icon part
//! exists in the contract.

use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{ButtonSpec, InlineRemediationSpec};

use crate::button::button;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

pub fn inline_remediation(spec: &InlineRemediationSpec, ctx: &RenderContext<'_>) -> Node {
    // Contract §6 Border: tone → color.status.* (from border_token()).
    let border = ctx.theme().resolve_color(spec.border_token());
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    // Contract §2 Message → text-secondary.
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");

    // Contract §6 Root gap: title-to-message vertical gap = space.stack.sm.
    let content_gap = ctx.theme().resolve_space(spec.gap_token());
    // Title (typography-label) / message (typography-body) / hint sizes.
    let title_size = rem_to_px(0.8125);
    let message_size = rem_to_px(0.8125);
    let hint_size = rem_to_px(0.75);
    // Left accent border weight + inset row spacing.
    let accent_border_w = rem_to_px(0.125);
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let row_gap = ctx.theme().resolve_space(spec.gap_token());

    // Root <aside>: tone-colored LEFT border only (contract §2), no fill tint
    // (contract §6 lists no background-fill token).
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.border_left_width = Some(accent_border_w);
        s.border_color_left = Some(border);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = row_gap;
    }

    // Content column (Title?, Message, referenced-field hint).
    let mut content = Node::container();
    {
        let s = &mut content.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = content_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
    }

    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.style.descriptor.text_color = Some(text_primary);
        t.style.text_size = Some(title_size);
        t.style.text_weight = Some(600);
        content = content.child(t);
    }

    let mut msg = Node::text(&spec.message);
    msg.style.descriptor.text_color = Some(text_secondary);
    msg.style.text_size = Some(message_size);
    let mut content = content.child(msg);

    // Referenced-field count hint (supports aria-describedby host wiring; the
    // wiring itself lives in the host form, per contract §5 / accepted delta).
    if spec.reference_count() > 0 {
        let hint_text = format!(
            "{} field{} affected",
            spec.reference_count(),
            if spec.reference_count() == 1 { "" } else { "s" }
        );
        let mut hint = Node::text(&hint_text);
        hint.style.descriptor.text_color = Some(text_secondary);
        hint.style.text_size = Some(hint_size);
        content = content.child(hint);
    }

    let mut el = el.child(content);

    // Action: contract §2 "delegates to Button primitive" — render a real
    // button using the action's variant + disabled state.
    if let Some(ref action) = spec.action {
        let mut btn = button(
            &ButtonSpec::new()
                .with_variant(action.variant)
                .with_label(action.label.clone())
                .with_disabled(action.is_disabled),
            ctx,
            None,
        );
        btn.id = Some(format!("inline-remediation-action:{}", action.id));
        el = el.child(btn);
    }

    el
}
