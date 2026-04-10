use poodle_tokens::semantic;

use crate::types::{ChoiceOption, ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentedControlSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub options: Vec<ChoiceOption>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    /// When true, every segment takes equal horizontal space instead
    /// of sizing to its label content. Matches the Svelte
    /// `equalWidth` prop.
    pub equal_width: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SegmentedControlSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            is_disabled: false,
            aria_label: None,
            equal_width: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SegmentedControlSpec {
    pub fn new(options: Vec<ChoiceOption>) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_equal_width(mut self, equal_width: bool) -> Self {
        self.equal_width = equal_width;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
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
