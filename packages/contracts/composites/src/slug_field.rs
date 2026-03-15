use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlugFieldSpec {
    pub value: String,
    pub source_value: Option<String>,
    pub prefix: Option<String>,
    pub is_locked: bool,
    pub is_disabled: bool,
}

impl SlugFieldSpec {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            source_value: None,
            prefix: None,
            is_locked: false,
            is_disabled: false,
        }
    }

    pub fn with_source_value(mut self, source_value: impl Into<String>) -> Self {
        self.source_value = Some(source_value.into());
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_locked(mut self, is_locked: bool) -> Self {
        self.is_locked = is_locked;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn display_value(&self) -> String {
        match &self.prefix {
            Some(prefix) => format!("{}{}", prefix, self.value),
            None => self.value.clone(),
        }
    }

    pub fn border_token(&self) -> &'static str {
        if self.is_locked {
            semantic::COLOR_BORDER_SUBTLE
        } else {
            semantic::COLOR_BORDER_DEFAULT
        }
    }

    pub fn text_color_token(&self) -> &'static str {
        if self.is_disabled {
            semantic::COLOR_TEXT_SECONDARY
        } else {
            semantic::COLOR_TEXT_PRIMARY
        }
    }
}
