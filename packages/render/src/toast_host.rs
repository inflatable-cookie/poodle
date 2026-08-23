//! ToastHost — positioned toast container. Wraps a ToastStack and positions
//! it at the chosen screen corner.
//!
//! Ported from: `packages/jetstream/components/src/toast_host.rs`.

use poodle_node::{LayoutDirection, LayoutSizing, Node, NodePosition, NodeRole};
use poodle_specs::{ToastHostPlacement, ToastHostSpec, ToastStackSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::toast_stack::{toast_stack, ToastStackHandlers};

pub fn toast_host(
    spec: &ToastHostSpec,
    ctx: &RenderContext<'_>,
    stack_spec: &ToastStackSpec,
    handlers: ToastStackHandlers,
) -> Node {
    // Empty toasts — render nothing.
    if stack_spec.toasts.is_empty() {
        let mut empty = Node::container();
        // Explicit Row (see switch.rs).
        empty.style.descriptor.layout.direction = LayoutDirection::Row;
        return empty;
    }

    // Contract §7/§8: inset 1rem, width cap 28rem — resolved from spec
    // accessors. The `calc(100vw - 2rem)` viewport clamp and `z-index: 80`
    // are web-only; the host is mounted last in the overlay so it stacks
    // above app chrome.
    let inset = rem_to_px(spec.inset_rem());
    let width = rem_to_px(spec.width_rem());

    let mut container = Node::container();
    {
        let s = &mut container.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(width);
        s.max_width = Some(width);
    }

    // Position at chosen corner using absolute positioning.
    container.position = match spec.placement {
        ToastHostPlacement::BottomEnd => NodePosition::Absolute {
            top: None,
            left: None,
            right: Some(inset),
            bottom: Some(inset),
        },
        ToastHostPlacement::BottomStart => NodePosition::Absolute {
            top: None,
            left: Some(inset),
            right: None,
            bottom: Some(inset),
        },
        ToastHostPlacement::TopEnd => NodePosition::Absolute {
            top: Some(inset),
            left: None,
            right: Some(inset),
            bottom: None,
        },
        ToastHostPlacement::TopStart => NodePosition::Absolute {
            top: Some(inset),
            left: Some(inset),
            right: None,
            bottom: None,
        },
    };

    let mut container = container.child(toast_stack(stack_spec, ctx, handlers));
    if !spec.aria_label.is_empty() {
        container.a11y.label = Some(spec.aria_label.clone());
    }
    container.a11y.role = Some(NodeRole::List);
    container
}
