use poodle_tokens::semantic;

use crate::text_input::TextInputSpec;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchInputSpec {
    pub value: Option<String>,
    pub default_value: String,
    pub placeholder: String,
    pub aria_label: String,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub show_clear_button: bool,
    pub submit_enabled: bool,
    pub validation_state: ValidationState,
    pub described_by: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SearchInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: String::new(),
            placeholder: String::from("Search"),
            aria_label: String::from("Search"),
            is_disabled: false,
            is_read_only: false,
            show_clear_button: true,
            submit_enabled: true,
            validation_state: ValidationState::None,
            described_by: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SearchInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = default_value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
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

    pub fn with_show_clear_button(mut self, show_clear_button: bool) -> Self {
        self.show_clear_button = show_clear_button;
        self
    }

    pub fn with_submit_enabled(mut self, submit_enabled: bool) -> Self {
        self.submit_enabled = submit_enabled;
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_described_by(mut self, described_by: impl Into<String>) -> Self {
        self.described_by = Some(described_by.into());
        self
    }

    pub fn current_value(&self) -> &str {
        self.value.as_deref().unwrap_or(self.default_value.as_str())
    }

    pub fn shows_clear_action(&self) -> bool {
        self.show_clear_button && !self.current_value().is_empty()
    }

    pub fn search_icon_color_token(&self) -> &'static str {
        semantic::COLOR_ICON_MUTED
    }

    pub fn clear_action_icon_color_token(&self) -> &'static str {
        semantic::COLOR_ICON_PRIMARY
    }

    pub fn as_text_input_spec(&self) -> TextInputSpec {
        let mut input = TextInputSpec::new()
            .with_input_type("search")
            .with_placeholder(self.placeholder.clone())
            .with_aria_label(self.aria_label.clone())
            .with_submit_enabled(self.submit_enabled)
            .with_read_only(self.is_read_only)
            .with_disabled(self.is_disabled)
            .with_validation_state(self.validation_state)
            .with_leading_icon("search");

        if let Some(value) = &self.value {
            input = input.with_value(value.clone());
        } else if !self.default_value.is_empty() {
            input = input.with_default_value(self.default_value.clone());
        }

        if let Some(ref desc) = self.described_by {
            input = input.with_description_id(desc.clone());
        }

        if self.shows_clear_action() {
            input = input.with_trailing_icon("clear");
        }

        input
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
