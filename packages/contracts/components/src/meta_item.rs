/// MetaItem — a label/value pair for use inside MetaBar.
///
/// Renders an uppercase compact label with inline value content.
/// Matches docs/contracts/components/meta-item.md.

use crate::InlineTypographyMode;
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaItemSpec {
    /// Uppercase label rendered before the value slot. When None,
    /// the value renders without a leading label.
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub typography: InlineTypographyMode,
    /// Presentational signal a parent `MetaBar` reads to decide whether to draw
    /// a leading separator dot before this item (contract §2/§6, default `true`).
    /// Pass `false` to opt out of the dot.
    pub separator: bool,
}

impl Default for MetaItemSpec {
    fn default() -> Self {
        Self {
            label: None,
            aria_label: None,
            typography: InlineTypographyMode::default(),
            separator: true,
        }
    }
}

impl MetaItemSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_typography(mut self, typography: InlineTypographyMode) -> Self {
        self.typography = typography;
        self
    }

    pub fn with_separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }

    pub fn gap_rem(&self) -> f32 {
        // Inherit ratios mirror the contract §7 `em` values from a 1rem baseline
        // (CSS runtimes apply them literally; non-CSS runtimes approximate).
        match self.typography {
            InlineTypographyMode::Default => 0.375,
            InlineTypographyMode::Inherit => 0.375,
        }
    }

    pub fn label_font_size_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.6875,
            InlineTypographyMode::Inherit => 0.6875,
        }
    }

    pub fn value_font_size_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.875,
            InlineTypographyMode::Inherit => 1.0,
        }
    }

    pub fn inherits_typography(&self) -> bool {
        self.typography == InlineTypographyMode::Inherit
    }

    // ── Token targets (contract §7) ──────────────────────────────

    /// Label color — `var(--poodle-color-text-secondary)`.
    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Value color — `var(--poodle-color-text-primary)`.
    pub fn value_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Label font-family — `var(--poodle-typography-label-family)`.
    pub fn label_family_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_FAMILY
    }

    /// Value font-family — `var(--poodle-typography-body-family)`.
    pub fn value_family_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_FAMILY
    }

    /// Label font-weight — `var(--poodle-typography-label-weight)` (= 500).
    /// The numeric weight has no string-resolver channel, so callers read the
    /// typed constant directly (as Code reads the code adjustment ratio).
    pub fn label_font_weight(&self) -> u16 {
        poodle_tokens::typed::semantic::TYPOGRAPHY_LABEL_WEIGHT as u16
    }

    /// Label line-height — `1` (contract §7 `.meta-item__label`).
    pub fn label_line_height(&self) -> f32 {
        1.0
    }

    /// Value line-height — `1.4` (contract §7 `.meta-item__value`).
    /// Under `typography="inherit"` the value inherits parent line-height.
    pub fn value_line_height(&self) -> f32 {
        1.4
    }
}
