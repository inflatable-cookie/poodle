//! Button — the primary interactive control.
//!
//! This module is the single authority for the Button declaration surface:
//! the struct, its defaults and builders, then the token recipes and derived
//! queries beside them. `g14.001`–`g14.007` briefly generated the first half
//! from a TypeScript interface; `g14.008` rejected that path and `g14.021`
//! restored the hand-written declaration.
//!
//! Contract: `docs/contracts/components/button.md`
//! Ported from: `packages/jetstream/components/src/button.rs`

use poodle_tokens::semantic;

use crate::types::{ButtonTone, ButtonVariant};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum ButtonFit {
    #[default] Default,
    Content,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonSpec {
    pub variant: crate::types::ButtonVariant,
    pub tone: crate::types::ButtonTone,
    pub size: Option<crate::types::ControlSize>,
    pub size_role: crate::types::SemanticControlSizeRole,
    pub density: Option<crate::types::ControlDensity>,
    pub is_disabled: bool,
    pub is_loading: bool,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub chevron: bool,
    pub truncate: bool,
    pub fit: ButtonFit,
    pub max_width: Option<crate::types::Dimension>,
    pub pressed: Option<bool>,
    pub default_pressed: Option<bool>,
    pub label: Option<String>,
    pub aria_label: Option<String>,
    pub aria_expanded: Option<bool>,
    pub controls: Option<String>,
    pub described_by: Option<String>,
}

impl Default for ButtonSpec {
    fn default() -> Self {
        Self {
            variant: crate::types::ButtonVariant::Secondary,
            tone: crate::types::ButtonTone::Default,
            size: None,
            size_role: crate::types::SemanticControlSizeRole::Control,
            density: None,
            is_disabled: false,
            is_loading: false,
            leading_icon: None,
            trailing_icon: None,
            chevron: false,
            truncate: false,
            fit: ButtonFit::Default,
            max_width: None,
            pressed: None,
            default_pressed: None,
            label: None,
            aria_label: None,
            aria_expanded: None,
            controls: None,
            described_by: None,
        }
    }
}

impl ButtonSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, value: crate::types::ButtonVariant) -> Self {
        self.variant = value;
        self
    }
    pub fn with_tone(mut self, value: crate::types::ButtonTone) -> Self {
        self.tone = value;
        self
    }
    pub fn with_size(mut self, value: crate::types::ControlSize) -> Self {
        self.size = Some(value);
        self
    }
    pub fn with_size_role(mut self, value: crate::types::SemanticControlSizeRole) -> Self {
        self.size_role = value;
        self
    }
    pub fn with_density(mut self, value: crate::types::ControlDensity) -> Self {
        self.density = Some(value);
        self
    }
    pub fn with_disabled(mut self, value: bool) -> Self {
        self.is_disabled = value;
        self
    }
    pub fn with_loading(mut self, value: bool) -> Self {
        self.is_loading = value;
        self
    }
    pub fn with_leading_icon(mut self, value: impl Into<String>) -> Self {
        self.leading_icon = Some(value.into());
        self
    }
    pub fn with_trailing_icon(mut self, value: impl Into<String>) -> Self {
        self.trailing_icon = Some(value.into());
        self
    }
    pub fn with_chevron(mut self, value: bool) -> Self {
        self.chevron = value;
        self
    }
    pub fn with_truncate(mut self, value: bool) -> Self {
        self.truncate = value;
        self
    }
    pub fn with_fit(mut self, value: ButtonFit) -> Self {
        self.fit = value;
        self
    }
    pub fn with_max_width(mut self, value: impl Into<crate::types::Dimension>) -> Self {
        self.max_width = Some(value.into());
        self
    }
    pub fn with_pressed(mut self, value: bool) -> Self {
        self.pressed = Some(value);
        self
    }
    pub fn with_default_pressed(mut self, value: bool) -> Self {
        self.default_pressed = Some(value);
        self
    }
    pub fn with_label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }
    pub fn with_aria_label(mut self, value: impl Into<String>) -> Self {
        self.aria_label = Some(value.into());
        self
    }
    pub fn with_aria_expanded(mut self, value: bool) -> Self {
        self.aria_expanded = Some(value);
        self
    }
    pub fn with_controls(mut self, value: impl Into<String>) -> Self {
        self.controls = Some(value.into());
        self
    }
    pub fn with_described_by(mut self, value: impl Into<String>) -> Self {
        self.described_by = Some(value.into());
        self
    }
}


impl ButtonSpec {
    pub fn with_danger(mut self) -> Self {
        self.tone = ButtonTone::Danger;
        self
    }

    /// Clear disclosure state so `aria-expanded` is not surfaced.
    pub fn without_aria_expanded(mut self) -> Self {
        self.aria_expanded = None;
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

    /// Effective tone: Danger variant forces Danger tone.
    pub fn effective_tone(&self) -> ButtonTone {
        if self.variant == ButtonVariant::Danger {
            ButtonTone::Danger
        } else {
            self.tone
        }
    }

    pub fn resolved_fill_token(&self) -> &'static str {
        self.variant.fill_token(self.effective_tone())
    }

    pub fn resolved_border_token(&self) -> &'static str {
        self.variant.border_token(self.effective_tone())
    }

    pub fn resolved_text_token(&self) -> &'static str {
        self.variant.text_token(self.effective_tone())
    }

    pub fn control_height_token(&self) -> &'static str {
        self.size.unwrap_or_default().control_height_token()
    }

    pub fn control_min_width_token(&self) -> &'static str {
        self.size.unwrap_or_default().control_min_width_token()
    }

    /// Icon size token — always sm in buttons per contract.
    pub fn icon_size_token(&self) -> &'static str {
        semantic::SIZE_ICON_SM
    }

    /// Gap between label and icons (contract §8: `0.375rem`).
    pub fn content_gap_token() -> &'static str {
        semantic::SPACE_BUTTON_GAP
    }

    /// Padding reduction on the icon side (contract §8: `0.125rem`).
    pub fn icon_side_inset_token() -> &'static str {
        semantic::SPACE_BUTTON_ICON_INSET
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

    /// Returns true when button is in toggle mode (pressed or
    /// default_pressed explicitly provided — absence is `None` on both
    /// surfaces).
    pub fn is_toggle_mode(&self) -> bool {
        self.pressed.is_some() || self.default_pressed.is_some()
    }

    /// Current pressed state for toggle-mode buttons.
    pub fn current_pressed(&self) -> bool {
        self.pressed.or(self.default_pressed).unwrap_or(false)
    }
}
