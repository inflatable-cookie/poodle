use poodle_tokens::semantic;
use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Color input mode for the picker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorInputMode {
    Hex,
    Rgb,
    Hsl,
}

impl Default for ColorInputMode {
    fn default() -> Self {
        Self::Hex
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorPickerSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub swatches: Vec<String>,
    pub is_open: bool,
    pub is_disabled: bool,
    pub show_alpha: bool,
    pub show_input: bool,
    pub default_mode: ColorInputMode,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ColorPickerSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            swatches: Vec::new(),
            is_open: false,
            is_disabled: false,
            show_alpha: false,
            show_input: true,
            default_mode: ColorInputMode::Hex,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
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

    pub fn with_swatches(mut self, swatches: Vec<String>) -> Self {
        self.swatches = swatches;
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

    pub fn with_show_input(mut self, show_input: bool) -> Self {
        self.show_input = show_input;
        self
    }

    pub fn with_default_mode(mut self, mode: ColorInputMode) -> Self {
        self.default_mode = mode;
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

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn trigger_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn surface_radius_token(&self) -> &'static str {
        semantic::RADIUS_SURFACE
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
