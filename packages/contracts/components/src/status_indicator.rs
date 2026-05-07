use crate::types::StatusTone;
use crate::InlineTypographyMode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusIndicatorSpec {
    pub status: StatusTone,
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub typography: InlineTypographyMode,
}

impl Default for StatusIndicatorSpec {
    fn default() -> Self {
        Self {
            status: StatusTone::Neutral,
            label: None,
            aria_label: None,
            typography: InlineTypographyMode::default(),
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

    pub fn status_color_token(&self) -> &'static str {
        self.status.color_token()
    }

    /// Contract: dot size 0.5625rem (9px).
    pub fn dot_size_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.5625,
            InlineTypographyMode::Inherit => 0.75,
        }
    }

    /// Contract: gap between dot and label 0.4375rem (7px).
    pub fn gap_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.4375,
            InlineTypographyMode::Inherit => 0.5833,
        }
    }

    /// Contract: label font-size 0.75rem (12px).
    pub fn label_font_size_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.75,
            InlineTypographyMode::Inherit => 0.8571,
        }
    }

    /// Contract: label color token.
    pub fn label_color_token(&self) -> &'static str {
        "color.text.primary"
    }

    pub fn inherits_typography(&self) -> bool {
        self.typography == InlineTypographyMode::Inherit
    }
}
