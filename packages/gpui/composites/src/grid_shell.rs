use crate::types::{BrowseState, MinColumnWidth, ScrollOwner};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridShellSpec {
    pub state: BrowseState,
    pub aria_label: Option<String>,
    pub item_count: Option<usize>,
    pub min_column_width: MinColumnWidth,
    pub scroll_owner: ScrollOwner,
}

impl Default for GridShellSpec {
    fn default() -> Self {
        Self {
            state: BrowseState::Ready,
            aria_label: None,
            item_count: None,
            min_column_width: MinColumnWidth::Md,
            scroll_owner: ScrollOwner::Content,
        }
    }
}

impl GridShellSpec {
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

    pub fn with_min_column_width(mut self, min_column_width: MinColumnWidth) -> Self {
        self.min_column_width = min_column_width;
        self
    }

    pub fn with_scroll_owner(mut self, scroll_owner: ScrollOwner) -> Self {
        self.scroll_owner = scroll_owner;
        self
    }

    pub fn differentiates_no_results(&self) -> bool {
        self.state == BrowseState::NoResults
    }
}
