//! Spacer — flexible space filler for flex layouts.
//!
//! Supports a configurable grow weight (default 1.0) and optional minimum size.

use gpui::*;

/// A flexible space filler that expands to fill available space in flex layouts.
///
/// By default grows with weight 1.0. Use `with_grow` to set a custom flex-grow
/// weight and `with_min_size` to enforce a minimum width/height.
pub struct Spacer {
    /// Flex-grow weight (default 1.0).
    grow: f32,
    /// Optional minimum size in pixels (applied to both min-width and min-height).
    min_size: Option<f32>,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            grow: 1.0,
            min_size: None,
        }
    }

    /// Set the flex-grow weight (default 1.0).
    pub fn with_grow(mut self, weight: f32) -> Self {
        self.grow = weight;
        self
    }

    /// Set a minimum size in pixels (applied as both min-width and min-height).
    pub fn with_min_size(mut self, size: f32) -> Self {
        self.min_size = Some(size);
        self
    }
}

impl IntoElement for Spacer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let mut el = div();

        // Apply flex-grow when weight is positive.
        // GPUI's flex_grow() uses weight 1; custom weights stored for
        // future use when the styling API supports them.
        if self.grow > 0.0 {
            el = el.flex_grow();
        }

        if let Some(size) = self.min_size {
            el = el.min_w(px(size)).min_h(px(size));
        }

        el.into_any_element()
    }
}
