use pug_gpui_tokens::semantic;

use crate::types::{ButtonVariant, ControlSize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonSpec {
    pub variant: ButtonVariant,
    pub size: ControlSize,
    pub is_disabled: bool,
    pub is_loading: bool,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub aria_label: Option<String>,
    pub described_by: Option<String>,
    pub label: Option<String>,
}

impl Default for ButtonSpec {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            size: ControlSize::Md,
            is_disabled: false,
            is_loading: false,
            leading_icon: None,
            trailing_icon: None,
            aria_label: None,
            described_by: None,
            label: None,
        }
    }
}

impl ButtonSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn with_leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn with_trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_described_by(mut self, described_by: impl Into<String>) -> Self {
        self.described_by = Some(described_by.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn activation_allowed(&self) -> bool {
        !self.is_disabled && !self.is_loading
    }

    pub fn requires_aria_label(&self) -> bool {
        self.label
            .as_ref()
            .map(|label| label.trim().is_empty())
            .unwrap_or(true)
    }

    pub fn resolved_fill_token(&self) -> &'static str {
        self.variant.fill_token()
    }

    pub fn resolved_border_token(&self) -> &'static str {
        self.variant.border_token()
    }

    pub fn resolved_text_token(&self) -> &'static str {
        self.variant.text_token()
    }

    pub fn control_height_token(&self) -> &'static str {
        self.size.control_height_token()
    }

    pub fn control_min_width_token(&self) -> &'static str {
        self.size.control_min_width_token()
    }

    pub fn icon_size_token(&self) -> &'static str {
        self.size.icon_size_token()
    }

    pub fn horizontal_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_X
    }

    pub fn vertical_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_Y
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn focus_ring_width_token(&self) -> &'static str {
        semantic::BORDER_WIDTH_FOCUS
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }
}
