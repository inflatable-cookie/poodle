use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeSpec {
    pub content: String,
    pub language: Option<String>,
    pub show_line_numbers: bool,
    pub is_copyable: bool,
}

impl Default for CodeSpec {
    fn default() -> Self {
        Self {
            content: String::new(),
            language: None,
            show_line_numbers: false,
            is_copyable: true,
        }
    }
}

impl CodeSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_show_line_numbers(mut self, show_line_numbers: bool) -> Self {
        self.show_line_numbers = show_line_numbers;
        self
    }

    pub fn with_copyable(mut self, is_copyable: bool) -> Self {
        self.is_copyable = is_copyable;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn font_family_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_CODE_FAMILY
    }

    pub fn font_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_CODE_SIZE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }
}
