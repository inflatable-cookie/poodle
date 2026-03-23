use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutonomousListSpec {
    pub item_count: usize,
    pub max_items: Option<usize>,
    pub can_add: bool,
    pub can_remove: bool,
    pub can_reorder: bool,
    pub is_disabled: bool,
}

impl AutonomousListSpec {
    pub fn new() -> Self {
        Self {
            item_count: 0,
            max_items: None,
            can_add: true,
            can_remove: true,
            can_reorder: false,
            is_disabled: false,
        }
    }

    pub fn with_item_count(mut self, item_count: usize) -> Self {
        self.item_count = item_count;
        self
    }

    pub fn with_max_items(mut self, max_items: usize) -> Self {
        self.max_items = Some(max_items);
        self
    }

    pub fn with_can_add(mut self, can_add: bool) -> Self {
        self.can_add = can_add;
        self
    }

    pub fn with_can_remove(mut self, can_remove: bool) -> Self {
        self.can_remove = can_remove;
        self
    }

    pub fn with_can_reorder(mut self, can_reorder: bool) -> Self {
        self.can_reorder = can_reorder;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    pub fn is_at_capacity(&self) -> bool {
        match self.max_items {
            Some(max) => self.item_count >= max,
            None => false,
        }
    }
}
