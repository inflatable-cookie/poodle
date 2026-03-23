//! Style mapping from Flint's StyleDescriptor to Jetstream-compatible types.
//!
//! Produces a split output matching Jetstream's `create_node()` API:
//! - `taffy::Style` for layout (flexbox direction, sizing, alignment, padding,
//!   margin, gap, position, overflow, insets, min/max constraints)
//! - `JetstreamVisuals` for non-layout visual properties (background, border,
//!   corner radius, opacity, shadow, text, transforms, focus ring)
//!
//! This eliminates the intermediate `JetstreamStyle` struct that previously
//! mirrored UiStyle — the adapter now speaks Taffy directly.

use glam::Vec4;
use flint_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutIntent, LayoutSizing, MainAxisAlignment,
};
use flint_style::StyleDescriptor;

/// Jetstream-compatible color value (Vec4: r, g, b, a in 0.0..1.0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JetstreamColor(pub f32, pub f32, pub f32, pub f32);

impl Default for JetstreamColor {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<flint_tokens::typed::ColorValue> for JetstreamColor {
    fn from(c: flint_tokens::typed::ColorValue) -> Self {
        Self(c.0, c.1, c.2, c.3)
    }
}

impl JetstreamColor {
    /// Convert to a glam Vec4 for use with Jetstream's NodeStyle.
    pub fn to_vec4(self) -> Vec4 {
        Vec4::new(self.0, self.1, self.2, self.3)
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

/// Jetstream box shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JetstreamBoxShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: JetstreamColor,
}

/// Visual (non-layout) properties for a Jetstream UI node.
///
/// Maps 1:1 to `NodeStyle` fields in `jetstream-runtime`. The integration
/// layer (preview app) converts this to `NodeStyle` with a trivial mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct JetstreamVisuals {
    // Colors
    pub background: Option<JetstreamColor>,
    pub text_color: Option<JetstreamColor>,
    pub icon_color: Option<JetstreamColor>,
    pub text_size: Option<f32>,

    // Border
    pub border_color: Option<JetstreamColor>,
    pub border_width: f32,
    pub corner_radii: [f32; 4],

    // Effects
    pub opacity: f32,
    pub visible: bool,
    pub disabled: bool,
    pub shadow: Option<JetstreamBoxShadow>,
    pub z_index: i32,

    // Focus
    pub focus_ring_color: Option<JetstreamColor>,
    pub focus_ring_width: f32,

    // Transform (applied post-layout)
    pub transform_translate: [f32; 2],
    pub transform_rotate: f32,
    pub transform_scale: [f32; 2],
}

impl Default for JetstreamVisuals {
    fn default() -> Self {
        Self {
            background: None,
            text_color: None,
            icon_color: None,
            text_size: None,
            border_color: None,
            border_width: 0.0,
            corner_radii: [0.0; 4],
            opacity: 1.0,
            visible: true,
            disabled: false,
            shadow: None,
            z_index: 0,
            focus_ring_color: None,
            focus_ring_width: 0.0,
            transform_translate: [0.0, 0.0],
            transform_rotate: 0.0,
            transform_scale: [1.0, 1.0],
        }
    }
}

/// Complete mapped output — a taffy layout style + visual properties.
///
/// Produced by `map_style()` and consumed by the preview app to create
/// `UiNode` instances via `tree.create_node(widget, style.layout, style.visuals.into())`.
#[derive(Debug, Clone)]
pub struct JetstreamMappedStyle {
    pub layout: taffy::Style,
    pub visuals: JetstreamVisuals,
}

impl Default for JetstreamMappedStyle {
    fn default() -> Self {
        Self {
            layout: taffy::Style {
                display: taffy::Display::Flex,
                flex_direction: taffy::FlexDirection::Column,
                ..Default::default()
            },
            visuals: JetstreamVisuals::default(),
        }
    }
}

// ── Layout mapping ──

/// Map a `LayoutIntent` directly to a `taffy::Style`.
///
/// This is the core translation from Flint's layout abstraction to the same
/// flexbox engine that GPUI uses internally. No intermediate enum layer.
pub fn map_layout(intent: &LayoutIntent) -> taffy::Style {
    use taffy::style::{
        AlignItems, Dimension, FlexDirection, JustifyContent, LengthPercentage,
        LengthPercentageAuto,
    };

    let flex_direction = match intent.direction {
        LayoutDirection::Row => FlexDirection::Row,
        LayoutDirection::Column => FlexDirection::Column,
    };

    let (width, min_width, max_width, width_grow, width_shrink) = map_sizing(&intent.width);
    let (height, min_height, max_height, height_grow, height_shrink) = map_sizing(&intent.height);

    // flex_grow applies to the parent's main axis, but we don't know the
    // parent direction at this point. We use flex_grow only when BOTH axes
    // want to grow (Grow×Grow), because:
    //
    // - Fixed×Grow (e.g. sidebar): flex_grow=0 prevents the fixed-width node
    //   from expanding horizontally in a Row parent. align_self:Stretch
    //   handles the Grow axis (cross axis).
    // - Grow×Fixed (e.g. tab bar): flex_grow=0 prevents the fixed-height node
    //   from expanding vertically in a Column parent.
    // - Grow×Fit (e.g. hero header): flex_grow=0 prevents content-hugging
    //   nodes from expanding on the main axis.
    // - Grow×Grow (e.g. content area): flex_grow=1 fills remaining space on
    //   whichever axis is the parent's main axis.
    let flex_grow = if width_grow > 0.0 && height_grow > 0.0 {
        width_grow.max(height_grow)
    } else {
        0.0
    };
    let flex_shrink = if width_shrink == 0.0 || height_shrink == 0.0 {
        0.0
    } else {
        1.0
    };

    // If either axis explicitly wants Grow, set align_self to Stretch
    // so the cross axis fills the parent.
    let align_self = if matches!(intent.width, LayoutSizing::Grow)
        || matches!(intent.height, LayoutSizing::Grow)
    {
        Some(taffy::style::AlignSelf::Stretch)
    } else {
        None
    };

    let justify_content = match intent.alignment.main {
        MainAxisAlignment::Start => Some(JustifyContent::FlexStart),
        MainAxisAlignment::Center => Some(JustifyContent::Center),
        MainAxisAlignment::End => Some(JustifyContent::FlexEnd),
        MainAxisAlignment::SpaceBetween => Some(JustifyContent::SpaceBetween),
    };

    let align_items = match intent.alignment.cross {
        CrossAxisAlignment::Start => Some(AlignItems::FlexStart),
        CrossAxisAlignment::Center => Some(AlignItems::Center),
        CrossAxisAlignment::End => Some(AlignItems::FlexEnd),
        CrossAxisAlignment::Stretch => Some(AlignItems::Stretch),
    };

    let lp = |v: f32| -> LengthPercentage { LengthPercentage::length(v) };

    let map_overflow = |o: flint_layout::LayoutOverflow| -> taffy::Overflow {
        match o {
            flint_layout::LayoutOverflow::Visible => taffy::Overflow::Visible,
            flint_layout::LayoutOverflow::Hidden => taffy::Overflow::Hidden,
            flint_layout::LayoutOverflow::Scroll => taffy::Overflow::Scroll,
        }
    };
    let overflow_x = map_overflow(intent.overflow_x);
    let overflow_y = map_overflow(intent.overflow_y);

    taffy::Style {
        display: taffy::Display::Flex,
        flex_direction,
        flex_grow,
        flex_shrink,
        flex_basis: Dimension::auto(),
        size: taffy::Size { width, height },
        min_size: taffy::Size { width: min_width, height: min_height },
        max_size: taffy::Size { width: max_width, height: max_height },
        align_items,
        align_self,
        justify_content,
        gap: taffy::Size {
            width: lp(intent.spacing.gap),
            height: lp(intent.spacing.gap),
        },
        padding: taffy::Rect {
            left: lp(intent.spacing.padding.left),
            right: lp(intent.spacing.padding.right),
            top: lp(intent.spacing.padding.top),
            bottom: lp(intent.spacing.padding.bottom),
        },
        margin: taffy::Rect {
            left: LengthPercentageAuto::length(intent.spacing.margin.left),
            right: LengthPercentageAuto::length(intent.spacing.margin.right),
            top: LengthPercentageAuto::length(intent.spacing.margin.top),
            bottom: LengthPercentageAuto::length(intent.spacing.margin.bottom),
        },
        overflow: taffy::Point {
            x: overflow_x,
            y: overflow_y,
        },
        ..Default::default()
    }
}

/// Map a `LayoutSizing` to (dimension, min, max, flex_grow, flex_shrink).
fn map_sizing(sizing: &LayoutSizing) -> (taffy::Dimension, taffy::Dimension, taffy::Dimension, f32, f32) {
    use taffy::Dimension;

    match sizing {
        LayoutSizing::Fixed(v) => (
            Dimension::length(*v),
            Dimension::auto(),
            Dimension::auto(),
            0.0,
            0.0, // Fixed items don't shrink
        ),
        LayoutSizing::Grow => (
            Dimension::auto(),
            Dimension::length(0.0), // min=0 allows shrinking below content (critical for scroll)
            Dimension::auto(),
            1.0,
            1.0,
        ),
        LayoutSizing::Fit => (
            Dimension::auto(),
            Dimension::length(0.0), // min=0 allows parent-constrained shrinking
            Dimension::auto(),
            0.0,
            1.0,
        ),
        LayoutSizing::Constrained { min, max } => {
            let min_d = match min {
                Some(v) if *v > 0.0 => Dimension::length(*v),
                _ => Dimension::auto(),
            };
            let max_d = match max {
                Some(v) if *v < f32::INFINITY => Dimension::length(*v),
                _ => Dimension::auto(),
            };
            (
                Dimension::auto(),
                min_d,
                max_d,
                1.0, // Constrained → grow to fill within bounds
                1.0,
            )
        }
    }
}

// ── Full style mapping ──

/// Convert a complete `StyleDescriptor` to a `JetstreamMappedStyle`.
///
/// Produces a `taffy::Style` for layout and `JetstreamVisuals` for visual
/// properties, ready to feed into `UiTree::create_node()`.
pub fn map_style(desc: &StyleDescriptor) -> JetstreamMappedStyle {
    let layout = map_layout(&desc.layout);
    let mut visuals = JetstreamVisuals::default();

    // Colors
    visuals.background = desc.background.map(JetstreamColor::from);
    visuals.text_color = desc.text_color.map(JetstreamColor::from);
    visuals.icon_color = desc.icon_color.map(JetstreamColor::from);

    // Typography
    if let Some(ref typo) = desc.typography {
        visuals.text_size = Some(typo.size);
    }

    // Border
    let border = &desc.border;
    if border.width > 0.0 {
        visuals.border_color = Some(JetstreamColor::from(border.color));
        visuals.border_width = border.width;
    }
    visuals.corner_radii = [
        desc.corner_radii.top_left,
        desc.corner_radii.top_right,
        desc.corner_radii.bottom_right,
        desc.corner_radii.bottom_left,
    ];

    // Shadow
    if let Some(ref shadow) = desc.shadow {
        visuals.shadow = Some(JetstreamBoxShadow {
            offset_x: shadow.offset_x,
            offset_y: shadow.offset_y,
            blur: shadow.blur,
            spread: 0.0,
            color: JetstreamColor::from(shadow.color),
        });
    }

    // Opacity and visibility
    visuals.opacity = desc.opacity;
    visuals.visible = desc.visible;

    // Focus ring
    visuals.focus_ring_color = desc.focus_ring_color.map(JetstreamColor::from);
    visuals.focus_ring_width = desc.focus_ring_width;

    JetstreamMappedStyle { layout, visuals }
}

#[cfg(test)]
mod tests {
    use flint_layout::LayoutEdges;
    use super::*;

    #[test]
    fn map_layout_produces_correct_direction() {
        let intent = LayoutIntent::new().with_direction(LayoutDirection::Row);
        let layout = map_layout(&intent);
        assert_eq!(layout.flex_direction, taffy::FlexDirection::Row);
    }

    #[test]
    fn map_layout_produces_column_by_default() {
        let layout = map_layout(&LayoutIntent::default());
        assert_eq!(layout.flex_direction, taffy::FlexDirection::Column);
    }

    #[test]
    fn map_layout_handles_fixed_sizing() {
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Fixed(200.0))
            .with_height(LayoutSizing::Fixed(100.0));
        let layout = map_layout(&intent);
        assert_eq!(layout.size.width, taffy::Dimension::length(200.0));
        assert_eq!(layout.size.height, taffy::Dimension::length(100.0));
        // Fixed items should not grow or shrink
        assert_eq!(layout.flex_grow, 0.0);
        assert_eq!(layout.flex_shrink, 0.0);
    }

    #[test]
    fn map_layout_handles_grow_sizing() {
        // Grow×Fixed: flex_grow is 0 (only one axis grows — rely on
        // align_self:Stretch for the cross axis).
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Fixed(32.0));
        let layout = map_layout(&intent);
        assert_eq!(layout.size.width, taffy::Dimension::auto());
        assert_eq!(layout.size.height, taffy::Dimension::length(32.0));
        assert_eq!(layout.flex_grow, 0.0);
        assert_eq!(layout.align_self, Some(taffy::style::AlignSelf::Stretch));

        // Grow×Grow: flex_grow is positive — fills both main and cross axes.
        let intent_both = LayoutIntent::default()
            .with_width(LayoutSizing::Grow)
            .with_height(LayoutSizing::Grow);
        let layout_both = map_layout(&intent_both);
        assert!(layout_both.flex_grow > 0.0);
    }

    #[test]
    fn map_layout_handles_constrained_sizing() {
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Constrained { min: Some(100.0), max: Some(300.0) });
        let layout = map_layout(&intent);
        assert_eq!(layout.min_size.width, taffy::Dimension::length(100.0));
        assert_eq!(layout.max_size.width, taffy::Dimension::length(300.0));
        // Constrained×Fit: flex_grow=0 (only one axis grows); min/max constrain.
        assert_eq!(layout.flex_grow, 0.0);
    }

    #[test]
    fn map_layout_maps_alignment() {
        let intent = LayoutIntent::default()
            .with_alignment(MainAxisAlignment::Center, CrossAxisAlignment::Stretch);
        let layout = map_layout(&intent);
        assert_eq!(layout.justify_content, Some(taffy::JustifyContent::Center));
        assert_eq!(layout.align_items, Some(taffy::AlignItems::Stretch));
    }

    #[test]
    fn map_layout_maps_gap() {
        let intent = LayoutIntent::default().with_gap(8.0);
        let layout = map_layout(&intent);
        assert_eq!(layout.gap.width, taffy::LengthPercentage::length(8.0));
    }

    #[test]
    fn map_layout_maps_padding() {
        let intent = LayoutIntent::default()
            .with_padding(LayoutEdges::uniform(4.0));
        let layout = map_layout(&intent);
        assert_eq!(layout.padding.left, taffy::LengthPercentage::length(4.0));
        assert_eq!(layout.padding.top, taffy::LengthPercentage::length(4.0));
    }

    #[test]
    fn map_style_converts_full_descriptor() {
        let desc = StyleDescriptor::new()
            .with_background(flint_tokens::typed::ColorValue(0.1, 0.2, 0.3, 1.0))
            .with_border(2.0, flint_tokens::typed::ColorValue(0.5, 0.5, 0.5, 1.0))
            .with_corner_radii(flint_style::CornerRadii::uniform(4.0))
            .with_opacity(0.8);
        let mapped = map_style(&desc);
        assert_eq!(mapped.visuals.background, Some(JetstreamColor(0.1, 0.2, 0.3, 1.0)));
        assert_eq!(mapped.visuals.border_width, 2.0);
        assert_eq!(mapped.visuals.corner_radii, [4.0; 4]);
        assert!((mapped.visuals.opacity - 0.8).abs() < 0.001);
    }

    #[test]
    fn map_style_converts_shadow() {
        let desc = StyleDescriptor::new()
            .with_shadow(flint_tokens::typed::ShadowValue {
                offset_x: 0.0,
                offset_y: 2.0,
                blur: 4.0,
                color: flint_tokens::typed::ColorValue(0.0, 0.0, 0.0, 0.25),
            });
        let mapped = map_style(&desc);
        assert!(mapped.visuals.shadow.is_some());
        let shadow = mapped.visuals.shadow.unwrap();
        assert_eq!(shadow.offset_y, 2.0);
        assert_eq!(shadow.blur, 4.0);
    }

    #[test]
    fn map_style_converts_typography() {
        let desc = StyleDescriptor::new()
            .with_typography(flint_style::TypographyDescriptor {
                family: flint_style::FontFamily::Sans,
                size: 16.0,
                line_height: 24.0,
                weight: 600,
            });
        let mapped = map_style(&desc);
        assert_eq!(mapped.visuals.text_size, Some(16.0));
    }

    #[test]
    fn default_visuals_are_visible_and_opaque() {
        let v = JetstreamVisuals::default();
        assert!(v.visible);
        assert_eq!(v.opacity, 1.0);
        assert_eq!(v.z_index, 0);
        assert_eq!(v.transform_translate, [0.0, 0.0]);
        assert_eq!(v.transform_scale, [1.0, 1.0]);
        assert_eq!(v.transform_rotate, 0.0);
    }

    #[test]
    fn constrained_sizing_uses_min_max() {
        let intent = LayoutIntent::default()
            .with_width(LayoutSizing::Constrained { min: Some(50.0), max: None });
        let layout = map_layout(&intent);
        assert_eq!(layout.min_size.width, taffy::Dimension::length(50.0));
        assert_eq!(layout.max_size.width, taffy::Dimension::auto());
        // Constrained×Fit: flex_grow=0, relies on stretch + min/max constraints.
        assert_eq!(layout.flex_grow, 0.0);
    }
}
