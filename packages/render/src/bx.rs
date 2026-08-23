//! Box — neutral layout container with sizing, padding, and overflow.
//!
//! Contract: `docs/contracts/components/box.md`

use poodle_node::{LayoutDirection, LayoutOverflow, LayoutSizing, Node};
use poodle_specs::{BoxSpec, Dimension, Overflow};

use crate::context::RenderContext;

fn parse_dimension_px(dimension: &Dimension) -> Option<f32> {
    let value = dimension.as_str().trim();
    if let Some(px) = value.strip_suffix("px") {
        px.trim().parse::<f32>().ok()
    } else if let Some(rem) = value.strip_suffix("rem") {
        rem.trim().parse::<f32>().ok().map(|value| value * 16.0)
    } else {
        value.parse::<f32>().ok()
    }
}

pub fn bx(spec: &BoxSpec, ctx: &RenderContext<'_>, children: Vec<Node>) -> Node {
    let theme = ctx.theme();
    let padding = spec.resolved_padding();
    let mut node = Node::container();
    // Preserve the neutral div default used by the existing Rust backends.
    node.style.descriptor.layout.direction = LayoutDirection::Row;

    if let Some(width) = &spec.width {
        if width.as_str().trim() == "100%" {
            node.style.fill_width = true;
        } else if let Some(width) = parse_dimension_px(width) {
            node.style.descriptor.layout.width = LayoutSizing::Fixed(width);
        }
    }
    if let Some(height) = &spec.height {
        if height.as_str().trim() == "100%" {
            node.style.fill_height = true;
        } else if let Some(height) = parse_dimension_px(height) {
            node.style.descriptor.layout.height = LayoutSizing::Fixed(height);
        }
    }
    if let Some(min_width) = spec.min_width.as_ref().and_then(parse_dimension_px) {
        node.style.min_width = Some(min_width);
    }
    if let Some(min_height) = spec.min_height.as_ref().and_then(parse_dimension_px) {
        node.style.min_height = Some(min_height);
    }

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

    let overflow = match spec.overflow {
        Overflow::Visible => LayoutOverflow::Visible,
        Overflow::Hidden | Overflow::Clip => LayoutOverflow::Hidden,
        Overflow::Auto | Overflow::Scroll => LayoutOverflow::Scroll,
    };
    node.style.descriptor.layout.overflow_x = overflow;
    node.style.descriptor.layout.overflow_y = overflow;

    if let Some(label) = spec.aria_label.as_deref().filter(|label| !label.is_empty()) {
        node.a11y.label = Some(label.to_string());
    }
    node.children = children;
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::PaddingScale;

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
