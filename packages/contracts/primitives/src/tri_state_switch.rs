use poodle_tokens::semantic;

use crate::types::{CheckState, ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TriStateSwitchSpec {
    pub state: CheckState,
    pub size: ControlSize,
    pub label: Option<String>,
    pub excluded_label: Option<String>,
    pub default_label: Option<String>,
    pub included_label: Option<String>,
    pub excluded_color: Option<String>,
    pub default_color: Option<String>,
    pub included_color: Option<String>,
    pub is_disabled: bool,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for TriStateSwitchSpec {
    fn default() -> Self {
        Self {
            state: CheckState::Unchecked,
            size: ControlSize::Md,
            label: None,
            excluded_label: None,
            default_label: None,
            included_label: None,
            excluded_color: None,
            default_color: None,
            included_color: None,
            is_disabled: false,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_excluded_label(mut self, label: impl Into<String>) -> Self {
        self.excluded_label = Some(label.into());
        self
    }

    pub fn with_default_label(mut self, label: impl Into<String>) -> Self {
        self.default_label = Some(label.into());
        self
    }

    pub fn with_included_label(mut self, label: impl Into<String>) -> Self {
        self.included_label = Some(label.into());
        self
    }

    pub fn with_excluded_color(mut self, color: impl Into<String>) -> Self {
        self.excluded_color = Some(color.into());
        self
    }

    pub fn with_default_color(mut self, color: impl Into<String>) -> Self {
        self.default_color = Some(color.into());
        self
    }

    pub fn with_included_color(mut self, color: impl Into<String>) -> Self {
        self.included_color = Some(color.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn excluded_label(&self) -> &str {
        self.excluded_label.as_deref().unwrap_or("Exclude")
    }

    pub fn default_label(&self) -> &str {
        self.default_label.as_deref().unwrap_or("Default")
    }

    pub fn included_label(&self) -> &str {
        self.included_label.as_deref().unwrap_or("Include")
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

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
