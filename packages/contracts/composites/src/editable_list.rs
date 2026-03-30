use poodle_tokens::semantic;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// EditableList -- an add/remove/reorder list with inline text entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditableListSpec {
    pub item_count: usize,
    pub add_label: String,
    pub placeholder: String,
    pub max_items: Option<usize>,
    pub is_disabled: bool,
    pub aria_label: String,
    pub is_reorderable: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl EditableListSpec {
    pub fn new() -> Self {
        Self {
            item_count: 0,
            add_label: String::from("Add item"),
            placeholder: String::from("New item"),
            max_items: None,
            is_disabled: false,
            aria_label: String::from("List"),
            is_reorderable: true,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_item_count(mut self, count: usize) -> Self {
        self.item_count = count;
        self
    }

    pub fn with_add_label(mut self, label: impl Into<String>) -> Self {
        self.add_label = label.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_max_items(mut self, max: usize) -> Self {
        self.max_items = Some(max);
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_reorderable(mut self, is_reorderable: bool) -> Self {
        self.is_reorderable = is_reorderable;
        self
    }

    /// Whether the add action is available (not disabled and under the maximum).
    pub fn can_add(&self) -> bool {
        if self.is_disabled {
            return false;
        }
        match self.max_items {
            Some(max) => self.item_count < max,
            None => true,
        }
    }

    /// Whether a max-items cap is configured and should display a counter.
    pub fn shows_counter(&self) -> bool {
        self.max_items.is_some()
    }

    pub fn input_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn input_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn input_focus_ring_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn remove_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn remove_hover_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    pub fn counter_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
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
