use poodle_tokens::semantic;

use crate::types::{
    ButtonTone, ButtonVariant, ControlDensity, ControlSize, Dimension, SemanticControlSizeRole,
};

/// Whether a button holds its default minimum width or shrinks to its content.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ButtonFit {
    #[default]
    Default,
    Content,
}

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
    /// Disclosure state for menu / accordion triggers. Mirrors web `ariaExpanded`.
    /// `None` omits the attribute (non-disclosure buttons).
    pub aria_expanded: Option<bool>,
    pub label: Option<String>,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Controlled toggle state. When non-null, button acts as a toggle with
    /// `aria-pressed`. Non-primary variants get accent fill when pressed.
    pub pressed: Option<bool>,
    /// Initial pressed state for uncontrolled toggle mode.
    pub default_pressed: bool,
    /// Whether the button holds its default minimum width or shrinks to its
    /// content.
    pub fit: ButtonFit,
    /// Whether an overlong label ellipsises instead of growing the button.
    pub truncate: bool,
    pub max_width: Option<Dimension>,
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
            aria_expanded: None,
            label: None,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            pressed: None,
            default_pressed: false,
            fit: ButtonFit::Default,
            truncate: false,
            max_width: None,
        }
    }
}

impl ButtonSpec {
    pub fn with_fit(mut self, fit: ButtonFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn with_truncate(mut self, truncates: bool) -> Self {
        self.truncate = truncates;
        self
    }

    pub fn with_max_width(mut self, width: impl Into<Dimension>) -> Self {
        self.max_width = Some(width.into());
        self
    }

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

    pub fn with_tone(mut self, tone: ButtonTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_danger(mut self) -> Self {
        self.tone = ButtonTone::Danger;
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

    pub fn with_aria_expanded(mut self, expanded: bool) -> Self {
        self.aria_expanded = Some(expanded);
        self
    }

    /// Clear disclosure state so `aria-expanded` is not surfaced.
    pub fn without_aria_expanded(mut self) -> Self {
        self.aria_expanded = None;
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
        self.size.control_height_token()
    }

    pub fn control_min_width_token(&self) -> &'static str {
        self.size.control_min_width_token()
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

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

    /// Set the controlled toggle pressed state.
    pub fn with_pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    /// Set the initial uncontrolled pressed state.
    pub fn with_default_pressed(mut self, default_pressed: bool) -> Self {
        self.default_pressed = default_pressed;
        self
    }

    /// Returns true when button is in toggle mode (pressed non-null or default_pressed set).
    pub fn is_toggle_mode(&self) -> bool {
        self.pressed.is_some() || self.default_pressed
    }

    /// Current pressed state for toggle-mode buttons.
    pub fn current_pressed(&self) -> bool {
        self.pressed.unwrap_or(self.default_pressed)
    }
}
