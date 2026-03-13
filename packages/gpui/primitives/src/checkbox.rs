use pug_gpui_tokens::semantic;

use crate::types::CheckState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckboxSpec {
    pub checked: Option<bool>,
    pub default_checked: bool,
    pub is_mixed: bool,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
}

impl Default for CheckboxSpec {
    fn default() -> Self {
        Self {
            checked: None,
            default_checked: false,
            is_mixed: false,
            is_disabled: false,
            is_read_only: false,
            label: None,
            aria_label: None,
            description_id: None,
        }
    }
}

impl CheckboxSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn with_default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn with_mixed(mut self, is_mixed: bool) -> Self {
        self.is_mixed = is_mixed;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_read_only(mut self, is_read_only: bool) -> Self {
        self.is_read_only = is_read_only;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_description_id(mut self, description_id: impl Into<String>) -> Self {
        self.description_id = Some(description_id.into());
        self
    }

    pub fn current_state(&self) -> CheckState {
        if self.is_mixed {
            CheckState::Mixed
        } else if self.checked.unwrap_or(self.default_checked) {
            CheckState::Checked
        } else {
            CheckState::Unchecked
        }
    }

    pub fn is_interactive(&self) -> bool {
        !self.is_disabled && !self.is_read_only
    }

    pub fn indicator_fill_token(&self) -> &'static str {
        match self.current_state() {
            CheckState::Checked | CheckState::Mixed => semantic::COLOR_ACCENT_BASE,
            CheckState::Unchecked => semantic::COLOR_BACKGROUND_SURFACE,
        }
    }
}
