use pug_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwitchSpec {
    pub checked: Option<bool>,
    pub default_checked: bool,
    pub is_disabled: bool,
    pub is_read_only: bool,
    pub label: Option<String>,
    pub aria_label: Option<String>,
}

impl Default for SwitchSpec {
    fn default() -> Self {
        Self {
            checked: None,
            default_checked: false,
            is_disabled: false,
            is_read_only: false,
            label: None,
            aria_label: None,
        }
    }
}

impl SwitchSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn with_default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn current_checked(&self) -> bool {
        self.checked.unwrap_or(self.default_checked)
    }

    pub fn track_fill_token(&self) -> &'static str {
        if self.current_checked() {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }
}
