//! ToastHost — positioned toast container. Wraps a ToastStack and positions
//! it at the chosen screen corner.
//!
//! Ported from: `packages/jetstream/components/src/toast_host.rs`.

use poodle_node::{LayoutDirection, LayoutSizing, Node, NodePosition};
use poodle_specs::{ToastHostPlacement, ToastHostSpec, ToastPosition, ToastStackSpec};

use crate::context::RenderContext;
use crate::presentation::rem_to_px;
use crate::toast_stack::{toast_stack, ToastStackHandlers};

fn placement_role(placement: ToastHostPlacement) -> &'static str {
    match placement {
        ToastHostPlacement::BottomEnd => "bottom-end",
        ToastHostPlacement::BottomStart => "bottom-start",
        ToastHostPlacement::TopEnd => "top-end",
        ToastHostPlacement::TopStart => "top-start",
    }
}

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

    let instance_id = handlers.instance_id.clone();
    let mut container = Node::container();
    container.runtime_id = instance_id
        .as_deref()
        .map(|scope| format!("toast-host:{scope}"));
    container.roles.insert(
        "placement".to_owned(),
        placement_role(spec.placement).to_owned(),
    );
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

    // ToastHost owns placement and presentation forwarding; ToastStack owns
    // the list semantics and visible notification composition.
    let mut forwarded = stack_spec.clone();
    forwarded.position = match spec.placement {
        ToastHostPlacement::BottomEnd => ToastPosition::BottomRight,
        ToastHostPlacement::BottomStart => ToastPosition::BottomLeft,
        ToastHostPlacement::TopEnd => ToastPosition::TopRight,
        ToastHostPlacement::TopStart => ToastPosition::TopLeft,
    };
    forwarded.size = spec.size;
    forwarded.size_role = spec.size_role;
    forwarded.density = spec.density;
    forwarded.aria_label = (!spec.aria_label.is_empty()).then(|| spec.aria_label.clone());

    container.child(toast_stack(&forwarded, ctx, handlers))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ControlDensity, ControlSize, Toast};

    #[test]
    fn host_forwards_placement_presentation_label_and_scope_to_stack() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let ctx = RenderContext::new(&theme);
        let host = ToastHostSpec::new()
            .with_placement(ToastHostPlacement::TopStart)
            .with_aria_label("Job notices")
            .with_size(ControlSize::Lg)
            .with_density(ControlDensity::Comfortable);
        let stack = ToastStackSpec::new()
            .with_toasts(vec![Toast::new("job", "Publishing")])
            .with_position(ToastPosition::BottomRight)
            .with_size(ControlSize::Xs)
            .with_density(ControlDensity::Compact);
        let node = toast_host(
            &host,
            &ctx,
            &stack,
            ToastStackHandlers {
                instance_id: Some("subject".to_owned()),
                ..ToastStackHandlers::default()
            },
        );

        assert_eq!(node.runtime_id.as_deref(), Some("toast-host:subject"));
        assert_eq!(
            node.roles.get("placement").map(String::as_str),
            Some("top-start")
        );
        assert!(matches!(
            node.position,
            NodePosition::Absolute {
                top: Some(_),
                left: Some(_),
                right: None,
                bottom: None,
            }
        ));
        assert_eq!(node.a11y.role, None);

        let stack = node.children.first().expect("forwarded ToastStack");
        assert_eq!(stack.a11y.role, Some(poodle_node::NodeRole::List));
        assert_eq!(stack.a11y.label.as_deref(), Some("Job notices"));
        assert_eq!(stack.roles.get("size").map(String::as_str), Some("lg"));
        assert_eq!(
            stack.roles.get("density").map(String::as_str),
            Some("comfortable")
        );
        assert_eq!(
            stack.roles.get("position").map(String::as_str),
            Some("top-left")
        );
        assert_eq!(
            stack.runtime_id.as_deref(),
            Some("toast-host:subject:stack")
        );
    }
}
