//! Floating overlay utility — positions a surface element relative to an
//! anchor.
//!
//! Ported from: `packages/jetstream/components/src/floating_overlay.rs`.
//! A relative wrapper anchors the absolute surface; the surface is taken out
//! of flow and positioned using `anchor_h` / `anchor_w` estimates supplied by
//! the caller.

use poodle_node::{LayoutDirection, LayoutSizing, Node, NodePosition};
use poodle_specs::OverlayPlacement;

/// Gap between anchor and floating surface (px).
const OVERLAY_GAP_PX: f32 = 4.0;

/// Wrap `anchor` in a relative container and, when `surface` is `Some`,
/// position it absolutely according to `placement`.
///
/// `anchor_h`: estimated anchor height (used for top/bottom families).
/// `anchor_w`: estimated anchor width (used for left/right families).
pub fn floating_overlay(
    anchor: Node,
    surface: Option<Node>,
    placement: OverlayPlacement,
    anchor_h: f32,
    anchor_w: f32,
) -> Node {
    let gap = OVERLAY_GAP_PX;

    let mut wrapper = Node::container();
    // Explicit Row (see switch.rs).
    wrapper.style.descriptor.layout.direction = LayoutDirection::Row;
    // A fit-sized GPUI div stretches across a column parent's cross axis.
    // Pin this box to the supplied anchor estimate so end placements resolve
    // against the trigger, not the surrounding specimen card.
    wrapper.style.descriptor.layout.width = LayoutSizing::Fixed(anchor_w);
    wrapper.style.descriptor.layout.height = LayoutSizing::Fixed(anchor_h);
    wrapper.position = NodePosition::Relative;
    wrapper.style.flex_none = true;
    let mut wrapper = wrapper.child(anchor);

    if let Some(surface_el) = surface {
        let position = match placement {
            // ── Bottom family ────────────────────────────────────────
            OverlayPlacement::Bottom | OverlayPlacement::BottomStart => NodePosition::Absolute {
                top: Some(anchor_h + gap),
                left: Some(0.0),
                right: None,
                bottom: None,
            },
            OverlayPlacement::BottomEnd => NodePosition::Absolute {
                top: Some(anchor_h + gap),
                left: None,
                right: Some(0.0),
                bottom: None,
            },

            // ── Top family ───────────────────────────────────────────
            OverlayPlacement::Top | OverlayPlacement::TopStart => NodePosition::Absolute {
                top: None,
                left: Some(0.0),
                right: None,
                bottom: Some(anchor_h + gap),
            },
            OverlayPlacement::TopEnd => NodePosition::Absolute {
                top: None,
                left: None,
                right: Some(0.0),
                bottom: Some(anchor_h + gap),
            },

            // ── Right family ─────────────────────────────────────────
            OverlayPlacement::Right | OverlayPlacement::RightStart => NodePosition::Absolute {
                top: Some(0.0),
                left: Some(anchor_w + gap),
                right: None,
                bottom: None,
            },
            OverlayPlacement::RightEnd => NodePosition::Absolute {
                top: None,
                left: Some(anchor_w + gap),
                right: None,
                bottom: Some(0.0),
            },

            // ── Left family ──────────────────────────────────────────
            OverlayPlacement::Left | OverlayPlacement::LeftStart => NodePosition::Absolute {
                top: Some(0.0),
                left: None,
                right: Some(anchor_w + gap),
                bottom: None,
            },
            OverlayPlacement::LeftEnd => NodePosition::Absolute {
                top: None,
                left: None,
                right: Some(anchor_w + gap),
                bottom: Some(0.0),
            },
        };
        let mut surface_container = Node::container();
        // Explicit Row (see switch.rs).
        surface_container.style.descriptor.layout.direction = LayoutDirection::Row;
        surface_container.position = position;
        wrapper = wrapper.child(surface_container.child(surface_el));
    }

    wrapper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_is_pinned_to_the_anchor_for_end_placement() {
        let node = floating_overlay(
            Node::container(),
            Some(Node::container()),
            OverlayPlacement::BottomEnd,
            28.0,
            28.0,
        );

        assert_eq!(
            node.style.descriptor.layout.width,
            LayoutSizing::Fixed(28.0)
        );
        assert_eq!(
            node.style.descriptor.layout.height,
            LayoutSizing::Fixed(28.0)
        );
        assert!(node.style.flex_none);
        assert!(matches!(
            node.children[1].position,
            NodePosition::Absolute {
                top: Some(32.0),
                right: Some(0.0),
                ..
            }
        ));
    }
}
