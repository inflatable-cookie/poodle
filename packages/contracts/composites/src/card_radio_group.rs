use poodle_primitives::ChoiceOption;
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardRadioGroupSpec {
    pub options: Vec<ChoiceOption>,
    pub default_value: Option<String>,
    pub value: Option<String>,
    pub is_disabled: bool,
}

impl CardRadioGroupSpec {
    pub fn new(options: Vec<ChoiceOption>) -> Self {
        Self {
            options,
            default_value: None,
            value: None,
            is_disabled: false,
        }
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
    }

    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn unselected_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }
}
