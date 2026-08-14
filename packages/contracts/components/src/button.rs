//! Button — the primary interactive control.
//!
//! The portable declaration surface — struct, defaults, builders — is
//! generated from the conformance interface module
//! (`packages/core/src/conformance/button.ts`) into
//! [`crate::generated::button`] (regenerate with `effigy conformance:build`,
//! gated by `effigy conformance:check`). This module is the hand-written
//! extension beside the generated surface: token recipes and derived
//! queries. Portable props renamed or removed on the interface fail the
//! generator or this compile, never silently.
//!
//! Contract: `docs/contracts/components/button.md`
//! Ported from: `packages/jetstream/components/src/button.rs`

use poodle_tokens::semantic;

use crate::types::{ButtonTone, ButtonVariant};

pub use crate::generated::button::{ButtonFit, ButtonSpec};

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
