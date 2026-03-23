use poodle_tokens::semantic;

use crate::types::ChoiceOption;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentedControlSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub options: Vec<ChoiceOption>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for SegmentedControlSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            is_disabled: false,
            aria_label: None,
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

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn selected_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
}
