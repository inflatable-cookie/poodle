use poodle_tokens::semantic;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, PartialEq)]
pub struct CodeSpec {
    pub content: String,
    pub language: Option<String>,
    pub show_line_numbers: bool,
    pub is_copyable: bool,
    pub highlight_lines: Vec<usize>,
    pub max_height: Option<f64>,
    pub is_inline: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CodeSpec {
    fn default() -> Self {
        Self {
            content: String::new(),
            language: None,
            show_line_numbers: false,
            is_copyable: true,
            highlight_lines: Vec::new(),
            max_height: None,
            is_inline: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
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

    pub fn with_highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.highlight_lines = lines;
        self
    }

    pub fn with_max_height(mut self, max_height: f64) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn with_inline(mut self, is_inline: bool) -> Self {
        self.is_inline = is_inline;
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
