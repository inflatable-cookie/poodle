use pug_tokens::semantic;

#[derive(Clone)]
pub struct DetailRowSpec {
    pub label: String,
    pub description: Option<String>,
    pub value: Option<String>,
    pub truncate_value: bool,
    pub aria_label: Option<String>,
}

impl DetailRowSpec {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            value: None,
            truncate_value: false,
            aria_label: None,
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
