use poodle_tokens::semantic;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReorderableListSpec {
    pub item_count: usize,
    pub is_disabled: bool,
    pub active_drag_index: Option<usize>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl ReorderableListSpec {
    pub fn new() -> Self {
        Self {
            item_count: 0,
            is_disabled: false,
            active_drag_index: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_item_count(mut self, item_count: usize) -> Self {
        self.item_count = item_count;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_active_drag_index(mut self, active_drag_index: usize) -> Self {
        self.active_drag_index = Some(active_drag_index);
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn handle_color_token(&self) -> &'static str {
        if self.is_disabled {
            semantic::COLOR_ICON_MUTED
        } else {
            semantic::COLOR_ICON_PRIMARY
        }
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
