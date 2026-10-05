//! Box — neutral layout container with sizing, padding, and overflow.
//!
//! Contract: `docs/contracts/components/box.md`

use poodle_node::{LayoutDirection, Node};
use poodle_specs::BoxSpec;

use crate::context::RenderContext;

pub fn bx(spec: &BoxSpec, ctx: &RenderContext<'_>, children: Vec<Node>) -> Node {
    let theme = ctx.theme();
    let padding = spec.resolved_padding();
    let mut node = Node::container();
    // Preserve the neutral div default used by the existing Rust backends.
    node.style.descriptor.layout.direction = LayoutDirection::Row;

    crate::layout_utils::apply_dimensions(
        &mut node,
        spec.width.as_ref(),
        spec.height.as_ref(),
        spec.min_width.as_ref(),
        spec.min_height.as_ref(),
    );

    if let Some(horizontal) = padding.horizontal {
        let value = theme.resolve_space(horizontal);
        node.style.descriptor.layout.spacing.padding.left = value;
        node.style.descriptor.layout.spacing.padding.right = value;
    }
    if let Some(vertical) = padding.vertical {
        let value = theme.resolve_space(vertical);
        node.style.descriptor.layout.spacing.padding.top = value;
        node.style.descriptor.layout.spacing.padding.bottom = value;
    }

    let overflow = crate::layout_utils::overflow(&spec.overflow);
    node.style.descriptor.layout.overflow_x = overflow;
    node.style.descriptor.layout.overflow_y = overflow;

    if let Some(label) = spec.aria_label.as_deref().filter(|label| !label.is_empty()) {
        node.a11y.label = Some(label.to_string());
    }
    node.a11y.role = spec
        .role
        .as_deref()
        .and_then(poodle_node::NodeRole::from_aria_role);
    node.children = children;
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::{LayoutOverflow, LayoutSizing};
    use poodle_specs::{Overflow, PaddingScale};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn resolves_dimensions_padding_overflow_and_children() {
        let spec = BoxSpec::new()
            .with_padding(PaddingScale::Md)
            .with_width("12rem")
            .with_height("96px")
            .with_overflow(Overflow::Hidden);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = bx(&spec, &ctx, vec![Node::text("content")]);

        assert_eq!(
            node.style.descriptor.layout.width,
            LayoutSizing::Fixed(192.0)
        );
        assert_eq!(
            node.style.descriptor.layout.height,
            LayoutSizing::Fixed(96.0)
        );
        assert!(node.style.descriptor.layout.spacing.padding.left > 0.0);
        assert_eq!(
            node.style.descriptor.layout.overflow_x,
            LayoutOverflow::Hidden
        );
        assert_eq!(node.children.len(), 1);
    }
}
