/// MetaItem — a label/value pair for use inside MetaBar.
///
/// Renders an uppercase compact label with inline value content.
/// Matches docs/contracts/components/meta-item.md.

use crate::InlineTypographyMode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaItemSpec {
    /// Uppercase label rendered before the value slot. When None,
    /// the value renders without a leading label.
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub typography: InlineTypographyMode,
}

impl Default for MetaItemSpec {
    fn default() -> Self {
        Self {
            label: None,
            aria_label: None,
            typography: InlineTypographyMode::default(),
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

    pub fn gap_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.375,
            InlineTypographyMode::Inherit => 0.4286,
        }
    }

    pub fn label_font_size_rem(&self) -> f32 {
        match self.typography {
            InlineTypographyMode::Default => 0.6875,
            InlineTypographyMode::Inherit => 0.7857,
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
}
