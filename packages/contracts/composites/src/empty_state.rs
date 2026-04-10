use poodle_tokens::semantic;

use crate::types::{EmptyStateVariant, RemediationAction};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmptyStateSpec {
    pub title: String,
    pub message: Option<String>,
    pub variant: EmptyStateVariant,
    pub aria_label: Option<String>,
    pub actions: Vec<RemediationAction>,
    /// When true the empty state renders in a tighter form suitable
    /// for embedding inside lists or small containers — reduced
    /// vertical padding, smaller title, smaller icon.
    pub compact: bool,
}

impl EmptyStateSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: None,
            variant: EmptyStateVariant::Neutral,
            aria_label: None,
            actions: Vec::new(),
            compact: false,
        }
    }

    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_variant(mut self, variant: EmptyStateVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_actions(mut self, actions: Vec<RemediationAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    pub fn layout_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_MD
    }
}
