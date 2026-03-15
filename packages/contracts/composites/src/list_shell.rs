use pug_tokens::semantic;

use crate::types::{BrowseState, ScrollOwner};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListShellSpec {
    pub state: BrowseState,
    pub aria_label: Option<String>,
    pub item_count: Option<usize>,
    pub scroll_owner: ScrollOwner,
}

impl Default for ListShellSpec {
    fn default() -> Self {
        Self {
            state: BrowseState::Ready,
            aria_label: None,
            item_count: None,
            scroll_owner: ScrollOwner::Content,
        }
    }
}

impl ListShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(mut self, state: BrowseState) -> Self {
        self.state = state;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_item_count(mut self, item_count: usize) -> Self {
        self.item_count = Some(item_count);
        self
    }

    pub fn with_scroll_owner(mut self, scroll_owner: ScrollOwner) -> Self {
        self.scroll_owner = scroll_owner;
        self
    }

    pub fn is_stateful(&self) -> bool {
        self.state != BrowseState::Ready
    }

    pub fn differentiates_no_results(&self) -> bool {
        self.state == BrowseState::NoResults
    }

    pub fn viewport_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_PANEL
    }
}
