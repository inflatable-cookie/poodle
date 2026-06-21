use crate::types::ControlDensity;
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailSectionSpec {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_separated: bool,
    pub aria_label: Option<String>,
    /// Number of columns for the detail rows (default 1).
    pub columns: u8,
    /// Density override for section spacing (gaps, separated top padding).
    pub density: ControlDensity,
}

impl Default for DetailSectionSpec {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            is_separated: true,
            aria_label: None,
            columns: 1,
            density: ControlDensity::Default,
        }
    }
}

impl DetailSectionSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_separated(mut self, is_separated: bool) -> Self {
        self.is_separated = is_separated;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_columns(mut self, columns: u8) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    // ── Density-driven spacing (contract §8 density table, rem) ───
    // Component-specific literals mirroring the Svelte `[data-density]`
    // custom-property table. These are layout gaps between siblings, not
    // component height.

    /// Root vertical gap (header↔body) in rem. Contract §8: compact 0.75,
    /// default `stack.md + 0.125` (≈0.875), comfortable `stack.lg - 0.125`
    /// (≈1.375).
    pub fn root_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.75,
            ControlDensity::Default => 0.875,
            ControlDensity::Comfortable => 1.375,
        }
    }

    /// Header gap (title-block↔actions) in rem. Contract §8: compact
    /// `space.inline.sm` (0.5), default 0.75, comfortable 0.875.
    pub fn header_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }
    }

    /// Title↔description gap in rem. Contract §8: compact 0.25, default
    /// 0.375, comfortable 0.5.
    pub fn title_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }
    }

    /// Body inter-row / inter-column gap in rem. Contract §8: compact 0.625,
    /// default 0.75, comfortable 1.0.
    pub fn body_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 1.0,
        }
    }

    /// Separated top-padding in rem. Contract §8: compact 0.875, default 1.0,
    /// comfortable 1.125.
    pub fn separated_gap_rem(&self) -> f32 {
        match self.density {
            ControlDensity::Compact => 0.875,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.125,
        }
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn body_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn section_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_LG
    }

    pub fn header_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }

    pub fn title_body_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }
}
