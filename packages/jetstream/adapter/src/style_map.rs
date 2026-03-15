//! Style mapping from Pug's StyleDescriptor to Jetstream-compatible types.
//!
//! Jetstream's UiStyle supports:
//! - Flexbox layout (direction, gap, align, justify)
//! - Fixed and grow sizing
//! - Padding and margin (Edges)
//! - Background color, border color, border width, corner radius
//! - Opacity
//! - Vertical scrolling with clipping
//!
//! Limitations vs GPUI/CSS:
//! - No CSS Grid (emulated with nested row/column panels)
//! - Solid colors only (no gradients)
//! - Single box shadow per node
//! - No transforms (rotation, scale, skew)
//! - No momentum or snap scrolling

use pug_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutIntent, LayoutSizing, MainAxisAlignment,
};
use pug_style::StyleDescriptor;

/// Jetstream-compatible color value (Vec4: r, g, b, a in 0.0..1.0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JetstreamColor(pub f32, pub f32, pub f32, pub f32);

impl Default for JetstreamColor {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<pug_tokens::typed::ColorValue> for JetstreamColor {
    fn from(c: pug_tokens::typed::ColorValue) -> Self {
        Self(c.0, c.1, c.2, c.3)
    }
}

/// Jetstream-compatible edges (top, right, bottom, left in pixels).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct JetstreamEdges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Jetstream flexbox direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JetstreamDirection {
    Row,
    Column,
}

/// Jetstream sizing mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JetstreamSizing {
    Fixed(f32),
    Grow,
    Fit,
}

/// Jetstream main axis alignment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JetstreamJustify {
    Start,
    Center,
    End,
    SpaceBetween,
}

/// Jetstream cross axis alignment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JetstreamAlign {
    Start,
    Center,
    End,
    Stretch,
}

/// Complete Jetstream style — maps to UiStyle fields.
#[derive(Debug, Clone, PartialEq)]
pub struct JetstreamStyle {
    // Layout
    pub direction: JetstreamDirection,
    pub width: JetstreamSizing,
    pub height: JetstreamSizing,
    pub justify: JetstreamJustify,
    pub align: JetstreamAlign,
    pub gap: f32,
    pub padding: JetstreamEdges,
    pub margin: JetstreamEdges,

    // Visual
    pub background: Option<JetstreamColor>,
    pub text_color: Option<JetstreamColor>,
    pub icon_color: Option<JetstreamColor>,
    pub border_color: Option<JetstreamColor>,
    pub border_width: f32,
    pub corner_radius: f32,
    pub opacity: f32,
    pub visible: bool,
    pub clip_overflow: bool,

    // Focus
    pub focus_ring_color: Option<JetstreamColor>,
    pub focus_ring_width: f32,
}

impl Default for JetstreamStyle {
    fn default() -> Self {
        Self {
            direction: JetstreamDirection::Column,
            width: JetstreamSizing::Fit,
            height: JetstreamSizing::Fit,
            justify: JetstreamJustify::Start,
            align: JetstreamAlign::Start,
            gap: 0.0,
            padding: JetstreamEdges::default(),
            margin: JetstreamEdges::default(),
            background: None,
            text_color: None,
            icon_color: None,
            border_color: None,
            border_width: 0.0,
            corner_radius: 0.0,
            opacity: 1.0,
            visible: true,
            clip_overflow: false,
            focus_ring_color: None,
            focus_ring_width: 0.0,
        }
    }
}

/// Map a `LayoutIntent` to Jetstream layout properties.
pub fn map_layout(intent: &LayoutIntent) -> JetstreamStyle {
    let mut style = JetstreamStyle::default();

    style.direction = match intent.direction {
        LayoutDirection::Row => JetstreamDirection::Row,
        LayoutDirection::Column => JetstreamDirection::Column,
    };

    style.width = map_sizing(&intent.width);
    style.height = map_sizing(&intent.height);

    style.justify = match intent.alignment.main {
        MainAxisAlignment::Start => JetstreamJustify::Start,
        MainAxisAlignment::Center => JetstreamJustify::Center,
        MainAxisAlignment::End => JetstreamJustify::End,
        MainAxisAlignment::SpaceBetween => JetstreamJustify::SpaceBetween,
    };

    style.align = match intent.alignment.cross {
        CrossAxisAlignment::Start => JetstreamAlign::Start,
        CrossAxisAlignment::Center => JetstreamAlign::Center,
        CrossAxisAlignment::End => JetstreamAlign::End,
        CrossAxisAlignment::Stretch => JetstreamAlign::Stretch,
    };

    style.gap = intent.spacing.gap;
    style.padding = JetstreamEdges {
        top: intent.spacing.padding.top,
        right: intent.spacing.padding.right,
        bottom: intent.spacing.padding.bottom,
        left: intent.spacing.padding.left,
    };

    style
}

fn map_sizing(sizing: &LayoutSizing) -> JetstreamSizing {
    match sizing {
        LayoutSizing::Fixed(v) => JetstreamSizing::Fixed(*v),
        LayoutSizing::Grow => JetstreamSizing::Grow,
        LayoutSizing::Fit => JetstreamSizing::Fit,
        LayoutSizing::Constrained { min, max } => {
            // Jetstream doesn't support min/max constraints natively;
            // use the midpoint as a fixed approximation
            let lo = min.unwrap_or(0.0);
            let hi = max.unwrap_or(lo);
            let mid = (lo + hi) / 2.0;
            JetstreamSizing::Fixed(mid)
        }
    }
}

/// Convert a complete `StyleDescriptor` to a `JetstreamStyle`.
pub fn map_style(desc: &StyleDescriptor) -> JetstreamStyle {
    let mut style = map_layout(&desc.layout);

    // Colors
    style.background = desc.background.map(JetstreamColor::from);
    style.text_color = desc.text_color.map(JetstreamColor::from);
    style.icon_color = desc.icon_color.map(JetstreamColor::from);

    // Border
    let border = &desc.border;
    if border.width > 0.0 {
        style.border_color = Some(JetstreamColor::from(border.color));
        style.border_width = border.width;
    }
    style.corner_radius = desc.corner_radii.top_left;

    // Opacity and visibility
    style.opacity = desc.opacity;
    style.visible = desc.visible;

    // Focus ring
    style.focus_ring_color = desc.focus_ring_color.map(JetstreamColor::from);
    style.focus_ring_width = desc.focus_ring_width;

    style
}

#[cfg(test)]
mod tests {
    use pug_layout::LayoutEdges;
    use super::*;

    #[test]
    fn map_layout_produces_correct_direction() {
        let intent = LayoutIntent::new().with_direction(LayoutDirection::Row);
        let style = map_layout(&intent);
        assert_eq!(style.direction, JetstreamDirection::Row);
    }

    #[test]
    fn map_layout_handles_fixed_sizing() {
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Fixed(200.0))
            .with_height(LayoutSizing::Fixed(100.0));
        let style = map_layout(&intent);
        assert_eq!(style.width, JetstreamSizing::Fixed(200.0));
        assert_eq!(style.height, JetstreamSizing::Fixed(100.0));
    }

    #[test]
    fn map_layout_handles_constrained_sizing() {
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Constrained { min: Some(100.0), max: Some(300.0) });
        let style = map_layout(&intent);
        assert_eq!(style.width, JetstreamSizing::Fixed(200.0));
    }

    #[test]
    fn map_layout_maps_alignment() {
        let intent = LayoutIntent::default()
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Stretch);
        let style = map_layout(&intent);
        assert_eq!(style.justify, JetstreamJustify::Center);
        assert_eq!(style.align, JetstreamAlign::Stretch);
    }

    #[test]
    fn map_layout_maps_spacing() {
        let intent = LayoutIntent::default()
            .with_gap(8.0)
            .with_padding(LayoutEdges::uniform(4.0));
        let style = map_layout(&intent);
        assert_eq!(style.gap, 8.0);
        assert_eq!(style.padding.top, 4.0);
    }

    #[test]
    fn map_style_converts_full_descriptor() {
        let desc = StyleDescriptor::new()
            .with_background(pug_tokens::typed::ColorValue(0.1, 0.2, 0.3, 1.0))
            .with_border(2.0, pug_tokens::typed::ColorValue(0.5, 0.5, 0.5, 1.0))
            .with_corner_radii(pug_style::CornerRadii::uniform(4.0))
            .with_opacity(0.8);
        let style = map_style(&desc);
        assert_eq!(style.background, Some(JetstreamColor(0.1, 0.2, 0.3, 1.0)));
        assert_eq!(style.border_width, 2.0);
        assert_eq!(style.corner_radius, 4.0);
        assert!((style.opacity - 0.8).abs() < 0.001);
    }

    #[test]
    fn default_style_is_visible_and_opaque() {
        let style = JetstreamStyle::default();
        assert!(style.visible);
        assert_eq!(style.opacity, 1.0);
    }
}
