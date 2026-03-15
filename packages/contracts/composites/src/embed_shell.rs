use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbedShellSpec {
    pub has_preview: bool,
    pub is_editing: bool,
}

impl EmbedShellSpec {
    pub fn new() -> Self {
        Self {
            has_preview: false,
            is_editing: true,
        }
    }

    pub fn with_has_preview(mut self, has_preview: bool) -> Self {
        self.has_preview = has_preview;
        self
    }

    pub fn with_editing(mut self, is_editing: bool) -> Self {
        self.is_editing = is_editing;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }
}
