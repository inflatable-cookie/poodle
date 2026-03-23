use poodle_primitives::ValidationState;
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineEditableFieldSpec {
    pub value: String,
    pub placeholder: Option<String>,
    pub is_editing: bool,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
}

impl InlineEditableFieldSpec {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            placeholder: None,
            is_editing: false,
            is_disabled: false,
            validation_state: ValidationState::None,
        }
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_editing(mut self, is_editing: bool) -> Self {
        self.is_editing = is_editing;
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
        if self.is_editing {
            match self.validation_state {
                ValidationState::Invalid => semantic::COLOR_STATUS_DANGER,
                ValidationState::Valid => semantic::COLOR_STATUS_SUCCESS,
                _ => semantic::COLOR_ACCENT_BASE,
            }
        } else {
            semantic::COLOR_BORDER_SUBTLE
        }
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }
}
