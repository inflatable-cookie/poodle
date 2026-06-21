use poodle_tokens::semantic;

/// Controls the gap between fields in the FieldSet grid.
///
/// Mirrors the Svelte `SpaceScale = "none" | "sm" | "md" | "lg"`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpaceScale {
    None,
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSetSpec {
    pub legend: Option<String>,
    pub description: Option<String>,
    pub columns: u8,
    pub gap: SpaceScale,
    /// Parent-grid span: `Some("full")` → `1 / -1`, `Some("<n>")` → `span <n>`.
    pub span: Option<String>,
}

impl Default for FieldSetSpec {
    fn default() -> Self {
        Self {
            legend: None,
            description: None,
            columns: 1,
            gap: SpaceScale::Md,
            span: None,
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

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
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

    pub fn with_span(mut self, span: impl Into<String>) -> Self {
        self.span = Some(span.into());
        self
    }

    /// Column-gap token = `scaleToSpace(gap)` (contract §6, mirrors Svelte
    /// `internal.ts`: sm→inline-sm, md→panel-y, lg→panel-x). Returns `None` for
    /// `SpaceScale::None`, which resolves to a `0` gap.
    pub fn gap_token(&self) -> Option<&'static str> {
        match self.gap {
            SpaceScale::None => None,
            SpaceScale::Sm => Some(semantic::SPACE_INLINE_SM),
            SpaceScale::Md => Some(semantic::SPACE_PANEL_Y),
            SpaceScale::Lg => Some(semantic::SPACE_PANEL_X),
        }
    }

    /// Column-gap token, alias of [`Self::gap_token`] for builder clarity.
    pub fn column_gap_token(&self) -> Option<&'static str> {
        self.gap_token()
    }

    /// Row-gap is asymmetric: `column-gap + 0.5rem` (contract §6). Builders add
    /// this rem (resolved via `rem_to_px`) to the resolved `column_gap_token`.
    pub const ROW_GAP_EXTRA_REM: f32 = 0.5;

    pub fn legend_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn legend_family_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_FAMILY
    }

    /// Legend font-size is a fixed eyebrow scale, NOT `typography-label-size`
    /// (contract §6: `0.6875rem`). Resolve via `rem_to_px(0.6875)`.
    pub const LEGEND_SIZE_REM: f32 = 0.6875;

    /// Legend tracking (contract §6: `0.12em`).
    pub const LEGEND_LETTER_SPACING_EM: f32 = 0.12;

    /// Margin below the legend (contract §6: `space-stack-sm`).
    pub fn legend_margin_bottom_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn description_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    /// Margin below the description (contract §6: `space-stack-md`).
    pub fn description_margin_bottom_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }

    /// Span CSS-style value for parent grids: `"1 / -1"` for `"full"`, else
    /// `"span <n>"`. Returns `None` when no span is set.
    pub fn span_value(&self) -> Option<String> {
        self.span.as_ref().map(|s| {
            if s == "full" {
                "1 / -1".to_string()
            } else {
                format!("span {s}")
            }
        })
    }
}
