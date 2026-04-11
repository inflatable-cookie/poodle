use crate::StatusTone;
use poodle_tokens::semantic;

use crate::composite_types::{BrowseState, PickerVariant, SelectionMode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PickerShellSpec {
    pub title: String,
    pub description: Option<String>,
    pub variant: PickerVariant,
    pub selection_mode: SelectionMode,
    pub state: BrowseState,
    pub query: String,
    pub result_count: Option<usize>,
    pub selected_count: usize,
    pub aria_label: Option<String>,
}

impl PickerShellSpec {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: PickerVariant::Inline,
            selection_mode: SelectionMode::Multiple,
            state: BrowseState::Ready,
            query: String::new(),
            result_count: None,
            selected_count: 0,
            aria_label: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_variant(mut self, variant: PickerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_selection_mode(mut self, selection_mode: SelectionMode) -> Self {
        self.selection_mode = selection_mode;
        self
    }

    pub fn with_state(mut self, state: BrowseState) -> Self {
        self.state = state;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_result_count(mut self, result_count: usize) -> Self {
        self.result_count = Some(result_count);
        self
    }

    pub fn with_selected_count(mut self, selected_count: usize) -> Self {
        self.selected_count = selected_count;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn is_modal_like(&self) -> bool {
        matches!(self.variant, PickerVariant::Modal | PickerVariant::Popover)
    }

    pub fn summary_tone(&self) -> StatusTone {
        if self.state == BrowseState::Error {
            StatusTone::Danger
        } else if self.state == BrowseState::Loading {
            StatusTone::Pending
        } else {
            StatusTone::Neutral
        }
    }

    pub fn footer_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_MD
    }
}
