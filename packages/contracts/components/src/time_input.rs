//! `TimeInputSpec` — spec for the `TimeInput` component.
//! Contract: `docs/contracts/components/time-input.md`.

use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeInputSpec {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: u32,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub described_by: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for TimeInputSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            min: None,
            max: None,
            step: 60,
            is_disabled: false,
            aria_label: None,
            described_by: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
        }
    }
}

impl TimeInputSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_step(mut self, step: u32) -> Self {
        self.step = step;
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value.as_deref().or(self.default_value.as_deref())
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        // Contract §8 + Svelte: the time input border is always the default
        // border color. There is no validation/invalid state in the contract
        // or in TimeInput.svelte, so the border never recolors.
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn placeholder_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
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
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}
