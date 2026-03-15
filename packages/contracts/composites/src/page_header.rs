use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageHeaderSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub action_count: usize,
    pub has_back_button: bool,
}

impl PageHeaderSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            action_count: 0,
            has_back_button: false,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_action_count(mut self, action_count: usize) -> Self {
        self.action_count = action_count;
        self
    }

    pub fn with_back_button(mut self, has_back_button: bool) -> Self {
        self.has_back_button = has_back_button;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }
}
