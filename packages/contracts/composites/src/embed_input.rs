use poodle_primitives::ValidationState;
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbedInputSpec {
    pub value: String,
    pub placeholder: Option<String>,
    pub is_loading: bool,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
}

impl EmbedInputSpec {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            is_loading: false,
            is_disabled: false,
            validation_state: ValidationState::None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn border_token(&self) -> &'static str {
        match self.validation_state {
            ValidationState::Invalid => semantic::COLOR_STATUS_DANGER,
            ValidationState::Valid => semantic::COLOR_STATUS_SUCCESS,
            ValidationState::Pending => semantic::COLOR_STATUS_WARNING,
            _ => semantic::COLOR_BORDER_DEFAULT,
        }
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }
}
