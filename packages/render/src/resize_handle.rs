//! ResizeHandle — a draggable divider.
//!
//! Contract: `docs/contracts/components/resize-handle.md`
//! Ported from: `packages/jetstream/components/src/resize_handle.rs`.
//!
//! `on_resize` carries the drag's per-frame delta along the handle's axis —
//! pixels, signed. The handle cannot know the panes' sizes, so an absolute
//! position would be a guess; a delta is a fact, and the host applies it to
//! the ratio it already holds. Start and end mark the gesture's bounds for
//! hosts that commit on release.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeDragEvent, NodeDragPhase, NodePosition, NodeRole, StylePatch,
};
use poodle_specs::{Orientation, ResizeHandleSpec};

use crate::presentation::rem_to_px;

/// Where in the gesture a resize event sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizePhase {
    Start,
    Move,
    End,
}

pub fn resize_handle(
    spec: &ResizeHandleSpec,
    theme: &dyn ThemeProvider,
    on_resize: Option<Arc<dyn Fn(ResizePhase, f32) + Send + Sync>>,
) -> Node {
    let handle_color = theme.resolve_color(spec.border_color_token());
    // Contract §8 hover/dragging: line recolors to accent-base.
    let hover_color = theme.resolve_color(spec.hover_color_token());
    let is_disabled = spec.is_disabled;

    // Contract §7: the root is only as thick as the line (0.125rem), so the
    // divider costs no layout space beyond the hairline. The grab area
    // (0.5rem) is an absolutely positioned overlay centred on the line, which
    // overlaps the neighbouring regions instead of widening the gap.
    let visual_size = rem_to_px(spec.thickness_rem());
    let hit_size = rem_to_px(spec.hit_size_rem());
    let hit_offset = rem_to_px(spec.hit_offset_rem());

    // The affordance line. Contract §8 hover/dragging recolors it to
    // accent-base; wired on the line itself (no group-hover channel). The line
    // spans the full length of the hit target so its hover region matches.
    let build_line = |mut line: Node| -> Node {
        line.style.descriptor.background = Some(handle_color);
        if !is_disabled {
            let patch = StylePatch {
                background: Some(hover_color),
                border_color: None,
                text_color: None,
                opacity: None,
            };
            line.style.hover = Some(patch.clone());
            line.style.active = Some(patch);
        }
        line
    };

    // Drags do not bubble: the gesture starts only if the node under the
    // pointer carries the handler, and the pointer lands on the grab overlay
    // or the line, never the root. Same lesson the sliders taught — every hit
    // target gets the handler.
    let drag_handler: Option<Arc<dyn Fn(&NodeDragEvent) + Send + Sync>> = if spec.is_disabled {
        None
    } else if let Some(handler) = &on_resize {
        let handler = Arc::clone(handler);
        let horizontal = matches!(spec.orientation, Orientation::Horizontal);
        Some(Arc::new(move |event: &NodeDragEvent| match event.phase {
            NodeDragPhase::Start => handler(ResizePhase::Start, 0.0),
            NodeDragPhase::Move => {
                // A horizontal handle is a vertical line: it moves along x.
                let delta = if horizontal {
                    event.delta_x
                } else {
                    event.delta_y
                };
                handler(ResizePhase::Move, delta);
            }
            NodeDragPhase::End => handler(ResizePhase::End, 0.0),
        }))
    } else {
        None
    };

    let arm = |mut el: Node| -> Node {
        if let Some(handler) = &drag_handler {
            let handler = Arc::clone(handler);
            el.interaction.on_drag = Some(Arc::new(move |event| handler(event)));
        }
        el
    };
    let pill = |node: &mut Node| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = 999.0;
        c.top_right = 999.0;
        c.bottom_right = 999.0;
        c.bottom_left = 999.0;
    };

    let mut el = match spec.orientation {
        Orientation::Horizontal => {
            // Contract §7: horizontal orientation = vertical line.
            // Root: width 0.125rem (the line), height 100% (stretch to parent —
            // NOT flex-grow, which would fill the whole row). col-resize
            // cursor. Grab overlay: 0.5rem wide, centred, absolute.
            let mut root = Node::container();
            {
                let s = &mut root.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(visual_size);
                s.self_stretch = true;
                s.flex_shrink_zero = true;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.cursor = CursorHint::ColResize;
            }
            root.position = NodePosition::Relative;

            let mut overlay = Node::container();
            {
                let s = &mut overlay.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Fixed(hit_size);
                s.fill_height = true;
            }
            overlay.position = NodePosition::Absolute {
                top: Some(0.0),
                left: Some(hit_offset),
                right: None,
                bottom: None,
            };

            let mut line = Node::container();
            {
                let s = &mut line.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.fill_width = true;
                s.fill_height = true;
            }
            pill(&mut line);
            root.child(overlay).child(arm(build_line(line)))
        }
        Orientation::Vertical => {
            // Contract §7: vertical orientation = horizontal line.
            // Root: height 0.125rem (the line), width 100% (stretch).
            // row-resize. Grab overlay: 0.5rem tall, centred, absolute.
            let mut root = Node::container();
            {
                let s = &mut root.style;
                s.descriptor.layout.height = LayoutSizing::Fixed(visual_size);
                s.self_stretch = true;
                s.flex_shrink_zero = true;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.descriptor.cursor = CursorHint::RowResize;
            }
            root.position = NodePosition::Relative;

            let mut overlay = Node::container();
            {
                let s = &mut overlay.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.height = LayoutSizing::Fixed(hit_size);
                s.fill_width = true;
            }
            overlay.position = NodePosition::Absolute {
                top: Some(hit_offset),
                left: Some(0.0),
                right: None,
                bottom: None,
            };

            let mut line = Node::container();
            {
                let s = &mut line.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.fill_width = true;
                s.fill_height = true;
            }
            pill(&mut line);
            root.child(arm(overlay)).child(arm(build_line(line)))
        }
    };

    if is_disabled {
        // Contract §8 disabled: default cursor + 0.4 opacity, no interaction.
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        el.style.descriptor.cursor = CursorHint::Default;
    }

    let mut el = arm(el);
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el.a11y.role = Some(NodeRole::Splitter);
    el
}
