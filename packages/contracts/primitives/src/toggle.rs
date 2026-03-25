use poodle_tokens::semantic;

use crate::types::{ButtonVariant, ControlSize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleLayout {
    Inline,
    Stack,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleSpec {
    pub is_pressed: Option<bool>,
    pub default_pressed: bool,
    pub variant: ButtonVariant,
    pub size: ControlSize,
    pub layout: ToggleLayout,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub label: Option<String>,
    /// Icon name (e.g. "bold", "italic", "pin") rendered before label.
    pub icon: Option<String>,
}

impl Default for ToggleSpec {
    fn default() -> Self {
        Self {
            is_pressed: None,
            default_pressed: false,
            variant: ButtonVariant::Ghost,
            size: ControlSize::Md,
            layout: ToggleLayout::Inline,
            is_disabled: false,
            aria_label: None,
            label: None,
            icon: None,
        }
    }
}

impl ToggleSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pressed(mut self, pressed: bool) -> Self { self.is_pressed = Some(pressed); self }
    pub fn with_default_pressed(mut self, pressed: bool) -> Self { self.default_pressed = pressed; self }
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self { self.variant = variant; self }
    pub fn with_size(mut self, size: ControlSize) -> Self { self.size = size; self }
    pub fn with_layout(mut self, layout: ToggleLayout) -> Self { self.layout = layout; self }
    pub fn with_disabled(mut self, is_disabled: bool) -> Self { self.is_disabled = is_disabled; self }
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self { self.aria_label = Some(label.into()); self }
    pub fn with_label(mut self, label: impl Into<String>) -> Self { self.label = Some(label.into()); self }
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self { self.icon = Some(icon.into()); self }

    pub fn current_pressed(&self) -> bool {
        self.is_pressed.unwrap_or(self.default_pressed)
    }

    pub fn control_height_token(&self) -> &'static str {
        self.size.control_height_token()
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }
}
