//! Resolved style descriptor intermediate representation for Flint.
//!
//! A `StyleDescriptor` captures the visual properties of a component instance
//! after token resolution but before renderer-specific translation. Both GPUI
//! and Jetstream adapters can use it as rendering input, eliminating duplicate
//! token resolution logic.

use flint_layout::LayoutIntent;
use flint_tokens::typed::{ColorValue, ShadowValue};

/// Font family category for typography resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontFamily {
    Sans,
    Mono,
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::Sans
    }
}

/// Resolved typography properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypographyDescriptor {
    pub family: FontFamily,
    pub size: f32,
    pub line_height: f32,
    pub weight: u16,
}

impl Default for TypographyDescriptor {
    fn default() -> Self {
        Self {
            family: FontFamily::Sans,
            size: 14.0,
            line_height: 20.0,
            weight: 400,
        }
    }
}

/// Resolved border properties for a single edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderDescriptor {
    pub width: f32,
    pub color: ColorValue,
}

impl Default for BorderDescriptor {
    fn default() -> Self {
        Self {
            width: 0.0,
            color: ColorValue(0.0, 0.0, 0.0, 0.0),
        }
    }
}

/// Corner radii in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl CornerRadii {
    pub const ZERO: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };

    pub const fn uniform(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }
}

impl Default for CornerRadii {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Cursor hint for pointer behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorHint {
    Default,
    Pointer,
    Text,
    NotAllowed,
    Grab,
    Grabbing,
    ColResize,
    RowResize,
}

impl Default for CursorHint {
    fn default() -> Self {
        Self::Default
    }
}

/// Complete resolved visual style for a component instance.
///
/// Produced by a spec struct's `resolve_style()` method after token resolution.
/// Rendering adapters consume this to produce their native visual output:
///
/// - **GPUI**: Maps to `Style` struct fields + GPUI-specific paint calls
/// - **Jetstream**: Maps to `UiStyle` fields + `Theme` color/size lookups
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StyleDescriptor {
    // --- Colors ---
    pub background: Option<ColorValue>,
    pub text_color: Option<ColorValue>,
    pub icon_color: Option<ColorValue>,

    // --- Border ---
    pub border: BorderDescriptor,
    pub corner_radii: CornerRadii,

    // --- Shadow ---
    pub shadow: Option<ShadowValue>,

    // --- Typography ---
    pub typography: Option<TypographyDescriptor>,

    // --- Opacity and visibility ---
    pub opacity: f32,
    pub visible: bool,

    // --- Cursor ---
    pub cursor: CursorHint,

    // --- Layout ---
    pub layout: LayoutIntent,

    // --- Focus ring ---
    pub focus_ring_color: Option<ColorValue>,
    pub focus_ring_width: f32,
}

impl StyleDescriptor {
    pub fn new() -> Self {
        Self {
            opacity: 1.0,
            visible: true,
            ..Default::default()
        }
    }

    pub fn with_background(mut self, color: ColorValue) -> Self {
        self.background = Some(color);
        self
    }

    pub fn with_text_color(mut self, color: ColorValue) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn with_icon_color(mut self, color: ColorValue) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn with_border(mut self, width: f32, color: ColorValue) -> Self {
        self.border = BorderDescriptor { width, color };
        self
    }

    pub fn with_corner_radii(mut self, radii: CornerRadii) -> Self {
        self.corner_radii = radii;
        self
    }

    pub fn with_shadow(mut self, shadow: ShadowValue) -> Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn with_typography(mut self, typography: TypographyDescriptor) -> Self {
        self.typography = Some(typography);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_cursor(mut self, cursor: CursorHint) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn with_layout(mut self, layout: LayoutIntent) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_focus_ring(mut self, color: ColorValue, width: f32) -> Self {
        self.focus_ring_color = Some(color);
        self.focus_ring_width = width;
        self
    }

    /// Whether the style has any visible content (not fully transparent or hidden).
    pub fn is_visible(&self) -> bool {
        self.visible && self.opacity > 0.0
    }

    /// Whether a focus ring should be painted.
    pub fn has_focus_ring(&self) -> bool {
        self.focus_ring_color.is_some() && self.focus_ring_width > 0.0
    }
}

#[cfg(test)]
mod tests {
    use flint_layout::{LayoutDirection, LayoutIntent, LayoutSizing};

    use super::*;

    #[test]
    fn default_style_is_visible_and_opaque() {
        let style = StyleDescriptor::new();
        assert!(style.is_visible());
        assert_eq!(style.opacity, 1.0);
        assert!(!style.has_focus_ring());
    }

    #[test]
    fn builder_composes_full_style() {
        let style = StyleDescriptor::new()
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
            .with_focus_ring(ColorValue(0.34, 0.65, 1.0, 1.0), 2.0)
            .with_layout(
                LayoutIntent::new()
                    .with_direction(LayoutDirection::Row)
                    .with_width(LayoutSizing::Grow),
            );

        assert!(style.background.is_some());
        assert!(style.text_color.is_some());
        assert_eq!(style.border.width, 1.0);
        assert_eq!(style.corner_radii.top_left, 6.0);
        assert!(style.typography.is_some());
        assert_eq!(style.cursor, CursorHint::Pointer);
        assert!(style.has_focus_ring());
        assert_eq!(style.layout.direction, LayoutDirection::Row);
    }

    #[test]
    fn hidden_style_reports_not_visible() {
        let hidden = StyleDescriptor {
            visible: false,
            ..StyleDescriptor::new()
        };
        assert!(!hidden.is_visible());

        let transparent = StyleDescriptor {
            opacity: 0.0,
            ..StyleDescriptor::new()
        };
        assert!(!transparent.is_visible());
    }

    #[test]
    fn typography_defaults_to_body_sans() {
        let typo = TypographyDescriptor::default();
        assert_eq!(typo.family, FontFamily::Sans);
        assert_eq!(typo.size, 14.0);
        assert_eq!(typo.weight, 400);
    }
}
