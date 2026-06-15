use crate::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardToggleOption {
    pub value: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl CardToggleOption {
    pub fn new(value: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CardToggleGroupSpec {
    pub options: Vec<CardToggleOption>,
    pub values: Vec<String>,
    pub disabled: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CardToggleGroupSpec {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            values: Vec::new(),
            disabled: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl CardToggleGroupSpec {
    pub fn new(options: Vec<CardToggleOption>) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    pub fn with_values(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_selected(&self, value: &str) -> bool {
        self.values.iter().any(|item| item == value)
    }
}
