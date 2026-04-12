use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownEditorSpec {
    pub value: String,
    pub placeholder: Option<String>,
    pub mode: String,
    pub is_disabled: bool,
    pub min_height: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl MarkdownEditorSpec {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            mode: String::from("edit"),
            is_disabled: false,
            min_height: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_mode(mut self, mode: impl Into<String>) -> Self {
        self.mode = mode.into();
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_min_height(mut self, min_height: impl Into<String>) -> Self {
        self.min_height = Some(min_height.into());
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn toolbar_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
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
