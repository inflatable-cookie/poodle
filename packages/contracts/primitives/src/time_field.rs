use crate::types::ValidationState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeFieldSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: u32,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub described_by: Option<String>,
    pub validation_state: ValidationState,
}

impl Default for TimeFieldSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            min: None,
            max: None,
            step: 60,
            is_disabled: false,
            aria_label: None,
            described_by: None,
            validation_state: ValidationState::None,
        }
    }
}

impl TimeFieldSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_step(mut self, step: u32) -> Self {
        self.step = step;
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }
}
