use crate::picker_shell::PickerShellSpec;
use crate::types::{BrowseState, PickerItemSpec, PickerVariant, SelectionMode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationPickerSpec {
    pub items: Vec<PickerItemSpec>,
    pub selected_ids: Vec<String>,
    pub query: String,
    pub selection_mode: SelectionMode,
    pub variant: PickerVariant,
    pub state: BrowseState,
}

impl RelationPickerSpec {
    pub fn new(items: Vec<PickerItemSpec>) -> Self {
        Self {
            items,
            selected_ids: Vec::new(),
            query: String::new(),
            selection_mode: SelectionMode::Multiple,
            variant: PickerVariant::Inline,
            state: BrowseState::Ready,
        }
    }

    pub fn with_selected_ids(mut self, selected_ids: Vec<String>) -> Self {
        self.selected_ids = selected_ids;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    pub fn with_selection_mode(mut self, selection_mode: SelectionMode) -> Self {
        self.selection_mode = selection_mode;
        self
    }

    pub fn with_variant(mut self, variant: PickerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_state(mut self, state: BrowseState) -> Self {
        self.state = state;
        self
    }

    pub fn selected_item_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| {
                self.selected_ids
                    .iter()
                    .any(|selected| selected == &item.id)
            })
            .count()
    }

    pub fn current_query(&self) -> &str {
        self.query.as_str()
    }

    pub fn as_picker_shell(&self, title: impl Into<String>) -> PickerShellSpec {
        PickerShellSpec::new(title)
            .with_variant(self.variant)
            .with_selection_mode(self.selection_mode)
            .with_state(self.state)
            .with_query(self.query.clone())
            .with_result_count(self.items.len())
            .with_selected_count(self.selected_item_count())
    }
}
