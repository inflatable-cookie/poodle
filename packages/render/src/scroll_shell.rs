//! ScrollShell — scrollable container.
//!
//! Contract: `docs/contracts/components/scroll-shell.md`
//! Ported from: `packages/jetstream/components/src/scroll_shell.rs`.
//!
//! Three-layer anatomy per contract §2:
//!   Root      → clip boundary: radius-surface
//!   Viewport  → scroll owner: per-axis overflow, padding
//!   Content   → sizing wrapper: horizontal max-content
//!
//! Keyboard scroll is host-owned.

use poodle_node::{LayoutDirection, LayoutOverflow, LayoutSizing, Node, NodeRole};
use poodle_specs::{Direction, ScrollShellSpec};

use crate::context::RenderContext;

pub fn scroll_shell(
    spec: &ScrollShellSpec,
    ctx: &RenderContext<'_>,
    children: Vec<Node>,
) -> Node {
    let needs_horizontal = matches!(spec.direction, Direction::Horizontal | Direction::Both);

    // ── Content — sizing wrapper ──
    // For horizontal/both the content must not collapse: a non-shrinking row
    // sized to its children is the `min-width: max-content` analogue
    // (contract §8 Content). Vertical content stacks and fills the width.
    let mut content = Node::container();
    {
        let s = &mut content.style;
        if needs_horizontal {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_shrink_zero = true;
        } else {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.fill_width = true;
        }
    }
    for child in children {
        content = content.child(child);
    }

    // ── Viewport — scroll owner ──
    // Direction sets the layout axis + which overflow scrolls.
    let mut viewport = Node::container();
    {
        let s = &mut viewport.style;
        match spec.direction {
            Direction::Horizontal => {
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
                s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            }
            Direction::Vertical => {
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
                s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            }
            Direction::Both => {
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
                s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
            }
        }
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.fill_width = true;
        s.fill_height = true;
        s.min_width = Some(0.0);
        s.min_height = Some(0.0);
    }

    // Token-resolved padding inset on the viewport (contract §8 padding scale).
    let inset = spec.resolved_padding();
    if let Some(h) = inset.horizontal {
        let p = ctx.theme().resolve_space(h);
        let pad = &mut viewport.style.descriptor.layout.spacing.padding;
        pad.left = p;
        pad.right = p;
    }
    if let Some(v) = inset.vertical {
        let p = ctx.theme().resolve_space(v);
        let pad = &mut viewport.style.descriptor.layout.spacing.padding;
        pad.top = p;
        pad.bottom = p;
    }

    let viewport = viewport.child(content);

    // ── Root — clip boundary ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.fill_width = true;
        s.fill_height = true;
        s.min_width = Some(0.0);
        s.min_height = Some(0.0);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        let r = ctx.theme().resolve_radius("radius.surface");
        s.descriptor.corner_radii.top_left = r;
        s.descriptor.corner_radii.top_right = r;
        s.descriptor.corner_radii.bottom_right = r;
        s.descriptor.corner_radii.bottom_left = r;
    }
    let mut root = root.child(viewport);
    root.a11y.role = Some(NodeRole::Region);
    root
}
