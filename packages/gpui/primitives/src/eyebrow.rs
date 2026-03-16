use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EyebrowSpec {
    pub content: Option<String>,
}

impl Default for EyebrowSpec {
    fn default() -> Self {
        Self { content: None }
    }
}

impl EyebrowSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_LABEL_SIZE
    }
}
