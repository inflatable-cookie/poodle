//! Style mapping from Pug's StyleDescriptor to Jetstream-compatible types.
//!
//! Jetstream's UiStyle supports:
//! - Flexbox layout (direction, gap, align, justify)
//! - Fixed, grow, and constrained sizing (min/max)
//! - Padding and margin (Edges)
//! - Background color, border color, border width, corner radius
//! - Opacity, visibility, disabled state
//! - Vertical and horizontal scrolling with clipping
//! - Box shadows (single)
//! - 2D transforms (translate, rotate, scale) — applied post-layout
//! - Z-index stacking
//! - Absolute positioning with insets
//!
//! Limitations vs GPUI/CSS:
//! - No CSS Grid (emulated with nested row/column panels)
//! - Solid colors only (gradients supported but limited to 2 stops)
//! - Single box shadow per node
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

/// Jetstream positioning mode.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum JetstreamPosition {
    /// Normal flow (participates in flexbox layout).
    #[default]
    Relative,
    /// Removed from flow; positioned via inset fields.
    Absolute,
}

/// Jetstream overflow behavior.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum JetstreamOverflow {
    /// Children are not clipped.
    #[default]
    Visible,
    /// Children are clipped to the container's bounds.
    Hidden,
    /// Children are clipped and the container scrolls.
    Scroll,
}

/// Jetstream box shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JetstreamBoxShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: JetstreamColor,
}

/// Complete Jetstream style — maps to UiStyle fields.
#[derive(Debug, Clone, PartialEq)]
pub struct JetstreamStyle {
    // Layout
    pub direction: JetstreamDirection,
    pub width: JetstreamSizing,
    pub height: JetstreamSizing,
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub justify: JetstreamJustify,
    pub align: JetstreamAlign,
    pub gap: f32,
    pub padding: JetstreamEdges,
    pub margin: JetstreamEdges,

    // Positioning
    pub position: JetstreamPosition,
    pub inset_top: Option<f32>,
    pub inset_right: Option<f32>,
    pub inset_bottom: Option<f32>,
    pub inset_left: Option<f32>,
    pub z_index: i32,

    // Visual
    pub background: Option<JetstreamColor>,
    pub text_color: Option<JetstreamColor>,
    pub icon_color: Option<JetstreamColor>,
    pub text_size: Option<f32>,
    pub border_color: Option<JetstreamColor>,
    pub border_width: f32,
    pub corner_radius: f32,
    pub opacity: f32,
    pub visible: bool,
    pub disabled: bool,
    pub clip_overflow: bool,
    pub overflow: JetstreamOverflow,

    // Shadow
    pub shadow: Option<JetstreamBoxShadow>,

    // Focus
    pub focus_ring_color: Option<JetstreamColor>,
    pub focus_ring_width: f32,

    // Transform (applied post-layout, does not affect flexbox)
    pub transform_translate: [f32; 2],
    pub transform_rotate: f32,
    pub transform_scale: [f32; 2],
}

impl Default for JetstreamStyle {
    fn default() -> Self {
        Self {
            direction: JetstreamDirection::Column,
            width: JetstreamSizing::Fit,
            height: JetstreamSizing::Fit,
            min_width: 0.0,
            min_height: 0.0,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            justify: JetstreamJustify::Start,
            align: JetstreamAlign::Start,
            gap: 0.0,
            padding: JetstreamEdges::default(),
            margin: JetstreamEdges::default(),
            position: JetstreamPosition::Relative,
            inset_top: None,
            inset_right: None,
            inset_bottom: None,
            inset_left: None,
            z_index: 0,
            background: None,
            text_color: None,
            icon_color: None,
            text_size: None,
            border_color: None,
            border_width: 0.0,
            corner_radius: 0.0,
            opacity: 1.0,
            visible: true,
            disabled: false,
            clip_overflow: false,
            overflow: JetstreamOverflow::Visible,
            shadow: None,
            focus_ring_color: None,
            focus_ring_width: 0.0,
            transform_translate: [0.0, 0.0],
            transform_rotate: 0.0,
            transform_scale: [1.0, 1.0],
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

    let (sizing, min_w, max_w) = map_sizing(&intent.width);
    style.width = sizing;
    style.min_width = min_w;
    style.max_width = max_w;

    let (sizing, min_h, max_h) = map_sizing(&intent.height);
    style.height = sizing;
    style.min_height = min_h;
    style.max_height = max_h;

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

/// Map a `LayoutSizing` to a sizing mode plus min/max constraints.
fn map_sizing(sizing: &LayoutSizing) -> (JetstreamSizing, f32, f32) {
    match sizing {
        LayoutSizing::Fixed(v) => (JetstreamSizing::Fixed(*v), 0.0, f32::INFINITY),
        LayoutSizing::Grow => (JetstreamSizing::Grow, 0.0, f32::INFINITY),
        LayoutSizing::Fit => (JetstreamSizing::Fit, 0.0, f32::INFINITY),
        LayoutSizing::Constrained { min, max } => {
            let lo = min.unwrap_or(0.0);
            let hi = max.unwrap_or(f32::INFINITY);
            // Use Grow within the constraints, allowing the layout engine to
            // apply min/max clamping. This replaces the previous midpoint hack.
            (JetstreamSizing::Grow, lo, hi)
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

    // Typography
    if let Some(ref typo) = desc.typography {
        style.text_size = Some(typo.size);
    }

    // Border
    let border = &desc.border;
    if border.width > 0.0 {
        style.border_color = Some(JetstreamColor::from(border.color));
        style.border_width = border.width;
    }
    style.corner_radius = desc.corner_radii.top_left;

    // Shadow
    if let Some(ref shadow) = desc.shadow {
        style.shadow = Some(JetstreamBoxShadow {
            offset_x: shadow.offset_x,
            offset_y: shadow.offset_y,
            blur: shadow.blur,
            color: JetstreamColor::from(shadow.color),
        });
    }

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
        assert_eq!(style.width, JetstreamSizing::Grow);
        assert_eq!(style.min_width, 100.0);
        assert_eq!(style.max_width, 300.0);
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
    fn map_style_converts_shadow() {
        let desc = StyleDescriptor::new()
            .with_shadow(pug_tokens::typed::ShadowValue {
                offset_x: 0.0,
                offset_y: 2.0,
                blur: 4.0,
                color: pug_tokens::typed::ColorValue(0.0, 0.0, 0.0, 0.25),
            });
        let style = map_style(&desc);
        assert!(style.shadow.is_some());
        let shadow = style.shadow.unwrap();
        assert_eq!(shadow.offset_y, 2.0);
        assert_eq!(shadow.blur, 4.0);
    }

    #[test]
    fn map_style_converts_typography() {
        let desc = StyleDescriptor::new()
            .with_typography(pug_style::TypographyDescriptor {
                family: pug_style::FontFamily::Sans,
                size: 16.0,
                line_height: 24.0,
                weight: 600,
            });
        let style = map_style(&desc);
        assert_eq!(style.text_size, Some(16.0));
    }

    #[test]
    fn default_style_is_visible_and_opaque() {
        let style = JetstreamStyle::default();
        assert!(style.visible);
        assert_eq!(style.opacity, 1.0);
        assert_eq!(style.z_index, 0);
        assert_eq!(style.position, JetstreamPosition::Relative);
        assert_eq!(style.transform_translate, [0.0, 0.0]);
        assert_eq!(style.transform_scale, [1.0, 1.0]);
        assert_eq!(style.transform_rotate, 0.0);
        assert_eq!(style.min_width, 0.0);
        assert_eq!(style.max_width, f32::INFINITY);
    }

    #[test]
    fn constrained_sizing_uses_grow_with_bounds() {
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Constrained { min: Some(50.0), max: None });
        let style = map_layout(&intent);
        assert_eq!(style.width, JetstreamSizing::Grow);
        assert_eq!(style.min_width, 50.0);
        assert_eq!(style.max_width, f32::INFINITY);
    }
}
