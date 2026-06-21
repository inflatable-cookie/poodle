use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

/// EditableList -- an add/remove/reorder list with inline text entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditableListSpec {
    pub item_count: usize,
    pub add_label: String,
    pub placeholder: String,
    pub max_items: Option<usize>,
    /// When true, show add-item input and remove buttons.
    pub is_editable: bool,
    /// Show remove buttons without enabling add input. Finer-grained
    /// than `is_editable` — allows deletion without insertion.
    pub is_removable: bool,
    pub is_disabled: bool,
    pub aria_label: String,
    pub is_reorderable: bool,
    /// When true, the list advertises unsaved changes via a visible
    /// dirty indicator (typically a dot beside the add row). The
    /// caller owns the dirty flag; this only drives the rendering.
    pub is_dirty: bool,
    /// Submitting / in-flight flag. When true, the add row is
    /// disabled and a "Saving…" status may surface next to the
    /// counter. Matches Svelte `submitting` prop.
    pub is_submitting: bool,
    /// Error banner rendered below the list. Typically the server
    /// error returned when the last commit failed.
    pub error_message: Option<String>,
    /// Informational banner rendered below the list (success /
    /// guidance copy).
    pub info_message: Option<String>,
    /// Show large-list guidance when item count exceeds threshold.
    pub long_list_threshold: Option<usize>,
    /// Custom large-list guidance copy; defaults to generated text
    /// when None.
    pub long_list_warning_text: Option<String>,
    /// Optional page window size for very large reorder sessions.
    pub window_size: Option<usize>,
    /// Submit button label when workflow chrome is shown.
    pub submit_label: String,
    /// Cancel button label when workflow chrome is shown.
    pub cancel_label: String,
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
            is_editable: false,
            is_removable: false,
            is_disabled: false,
            aria_label: String::from("List"),
            is_reorderable: true,
            is_dirty: false,
            is_submitting: false,
            error_message: None,
            info_message: None,
            long_list_threshold: Some(50),
            long_list_warning_text: None,
            window_size: None,
            submit_label: String::from("Save Order"),
            cancel_label: String::from("Cancel"),
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

    pub fn with_editable(mut self, is_editable: bool) -> Self {
        self.is_editable = is_editable;
        self
    }

    pub fn with_removable(mut self, is_removable: bool) -> Self {
        self.is_removable = is_removable;
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

    pub fn with_dirty(mut self, is_dirty: bool) -> Self {
        self.is_dirty = is_dirty;
        self
    }

    pub fn with_submitting(mut self, is_submitting: bool) -> Self {
        self.is_submitting = is_submitting;
        self
    }

    pub fn with_error_message(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    pub fn with_info_message(mut self, message: impl Into<String>) -> Self {
        self.info_message = Some(message.into());
        self
    }

    pub fn with_long_list_threshold(mut self, threshold: usize) -> Self {
        self.long_list_threshold = Some(threshold);
        self
    }

    pub fn with_long_list_warning_text(mut self, text: impl Into<String>) -> Self {
        self.long_list_warning_text = Some(text.into());
        self
    }

    pub fn with_window_size(mut self, size: usize) -> Self {
        self.window_size = Some(size);
        self
    }

    pub fn with_submit_label(mut self, label: impl Into<String>) -> Self {
        self.submit_label = label.into();
        self
    }

    pub fn with_cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    pub fn has_info(&self) -> bool {
        self.info_message.is_some()
    }

    pub fn error_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_DANGER
    }

    /// Info banner text color. Contract §8 Info table: `text-primary`.
    pub fn info_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn dirty_indicator_color_token(&self) -> &'static str {
        semantic::COLOR_STATUS_WARNING
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
