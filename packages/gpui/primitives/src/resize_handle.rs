use pug_gpui_tokens::semantic;

use crate::types::Orientation;

/// Spec for the ResizeHandle component — a draggable divider for resizable panels.
#[derive(Clone, Debug, PartialEq)]
pub struct ResizeHandleSpec {
    pub orientation: Orientation,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub aria_value_now: Option<f32>,
    pub aria_value_min: f32,
    pub aria_value_max: f32,
}

impl Default for ResizeHandleSpec {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
            aria_value_now: None,
            aria_value_min: 0.0,
            aria_value_max: 100.0,
        }
    }
}

impl ResizeHandleSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_aria_value_now(mut self, value: f32) -> Self {
        self.aria_value_now = Some(value);
        self
    }

    pub fn with_aria_value_min(mut self, value: f32) -> Self {
        self.aria_value_min = value;
        self
    }

    pub fn with_aria_value_max(mut self, value: f32) -> Self {
        self.aria_value_max = value;
        self
    }

    pub fn border_color_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn hover_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    pub fn role(&self) -> &'static str {
        "separator"
    }

    pub fn aria_orientation(&self) -> &'static str {
        match self.orientation {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or("Resize")
    }

    pub fn is_focusable(&self) -> bool {
        !self.is_disabled
    }
}
