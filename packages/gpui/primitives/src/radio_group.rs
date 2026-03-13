use pug_gpui_tokens::semantic;

use crate::types::{ChoiceOption, Orientation};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioGroupSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub options: Vec<ChoiceOption>,
    pub orientation: Orientation,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
}

impl Default for RadioGroupSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            options: Vec::new(),
            orientation: Orientation::Vertical,
            is_disabled: false,
            aria_label: None,
            description_id: None,
        }
    }
}

impl RadioGroupSpec {
    pub fn new(options: Vec<ChoiceOption>) -> Self {
        Self {
            options,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn selected_option(&self) -> Option<&ChoiceOption> {
        let current = self.current_value()?;
        self.options.iter().find(|option| option.value == current)
    }

    pub fn option_gap_token(&self) -> &'static str {
        match self.orientation {
            Orientation::Horizontal => semantic::SPACE_INLINE_MD,
            Orientation::Vertical => semantic::SPACE_STACK_SM,
        }
    }
}
