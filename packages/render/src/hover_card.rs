//! HoverCard — the open hover-card surface.
//!
//! Contract: `docs/contracts/components/hover-card.md`
//! Ported from: `packages/jetstream/components/src/hover_card.rs`.
//!
//! The trigger relationship, delay timers, anchored placement and viewport
//! clamping are host-owned (contract §12 Known Delta); this renders the
//! surface at its current open state.

use poodle_node::{LayoutDirection, Node, NodeRole};
use poodle_specs::HoverCardSpec;

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;

pub fn hover_card(spec: &HoverCardSpec, ctx: &RenderContext<'_>, content: Option<Node>) -> Node {
    // Contract §8 background: color-mix(elevated 98%, panel).
    let elevated = ctx.theme().resolve_color(spec.fill_token());
    let panel = ctx.theme().resolve_color("color.background.panel");
    let fill = mix_srgb(elevated, panel, 0.98);

    // Contract §8 border: color-mix(border-default 72%, transparent).
    let border_base = ctx.theme().resolve_color("color.border.default");
    let border = with_alpha(border_base, border_base.3 * 0.72);
    let radius = ctx.theme().resolve_radius("radius.surface");

    // Contract §8 padding: space-panel-y / space-panel-x.
    let pad_x = ctx.theme().resolve_space("space.panel.x");
    let pad_y = ctx.theme().resolve_space("space.panel.y");

    // Contract §7 sizing: min-width 14rem, max-width min(22rem, 90vw) — the
    // token bounds; the 90vw clamp is host-driven.
    let min_w = ctx.theme().resolve_space("size.menu.minWidth");
    let max_w = ctx.theme().resolve_space("size.hoverCard.maxWidth");

    let mut el = Node::container();
    // Contract: the hover card surface is a `dialog`.
    el.a11y.role = Some(NodeRole::Dialog);
    {
        let s = &mut el.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.min_width = Some(min_w);
        s.max_width = Some(max_w);
        // Token-accurate elevation-overlay.
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        s.overlay = true;
    }

    let mut el = el;
    if let Some(c) = content {
        el = el.child(c);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
