use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, PartialEq)]
pub struct NumberInputSpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for NumberInputSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            is_disabled: false,
            validation_state: ValidationState::None,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl NumberInputSpec {
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

    pub fn stepper_icon_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn body_size_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_SIZE
    }

    pub fn body_line_height_token(&self) -> &'static str {
        semantic::TYPOGRAPHY_BODY_LINE_HEIGHT
    }

    pub fn control_height_token(&self) -> &'static str {
        semantic::SIZE_CONTROL_HEIGHT
    }

    pub fn horizontal_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_X
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
