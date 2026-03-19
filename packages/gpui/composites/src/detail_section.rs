use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailSectionSpec {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_separated: bool,
    pub aria_label: Option<String>,
}

impl Default for DetailSectionSpec {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            is_separated: true,
            aria_label: None,
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
