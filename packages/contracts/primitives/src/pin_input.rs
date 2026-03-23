use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PinInputSpec {
    pub length: usize,
    pub value: String,
    pub is_masked: bool,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for PinInputSpec {
    fn default() -> Self {
        Self {
            length: 6,
            value: String::new(),
            is_masked: false,
            is_disabled: false,
            aria_label: None,
        }
    }
}

impl PinInputSpec {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_masked(mut self, is_masked: bool) -> Self {
        self.is_masked = is_masked;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn filled_count(&self) -> usize {
        self.value.chars().count().min(self.length)
    }

    pub fn is_complete(&self) -> bool {
        self.value.chars().count() >= self.length
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}
