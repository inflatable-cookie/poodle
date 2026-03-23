use flint_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct TimeZoneSelectSpec {
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub is_open: bool,
    pub is_disabled: bool,
}

impl Default for TimeZoneSelectSpec {
    fn default() -> Self {
        Self {
            value: None,
            placeholder: None,
            is_open: false,
            is_disabled: false,
        }
    }
}

impl TimeZoneSelectSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn trigger_text(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.placeholder.as_deref())
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
}
