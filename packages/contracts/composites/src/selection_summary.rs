use poodle_tokens::semantic;

use crate::types::{RemediationAction, SelectionSummaryItem};
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSummarySpec {
    pub items: Vec<SelectionSummaryItem>,
    pub clear_action: Option<RemediationAction>,
    /// Optional cap on the number of item chips rendered inline. When
    /// the list exceeds this limit, the overflow is surfaced as a
    /// "+N more" chip instead of a long wrapping row. Matches the
    /// Svelte `maxVisibleItems` prop.
    pub max_visible_items: Option<usize>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SelectionSummarySpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            clear_action: None,
            max_visible_items: None,
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

    pub fn with_max_visible_items(mut self, max: usize) -> Self {
        self.max_visible_items = Some(max);
        self
    }

    pub fn selected_count(&self) -> usize {
        self.items.len()
    }

    pub fn has_clear_action(&self) -> bool {
        self.clear_action.is_some()
    }

    /// Number of items actually rendered inline, taking `max_visible_items`
    /// into account.
    pub fn visible_item_count(&self) -> usize {
        self.max_visible_items
            .map(|max| self.items.len().min(max))
            .unwrap_or(self.items.len())
    }

    /// Number of items hidden behind the "+N more" overflow chip.
    pub fn overflow_count(&self) -> usize {
        self.items.len().saturating_sub(self.visible_item_count())
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
