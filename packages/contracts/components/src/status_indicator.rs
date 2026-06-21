use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone};
use crate::InlineTypographyMode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusIndicatorSpec {
    pub status: StatusTone,
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub typography: InlineTypographyMode,
    /// Explicit size override. When `None`, resolves from `size_role` against
    /// the inherited presentation scale.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// Explicit density override (controls dot↔label gap). When `None`,
    /// resolves from the inherited presentation density.
    pub density: Option<ControlDensity>,
}

impl Default for StatusIndicatorSpec {
    fn default() -> Self {
        Self {
            status: StatusTone::Neutral,
            label: None,
            aria_label: None,
            typography: InlineTypographyMode::default(),
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl StatusIndicatorSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_status(mut self, status: StatusTone) -> Self {
        self.status = status;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_typography(mut self, typography: InlineTypographyMode) -> Self {
        self.typography = typography;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    pub fn status_color_token(&self) -> &'static str {
        self.status.color_token()
    }

    /// Contract §8 Size adjustments: dot size in rem per size.
    ///
    /// | xs 0.375 | sm 0.4375 | md 0.5625 | lg 0.6875 | xl 0.8125 |
    pub fn dot_size_rem_for(&self, size: ControlSize) -> f32 {
        match (self.typography, size) {
            // Inherit mode expresses dot size in em-equivalents from a 1rem
            // baseline (contract §8 inherit table). Non-em runtimes approximate.
            (InlineTypographyMode::Inherit, ControlSize::Xs) => 0.6,
            (InlineTypographyMode::Inherit, ControlSize::Sm) => 0.6364,
            (InlineTypographyMode::Inherit, ControlSize::Md) => 0.75,
            (InlineTypographyMode::Inherit, ControlSize::Lg) => 0.8462,
            (InlineTypographyMode::Inherit, ControlSize::Xl) => 0.9286,
            (InlineTypographyMode::Default, ControlSize::Xs) => 0.375,
            (InlineTypographyMode::Default, ControlSize::Sm) => 0.4375,
            (InlineTypographyMode::Default, ControlSize::Md) => 0.5625,
            (InlineTypographyMode::Default, ControlSize::Lg) => 0.6875,
            (InlineTypographyMode::Default, ControlSize::Xl) => 0.8125,
        }
    }

    /// Contract §8: gap between dot and label in rem per size, modulated by
    /// density. Base per-size gap with density offset applied (contract §8
    /// density table: compact 0.25 / default 0.4375 / comfortable 0.625 at md).
    pub fn gap_rem_for(&self, size: ControlSize, density: ControlDensity) -> f32 {
        // Density fully owns the gap at md (the per-density Svelte table sets
        // `--poodle-status-gap` directly). For other sizes we use the per-size
        // gap and only swap in the density value when density is non-default.
        match self.typography {
            InlineTypographyMode::Inherit => match density {
                ControlDensity::Compact => 0.4,
                ControlDensity::Comfortable => 0.8,
                ControlDensity::Default => match size {
                    ControlSize::Xs => 0.5,
                    ControlSize::Sm => 0.5455,
                    ControlSize::Md => 0.5833,
                    ControlSize::Lg => 0.6154,
                    ControlSize::Xl => 0.6429,
                },
            },
            InlineTypographyMode::Default => match density {
                ControlDensity::Compact => 0.25,
                ControlDensity::Comfortable => 0.625,
                ControlDensity::Default => match size {
                    ControlSize::Xs => 0.3125,
                    ControlSize::Sm => 0.375,
                    ControlSize::Md => 0.4375,
                    ControlSize::Lg => 0.5,
                    ControlSize::Xl => 0.5625,
                },
            },
        }
    }

    /// Contract §8: label font-size in rem per size.
    ///
    /// | xs 0.625 | sm 0.6875 | md 0.75 | lg 0.8125 | xl 0.875 |
    pub fn label_font_size_rem_for(&self, size: ControlSize) -> f32 {
        match (self.typography, size) {
            (InlineTypographyMode::Inherit, ControlSize::Xs) => 0.7143,
            (InlineTypographyMode::Inherit, ControlSize::Sm) => 0.7857,
            (InlineTypographyMode::Inherit, ControlSize::Md) => 0.8571,
            (InlineTypographyMode::Inherit, ControlSize::Lg) => 0.9286,
            (InlineTypographyMode::Inherit, ControlSize::Xl) => 1.0,
            (InlineTypographyMode::Default, ControlSize::Xs) => 0.625,
            (InlineTypographyMode::Default, ControlSize::Sm) => 0.6875,
            (InlineTypographyMode::Default, ControlSize::Md) => 0.75,
            (InlineTypographyMode::Default, ControlSize::Lg) => 0.8125,
            (InlineTypographyMode::Default, ControlSize::Xl) => 0.875,
        }
    }

    /// Contract: dot size 0.5625rem (9px) at the md baseline.
    /// Back-compat accessor; prefer `dot_size_rem_for(size)`.
    pub fn dot_size_rem(&self) -> f32 {
        self.dot_size_rem_for(ControlSize::Md)
    }

    /// Contract: gap between dot and label 0.4375rem (7px) at md / default density.
    /// Back-compat accessor; prefer `gap_rem_for(size, density)`.
    pub fn gap_rem(&self) -> f32 {
        self.gap_rem_for(ControlSize::Md, ControlDensity::Default)
    }

    /// Contract: label font-size 0.75rem (12px) at the md baseline.
    /// Back-compat accessor; prefer `label_font_size_rem_for(size)`.
    pub fn label_font_size_rem(&self) -> f32 {
        self.label_font_size_rem_for(ControlSize::Md)
    }

    /// Contract: label color token.
    pub fn label_color_token(&self) -> &'static str {
        "color.text.primary"
    }

    pub fn inherits_typography(&self) -> bool {
        self.typography == InlineTypographyMode::Inherit
    }
}
