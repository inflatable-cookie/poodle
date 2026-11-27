use poodle_tokens::semantic;

/// Predefined layout skeleton for common patterns. Matches the
/// contract doc's `preset` prop — when set, the component renders a
/// layout composition of multiple Skeleton children instead of a
/// single shape.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SkeletonPreset {
    TableRow,
    Card,
    ListItem,
    DetailSection,
    AvatarLine,
}

/// Single-shape form factor for [`SkeletonSpec`]. Matches the contract `shape`
/// prop (`"line" | "block" | "circle"`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SkeletonShape {
    #[default]
    Line,
    Block,
    Circle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SkeletonSpec {
    pub shape: SkeletonShape,
    pub width: Option<String>,
    pub height: Option<String>,
    pub is_animated: bool,
    /// Optional preset layout. When set, the shape/width/height
    /// fields are ignored and the component renders the preset's
    /// internal children composition.
    pub preset: Option<SkeletonPreset>,
    /// Line count for the `DetailSection` preset. Ignored for
    /// other presets and for single-shape mode. Defaults to 3 to
    /// match the Svelte reference.
    pub lines: u32,
}

impl Default for SkeletonSpec {
    fn default() -> Self {
        Self {
            shape: SkeletonShape::Line,
            width: None,
            height: None,
            is_animated: true,
            preset: None,
            lines: 3,
        }
    }
}

impl SkeletonSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn with_height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn with_animated(mut self, is_animated: bool) -> Self {
        self.is_animated = is_animated;
        self
    }

    pub fn with_preset(mut self, preset: SkeletonPreset) -> Self {
        self.preset = Some(preset);
        self
    }

    pub fn with_lines(mut self, lines: u32) -> Self {
        self.lines = lines;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// Per-shape border radius token matching the contract shape vocabulary:
    /// - `Line`   → `radius.control` (contract base skeleton radius)
    /// - `Circle` → `radius.pill`    (contract `999rem`)
    /// - `Block`  → `radius.surface` (contract `calc(radius-surface − 0.25rem)`;
    ///   no token exists for the exact `−0.25rem` calc — closest is `radius.surface`)
    pub fn radius_token(&self) -> &'static str {
        match self.shape {
            SkeletonShape::Circle => semantic::RADIUS_PILL,
            SkeletonShape::Line => semantic::RADIUS_CONTROL,
            SkeletonShape::Block => semantic::RADIUS_SURFACE,
        }
    }

    /// Contract: default line height 0.875rem (14px).
    /// Uses body typography size token for consistency.
    pub fn default_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    /// Shimmer gradient base colour (the dimmer outer stops, contract §8:
    /// `color-mix(background-elevated 88%, transparent)`). Targets that cannot
    /// render a true gradient use this as the flat skeleton fill, so both Rust
    /// targets resolve the base tone from one place.
    pub fn shimmer_base_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Shimmer gradient highlight colour (the brighter centre stop, contract §8:
    /// `color-mix(background-surface 92%, white)`). The moving sweep blends from
    /// `shimmer_base_token` toward this; on non-animating targets it is the
    /// static highlight tone.
    pub fn shimmer_highlight_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_shape_is_line_and_radii_match_contract() {
        let default_spec = SkeletonSpec::new();
        assert_eq!(default_spec.shape, SkeletonShape::Line);
        assert_eq!(default_spec.radius_token(), semantic::RADIUS_CONTROL);

        assert_eq!(
            SkeletonSpec::new()
                .with_shape(SkeletonShape::Circle)
                .radius_token(),
            semantic::RADIUS_PILL
        );
        assert_eq!(
            SkeletonSpec::new()
                .with_shape(SkeletonShape::Block)
                .radius_token(),
            semantic::RADIUS_SURFACE
        );
    }
}
