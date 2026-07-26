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
    pub state_title: Option<String>,
    pub state_message: Option<String>,
    pub status_text: Option<String>,
    pub status_id: Option<String>,
    /// How many items are currently selected — drives the summary copy.
    pub selection_count: usize,
}

impl PickerShellSpec {
    pub fn with_selection_count(mut self, value: usize) -> Self {
        self.selection_count = value;
        self
    }

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
            state_title: None,
            state_message: None,
            status_text: None,
            status_id: None,
            selection_count: 0,
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

    pub fn with_state_title(mut self, state_title: impl Into<String>) -> Self {
        self.state_title = Some(state_title.into());
        self
    }

    pub fn with_state_message(mut self, state_message: impl Into<String>) -> Self {
        self.state_message = Some(state_message.into());
        self
    }

    pub fn with_status_text(mut self, status_text: impl Into<String>) -> Self {
        self.status_text = Some(status_text.into());
        self
    }

    pub fn with_status_id(mut self, status_id: impl Into<String>) -> Self {
        self.status_id = Some(status_id.into());
        self
    }

    pub fn is_modal_like(&self) -> bool {
        matches!(self.variant, PickerVariant::Modal | PickerVariant::Popover)
    }

    /// True only for the popover variant (overlay elevation, width clamp).
    pub fn is_popover(&self) -> bool {
        matches!(self.variant, PickerVariant::Popover)
    }

    /// True only for the modal variant (dialog elevation, elevated background).
    pub fn is_modal(&self) -> bool {
        matches!(self.variant, PickerVariant::Modal)
    }

    /// Contract §7/§8 popover width cap, in rem. Svelte (authority) caps at
    /// `min(32rem, calc(100vw - 2rem))`; the viewport clamp is a web-only
    /// concern, so the Rust targets apply the `32rem` ceiling.
    pub fn popover_max_width_rem(&self) -> f32 {
        32.0
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

    pub fn selected_count_text(&self) -> String {
        match self.selection_mode {
            SelectionMode::Single => {
                if self.selected_count == 1 {
                    String::from("1 selected")
                } else {
                    String::from("0 selected")
                }
            }
            SelectionMode::Multiple => format!("{} selected", self.selected_count),
        }
    }

    pub fn result_count_text(&self) -> Option<String> {
        self.result_count.map(|count| {
            format!("{} result{}", count, if count == 1 { "" } else { "s" })
        })
    }

    pub fn effective_state_title(&self) -> &str {
        if let Some(ref title) = self.state_title {
            return title;
        }

        match self.state {
            BrowseState::Loading => "Loading results",
            BrowseState::Error => "Something went wrong",
            BrowseState::Empty => "Nothing here yet",
            BrowseState::NoResults => "No results",
            BrowseState::Ready => "Picker state",
        }
    }

    pub fn effective_state_message(&self) -> Option<&str> {
        if let Some(ref message) = self.state_message {
            return Some(message.as_str());
        }

        match self.state {
            BrowseState::Loading => Some("Searching…"),
            BrowseState::Error => Some("Search failed."),
            BrowseState::Empty => Some("No items are available yet."),
            BrowseState::NoResults => Some("No results found."),
            BrowseState::Ready => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_predicates_are_exclusive() {
        let inline = PickerShellSpec::new("t");
        assert!(!inline.is_popover() && !inline.is_modal() && !inline.is_modal_like());

        let popover = PickerShellSpec::new("t").with_variant(PickerVariant::Popover);
        assert!(popover.is_popover() && !popover.is_modal() && popover.is_modal_like());

        let modal = PickerShellSpec::new("t").with_variant(PickerVariant::Modal);
        assert!(modal.is_modal() && !modal.is_popover() && modal.is_modal_like());
    }

    #[test]
    fn popover_max_width_is_32rem() {
        assert_eq!(PickerShellSpec::new("t").popover_max_width_rem(), 32.0);
    }
}
