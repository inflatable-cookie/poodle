use flint_tokens::semantic;

use crate::types::ChoiceOption;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub options: Vec<ChoiceOption>,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub description_id: Option<String>,
    pub open: Option<bool>,
    pub default_open: bool,
}

impl Default for SelectSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            placeholder: None,
            options: Vec::new(),
            is_disabled: false,
            aria_label: None,
            description_id: None,
            open: None,
            default_open: false,
        }
    }
}

impl SelectSpec {
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

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn selected_option(&self) -> Option<&ChoiceOption> {
        let current = self.current_value()?;
        self.options.iter().find(|option| option.value == current)
    }

    pub fn trigger_text(&self) -> Option<&str> {
        self.selected_option()
            .map(|option| option.label.as_str())
            .or(self.placeholder.as_deref())
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
}
