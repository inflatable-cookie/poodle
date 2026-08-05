//! Popover — anchored floating surface (the open panel at current state).
//!
//! Contract: `docs/contracts/components/popover.md`
//! Ported from: `packages/jetstream/components/src/popover.rs`.
//!
//! Open/close, placement anchoring, outside-dismiss and Escape are
//! host-owned; `overlay` lifts the surface above normal content.

use poodle_adapter::ThemeProvider;
use poodle_node::{LayoutDirection, Node, NodeRole, ShadowLayer};
use poodle_specs::PopoverSpec;

use crate::color::with_alpha;
use crate::presentation::rem_to_px;

pub fn popover(spec: &PopoverSpec, theme: &dyn ThemeProvider, content: Option<Node>) -> Node {
    // Contract §8 surface: background = background-elevated, border =
    // border-subtle at 74%, radius = radius-surface.
    let fill = theme.resolve_color(spec.surface_fill_token());
    let border_base = theme.resolve_color(spec.surface_border_token());
    let border = with_alpha(border_base, border_base.3 * spec.surface_border_alpha());
    let border_width = theme.resolve_space("border.width.default");
    let radius = theme.resolve_radius("radius.surface");

    // Contract §8 padding = space.panel.y / space.panel.x.
    let pad_x = theme.resolve_space("space.panel.x");
    let pad_y = theme.resolve_space("space.panel.y");

    // Contract §7: min-width 14rem, max-width min(24rem, 90vw) — the 24rem
    // arm; both overridable via surfaceMinWidth/surfaceMaxWidth.
    let min_w = rem_to_px(spec.effective_surface_min_width_rem());
    let max_w = rem_to_px(spec.effective_surface_max_width_rem());

    let mut el = Node::container();
    // Contract: the popover surface is a `dialog`.
    el.a11y.role = Some(NodeRole::Dialog);
    {
        let s = &mut el.style;
        // Explicit Row (see switch.rs): the old surface kept the default.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
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
        // Contract §8 box-shadow: token-accurate elevation-overlay primary
        // drop + the inset top highlight layered alongside.
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: poodle_node::ColorValue(1.0, 1.0, 1.0, 0.08),
            inset: true,
        }];
        s.overlay = true; // Render above normal content.
    }

    let mut el = el;
    if let Some(content_el) = content {
        el = el.child(content_el);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
