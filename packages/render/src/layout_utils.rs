//! Shared lowering for layout-container dimensions and overflow.

use poodle_node::{LayoutOverflow, LayoutSizing, Node};
use poodle_specs::{Dimension, Overflow};

fn dimension_px(dimension: &Dimension) -> Option<f32> {
    let value = dimension.as_str().trim();
    if let Some(px) = value.strip_suffix("px") {
        px.trim().parse::<f32>().ok()
    } else if let Some(rem) = value.strip_suffix("rem") {
        rem.trim().parse::<f32>().ok().map(|value| value * 16.0)
    } else {
        value.parse::<f32>().ok()
    }
}

pub(crate) fn apply_dimensions(
    node: &mut Node,
    width: Option<&Dimension>,
    height: Option<&Dimension>,
    min_width: Option<&Dimension>,
    min_height: Option<&Dimension>,
) {
    if let Some(width) = width {
        if width.as_str().trim() == "100%" {
            node.style.fill_width = true;
        } else if let Some(width) = dimension_px(width) {
            node.style.descriptor.layout.width = LayoutSizing::Fixed(width);
        }
    }
    if let Some(height) = height {
        if height.as_str().trim() == "100%" {
            node.style.fill_height = true;
        } else if let Some(height) = dimension_px(height) {
            node.style.descriptor.layout.height = LayoutSizing::Fixed(height);
        }
    }
    if let Some(min_width) = min_width.and_then(dimension_px) {
        node.style.min_width = Some(min_width);
    }
    if let Some(min_height) = min_height.and_then(dimension_px) {
        node.style.min_height = Some(min_height);
    }
}

pub(crate) fn overflow(value: &Overflow) -> LayoutOverflow {
    match value {
        Overflow::Visible => LayoutOverflow::Visible,
        Overflow::Hidden | Overflow::Clip => LayoutOverflow::Hidden,
        Overflow::Auto | Overflow::Scroll => LayoutOverflow::Scroll,
    }
}
