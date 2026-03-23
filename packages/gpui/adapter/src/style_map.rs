//! Style descriptor → GPUI style mapping.
//!
//! Converts the renderer-agnostic `StyleDescriptor` and layout intent types
//! into GPUI-compatible intermediate representations. In a full GPUI integration,
//! these types would map directly to `gpui::Style`, `gpui::Edges`, etc.
//!
//! For now they are standalone structs that mirror GPUI's styling API, allowing
//! the mapping logic to be tested and compiled without a GPUI runtime dependency.

use poodle_layout::{
    CrossAxisAlignment, LayoutDirection, LayoutEdges, LayoutIntent, LayoutOverflow,
    LayoutSizing, MainAxisAlignment,
};
use poodle_style::{
    BorderDescriptor, CornerRadii, CursorHint, FontFamily, StyleDescriptor,
    TypographyDescriptor,
};
use poodle_tokens::typed::{ColorValue, ShadowValue};

/// GPUI-compatible RGBA color (mirrors `gpui::Rgba` / `gpui::Hsla`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpuiColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<ColorValue> for GpuiColor {
    fn from(c: ColorValue) -> Self {
        Self {
            r: c.0,
            g: c.1,
            b: c.2,
            a: c.3,
        }
    }
}

/// GPUI-compatible edge insets (mirrors `gpui::Edges`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GpuiEdges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl From<LayoutEdges> for GpuiEdges {
    fn from(e: LayoutEdges) -> Self {
        Self {
            top: e.top,
            right: e.right,
            bottom: e.bottom,
            left: e.left,
        }
    }
}

/// GPUI-compatible corner radii (mirrors `gpui::Corners`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GpuiCornerRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl From<CornerRadii> for GpuiCornerRadii {
    fn from(c: CornerRadii) -> Self {
        Self {
            top_left: c.top_left,
            top_right: c.top_right,
            bottom_right: c.bottom_right,
            bottom_left: c.bottom_left,
        }
    }
}

/// GPUI-compatible box shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpuiShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: GpuiColor,
}

impl From<ShadowValue> for GpuiShadow {
    fn from(s: ShadowValue) -> Self {
        Self {
            offset_x: s.offset_x,
            offset_y: s.offset_y,
            blur: s.blur,
            color: GpuiColor::from(s.color),
        }
    }
}

/// GPUI-compatible typography descriptor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpuiTypography {
    pub font_family: GpuiFontFamily,
    pub font_size: f32,
    pub line_height: f32,
    pub font_weight: u16,
}

/// GPUI font family category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuiFontFamily {
    Sans,
    Mono,
}

impl From<FontFamily> for GpuiFontFamily {
    fn from(f: FontFamily) -> Self {
        match f {
            FontFamily::Sans => Self::Sans,
            FontFamily::Mono => Self::Mono,
        }
    }
}

/// GPUI flex direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuiFlexDirection {
    Row,
    Column,
}

/// GPUI justify-content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuiJustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
}

/// GPUI align-items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuiAlignItems {
    Start,
    Center,
    End,
    Stretch,
}

/// GPUI overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuiOverflow {
    Visible,
    Hidden,
    Scroll,
}

/// GPUI length (for width/height).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuiLength {
    Auto,
    Definite(f32),
}

/// GPUI cursor style (mirrors `gpui::CursorStyle`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuiCursorStyle {
    Arrow,
    PointingHand,
    IBeam,
    NotAllowed,
    OpenHand,
    ClosedHand,
    ResizeColumn,
    ResizeRow,
}

/// Complete GPUI-compatible style (mirrors `gpui::Style` subset).
#[derive(Debug, Clone, PartialEq)]
pub struct GpuiStyle {
    // Layout
    pub flex_direction: GpuiFlexDirection,
    pub width: GpuiLength,
    pub height: GpuiLength,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub gap: f32,
    pub row_gap: f32,
    pub padding: GpuiEdges,
    pub margin: GpuiEdges,
    pub justify_content: GpuiJustifyContent,
    pub align_items: GpuiAlignItems,
    pub overflow_x: GpuiOverflow,
    pub overflow_y: GpuiOverflow,

    // Visual
    pub background: Option<GpuiColor>,
    pub text_color: Option<GpuiColor>,
    pub border_width: f32,
    pub border_color: Option<GpuiColor>,
    pub corner_radii: GpuiCornerRadii,
    pub shadow: Option<GpuiShadow>,
    pub opacity: f32,
    pub visible: bool,
    pub cursor: GpuiCursorStyle,

    // Typography
    pub typography: Option<GpuiTypography>,

    // Focus ring
    pub focus_ring_color: Option<GpuiColor>,
    pub focus_ring_width: f32,
}

/// Map a `LayoutIntent` to GPUI layout properties in a `GpuiStyle`.
pub fn map_layout(layout: &LayoutIntent) -> GpuiStyle {
    let flex_direction = match layout.direction {
        LayoutDirection::Row => GpuiFlexDirection::Row,
        LayoutDirection::Column => GpuiFlexDirection::Column,
    };

    let (width, flex_grow_w, flex_shrink_w, min_w, max_w) = map_sizing(&layout.width);
    let (height, _, _, min_h, max_h) = map_sizing(&layout.height);

    GpuiStyle {
        flex_direction,
        width,
        height,
        min_width: min_w,
        max_width: max_w,
        min_height: min_h,
        max_height: max_h,
        flex_grow: flex_grow_w,
        flex_shrink: flex_shrink_w,
        gap: layout.spacing.gap,
        row_gap: layout.spacing.gap,
        padding: GpuiEdges::from(layout.spacing.padding),
        margin: GpuiEdges::from(layout.spacing.margin),
        justify_content: map_justify(layout.alignment.main),
        align_items: map_align(layout.alignment.cross),
        overflow_x: map_overflow(layout.overflow_x),
        overflow_y: map_overflow(layout.overflow_y),

        // Visual defaults — overridden by map_style_descriptor
        background: None,
        text_color: None,
        border_width: 0.0,
        border_color: None,
        corner_radii: GpuiCornerRadii::default(),
        shadow: None,
        opacity: 1.0,
        visible: true,
        cursor: GpuiCursorStyle::Arrow,
        typography: None,
        focus_ring_color: None,
        focus_ring_width: 0.0,
    }
}

fn map_sizing(sizing: &LayoutSizing) -> (GpuiLength, f32, f32, Option<f32>, Option<f32>) {
    match sizing {
        LayoutSizing::Fit => (GpuiLength::Auto, 0.0, 1.0, None, None),
        LayoutSizing::Grow => (GpuiLength::Auto, 1.0, 1.0, None, None),
        LayoutSizing::Fixed(px) => (GpuiLength::Definite(*px), 0.0, 0.0, None, None),
        LayoutSizing::Constrained { min, max } => {
            (GpuiLength::Auto, 1.0, 1.0, *min, *max)
        }
    }
}

fn map_justify(main: MainAxisAlignment) -> GpuiJustifyContent {
    match main {
        MainAxisAlignment::Start => GpuiJustifyContent::Start,
        MainAxisAlignment::Center => GpuiJustifyContent::Center,
        MainAxisAlignment::End => GpuiJustifyContent::End,
        MainAxisAlignment::SpaceBetween => GpuiJustifyContent::SpaceBetween,
    }
}

fn map_align(cross: CrossAxisAlignment) -> GpuiAlignItems {
    match cross {
        CrossAxisAlignment::Start => GpuiAlignItems::Start,
        CrossAxisAlignment::Center => GpuiAlignItems::Center,
        CrossAxisAlignment::End => GpuiAlignItems::End,
        CrossAxisAlignment::Stretch => GpuiAlignItems::Stretch,
    }
}

fn map_overflow(overflow: LayoutOverflow) -> GpuiOverflow {
    match overflow {
        LayoutOverflow::Visible => GpuiOverflow::Visible,
        LayoutOverflow::Hidden => GpuiOverflow::Hidden,
        LayoutOverflow::Scroll => GpuiOverflow::Scroll,
    }
}

/// Map a `BorderDescriptor` to GPUI border properties.
pub fn map_border(border: &BorderDescriptor) -> (f32, GpuiColor) {
    (border.width, GpuiColor::from(border.color))
}

/// Map `CornerRadii` to GPUI corner radii.
pub fn map_corner_radii(radii: &CornerRadii) -> GpuiCornerRadii {
    GpuiCornerRadii::from(*radii)
}

/// Map a `ShadowValue` to a GPUI shadow.
pub fn map_shadow(shadow: &ShadowValue) -> GpuiShadow {
    GpuiShadow::from(*shadow)
}

/// Map a `TypographyDescriptor` to GPUI typography.
pub fn map_typography(typo: &TypographyDescriptor) -> GpuiTypography {
    GpuiTypography {
        font_family: GpuiFontFamily::from(typo.family),
        font_size: typo.size,
        line_height: typo.line_height,
        font_weight: typo.weight,
    }
}

/// Map a `CursorHint` to GPUI cursor style.
pub fn map_cursor(hint: &CursorHint) -> GpuiCursorStyle {
    match hint {
        CursorHint::Default => GpuiCursorStyle::Arrow,
        CursorHint::Pointer => GpuiCursorStyle::PointingHand,
        CursorHint::Text => GpuiCursorStyle::IBeam,
        CursorHint::NotAllowed => GpuiCursorStyle::NotAllowed,
        CursorHint::Grab => GpuiCursorStyle::OpenHand,
        CursorHint::Grabbing => GpuiCursorStyle::ClosedHand,
        CursorHint::ColResize => GpuiCursorStyle::ResizeColumn,
        CursorHint::RowResize => GpuiCursorStyle::ResizeRow,
    }
}

/// Map a `LayoutEdges` to GPUI edges.
pub fn map_edges(edges: &LayoutEdges) -> GpuiEdges {
    GpuiEdges::from(*edges)
}

/// Convert a complete `StyleDescriptor` into a `GpuiStyle`.
///
/// This is the main entry point for style conversion. It takes the
/// renderer-agnostic style descriptor (produced by spec resolution)
/// and produces a GPUI-native style struct.
pub fn map_style(desc: &StyleDescriptor) -> GpuiStyle {
    let mut style = map_layout(&desc.layout);

    // Colors
    style.background = desc.background.map(GpuiColor::from);
    style.text_color = desc.text_color.map(GpuiColor::from);

    // Border
    style.border_width = desc.border.width;
    style.border_color = if desc.border.width > 0.0 {
        Some(GpuiColor::from(desc.border.color))
    } else {
        None
    };

    // Corner radii
    style.corner_radii = GpuiCornerRadii::from(desc.corner_radii);

    // Shadow
    style.shadow = desc.shadow.map(GpuiShadow::from);

    // Typography
    style.typography = desc.typography.map(|t| map_typography(&t));

    // Opacity and visibility
    style.opacity = desc.opacity;
    style.visible = desc.visible;

    // Cursor
    style.cursor = map_cursor(&desc.cursor);

    // Focus ring
    style.focus_ring_color = desc.focus_ring_color.map(GpuiColor::from);
    style.focus_ring_width = desc.focus_ring_width;

    style
}

#[cfg(test)]
mod tests {
    use poodle_layout::{LayoutDirection, LayoutEdges, LayoutIntent, LayoutSizing, MainAxisAlignment, CrossAxisAlignment};
    use poodle_style::{CursorHint, StyleDescriptor, TypographyDescriptor, FontFamily, CornerRadii};
    use poodle_tokens::typed::ColorValue;

    use super::*;

    #[test]
    fn map_layout_produces_correct_flex_direction() {
        let layout = LayoutIntent::new().with_direction(LayoutDirection::Row);
        let style = map_layout(&layout);
        assert_eq!(style.flex_direction, GpuiFlexDirection::Row);
    }

    #[test]
    fn map_layout_handles_fixed_sizing() {
        let layout = LayoutIntent::new()
            .with_width(LayoutSizing::Fixed(200.0))
            .with_height(LayoutSizing::Fixed(48.0));
        let style = map_layout(&layout);
        assert_eq!(style.width, GpuiLength::Definite(200.0));
        assert_eq!(style.height, GpuiLength::Definite(48.0));
        assert_eq!(style.flex_grow, 0.0);
    }

    #[test]
    fn map_layout_handles_grow_sizing() {
        let layout = LayoutIntent::new().with_width(LayoutSizing::Grow);
        let style = map_layout(&layout);
        assert_eq!(style.width, GpuiLength::Auto);
        assert_eq!(style.flex_grow, 1.0);
    }

    #[test]
    fn map_layout_handles_constrained_sizing() {
        let layout = LayoutIntent::new().with_width(LayoutSizing::Constrained {
            min: Some(100.0),
            max: Some(400.0),
        });
        let style = map_layout(&layout);
        assert_eq!(style.min_width, Some(100.0));
        assert_eq!(style.max_width, Some(400.0));
    }

    #[test]
    fn map_layout_maps_alignment() {
        let layout = LayoutIntent::new()
            .with_alignment(MainAxisAlignment::SpaceBetween, CrossAxisAlignment::Center);
        let style = map_layout(&layout);
        assert_eq!(style.justify_content, GpuiJustifyContent::SpaceBetween);
        assert_eq!(style.align_items, GpuiAlignItems::Center);
    }

    #[test]
    fn map_layout_maps_spacing() {
        let layout = LayoutIntent::new()
            .with_gap(8.0)
            .with_padding(LayoutEdges::uniform(16.0));
        let style = map_layout(&layout);
        assert_eq!(style.gap, 8.0);
        assert_eq!(style.padding.top, 16.0);
        assert_eq!(style.padding.left, 16.0);
    }

    #[test]
    fn map_style_converts_full_descriptor() {
        let desc = StyleDescriptor::new()
            .with_background(ColorValue(0.9, 0.93, 0.96, 1.0))
            .with_text_color(ColorValue(0.07, 0.1, 0.13, 1.0))
            .with_border(1.0, ColorValue(0.79, 0.83, 0.88, 1.0))
            .with_corner_radii(CornerRadii::uniform(6.0))
            .with_typography(TypographyDescriptor {
                family: FontFamily::Sans,
                size: 14.0,
                line_height: 20.0,
                weight: 400,
            })
            .with_cursor(CursorHint::Pointer)
            .with_focus_ring(ColorValue(0.34, 0.65, 1.0, 1.0), 2.0);

        let style = map_style(&desc);

        assert!(style.background.is_some());
        assert!(style.text_color.is_some());
        assert_eq!(style.border_width, 1.0);
        assert!(style.border_color.is_some());
        assert_eq!(style.corner_radii.top_left, 6.0);
        assert!(style.typography.is_some());
        let typo = style.typography.unwrap();
        assert_eq!(typo.font_family, GpuiFontFamily::Sans);
        assert_eq!(typo.font_size, 14.0);
        assert_eq!(style.cursor, GpuiCursorStyle::PointingHand);
        assert!(style.focus_ring_color.is_some());
        assert_eq!(style.focus_ring_width, 2.0);
    }

    #[test]
    fn map_style_defaults_are_visible_and_opaque() {
        let desc = StyleDescriptor::new();
        let style = map_style(&desc);
        assert!(style.visible);
        assert_eq!(style.opacity, 1.0);
        assert_eq!(style.cursor, GpuiCursorStyle::Arrow);
    }

    #[test]
    fn cursor_mapping_covers_all_variants() {
        assert_eq!(map_cursor(&CursorHint::Default), GpuiCursorStyle::Arrow);
        assert_eq!(map_cursor(&CursorHint::Pointer), GpuiCursorStyle::PointingHand);
        assert_eq!(map_cursor(&CursorHint::Text), GpuiCursorStyle::IBeam);
        assert_eq!(map_cursor(&CursorHint::NotAllowed), GpuiCursorStyle::NotAllowed);
        assert_eq!(map_cursor(&CursorHint::Grab), GpuiCursorStyle::OpenHand);
        assert_eq!(map_cursor(&CursorHint::Grabbing), GpuiCursorStyle::ClosedHand);
        assert_eq!(map_cursor(&CursorHint::ColResize), GpuiCursorStyle::ResizeColumn);
        assert_eq!(map_cursor(&CursorHint::RowResize), GpuiCursorStyle::ResizeRow);
    }
}
