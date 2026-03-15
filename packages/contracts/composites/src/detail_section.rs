use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailSectionSpec {
    pub title: String,
    pub is_collapsible: bool,
    pub is_open: bool,
}

impl DetailSectionSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_collapsible: false,
            is_open: true,
        }
    }

    pub fn with_collapsible(mut self, is_collapsible: bool) -> Self {
        self.is_collapsible = is_collapsible;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn header_text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }
}
