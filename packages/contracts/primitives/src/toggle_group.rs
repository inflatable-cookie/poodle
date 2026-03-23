use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleGroupSelectionMode {
    Single,
    Multiple,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleGroupOption {
    pub value: String,
    pub label: String,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl ToggleGroupOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
            aria_label: None,
        }
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleGroupSpec {
    pub value: Option<Vec<String>>,
    pub default_value: Option<Vec<String>>,
    pub options: Vec<ToggleGroupOption>,
    pub selection_mode: ToggleGroupSelectionMode,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for ToggleGroupSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            selection_mode: ToggleGroupSelectionMode::Single,
            is_disabled: false,
            aria_label: None,
        }
    }
}

impl ToggleGroupSpec {
    pub fn new(options: Vec<ToggleGroupOption>) -> Self {
        Self { options, ..Self::default() }
    }

    pub fn with_value(mut self, value: Vec<String>) -> Self { self.value = Some(value); self }
    pub fn with_default_value(mut self, value: Vec<String>) -> Self { self.default_value = Some(value); self }
    pub fn with_selection_mode(mut self, mode: ToggleGroupSelectionMode) -> Self { self.selection_mode = mode; self }
    pub fn with_disabled(mut self, is_disabled: bool) -> Self { self.is_disabled = is_disabled; self }
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self { self.aria_label = Some(label.into()); self }

    pub fn selected_values(&self) -> &[String] {
        self.value.as_deref()
            .or(self.default_value.as_deref())
            .unwrap_or(&[])
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values().iter().any(|v| v == value)
    }

    pub fn item_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
