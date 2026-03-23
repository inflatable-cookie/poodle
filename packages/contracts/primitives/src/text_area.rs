use poodle_tokens::semantic;

use crate::types::ValidationState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextAreaSpec {
    pub value: Option<String>,
    pub default_value: String,
    pub placeholder: Option<String>,
    pub rows: u16,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub validation_state: ValidationState,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
    pub error_message_id: Option<String>,
    pub submit_enabled: bool,
    pub cancel_enabled: bool,
}

impl Default for TextAreaSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: String::new(),
            placeholder: None,
            rows: 4,
            is_disabled: false,
            is_read_only: false,
            validation_state: ValidationState::None,
            aria_label: None,
            description_id: None,
            error_message_id: None,
            submit_enabled: false,
            cancel_enabled: false,
        }
    }
}

impl TextAreaSpec {
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
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_rows(mut self, rows: u16) -> Self {
        self.rows = rows;
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

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
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

    pub fn with_error_message_id(mut self, error_message_id: impl Into<String>) -> Self {
        self.error_message_id = Some(error_message_id.into());
        self
    }

    pub fn with_submit_enabled(mut self, submit_enabled: bool) -> Self {
        self.submit_enabled = submit_enabled;
        self
    }

    pub fn with_cancel_enabled(mut self, cancel_enabled: bool) -> Self {
        self.cancel_enabled = cancel_enabled;
        self
    }

    pub fn is_controlled(&self) -> bool {
        self.value.is_some()
    }

    pub fn current_value(&self) -> &str {
        self.value.as_deref().unwrap_or(self.default_value.as_str())
    }

    pub fn described_by(&self) -> Option<String> {
        let ids = [
            self.description_id.clone(),
            match self.validation_state {
                ValidationState::Invalid => self.error_message_id.clone(),
                ValidationState::None | ValidationState::Valid | ValidationState::Pending => None,
            },
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if ids.is_empty() {
            None
        } else {
            Some(ids.join(" "))
        }
    }

    pub fn aria_invalid(&self) -> Option<&'static str> {
        self.validation_state.aria_invalid()
    }

    pub fn aria_busy(&self) -> Option<&'static str> {
        self.validation_state.aria_busy()
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        self.validation_state.border_token()
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
}
