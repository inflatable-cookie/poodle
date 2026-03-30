use poodle_tokens::semantic;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPickerSpec {
    pub title: String,
    pub is_open: bool,
    pub is_multiple: bool,
    pub accepted_types: Option<String>,
    pub selected_count: usize,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl MediaPickerSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_open: false,
            is_multiple: false,
            accepted_types: None,
            selected_count: 0,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_multiple(mut self, is_multiple: bool) -> Self {
        self.is_multiple = is_multiple;
        self
    }

    pub fn with_accepted_types(mut self, accepted_types: impl Into<String>) -> Self {
        self.accepted_types = Some(accepted_types.into());
        self
    }

    pub fn with_selected_count(mut self, selected_count: usize) -> Self {
        self.selected_count = selected_count;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_DIALOG
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
