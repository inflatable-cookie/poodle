use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorPickerSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub is_open: bool,
    pub is_disabled: bool,
    pub show_alpha: bool,
}

impl Default for ColorPickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            is_open: false,
            is_disabled: false,
            show_alpha: false,
        }
    }
}

impl ColorPickerSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
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

    pub fn with_show_alpha(mut self, show_alpha: bool) -> Self {
        self.show_alpha = show_alpha;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
