use pug_gpui_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct PillSpec {
    pub label: String,
    pub is_removable: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
}

impl Default for PillSpec {
    fn default() -> Self {
        Self {
            label: String::new(),
            is_removable: false,
            is_selected: false,
            is_disabled: false,
        }
    }
}

impl PillSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_removable(mut self, is_removable: bool) -> Self {
        self.is_removable = is_removable;
        self
    }

    pub fn with_selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        if self.is_selected {
            semantic::COLOR_ACCENT_BASE
        } else {
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn text_color_token(&self) -> &'static str {
        if self.is_selected {
            semantic::COLOR_TEXT_INVERSE
        } else {
            semantic::COLOR_TEXT_PRIMARY
        }
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
