use poodle_tokens::semantic;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockEditorSpec {
    pub block_count: usize,
    pub is_disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl BlockEditorSpec {
    pub fn new() -> Self {
        Self {
            block_count: 0,
            is_disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_block_count(mut self, block_count: usize) -> Self {
        self.block_count = block_count;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn block_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
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
