use poodle_tokens::semantic;

/// Controls the gap between fields in the FieldSet grid.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpaceScale {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSetSpec {
    pub legend: Option<String>,
    pub columns: u8,
    pub gap: SpaceScale,
}

impl Default for FieldSetSpec {
    fn default() -> Self {
        Self {
            legend: None,
            columns: 1,
            gap: SpaceScale::Md,
        }
    }
}

impl FieldSetSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_legend(mut self, legend: impl Into<String>) -> Self {
        self.legend = Some(legend.into());
        self
    }

    pub fn with_columns(mut self, columns: u8) -> Self {
        self.columns = columns;
        self
    }

    pub fn with_gap(mut self, gap: SpaceScale) -> Self {
        self.gap = gap;
        self
    }

    pub fn gap_token(&self) -> &'static str {
        match self.gap {
            SpaceScale::Sm => semantic::SPACE_STACK_SM,
            SpaceScale::Md => semantic::SPACE_STACK_MD,
            SpaceScale::Lg => semantic::SPACE_STACK_LG,
        }
    }

    pub fn legend_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn legend_family_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_FAMILY
    }

    pub fn legend_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }
}
