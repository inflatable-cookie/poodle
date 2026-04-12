use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState};

#[derive(Clone, Debug, PartialEq)]
pub struct DurationInputSpec {
    pub value: Option<String>,
    pub is_disabled: bool,
    pub validation_state: ValidationState,
    pub show_seconds: bool,
    /// Maximum hours value for the hours segment. Defaults to 99.
    pub max_hours: u32,
    /// Minimum total duration in seconds. Zero means no minimum.
    pub min_total_seconds: u64,
    /// Maximum total duration in seconds. None means no maximum.
    pub max_total_seconds: Option<u64>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for DurationInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            is_disabled: false,
            validation_state: ValidationState::None,
            show_seconds: false,
            max_hours: 99,
            min_total_seconds: 0,
            max_total_seconds: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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

    pub fn with_max_hours(mut self, max_hours: u32) -> Self {
        self.max_hours = max_hours;
        self
    }

    pub fn with_min_total_seconds(mut self, min: u64) -> Self {
        self.min_total_seconds = min;
        self
    }

    pub fn with_max_total_seconds(mut self, max: u64) -> Self {
        self.max_total_seconds = Some(max);
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
