//! Renderer-agnostic layout intent types for Pug.
//!
//! These types describe layout intent without committing to a specific rendering
//! target. Both GPUI (CSS-like flexbox) and Jetstream (`UiStyle` with `Direction`,
//! `Sizing`, `Align`, `Justify`, `Edges`) can map losslessly from these types.

/// Primary axis for content flow.
///
/// Maps to:
/// - GPUI: `FlexDirection::Row` / `FlexDirection::Column`
/// - Jetstream: `Direction::Horizontal` / `Direction::Vertical`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutDirection {
    Row,
    Column,
}

impl Default for LayoutDirection {
    fn default() -> Self {
        Self::Column
    }
}

/// How a child element sizes along a given axis.
///
/// Maps to:
/// - GPUI: `Length::Auto`, `Length::Definite`, `flex_grow`/`flex_shrink`
/// - Jetstream: `Sizing::Fixed`, `Sizing::Grow`, `Sizing::FitContent`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutSizing {
    /// Size to fit content (intrinsic size).
    Fit,
    /// Fill available space (flex-grow equivalent).
    Grow,
    /// Fixed pixel size.
    Fixed(f32),
    /// Fill available space with min/max constraints.
    Constrained {
        min: Option<f32>,
        max: Option<f32>,
    },
}

impl Default for LayoutSizing {
    fn default() -> Self {
        Self::Fit
    }
}

/// Edge insets (padding, margin) in pixels.
///
/// All values are resolved `f32` pixels (not token references).
/// Maps directly to both GPUI `Edges` and Jetstream `Edges`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutEdges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl LayoutEdges {
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    pub const fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

/// Spacing between children and edge insets.
///
/// Holds resolved pixel values for gap, padding, and margin.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutSpacing {
    /// Gap between children along the layout direction.
    pub gap: f32,
    /// Padding inside the element boundary.
    pub padding: LayoutEdges,
    /// Margin outside the element boundary.
    pub margin: LayoutEdges,
}

/// Alignment along the main axis (justify-content equivalent).
///
/// Maps to:
/// - GPUI: `JustifyContent::*`
/// - Jetstream: `Justify::*`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}

/// Alignment along the cross axis (align-items equivalent).
///
/// Maps to:
/// - GPUI: `AlignItems::*`
/// - Jetstream: `Align::*`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        Self::Stretch
    }
}

/// Combined alignment for both axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LayoutAlignment {
    pub main: MainAxisAlignment,
    pub cross: CrossAxisAlignment,
}

/// Overflow behavior for content that exceeds the element boundary.
///
/// Maps to:
/// - GPUI: `Overflow::Visible` / `Overflow::Hidden` / `Overflow::Scroll`
/// - Jetstream: `Direction::Vertical` scroll on `ScrollView` widget
///   (Jetstream only supports vertical scroll)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutOverflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for LayoutOverflow {
    fn default() -> Self {
        Self::Visible
    }
}

/// Complete layout intent for a single element.
///
/// Captures all layout properties in renderer-agnostic terms.
/// Renderers map this to their native layout types:
///
/// - **GPUI**: `Style { display: Flex, flex_direction, gap, padding, ... }`
/// - **Jetstream**: `UiStyle { direction, sizing_x, sizing_y, gap, padding, ... }`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutIntent {
    pub direction: LayoutDirection,
    pub width: LayoutSizing,
    pub height: LayoutSizing,
    pub spacing: LayoutSpacing,
    pub alignment: LayoutAlignment,
    pub overflow_x: LayoutOverflow,
    pub overflow_y: LayoutOverflow,
}

impl LayoutIntent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_width(mut self, width: LayoutSizing) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: LayoutSizing) -> Self {
        self.height = height;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.spacing.gap = gap;
        self
    }

    pub fn with_padding(mut self, padding: LayoutEdges) -> Self {
        self.spacing.padding = padding;
        self
    }

    pub fn with_margin(mut self, margin: LayoutEdges) -> Self {
        self.spacing.margin = margin;
        self
    }

    pub fn with_alignment(mut self, main: MainAxisAlignment, cross: CrossAxisAlignment) -> Self {
        self.alignment = LayoutAlignment { main, cross };
        self
    }

    pub fn with_overflow(mut self, x: LayoutOverflow, y: LayoutOverflow) -> Self {
        self.overflow_x = x;
        self.overflow_y = y;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_layout_is_column_fit_visible() {
        let layout = LayoutIntent::default();
        assert_eq!(layout.direction, LayoutDirection::Column);
        assert_eq!(layout.width, LayoutSizing::Fit);
        assert_eq!(layout.height, LayoutSizing::Fit);
        assert_eq!(layout.overflow_x, LayoutOverflow::Visible);
        assert_eq!(layout.overflow_y, LayoutOverflow::Visible);
    }

    #[test]
    fn builder_composes_intent() {
        let layout = LayoutIntent::new()
            .with_direction(LayoutDirection::Row)
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(48.0))
            .with_gap(8.0)
            .with_padding(LayoutEdges::symmetric(12.0, 8.0))
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center)
            .with_overflow(LayoutOverflow::Hidden, LayoutOverflow::Scroll);

        assert_eq!(layout.direction, LayoutDirection::Row);
        assert_eq!(layout.width, LayoutSizing::Grow);
        assert_eq!(layout.height, LayoutSizing::Fixed(48.0));
        assert_eq!(layout.spacing.gap, 8.0);
        assert_eq!(layout.spacing.padding.left, 12.0);
        assert_eq!(layout.spacing.padding.top, 8.0);
        assert_eq!(layout.alignment.main, MainAxisAlignment::SpaceBetween);
        assert_eq!(layout.alignment.cross, CrossAxisAlignment::Center);
        assert_eq!(layout.overflow_x, LayoutOverflow::Hidden);
        assert_eq!(layout.overflow_y, LayoutOverflow::Scroll);
    }

    #[test]
    fn edges_constructors_work() {
        let uniform = LayoutEdges::uniform(16.0);
        assert_eq!(uniform.top, 16.0);
        assert_eq!(uniform.right, 16.0);

        let symmetric = LayoutEdges::symmetric(12.0, 8.0);
        assert_eq!(symmetric.left, 12.0);
        assert_eq!(symmetric.top, 8.0);
    }

    #[test]
    fn constrained_sizing_holds_bounds() {
        let sizing = LayoutSizing::Constrained {
            min: Some(200.0),
            max: Some(600.0),
        };
        match sizing {
            LayoutSizing::Constrained { min, max } => {
                assert_eq!(min, Some(200.0));
                assert_eq!(max, Some(600.0));
            }
            _ => panic!("expected Constrained"),
        }
    }
}
