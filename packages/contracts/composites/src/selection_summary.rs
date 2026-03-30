use poodle_tokens::semantic;

use crate::types::{RemediationAction, SelectionSummaryItem};
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSummarySpec {
    pub items: Vec<SelectionSummaryItem>,
    pub clear_action: Option<RemediationAction>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SelectionSummarySpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            clear_action: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SelectionSummarySpec {
    pub fn new(items: Vec<SelectionSummaryItem>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    pub fn with_clear_action(mut self, clear_action: RemediationAction) -> Self {
        self.clear_action = Some(clear_action);
        self
    }

    pub fn selected_count(&self) -> usize {
        self.items.len()
    }

    pub fn has_clear_action(&self) -> bool {
        self.clear_action.is_some()
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
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
