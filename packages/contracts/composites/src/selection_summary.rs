use flint_tokens::semantic;

use crate::types::{RemediationAction, SelectionSummaryItem};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectionSummarySpec {
    pub items: Vec<SelectionSummaryItem>,
    pub clear_action: Option<RemediationAction>,
}

impl Default for SelectionSummarySpec {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            clear_action: None,
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
}
