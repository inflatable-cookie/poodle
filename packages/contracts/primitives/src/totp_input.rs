use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

/// TotpInput -- a segmented one-time-password code entry field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TotpInputSpec {
    pub length: usize,
    pub value: Option<String>,
    pub default_value: String,
    pub name: String,
    pub label: String,
    pub hint: Option<String>,
    pub error: Option<String>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub autocomplete: String,
    pub validation_state: ValidationState,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for TotpInputSpec {
    fn default() -> Self {
        Self {
            length: 6,
            value: None,
            default_value: String::new(),
            name: String::from("code"),
            label: String::from("Authenticator code"),
            hint: None,
            error: None,
            is_disabled: false,
            aria_label: None,
            autocomplete: String::from("one-time-code"),
            validation_state: ValidationState::None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl TotpInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = default_value.into();
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = autocomplete.into();
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    /// Returns the effective validation state, promoting to Invalid when an error message is set.
    pub fn effective_validation_state(&self) -> ValidationState {
        if self.error.is_some() {
            ValidationState::Invalid
        } else {
            self.validation_state
        }
    }

    /// Whether the input is controlled (value explicitly provided).
    pub fn is_controlled(&self) -> bool {
        self.value.is_some()
    }

    /// The current effective value of the input.
    pub fn current_value(&self) -> &str {
        match &self.value {
            Some(v) => v.as_str(),
            None => self.default_value.as_str(),
        }
    }

    /// Whether the code has been fully entered (all digits filled).
    pub fn is_complete(&self) -> bool {
        let sanitized_len = self.current_value().chars().filter(|c| c.is_ascii_digit()).count();
        sanitized_len >= self.length
    }

    /// The effective accessible label for the input group.
    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(self.label.as_str())
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
