use poodle_tokens::semantic;

use crate::types::ValidationState;

#[derive(Clone, Debug, PartialEq)]
pub struct DurationInputSpec {
    pub value: Option<String>,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
    pub show_seconds: bool,
}

impl Default for DurationInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            is_disabled: false,
            validation_state: ValidationState::None,
            show_seconds: false,
        }
    }
}

impl DurationInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_validation_state(mut self, validation_state: ValidationState) -> Self {
        self.validation_state = validation_state;
        self
    }

    pub fn with_show_seconds(mut self, show_seconds: bool) -> Self {
        self.show_seconds = show_seconds;
        self
    }

    pub fn border_token(&self) -> &'static str {
        self.validation_state.border_token()
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn text_secondary_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
    }
}
