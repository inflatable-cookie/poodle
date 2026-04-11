use crate::types::{ButtonTone, ButtonVariant, ControlDensity, ControlSize, OverlayPlacement, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IconButtonSpec {
    pub variant: ButtonVariant,
    /// Tone applied to the button fill/foreground. Mirrors
    /// `ButtonSpec.tone` — Default / Danger etc.
    pub tone: ButtonTone,
    pub size: ControlSize,
    pub icon: Option<String>,
    pub aria_label: Option<String>,
    pub is_disabled: bool,
    pub is_loading: bool,
    pub is_pressed: Option<bool>,
    /// Optional tooltip text shown on hover. Pairs with
    /// `tooltip_placement` for positioning. GPUI doesn't have a
    /// first-class tooltip primitive yet; this field is carried
    /// for consumer wiring.
    pub tooltip: Option<String>,
    pub tooltip_placement: OverlayPlacement,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for IconButtonSpec {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Ghost,
            tone: ButtonTone::Default,
            size: ControlSize::Md,
            icon: None,
            aria_label: None,
            is_disabled: false,
            is_loading: false,
            is_pressed: None,
            tooltip: None,
            tooltip_placement: OverlayPlacement::Bottom,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl IconButtonSpec {
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

    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn with_tooltip_placement(mut self, placement: OverlayPlacement) -> Self {
        self.tooltip_placement = placement;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
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

    pub fn with_pressed(mut self, is_pressed: bool) -> Self {
        self.is_pressed = Some(is_pressed);
        self
    }

    pub fn activation_allowed(&self) -> bool {
        !self.is_disabled && !self.is_loading
    }

    pub fn has_required_icon(&self) -> bool {
        self.icon
            .as_ref()
            .map(|icon| !icon.trim().is_empty())
            .unwrap_or(false)
    }

    pub fn has_required_accessible_name(&self) -> bool {
        self.aria_label
            .as_ref()
            .map(|label| !label.trim().is_empty())
            .unwrap_or(false)
    }

    pub fn uses_pressed_semantics(&self) -> bool {
        self.is_pressed.is_some()
    }

    pub fn icon_size_token(&self) -> &'static str {
        self.size.icon_size_token()
    }

    pub fn control_height_token(&self) -> &'static str {
        self.size.control_height_token()
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
