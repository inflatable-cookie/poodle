use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditableLabelSpec {
    pub value: String,
    pub placeholder: Option<String>,
    pub is_editing: bool,
    pub is_disabled: bool,
}

impl Default for EditableLabelSpec {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: None,
            is_editing: false,
            is_disabled: false,
        }
    }
}

impl EditableLabelSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_editing(mut self, is_editing: bool) -> Self {
        self.is_editing = is_editing;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn edit_border_token(&self) -> &'static str {
        if self.is_editing {
            semantic::COLOR_ACCENT_FOCUS_RING
        } else {
            semantic::COLOR_BORDER_SUBTLE
        }
    }

    pub fn placeholder_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn fill_token(&self) -> &'static str {
        if self.is_editing {
            semantic::COLOR_BACKGROUND_SURFACE
        } else {
            // transparent in display mode
            semantic::COLOR_BACKGROUND_SURFACE
        }
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}
