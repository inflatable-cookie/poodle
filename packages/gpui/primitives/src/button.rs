use pug_gpui_tokens::semantic;

use crate::types::{ButtonTone, ButtonVariant, ControlSize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonSpec {
    pub variant: ButtonVariant,
    pub tone: ButtonTone,
    pub size: ControlSize,
    pub is_disabled: bool,
    pub is_loading: bool,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub chevron: bool,
    pub aria_label: Option<String>,
    pub described_by: Option<String>,
    pub label: Option<String>,
}

impl Default for ButtonSpec {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            tone: ButtonTone::Default,
            size: ControlSize::Md,
            is_disabled: false,
            is_loading: false,
            leading_icon: None,
            trailing_icon: None,
            chevron: false,
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

    pub fn with_tone(mut self, tone: ButtonTone) -> Self {
        self.tone = tone;
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

    pub fn with_chevron(mut self, chevron: bool) -> Self {
        self.chevron = chevron;
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
        self.variant.fill_token(self.tone)
    }

    pub fn resolved_border_token(&self) -> &'static str {
        self.variant.border_token(self.tone)
    }

    pub fn resolved_text_token(&self) -> &'static str {
        self.variant.text_token(self.tone)
    }

    /// Base control height token — component applies size offset.
    pub fn control_height_token(&self) -> &'static str {
        semantic::SIZE_CONTROL_HEIGHT
    }

    /// Height offset in pixels from the base control height per contract §7.
    /// sm: -6px, md: 0, lg: +6px (0.375rem = 6px at 16px base)
    pub fn height_offset_px(&self) -> f32 {
        match self.size {
            ControlSize::Sm => -6.0,
            ControlSize::Md => 0.0,
            ControlSize::Lg => 6.0,
        }
    }

    /// Min-width in pixels per contract §8 size adjustments.
    /// sm: 68px (4.25rem), md: 80px (5rem), lg: 92px (5.75rem)
    pub fn min_width_px(&self) -> f32 {
        match self.size {
            ControlSize::Sm => 68.0,
            ControlSize::Md => 80.0,
            ControlSize::Lg => 92.0,
        }
    }

    /// Horizontal padding offset in pixels from base space-control-x per contract.
    /// sm: -2px, md: 0, lg: +2px (0.125rem = 2px at 16px base)
    pub fn padding_x_offset_px(&self) -> f32 {
        match self.size {
            ControlSize::Sm => -2.0,
            ControlSize::Md => 0.0,
            ControlSize::Lg => 2.0,
        }
    }

    /// Font size in pixels per contract §8 size adjustments.
    /// sm: 12px (0.75rem), md: 13px (typography-label-size = 0.8125rem), lg: 14px (0.875rem)
    pub fn font_size_px(&self) -> f32 {
        match self.size {
            ControlSize::Sm => 12.0,
            ControlSize::Md => 13.0,
            ControlSize::Lg => 14.0,
        }
    }

    /// Base padding token — component applies size offset.
    pub fn horizontal_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_X
    }

    pub fn vertical_padding_token(&self) -> &'static str {
        semantic::SPACE_CONTROL_Y
    }

    /// Icon size token — always sm in buttons per contract §9 Svelte Notes.
    pub fn icon_size_token(&self) -> &'static str {
        semantic::SIZE_ICON_SM
    }

    /// Icon wrapper size — always icon-md (16px) per contract §8 Icon wrapper.
    pub fn icon_wrapper_size_px(&self) -> f32 {
        16.0
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
