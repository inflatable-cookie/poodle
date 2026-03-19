use pug_gpui_tokens::semantic;

/// MetricTile — a compact metadata display tile showing a label and value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetricTileSpec {
    pub label: String,
    pub value: String,
    pub aria_label: Option<String>,
}

impl MetricTileSpec {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            aria_label: None,
        }
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn effective_aria_label(&self) -> String {
        self.aria_label
            .clone()
            .unwrap_or_else(|| format!("{}: {}", self.label, self.value))
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn value_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }
}
