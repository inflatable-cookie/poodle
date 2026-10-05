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
//! A focusable viewport is a tab stop with the contract's focus ring, a
//! region role, and the default "Scrollable content" name; the backend owns
//! keyboard scrolling for any focusable scroll viewport. The role and name sit
//! on the viewport, where Svelte puts them — the root is a plain clip boundary.

use std::sync::Arc;

use poodle_node::{
    FocusRing, LayoutDirection, LayoutOverflow, LayoutSizing, Node, NodeRole, NodeScrollEvent,
};
use poodle_specs::{Direction, ScrollShellSpec, SurfaceRole};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Svelte's default name for a focusable viewport with no `label`.
const DEFAULT_LABEL: &str = "Scrollable content";

/// `on_scroll` observes the viewport position after wheel or keyboard
/// scrolling (the contract's `onScroll`).
pub fn scroll_shell(
    spec: &ScrollShellSpec,
    ctx: &RenderContext<'_>,
    children: Vec<Node>,
    on_scroll: Option<Arc<dyn Fn(&NodeScrollEvent) + Send + Sync>>,
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

    viewport.interaction.on_scroll = on_scroll;
    if spec.is_focusable {
        viewport.interaction.focusable = true;
        viewport.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
            width: ctx
                .theme()
                .resolve_border_width(spec.focus_ring_width_token()),
            offset: rem_to_px(0.125),
        });
    }
    let role = match spec.role {
        Some(SurfaceRole::Group) => Some(NodeRole::Group),
        Some(SurfaceRole::Region) => Some(NodeRole::Region),
        None if spec.is_focusable => Some(NodeRole::Region),
        None => None,
    };
    viewport.a11y.role = role;
    viewport.a11y.label = match spec.label.as_deref() {
        Some(label) => Some(label.to_owned()),
        None if spec.is_focusable => Some(DEFAULT_LABEL.to_owned()),
        None => None,
    };

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
    root.child(viewport)
}
