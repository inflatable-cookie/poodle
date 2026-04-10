use poodle_tokens::semantic;

/// Layout direction for the label-value pair.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum DetailItemLayout {
    /// Label and value side by side (default).
    #[default]
    Inline,
    /// Label above value.
    Stacked,
}

/// Visual presentation mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DetailItemPresentation {
    /// Plain label-value pair with no container chrome.
    #[default]
    Simple,
    /// Elevated card-like styling with background and padding.
    Surface,
}

/// Column span in a parent grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailItemSpan {
    /// Spans all columns via `grid-column: 1 / -1`.
    Full,
    /// Spans half the available columns.
    Half,
}

#[derive(Clone)]
pub struct DetailItemSpec {
    pub label: String,
    pub description: Option<String>,
    pub value: Option<String>,
    /// Display text when value is null and no slot content is provided.
    pub empty_text: String,
    pub truncate_value: bool,
    pub aria_label: Option<String>,
    pub layout: DetailItemLayout,
    /// Visual presentation: Simple (default) or Surface (elevated card).
    pub presentation: DetailItemPresentation,
    /// Optional column span in a parent grid.
    pub span: Option<DetailItemSpan>,
}

impl DetailItemSpec {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            value: None,
            empty_text: String::from("--"),
            truncate_value: false,
            aria_label: None,
            layout: DetailItemLayout::default(),
            presentation: DetailItemPresentation::default(),
            span: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_truncate_value(mut self, truncate: bool) -> Self {
        self.truncate_value = truncate;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_layout(mut self, layout: DetailItemLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_empty_text(mut self, empty_text: impl Into<String>) -> Self {
        self.empty_text = empty_text.into();
        self
    }

    pub fn with_presentation(mut self, presentation: DetailItemPresentation) -> Self {
        self.presentation = presentation;
        self
    }

    pub fn with_span(mut self, span: DetailItemSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Returns the effective display value: the value if set, otherwise empty_text.
    pub fn effective_value(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.empty_text)
    }

    /// Returns true when the item has no resolved value (falls back to empty_text).
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    pub fn label_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn value_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn description_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
    }

    pub fn background_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn padding_x_token(&self) -> &'static str {
        semantic::SPACE_PANEL_X
    }

    pub fn padding_y_token(&self) -> &'static str {
        semantic::SPACE_PANEL_Y
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }
}
