use poodle_tokens::semantic;

use crate::types::CheckState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TriStateSwitchSpec {
    pub state: CheckState,
    pub label: Option<String>,
    pub is_disabled: bool,
}

impl Default for TriStateSwitchSpec {
    fn default() -> Self {
        Self {
            state: CheckState::Unchecked,
            label: None,
            is_disabled: false,
        }
    }
}

impl TriStateSwitchSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(mut self, state: CheckState) -> Self {
        self.state = state;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn aria_checked(&self) -> &'static str {
        self.state.aria_checked()
    }

    pub fn track_fill_token(&self) -> &'static str {
        match self.state {
            CheckState::Checked => semantic::COLOR_ACCENT_BASE,
            CheckState::Mixed => semantic::COLOR_ACCENT_HOVER,
            CheckState::Unchecked => semantic::COLOR_BACKGROUND_SURFACE,
        }
    }
}
