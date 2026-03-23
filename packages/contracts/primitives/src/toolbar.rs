use flint_tokens::semantic;

use crate::types::Alignment;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolbarSpec {
    pub alignment: Alignment,
    pub has_separator: bool,
    pub aria_label: Option<String>,
}

impl Default for ToolbarSpec {
    fn default() -> Self {
        Self {
            alignment: Alignment::Start,
            has_separator: false,
            aria_label: None,
        }
    }
}

impl ToolbarSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_separator(mut self, has_separator: bool) -> Self {
        self.has_separator = has_separator;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn bg_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }

    pub fn padding_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }
}
