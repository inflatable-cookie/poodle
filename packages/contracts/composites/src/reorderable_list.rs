use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReorderableListSpec {
    pub item_count: usize,
    pub is_disabled: bool,
    pub active_drag_index: Option<usize>,
}

impl ReorderableListSpec {
    pub fn new() -> Self {
        Self {
            item_count: 0,
            is_disabled: false,
            active_drag_index: None,
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
}
