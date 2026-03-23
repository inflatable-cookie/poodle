use poodle_tokens::semantic;

use crate::types::ValidationState;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberEntrySpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
    pub aria_label: Option<String>,
}

impl Default for NumberEntrySpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            is_disabled: false,
            validation_state: ValidationState::None,
            aria_label: None,
        }
    }
}

impl NumberEntrySpec {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_step(mut self, step: f64) -> Self {
        self.step = step;
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

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn clamped_value(&self) -> f64 {
        self.value.max(self.min).min(self.max)
    }

    pub fn border_token(&self) -> &'static str {
        self.validation_state.border_token()
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn stepper_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
}
