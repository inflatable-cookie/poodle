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

#[derive(Clone, Debug, PartialEq)]
pub struct SkeletonSpec {
    pub shape: String,
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
            shape: String::from("rectangle"),
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

    pub fn with_shape(mut self, shape: impl Into<String>) -> Self {
        self.shape = shape.into();
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

    /// Per-shape border radius token. Accepts both the contract shape
    /// vocabulary (`line`/`block`/`circle`) and the legacy Rust vocabulary
    /// (`text`/`rectangle`) so existing callers keep working:
    /// - `line` / `text`  → `radius.control` (contract base skeleton radius)
    /// - `circle`         → `radius.pill`    (contract `999rem`)
    /// - `block` / other  → `radius.surface` (contract `calc(radius-surface − 0.25rem)`;
    ///   no token exists for the exact `−0.25rem` calc — closest is `radius.surface`)
    pub fn radius_token(&self) -> &'static str {
        match self.shape.as_str() {
            "circle" => semantic::RADIUS_PILL,
            "line" | "text" => semantic::RADIUS_CONTROL,
            // "block", "rectangle", and anything else
            _ => semantic::RADIUS_SURFACE,
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
